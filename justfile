clean:
  clear
build: clean
  cargo build
test: clean
  cargo test
check: clean
  cargo clippy
run-help: build clean
  ./target/debug/pastol -h
run-example: build clean
  ./target/debug/pastol -t example -c "Example content."
run-example-file: build clean
  ./target/debug/pastol -f example.md
run-example-no-title: build clean
  ./target/debug/pastol -c example
run-example-no-content: build clean
  ./target/debug/pastol -t example
run-exaple-remove: run-example clean
  ./target/debug/pastol -r example
run-list: build clean
  ./target/debug/pastol -l