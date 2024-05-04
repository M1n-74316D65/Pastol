build:
  cargo build
test:
  cargo test
check:
  cargo clippy
run-help:
  ./target/debug/pastol -h
run-example:
  ./target/debug/pastol -t example -c "Example content."
run-example-file:
  ./target/debug/pastol -f example.md