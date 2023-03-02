// These type aliases are here to for models to be
// as close as possible to CREATE TABLE queries
type Nullable<T> = Option<T>;
type Text = String;
type Integer = i32;
type DateTime = time::PrimitiveDateTime;

/// One row in `pull` table
#[derive(sqlx::FromRow)]
pub struct Pull {
    pub repo: Text,
    pub num: Integer,
    pub status: Text,
    pub merge_sha: Nullable<Text>,
    pub body: Nullable<Text>,
    pub head_sha: Nullable<Text>,
    pub head_ref: Nullable<Text>,
    pub base_ref: Nullable<Text>,
    pub assignee: Nullable<Text>,
    pub approved_by: Nullable<Text>,
    pub priority: Nullable<Integer>,
    pub try_: Nullable<Integer>,
    pub rollup: Nullable<Integer>,
    pub squash: Nullable<Integer>,
    pub delegate: Nullable<Text>,
}

/// One row in `build_res` table
#[derive(sqlx::FromRow)]
pub struct BuildRes {
    pub repo: Text,
    pub num: Integer,
    pub builder: Text,
    pub res: Nullable<Integer>,
    pub url: Text,
    pub merge_sha: Text,
}

/// One row in `mergeable` table
#[derive(sqlx::FromRow)]
pub struct Mergeable {
    pub repo: Text,
    pub num: Integer,
    pub mergeable: Integer,
}

/// One row in `repos` table
#[derive(sqlx::FromRow)]
pub struct Repo {
    pub repo: Text,
    pub treeclosed: Integer,
    pub treeclosed_src: Nullable<Text>,
}

/// One row in `retry_log` table
#[derive(sqlx::FromRow)]
pub struct RetryLog {
    pub repo: Text,
    pub num: Integer,
    pub time: DateTime,
    pub src: Text,
    pub msg: Text,
}
