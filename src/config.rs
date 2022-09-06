use std::{
    collections::HashMap,
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

use regex::Regex;
use serde::Deserialize;
use toml::Value;

fn retry_log_expire() -> String {
    String::from("-42 days")
}

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Priority values above max_priority will be refused.
    pub max_priority: i32,
    /// How long to keep the retry log
    ///
    /// Should be a negative interval of time recognized by SQLite3.
    #[serde(default = "retry_log_expire")]
    pub retry_log_expire: String,
    /* Sections */
    /// DataBase section
    pub db: Database,
    /// Web section
    pub web: Web,
    /// Git section
    pub git: Git,
    /// Github section
    pub github: Github,
    /// Repo section(s)
    pub repo: HashMap<String, Repo>,
}

fn db_file() -> PathBuf {
    PathBuf::from("main.db")
}

/// The database homu uses
#[derive(Debug, Deserialize)]
pub struct Database {
    /// SQLite file
    #[serde(default = "db_file")]
    pub file: PathBuf,
}

const fn ip() -> Ipv4Addr {
    Ipv4Addr::new(0, 0, 0, 0)
}

/// The database homu uses
#[derive(Debug, Deserialize)]
pub struct Web {
    #[serde(default = "ip")]
    host: Ipv4Addr,
    /// The port homu listens on.
    port: i32,
    /// Synchronize all open PRs on startup. "Synchronize" means fetch the state of
    /// all open PRs.
    #[serde(default)]
    sync_on_start: bool,
    base_url: Option<String>,
    canonical_url: Option<String>,
    #[serde(default)]
    remove_path_prefixes: Vec<String>,
    announcement: Option<String>,
}

fn cache_dir() -> PathBuf {
    PathBuf::from("cache")
}

#[derive(Debug, Deserialize)]
pub struct Git {
    ///Use the local Git command. Required to use some advanced features. It also
    /// speeds up Travis by reducing temporary commits.
    #[serde(default)]
    pub local_git: bool,
    /// Directory storing the local clones of the git repositories. If this is on an
    /// ephemeral file system, there will be a delay to start new builds after a
    /// restart while homu clones the repository.
    #[serde(default = "cache_dir")]
    pub cache_dir: PathBuf,
    /// SSH private key. Needed only when the local Git command is used.
    pub ssh_key: Option<String>,
    /// Git name for commits
    ///
    /// By default, Homu extracts the name+email from the Github account it will be using.
    pub user: Option<String>,
    /// Git email for commits
    ///
    /// By default, Homu extracts the name+email from the Github account it will be using.
    pub email: Option<String>,
}

/// Information for securely interacting with GitHub. These are found/generated
/// under <https://github.com/settings/applications>.
#[derive(Debug, Deserialize)]
pub struct Github {
    /// A GitHub personal access token.
    pub access_token: String,
    /// A GitHub oauth application id for this instance of homu
    pub app_client_id: String,
    /// A GitHub oauth application secret for this instance of homu
    pub app_client_secret: String,
}

/// GitHub username
pub type User = String;

/// returns 10h in seconds
const fn ten_hours() -> i32 {
    10 * 60 * 60
}

