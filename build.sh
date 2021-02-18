#!/bin/sh

cargo build --release --bins --target x86_64-unknown-linux-musl

cargo build --release --bins --target x86_64-pc-windows-gnu