
build:
    cargo build --release

dev:
    cargo run

flamegraph:
    -CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --features bench
    just perf-save

fmt:
    cargo fmt

perf-save:
    mkdir -p perf/
    mv perf.data perf/perf.data
    mv flamegraph.svg perf/flamegraph.svg

report:
    cd perf/ && perf report --no-inline

run: build
    cargo run --release
