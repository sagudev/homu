use db::DB;
mod cli;
mod config;
mod db;
mod hardcoded;

static DB: DB = DB::empty();

#[derive(Debug)]
struct Repository {
    treeclosed: i32,
    treeclosed_src: Option<String>,
    gh: String,
    gh_test_on_fork: Option<config::TestOnFork>,
    label: Option<String>,
    repo_label: String,
}

impl PartialOrd for Repository {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.gh.partial_cmp(&other.gh)
    }
}

impl PartialEq for Repository {
    fn eq(&self, other: &Self) -> bool {
        self.gh == other.gh
    }
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            treeclosed: -1,
            treeclosed_src: Default::default(),
            gh: Default::default(),
            gh_test_on_fork: Default::default(),
            label: Default::default(),
            repo_label: Default::default(),
        }
    }
}

impl Repository {
    async fn new(gh: String, repo_label: String) -> Self {
        let (treeclosed, treeclosed_src) = if let Ok(repo) = sqlx::query_as::<_, db::Repo>(
            "SELECT treeclosed, treeclosed_src FROM repos WHERE repo = ?",
        )
        .bind(&repo_label)
        .fetch_one(&mut *DB.db().await)
        .await
        {
            (repo.treeclosed, repo.treeclosed_src)
        } else {
            (-1, None)
        };

        Self {
            gh,
            treeclosed,
            treeclosed_src,
            repo_label,
            ..Default::default()
        }
    }

    async fn update_treeclosed(&mut self, value: i32, src: String) {
        self.treeclosed = value;
        self.treeclosed_src = Some(src);

        let db = &mut *DB.db().await;

        sqlx::query("DELETE FROM repos where repo = ?")
            .bind(&self.repo_label)
            .execute(&mut *db)
            .await
            .unwrap();

        if value > 0 {
            sqlx::query(
                "INSERT INTO repos (repo, treeclosed, treeclosed_src)
            VALUES (?, ?, ?)",
            )
            .bind(&self.repo_label)
            .bind(value)
            .bind(self.treeclosed_src.as_ref().unwrap())
            .execute(&mut *db)
            .await
            .unwrap();
        }
    }
}

enum AuthState {
    // Higher is more privileged
    Reviewer = 3,
    Try = 2,
    None = 1,
}

#[derive(Debug)]
enum LabelEvent {
    Approved,
    Rejected,
    Conflict,
    Succeed,
    Failed,
    Try,
    TrySucceed,
    TryFailed,
    Exempted,
    TimedOut,
    Interrupted,
    Pushed,
}

pub const PORTAL_TURRET_DIALOG: [&str; 3] = ["Target acquired", "Activated", "There you are"];
pub const PORTAL_TURRET_IMAGE: &str = "https://cloud.githubusercontent.com/assets/1617736/22222924/c07b2a1c-e16d-11e6-91b3-ac659550585c.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse arguments
    let args = cli::Cli::arguments();
    // set logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or({
        if args.verbose {
            "debug"
        } else {
            "info"
        }
    }))
    .init();
    // parse config
    let mut cfg = config::config(args.config)?;
    // login into github and use this information for some aditional post processing on config
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(cfg.github.access_token)
        .build()?;

    // now we need async
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let user = octocrab.current().user().await?;
            if cfg.git.user.is_none() {
                cfg.git.user = Some(user.login);
            }
            if cfg.git.email.is_none() {
                //cfg.git.email = Some(octa);
            }

            DB.init(cfg.db.file).await?;

            DB.create().await?;

            Ok::<_, Box<dyn std::error::Error>>(())
        })
}
