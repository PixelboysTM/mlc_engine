use crate::config::{FixtureChannel, FixtureType, ValueResolution};
use crate::patched::feature::FixtureFeature;
use get_size::GetSize;
use schemars::JsonSchema;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Add;

pub mod feature;

pub type FixtureId = uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct PatchedChannel {
    pub config: FixtureChannel,
    pub channel_address: UniverseAddress,
    pub resolution: ValueResolution,
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
    get_size::GetSize,
    JsonSchema,
)]
pub struct UniverseId(pub u16);

impl Ord for UniverseId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for UniverseId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, get_size::GetSize, JsonSchema)]
pub struct UniverseAddress {
    add_256: bool,
    adds: u8,
}

impl UniverseAddress {
    pub fn i(&self) -> usize {
        let u: u16 = (*self).into();
        u as usize
    }
}

impl Serialize for UniverseAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16((*self).into())
    }
}

impl<'de> Deserialize<'de> for UniverseAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u128(UAVisitor)
    }
}

struct UAVisitor;

impl<'de> Visitor<'de> for UAVisitor {
    type Value = UniverseAddress;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        UniverseAddress::create(v).map_err(|e| {
            eprintln!("{:#?}", e);
            E::custom(e)
        })
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(UniverseAddress::from(v as usize))
    }
}

impl UniverseAddress {
    pub fn create(adds: u16) -> Result<Self, &'static str> {
        if adds >= 512 {
            return Err("A universe only has 512 channels.");
        }

        let start = adds as u8;
        let add_256 = adds > 255;

        Ok(UniverseAddress {
            add_256,
            adds: start,
        })
    }
}

impl From<u16> for UniverseAddress {
    fn from(value: u16) -> Self {
        Self::create(value).unwrap()
    }
}

impl From<usize> for UniverseAddress {
    fn from(value: usize) -> Self {
        Self::create(value as u16).unwrap()
    }
}

impl From<UniverseAddress> for u16 {
    fn from(value: UniverseAddress) -> Self {
        value.adds as u16 + if value.add_256 { 256 } else { 0 }
    }
}

impl From<UniverseAddress> for usize {
    fn from(value: UniverseAddress) -> Self {
        value.adds as usize + if value.add_256 { 256 } else { 0 }
    }
}

impl Add<usize> for UniverseAddress {
    type Output = UniverseAddress;

    fn add(self, rhs: usize) -> Self::Output {
        let s: usize = self.into();
        (s + rhs).into()
    }
}

impl Debug for UniverseAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: u16 = (*self).into();
        f.debug_struct("UniverseAddress")
            .field("adds", &val)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::UniverseAddress;

    #[test]
    fn universe_adds() {
        let a1 = UniverseAddress::create(0).unwrap();
        let a2 = UniverseAddress::create(10).unwrap();
        let a3 = UniverseAddress::create(255).unwrap();
        let a4 = UniverseAddress::create(256).unwrap();
        let a5 = UniverseAddress::create(300).unwrap();
        let a6 = UniverseAddress::create(511).unwrap();

        assert_eq!(
            a1,
            UniverseAddress {
                adds: 0,
                add_256: false,
            }
        );

        assert_eq!(
            a2,
            UniverseAddress {
                adds: 10,
                add_256: false,
            }
        );

        assert_eq!(
            a3,
            UniverseAddress {
                adds: 255,
                add_256: false,
            }
        );

        assert_eq!(
            a4,
            UniverseAddress {
                adds: 0,
                add_256: true,
            }
        );

        assert_eq!(
            a5,
            UniverseAddress {
                adds: 44,
                add_256: true,
            }
        );

        assert_eq!(
            a6,
            UniverseAddress {
                adds: 255,
                add_256: true,
            }
        );

        println!("{:?}", a1);
        println!("{:?}", a2);
        println!("{:?}", a3);
        println!("{:?}", a4);
        println!("{:?}", a5);
        println!("{:?}", a6);
    }
}
