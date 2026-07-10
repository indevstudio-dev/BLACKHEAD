.PHONY: check fmt lint test codegen android integration clean

check: fmt lint test

fmt:
	cargo fmt --all --check
	cd go/image && gofmt -w .
	cd go/pkg && gofmt -w .

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	cd go/image && go vet ./...
	cd go/pkg && go vet ./...

test:
	cargo test --workspace
	cd go/image && go test ./...
	cd go/pkg && go test ./...

codegen:
	buf lint
	buf generate

android:
	@echo "Android toolchain baseline not pinned yet."

integration:
	@echo "Requires a supported Linux namespace host or rooted Android runner."

clean:
	cargo clean
	rm -rf api/generated/* dist out artifacts
