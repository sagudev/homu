use diesel::prelude::*;
use diesel::sql_types::Timestamp;
use diesel::sqlite::Sqlite;

// Rust type aliases
type Nullable<T> = Option<T>;
type Text = String;
type Integer = i32;

#[derive(Queryable)]
pub struct Pull {
    repo: Text,
    num: Integer,
    status: Text,
    merge_sha: Nullable<Text>,
    body: Nullable<Text>,
    head_sha: Nullable<Text>,
    head_ref: Nullable<Text>,
    base_ref: Nullable<Text>,
    assignee: Nullable<Text>,
    approved_by: Nullable<Text>,
    priority: Nullable<Integer>,
    try_: Nullable<Integer>,
    rollup: Nullable<Integer>,
    squash: Nullable<Integer>,
    delegate: Nullable<Text>,
}

#[derive(Queryable)]
pub struct build_res {
    repo: Text,
    num: Integer,
    builder: Text,
    res: Nullable<Integer>,
    url: Text,
    merge_sha: Text,
}

#[derive(Queryable)]
pub struct mergeable {
    repo: Text,
    num: Integer,
    // in diesel colum cannot have same name as table
    col_mergeable: Integer,
}

#[derive(Queryable, Insertable)]
pub struct repos {
    repo: Text,
    treeclosed: Integer,
    treeclosed_src: Nullable<Text>,
}

#[derive(Queryable)]
pub struct retry_log {
    repo: Text,
    num: Integer,
    // TODO: check
    time: Timestamp,
    src: Text,
    msg: Text,
}
