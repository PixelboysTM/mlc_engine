alias d := dioxus
alias d-r := dioxus-release
alias r := run
alias v := viewer
alias i := install
alias update := install

set shell := ["cmd.exe", "/c"]

default: dioxus-release

dioxus:
    cd ./mlc_interface && dx build --platform web
    del out /f /s /q
    xcopy /s target\dx\mlc_interface\debug\web\public out
    
dioxus-release:
    cd ./mlc_interface && dx build --release --platform web
    del out /f /s /q
    xcopy /s target\dx\mlc_interface\release\web\public out

make-icon ICON:
    cd ./mlc_dioxus/public/icons && dx translate --file {{ICON}}.svg

build: dioxus
    cargo build

build-release: dioxus-release
    cargo build --release
    
run: dioxus-release
    cargo run --bin mlc_engine
    
run-release: dioxus-release
    cargo run --bin mlc_engine --release

viewer:
    cargo build --bin mlc_viewer --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --out-name mlc_viewer --out-dir out/ --target web target/wasm32-unknown-unknown/release/mlc_viewer.wasm

install:
    cargo install wasm-bindgen-cli
    cargo install dioxus-cli

lines:
    tokei ./