use std::{fmt::Debug, rc::Rc};

use super::{FixtureChannel, FixtureType};

#[derive(Debug)]
pub struct PatchedFixture<'a> {
    pub(in crate::fixture) config: &'a FixtureType,
    pub(in crate::fixture) num_channels: u8,
    pub(in crate::fixture) channels: Vec<Rc<PatchedChannel<'a>>>,
    pub(in crate::fixture) start_channel: UniverseAddress,
}

#[derive(Debug)]
pub struct PatchedChannel<'a> {
    pub(in crate::fixture) config: &'a FixtureChannel,
}

#[derive(Clone, Copy, PartialEq)]
pub struct UniverseAddress {
    add_256: bool,
    adds: u8,
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

#[derive(Debug, Clone, Copy)]
pub struct DmxUniverse(pub u16);

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
