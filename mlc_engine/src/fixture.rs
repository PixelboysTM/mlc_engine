use get_size::GetSize;

pub use config::*;
pub use fixture_parser::parse_fixture;
use mlc_common::patched::UniverseId;
pub use patched::*;
pub use universe::*;

mod config;
mod fixture_parser;
mod patched;
mod universe;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, get_size::GetSize)]
pub struct FaderAddress {
    pub universe: UniverseId,
    pub address: UniverseAddress,
}
