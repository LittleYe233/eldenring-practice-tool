use hudhook::tracing::info;
use pkg_version::*;
use semver::*;

const PRACTICE_TOOL_VERSION: Version = Version {
    major: pkg_version_major!(),
    minor: pkg_version_minor!(),
    patch: pkg_version_patch!(),
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
};

const UPDATE_URL: &str =
    "https://api.github.com/repos/LittleYe233/eldenring-practice-tool/releases/latest";

pub enum Update {
    Available { url: String, notes: String },
    UpToDate,
    Error(String),
}

impl Update {
    pub fn check() -> Self {
        info!("正在检查更新...");
        #[derive(serde::Deserialize)]
        struct GithubRelease {
            tag_name: String, // looks like "vX.Y.Z-zhCN-withCER"
            html_url: String,
            body: String,
        }

        let mut release = match ureq::get(UPDATE_URL).call() {
            Ok(release) => release,
            Err(e) => return Update::Error(e.to_string()),
        };

        let release = match release.body_mut().read_json::<GithubRelease>() {
            Ok(release) => release,
            Err(e) => return Update::Error(e.to_string()),
        };

        // Parse tag name
        if !release.tag_name.starts_with("v") || !release.tag_name.ends_with("-zhCN-withCER") {
            return Update::Error(format!(
                "当前最新版本并不符合此分支版本 (实际: {})",
                release.tag_name
            ));
        }
        let parsed_tag_name =
            release.tag_name.strip_prefix("v").unwrap().strip_suffix("-zhCN-withCER").unwrap();

        let version = match Version::parse(parsed_tag_name) {
            Ok(version) => version,
            Err(e) => return Update::Error(e.to_string()),
        };

        if version > PRACTICE_TOOL_VERSION {
            let notes = release.body;
            let notes = format!(
                "发现有新版练习工具！\n\n最新版本:    {version}\n已安装版本: \
                 {PRACTICE_TOOL_VERSION}\n\n更新内容:\n{notes}\n",
            );

            let url = release.html_url;

            Update::Available { url, notes }
        } else {
            Update::UpToDate
        }
    }
}
