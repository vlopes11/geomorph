default:
  just --list --list-submodules

check: check-format check-clippy check-test check-doc check-features

check-format:
  cargo fmt --all -- --check

check-clippy:
  cargo clippy --all-targets -- -D warnings

check-test:
  cargo hack test --feature-powerset

check-doc:
  RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

check-features:
  RUSTFLAGS="-D warnings" cargo hack check \
    --feature-powerset \
    --no-dev-deps
  RUSTFLAGS="-D warnings" cargo hack check \
    --target thumbv7em-none-eabi \
    --feature-powerset \
    --exclude-features std,default \
    --no-dev-deps
