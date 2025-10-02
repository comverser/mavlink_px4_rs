# Show available commands
default:
    @just --list --unsorted

run connection:
    cargo run -- {{connection}}

run-px4:
    cargo run -- udpin:0.0.0.0:14540

