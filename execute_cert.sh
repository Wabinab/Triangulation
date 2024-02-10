#!/bin/bash
cd cert_server
cargo build --release
cd ..

./cert_server/target/release/cert_server --cert-path ./cert