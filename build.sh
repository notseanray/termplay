#!/bin/bash
cargo build --release && doas cp target/release/termplay /usr/local/bin
