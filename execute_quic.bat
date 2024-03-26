@REM Maybe need background job to deal with multiples. 
cd quic_backend
cargo build --release
cd ..

.\quic_backend\target\release\quic_backend.exe --tls-cert .\cert\localhost.crt --tls-key .\cert\localhost.key --data-path .\data --cert-path .\cert --ng-asset-path .\client-app\src\assets