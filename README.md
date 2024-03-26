# Marvin Light Control

![MLC](/images/mlc_configure_page.png)

Marvin Light Control (MLC for short) is a DmxShow Creation and Playback tool.
It utilizes a Timeline based approach to time and playback Light and Stage effects.
The Dmx protocol with its wide use cases and Network adaptions makes it highly compatible with most common stage
fixtures.
Currently, the usage of Endpoints
via [sACN](https://en.wikipedia.org/wiki/Architecture_for_Control_Networks#External_extensions)
and [ArtNet](https://art-net.org.uk/) (untested) with support for the Entec enabled USB-Interface coming soon

**Note:** MLC is in a pre-alpha state most of its features are only partially implemented, if at all, and the api and
data
structure will change.
We do **not** recommend using MLC in any situations that go beyond playing around yet!

## Tools

> [SACNView](https://sacnview.org/) is a great tool for debugging and viewing raw sACN output.

> For debugging purposes Swagger and Rapidoc is supported under (/api /rapi)

> MLC works towards supporting all fixtures from [OFL](https://open-fixture-library.org/) out of the box.

## Usage

While the Project is in early development no prebuilt executables are provided.

To run MLC on your local machine the following steps will help you:

1. Make sure you have Rust version 1.75 or newer
2. Because the frontend utilizes Rusts WebAssembly capabilities make sure you have installed the '
   wasm32-unknown-unknown' target installed.
   ``rustup target add wasm32-unknown-unknown``
3. As the frontend toolkit MLC uses [dioxus](https://dioxuslabs.com/) so to compile the fronted you have to install the
   dioxus-cli
   ``cargo install dioxus-cli`
4. MLC contains a [justfile](https://github.com/casey/just) to make the compilation process easier. Using just you can
   just clone the repo and run ``just r`` in the main directory.
5. If you want to compile and run manually you need to:
    - First compile the ui by moving into the ``mlc_diosxus`` directory and running `dx build --release`
    - You then can run then run MLC by moving back up into the parent dir and running ``cargo run --bin mlc_engine``
      from there.

## Todo

- [X] Fix Okapi openapi to handle lifetimes properly `openapi_attr\mod.rs get_add_operation_fn_name` (Requires local
  copy of my fork until merged)
- [X] Write a real Readme
- [ ] Add a License
- [X] Reorganize mlc_common code and cleanup empty modules in mlc_engine
- [X] Decouple FixtureType Data Structure from AGLight and built separate importer
- [ ] Add a new binary more kompakt file format for projects
- [ ] Ui
    - [X] Fixture Details/Patch Modal
    - [ ] Endpoint Mapping Modal
    - [ ] New Project Modal
    - [ ] Widgets
        - [X] Color
        - [X] 2D Vec
        - [X] Slider
        - [ ] Knob
    - [ ] Project Panel Layout
    - [ ] Effect Browser
    - [ ] Effect Settings
    - [ ] Effect Timeline
        - [ ] Basic
        - [ ] Fader Tracks
        - [ ] Feature Tracks
- [ ] Async Project loading with progress stream and ui
- [ ] Entec Endpoint

## Feature requests / Contribution

If you have a feature you want to see in the Program or even think about implementing it yourself, feel free to open an
issue describing your idea.
We have a lot of ideas where we want to take MLC in the future and would love to discuss your ideas with you.
The same is with bugs feel free to report any bugs you find, but keep in mind MLC is not nearly in a stable state so
keeping the software 100% bug free is not a priority (yet).
