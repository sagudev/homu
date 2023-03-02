use once_cell::sync::Lazy;
use regex::Regex;

pub fn status_to_priority(s: &str) -> i32 {
    match s {
        "pending" => 1,
        "approved" => 2,
        "" => 3,
        "error" => 4,
        "failure" => 5,
        "success" => 6,
        _ => -1,
    }
}

pub const DEFAULT_TEST_TIMEOUT: i32 = 3600 * 10; //s
pub const INTERRUPTED_BY_HOMU_FMT: &str = "Interrupted by Homu ({})";
pub static INTERRUPTED_BY_HOMU_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"nterrupted by Homu \((.+?)\)").unwrap());

pub static VARIABLES_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{([a-zA-Z_]+)\}").unwrap());

pub const IGNORE_BLOCK_START: &str = "<!-- homu-ignore:start -->";
pub const IGNORE_BLOCK_END: &str = "<!-- homu-ignore:end -->";
/// TODO: not working yet
pub static IGNORE_BLOCK_RE: Lazy<Regex> = Lazy::new(|| {
    regex::RegexBuilder::new(r"\$\{([a-zA-Z_]+)\}")
        .multi_line(true)
        .case_insensitive(true)
        .build()
        .unwrap()
});
