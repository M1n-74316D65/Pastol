clean:
  clear
build: clean
  cargo build
test: clean
  cargo test
clippy: clean
  cargo clippy
check: clean
  cargo check
install: clean
  cargo install --path .
run-help: build clean
  ./target/debug/pastol -h
run-listed-true: build clean
  ./target/debug/pastol --setunlist true
run-listed-false: build clean
  ./target/debug/pastol --setunlist false
run-example: download clean
  ./target/debug/pastol -t example -c "Example content."
run-example-file: build clean
  ./target/debug/pastol -f example.md
run-example-no-title: build clean
  ./target/debug/pastol -c example
run-example-no-content: build clean
  ./target/debug/pastol -t example
run-exaple-remove: run-example clean
  ./target/debug/pastol -r example
run-exaple-remove-fail: build clean
  ./target/debug/pastol -r exampledwadawdas
run-list: build clean
  ./target/debug/pastol --list
run-list-fail: build clean
  ./target/debug/pastol --list
run-info: run-example clean
  ./target/debug/pastol --info example
run-info-fail: build clean
  ./target/debug/pastol --info qZ5pR9KcX7dA2eW3
run-download: run-example clean
  ./target/debug/pastol --download example
run-download-fail: build clean
  ./target/debug/pastol --download qZ5pR9KcX7dA2eW3