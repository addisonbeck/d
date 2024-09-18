default:
    @just --list

# Format all source files
format:
    treefmt

# Run 'cargo run' on the project
d *ARGS:
    cargo run -- {{ARGS}}
