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
run-example: build clean
  ./target/debug/pastol -t example -c "Example content."
run-example-file: run-download clean
  ./target/debug/pastol example.md
run-example-file-title: run-download clean
  ./target/debug/pastol example.md -t adw
run-example-file-content: run-download clean
  ./target/debug/pastol example.md -c dwa
run-example-no-title: build clean
  ./target/debug/pastol -c example
run-example-no-content: build clean
  ./target/debug/pastol -t example
run-exaple-remove: run-example clean
  ./target/debug/pastol -r example
run-exaple-remove-fail: build clean
  ./target/debug/pastol -r exampledwadawdas
run-list: build clean
  ./target/debug/pastol -l
run-list-fail: build clean
  ./target/debug/pastol -l
run-search: build clean
  ./target/debug/pastol -s logi
run-search-fail: build clean
  ./target/debug/pastol -s logidwaaaaaaaaaaa
run-view: run-example clean
  ./target/debug/pastol -v example
run-view-fail: build clean
  ./target/debug/pastol -v qZ5pR9KcX7dA2eW3
run-download: run-example clean
  ./target/debug/pastol --download example
run-download-fail: build clean
  ./target/debug/pastol --download qZ5pR9KcX7dA2eW3