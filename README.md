# Marvin Light Control

> [!IMPORTANT]
> This version is not the active delveloped one. To see where MLC now lives visit [marvin_light_control](https://github.com/PixelboysTM/marvin_light_control/)

![MLC](/images/mlc_configure_page.png)

Marvin Light Control (MLC for short) is a DmxShow Creation and Playback tool.
It utilizes a Timeline based approach to time and playback Light and Stage effects.
The Dmx protocol with its wide use cases and Network adaptions makes it highly compatible with most common stage
fixtures.
Currently, the usage of Endpoints
via [sACN](https://en.wikipedia.org/wiki/Architecture_for_Control_Networks#External_extensions)
, [ArtNet](https://art-net.org.uk/) (untested) and Entec enabled USB-Interfaces is supported.

**Note:** MLC is in a pre-alpha state most of its features are only partially implemented, if at all, and the api and
data
structure will change.
We do **not** recommend using MLC in any situations that go beyond playing around yet!

## Tools

- [SACNView](https://sacnview.org/) is a great tool for debugging and viewing raw sACN output.

- For debugging purposes Swagger and Rapidoc is supported under (/api /rapi)

- MLC works towards supporting all fixtures from [OFL](https://open-fixture-library.org/) out of the box.

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

## Roadmap

### V0.1.1

- [ ] Relayout Programm panel
- [ ] update effect timeline
- [ ] make keyframe edits not janky anymore (in seperate section no popover)
- [ ] make keyframes dragable
- [ ] Actually test 

### V0.1.2

- [ ] Support more fixture features
- [ ] support matrix features
- [ ] Async Project loading with progress stream and ui
- [ ] revisit all TODOs in code
- [ ] revisit sliders to make dragging more suitable
- [ ] improve patch overlay

### V0.1.3

- [ ] Support easing between keyframes
- [ ] start a basic 3d viewer
- [ ] improve layout of show panel
- [ ] add push to play option on show panel

### V0.1.4

- [ ] add support for variable values in channels
- [ ] improve 3d viewer
- [ ] add mobile playback

### V0.1.5

- [ ] add a fixture type creator
- [ ] Write an actual license
- [ ] work on a  deploy pipeline

### Misc

- [X] Fix Okapi openapi to handle lifetimes properly `openapi_attr\mod.rs get_add_operation_fn_name` (Requires local copy of my fork until merged)
- [ ] As soon as it gets reöeased implement modal and popover via popover api

## Feature requests / Contribution

If you have a feature you want to see in the Program or even think about implementing it yourself, feel free to open an
issue describing your idea.
We have a lot of ideas where we want to take MLC in the future and would love to discuss your ideas with you.
The same is with bugs feel free to report any bugs you find, but keep in mind MLC is not nearly in a stable state so
keeping the software 100% bug free is not a priority (yet).
