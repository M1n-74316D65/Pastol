build:
  cargo build
test:
  cargo test
check:
  cargo clippy
run-help: build
  ./target/debug/pastol -h
run-example: build
  ./target/debug/pastol -t example -c "Example content."
run-example-file: build
  ./target/debug/pastol -f example.md
run-example-no-title: build
  ./target/debug/pastol -c example
run-example-no-content: build
  ./target/debug/pastol -t example