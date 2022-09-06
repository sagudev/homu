use std::sync::{Arc, RwLock};

use db::DB;
use diesel::QueryDsl;
use once_cell::sync::OnceCell;

mod cli;
mod config;
mod db;

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
    fn new(gh: String, repo_label: String) -> Self {
        use db::schema::repos::dsl as repos;
        use diesel::prelude::*;

        let (treeclosed, treeclosed_src) = if let Ok(x) = repos::repos
            .filter(repos::repo.eq(&repo_label))
            .select((repos::treeclosed, repos::treeclosed_src))
            .first::<(i32, Option<String>)>(&mut *DB.db())
        {
            x
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

    fn update_treeclosed(&mut self, value: i32, src: String) {
        use db::schema::repos::dsl as repos;
        use diesel::prelude::*;

        self.treeclosed = value;
        self.treeclosed_src = Some(src);

        let db = &mut *DB.db();

        diesel::delete(repos::repos.filter(repos::repo.eq(&self.repo_label)))
            .execute(db)
            .unwrap();

        if value > 0 {
            diesel::insert_into(repos::repos)
                .values((self.repo_label, value, src))
                .execute(db)
                .unwrap();
        }
    }
}

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

            DB.init(cfg.db.file)?;

            DB.create()?;

            Ok::<_, Box<dyn std::error::Error>>(())
        })
}
