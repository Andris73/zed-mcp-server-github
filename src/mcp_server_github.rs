use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const REPO_NAME: &str = "github/github-mcp-server";
const BINARY_NAME: &str = "github-mcp-server";

#[derive(Debug, Deserialize, JsonSchema)]
struct GitHubContextServerSettings {
    github_personal_access_token: String,
    github_host: Option<String>,
    toolsets: Option<String>,
    tools: Option<String>,
    read_only: Option<bool>,
    dynamic_toolsets: Option<bool>,
    insiders: Option<bool>,
    lockdown_mode: Option<bool>,
    pre_release: Option<bool>,
}

fn push_env_if_set(env: &mut Vec<(String, String)>, key: &str, value: &Option<String>) {
    if let Some(v) = value.as_ref().filter(|s| !s.trim().is_empty()) {
        env.push((key.into(), v.clone()));
    }
}

fn push_env_if_true(env: &mut Vec<(String, String)>, key: &str, value: Option<bool>, flag: &str) {
    if value == Some(true) {
        env.push((key.into(), flag.into()));
    }
}

struct GitHubModelContextExtension {
    cached_binary_path: Option<String>,
}

impl GitHubModelContextExtension {
    fn context_server_binary_path(
        &mut self,
        _context_server_id: &ContextServerId,
        pre_release: bool,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        let release = zed::latest_github_release(
            REPO_NAME,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "{BINARY_NAME}_{os}_{arch}.{ext}",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "i386",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "Darwin",
                zed::Os::Linux => "Linux",
                zed::Os::Windows => "Windows",
            },
            ext = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;
        let binary_path = format!(
            "{version_dir}/{BINARY_NAME}{suffix}",
            suffix = match platform {
                zed::Os::Windows => ".exe",
                _ => "",
            }
        );

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            let file_kind = match platform {
                zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                zed::Os::Windows => zed::DownloadedFileType::Zip,
            };

            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            // Remove old versions (only directories matching our binary name prefix)
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str().map_or(false, |name| {
                    name.starts_with(BINARY_NAME) && name != version_dir
                }) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for GitHubModelContextExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("mcp-server-github", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `github_personal_access_token` setting".into());
        };
        let settings: GitHubContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        // Validate the personal access token
        let pat = settings.github_personal_access_token.trim();
        if pat.is_empty() {
            return Err("github_personal_access_token is empty — please provide a valid token".into());
        }
        if pat == "GITHUB_PERSONAL_ACCESS_TOKEN" {
            return Err(
                "github_personal_access_token still contains the placeholder value — \
                 please replace it with your actual GitHub personal access token"
                    .into(),
            );
        }

        let pre_release = settings.pre_release.unwrap_or(false);

        let mut env: Vec<(String, String)> = vec![(
            "GITHUB_PERSONAL_ACCESS_TOKEN".into(),
            settings.github_personal_access_token.clone(),
        )];

        // String-valued environment variables
        push_env_if_set(&mut env, "GITHUB_HOST", &settings.github_host);
        push_env_if_set(&mut env, "GITHUB_TOOLSETS", &settings.toolsets);
        push_env_if_set(&mut env, "GITHUB_TOOLS", &settings.tools);

        // Boolean-valued environment variables
        push_env_if_true(&mut env, "GITHUB_READ_ONLY", settings.read_only, "1");
        push_env_if_true(&mut env, "GITHUB_DYNAMIC_TOOLSETS", settings.dynamic_toolsets, "1");
        push_env_if_true(&mut env, "GITHUB_INSIDERS", settings.insiders, "true");
        push_env_if_true(&mut env, "GITHUB_LOCKDOWN_MODE", settings.lockdown_mode, "1");

        Ok(Command {
            command: self.context_server_binary_path(context_server_id, pre_release)?,
            args: vec!["stdio".to_string()],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(GitHubContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(GitHubModelContextExtension);