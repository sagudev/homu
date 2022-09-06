diesel::table! {
    pull (repo, num) {
        repo -> Text,
        num -> Integer,
        status -> Text,
        merge_sha -> Nullable<Text>,
        body -> Nullable<Text>,
        head_sha -> Nullable<Text>,
        head_ref -> Nullable<Text>,
        base_ref -> Nullable<Text>,
        assignee -> Nullable<Text>,
        approved_by -> Nullable<Text>,
        priority -> Nullable<Integer>,
        try_ -> Nullable<Integer>,
        rollup -> Nullable<Integer>,
        squash -> Nullable<Integer>,
        delegate -> Nullable<Text>,
    }
}

diesel::table! {
    build_res (repo, num, builder) {
        repo -> Text,
        num -> Integer,
        builder -> Text,
        res -> Nullable<Integer>,
        url -> Text,
        merge_sha -> Text,
    }
}

diesel::table! {
    mergeable (repo, num) {
        repo -> Text,
        num -> Integer,
        // in diesel colum cannot have same name as table
        #[sql_name = "mergeable"]
        col_mergeable -> Integer,
    }
}

diesel::table! {
    repos (repo) {
        repo -> Text,
        treeclosed -> Integer,
        treeclosed_src -> Nullable<Text>,
    }
}

diesel::table! {
    retry_log (repo) {
        repo -> Text,
        num -> Integer,
        // TODO: check
        time -> Timestamp,
        src -> Text,
        msg -> Text,
    }
}
