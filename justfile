# format all (with some special nightly only options that aren't strictly enforced but recommended)
fmt:
    cargo +nightly fmt -- --config group_imports=StdExternalCrate,imports_granularity=Module
    cargo fmt --all