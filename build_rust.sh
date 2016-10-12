# --release has overflow checks disabled
cargo rustc --release -- -C target-cpu=native
