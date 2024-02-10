#!/bin/bash
cd quic_backend
cargo build --release
cd ..

# Probably need to call separately. 
./quic_backend/target/release/quic_backend --tls-cert ./cert/localhost.crt --tls-key ./cert/localhost.key


