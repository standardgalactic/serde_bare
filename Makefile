# Provided so users not familiar with Go don't need to know the go bench invocation

.PHONY: bench go-bench

bench:
	cargo bench

go-bench:
	cd benches/go-reference && go test -bench=.

bench-all: bench go-bench
