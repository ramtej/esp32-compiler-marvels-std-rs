#!/bin/bash
# op-out defaults and execute on x86_64
rustup run stable cargo run --release --no-default-features --features target-native --target=x86_64-unknown-linux-gnu