/// The database homu uses
#[derive(Debug, Deserialize)]
pub struct Repo {
    /// Repo owner
    ///
    /// You can get this field from github.com/<owner>/<name>
    owner: User,
    /// Repo name
    ///
    /// You can get this field from github.com/<owner>/<name>
    name: String,
    #[serde(default)]
    /// If this repo should be integrated with the permissions defined in
    /// https://github.com/rust-lang/team uncomment the following line.
    /// Note that the other ACLs will *also* apply.
    rust_team: bool,
    /// Who can approve PRs (r+ rights)?
    reviewers: Vec<User>,
    /// Alternatively, set this  allow any github collaborator;
    /// note that you can *also* specify reviewers above.
    #[serde(default)]
    auth_collaborators: bool,
    /// Who has 'try' rights? (try, retry, force, clean, prioritization).
    try_users: Vec<User>,
    /// Keep the commit history linear. Requires the local Git command.
    #[serde(default)]
    linera: bool,
    /// Auto-squash commits. Requires the local Git command.
    #[serde(default)]
    autosquash: bool,
    /// If the PR already has the same success statuses that we expect on the `auto`
    /// branch, then push directly to branch if safe to do so. Requires the local Git
    /// command.
    #[serde(default)]
    status_based_exemption: bool,
    /// Maximum test duration allowed for testing a PR in this repository.
    ///
    /// Default to 10 hours.
    #[serde(default = "ten_hours")]
    timeout: i32,
    /// Branch names
    #[serde(default)]
    branch: RepoBranch,
    /// test-on-fork allows you to run the CI builds for a project in a separate fork
    /// instead of the main repository, while still approving PRs and merging the
    /// commits in the main one.
    test_on_fork: Option<TestOnFork>,
    github: RepoGithub,
    #[serde(default)]
    labels: Labels,
    /// this is chaotic
    checks: Value,
    /// this is chaotic
    status: Option<Value>,
    buildbot: Option<BuildBot>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Labels {
    /// after homu received `r+`
    approved: Option<Label>,
    /// after homu received `r-`
    rejected: Option<Label>,
    /// a merge conflict is detected
    conflict: Option<Label>,
    /// test successful
    succeed: Option<Label>,
    /// test failed
    failed: Option<Label>,
    /// test exempted
    exempted: Option<Label>,
    /// test timed out (after 10 hours)
    timed_out: Option<Label>,
    /// test interrupted (buildbot only)
    interrupted: Option<Label>,
    /// after homu received `try`
    r#try: Option<Label>,
    /// try-build successful
    try_succeed: Option<Label>,
    /// try-build failed
    try_failed: Option<Label>,
    /// user pushed a commit after `r+`/`try`
    pushed: Option<Label>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Label {
    /// list of labels to add
    #[serde(default)]
    add: Vec<String>,
    /// list of labels to remove
    #[serde(default)]
    remove: Vec<String>,
    /// avoid relabeling if any of these labels are present
    #[serde(default)]
    unless: Vec<String>,
}

const fn r#true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct BuildBot {
    url: String,
    secret: String,
    #[serde(default)]
    builders: Vec<String>,
    #[serde(default)]
    try_builders: Vec<String>,
    username: String,
    password: String,
    /// Boolean which indicates whether the builder is included in try builds (defaults to true)
    #[serde(default = "r#true")]
    r#try: bool,
}

#[derive(Debug, Deserialize)]
pub struct RepoGithub {
    /// Arbitrary secret. You can generate one with: openssl rand -hex 20
    secret: String,
}

fn auto() -> String {
    String::from("auto")
}

fn r#try() -> String {
    String::from("try")
}

#[derive(Debug, Deserialize)]
pub struct RepoBranch {
    /// auto branch name
    #[serde(default = "auto")]
    auto: String,
    /// try branch name
    #[serde(default = "r#try")]
    r#try: String,
}

impl Default for RepoBranch {
    fn default() -> Self {
        Self {
            auto: String::from("auto"),
            r#try: String::from("try"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TestOnFork {
    owner: User,
    name: String,
}

/// This function replaces ${VAR_PLACEHOLDER} with real env values
fn pre_process_config(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut res = s.to_owned();
    for m in Regex::new(r"\$\{([a-zA-Z_]+)\}").unwrap().find_iter(s) {
        let var = &m.as_str()[2..m.as_str().len() - 1];
        println!("{}", var);
        if res.contains(m.as_str()) {
            res = res.replace(m.as_str(), &std::env::var(var)?)
        }
    }
    Ok(res)
}

/// This function
pub fn config(path: Option<PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {
    match std::fs::read_to_string(path.as_ref().unwrap_or(&PathBuf::from("cfg.toml"))) {
        Ok(s) => toml::from_str(&pre_process_config(&s)?).map_err(Into::into),
        Err(e) => {
            // Fall back to cfg.json only if we're using the defaults
            if matches!(e.kind(), std::io::ErrorKind::NotFound) && path.is_none() {
                serde_json::from_str(&pre_process_config(&std::fs::read_to_string("cfg.json")?)?)
                    .map_err(Into::into)
            } else {
                Err(e.into())
            }
        }
    }
}

#[test]
fn sample() {
    let s = std::fs::read_to_string("./cfg.sample.toml").unwrap();
    let c: toml::Value = toml::from_str(&s).unwrap();
    println!("{:#?}", c);
}

#[test]
fn production() {
    let s = std::fs::read_to_string("./cfg.production.toml").unwrap();
    let c: toml::Value = toml::from_str(&pre_process_config(&s).unwrap()).unwrap();
    //println!("{:#?}", c);
}

#[test]
fn t_sample() {
    let s = std::fs::read_to_string("./cfg.sample.toml").unwrap();
    let c: Config = toml::from_str(&s).unwrap();
    println!("{:#?}", c);
}

#[test]
fn t_production() {
    let s = std::fs::read_to_string("./cfg.production.toml").unwrap();
    let c: Config = toml::from_str(&s).unwrap();
    println!("{:#?}", c);
}
