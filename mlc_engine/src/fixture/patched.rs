use std::fmt::Debug;
use std::ops::Add;

use get_size::GetSize;
use rocket::request::FromParam;
use serde::{de::Visitor, Deserialize, Serialize};
use serde::de::Error;

use mlc_common::patched::{UniverseAddress, UniverseId};

use super::{FixtureChannel, FixtureType, ValueResolution};

use self::feature::FixtureFeature;

pub mod feature;

pub type FixtureId = uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize)]
pub struct PatchedFixture {
    pub config: FixtureType,
    pub num_channels: u8,
    pub channels: Vec<PatchedChannel>,
    pub start_channel: UniverseAddress,
    pub name: String,
    pub mode: usize,
    pub features: Vec<FixtureFeature>,
    #[get_size(ignore)]
    pub id: FixtureId,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize)]
pub struct PatchedChannel {
    pub(in crate::fixture) config: FixtureChannel,
    pub(in crate::fixture) channel_address: UniverseAddress,
    pub(in crate::fixture) resolution: ValueResolution,
}

