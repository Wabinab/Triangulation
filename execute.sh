#!/bin/bash
cd quic_backend
cargo build --release
cd ..

cd cert_server
cargo build --release
cd ..

./quic_backend/target/release/quic_backend --tls-cert ./cert/localhost.crt --tls-key ./cert/localhost.key & ./cert_server/target/release/cert_server --cert-path ./cert