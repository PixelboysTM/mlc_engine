mod config;
mod fixture_parser;
mod patched;
mod universe;

pub use config::*;
pub use fixture_parser::parse_fixture;
pub use patched::*;
pub use universe::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaderAddress {
    pub universe: UniverseId,
    pub address: UniverseAddress,
}
