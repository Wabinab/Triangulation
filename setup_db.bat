d quic_backend
cargo install diesel_cli --no-default-features --features "sqlite-bundled"
diesel setup --database-url=../data/database.sqlite
diesel migration run --database-url=../data/database.sqlite
cd ..
@REM Migrations we'll pre-generate and upload to github. 