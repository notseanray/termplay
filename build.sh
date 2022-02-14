#!/bin/bash
cargo build --release && sudo cp target/release/termplay /usr/local/bin
