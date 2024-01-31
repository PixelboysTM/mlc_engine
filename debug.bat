cd mlc_web/
call npm run build
cd ..
cargo run

@REM find . -name '*.*' | xargs wc -l