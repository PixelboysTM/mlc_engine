use std::{fmt::Debug, num::ParseIntError};

use rocket::request::FromParam;
use serde::de::Error;
use serde::{de::Visitor, Deserialize, Serialize};

use super::{FixtureChannel, FixtureType};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PatchedFixture {
    pub(in crate::fixture) config: FixtureType,
    pub(in crate::fixture) num_channels: u8,
    pub(in crate::fixture) channels: Vec<PatchedChannel>,
    pub(in crate::fixture) start_channel: UniverseAddress,
    pub(in crate::fixture) name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PatchedChannel {
    pub(in crate::fixture) config: FixtureChannel,
    pub(in crate::fixture) channel_address: UniverseAddress,
}

#[derive(Clone, Copy, PartialEq)]
pub struct UniverseAddress {
    add_256: bool,
    adds: u8,
}

impl Serialize for UniverseAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.adds as u16 + if self.add_256 { 256 } else { 0 })
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

impl Debug for UniverseAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniverseAddress")
            .field(
                "adds",
                &(self.adds as u16 + if self.add_256 { 256 } else { 0 }),
            )
            .finish()
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct UniverseId(pub u16);

impl FromParam<'_> for UniverseId {
    type Error = ParseIntError;

    fn from_param(param: &'_ str) -> Result<Self, Self::Error> {
        u16::from_str_radix(param, 10).map(|d| UniverseId(d))
    }
}

impl Ord for UniverseId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for UniverseId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
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
                add_256: false
            }
        );

        assert_eq!(
            a2,
            UniverseAddress {
                adds: 10,
                add_256: false
            }
        );

        assert_eq!(
            a3,
            UniverseAddress {
                adds: 255,
                add_256: false
            }
        );

        assert_eq!(
            a4,
            UniverseAddress {
                adds: 0,
                add_256: true
            }
        );

        assert_eq!(
            a5,
            UniverseAddress {
                adds: 44,
                add_256: true
            }
        );

        assert_eq!(
            a6,
            UniverseAddress {
                adds: 255,
                add_256: true
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
