alias d := dioxus
alias d-r := dioxus-release
alias r := run

set shell := ["cmd.exe", "/c"]

default: dioxus-release

dioxus:
    cd ./mlc_dioxus && dx build
    
dioxus-release:
    cd ./mlc_dioxus && dx build --release

build: dioxus
    cargo build

build-release: dioxus-release
    cargo build --release
    
run: dioxus-release
    cargo run --bin mlc_engine
    
run-release: dioxus-release
    cargo run --bin mlc_engine --release
    

    