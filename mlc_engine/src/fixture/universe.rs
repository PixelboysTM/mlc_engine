use std::rc::Rc;

use super::{DmxUniverse, FixtureType, PatchedChannel, PatchedFixture, UniverseAddress};

pub const UNIVERSE_SIZE: usize = 512;

#[derive(Debug)]
pub struct FixtureUniverse<'a> {
    id: DmxUniverse,
    channels: [Option<Rc<PatchedChannel<'a>>>; UNIVERSE_SIZE],
    fixtures: Vec<PatchedFixture<'a>>,
}

impl<'a> FixtureUniverse<'a> {
    const INIT: Option<Rc<PatchedChannel<'a>>> = None;

    pub fn empty(id: DmxUniverse) -> Self {
        FixtureUniverse {
            id,
            channels: [Self::INIT; UNIVERSE_SIZE],
            fixtures: vec![],
        }
    }

    pub fn can_patch(&self, fixture: &FixtureType, mode_index: usize) -> bool {
        let mode = &fixture.get_modes()[mode_index];

        let len = mode.get_channels().len();

        let mut i = 0;
        for channel in &self.channels {
            if channel.is_some() {
                i = 0;
            } else {
                i += 1;
            }

            if i == len {
                return true;
            }
        }

        return false;
    }

    pub fn patch(
        &mut self,
        fixture: &'a FixtureType,
        mode_index: usize,
    ) -> Result<UniverseAddress, &'static str> {
        if !self.can_patch(fixture, mode_index) {
            return Err("Can't fit the fixture in the Universe.");
        }

        let mode = &fixture.get_modes()[mode_index];

        let len = mode.get_channels().len();

        let mut i = 0;
        let mut start_index = 0;
        for channel in &self.channels {
            if channel.is_some() {
                i = 0;
            } else {
                i += 1;
            }

            start_index += 1;

            if i == len {
                break;
            }
        }

        start_index -= len;

        let patched_fixture = PatchedFixture {
            config: fixture,
            num_channels: len as u8,
            channels: (0..len)
                .map(|i| PatchedChannel {
                    config: &fixture.get_available_channels()
                        [&fixture.get_modes()[mode_index].get_channels()[i]],
                })
                .map(|c| Rc::new(c))
                .collect(),
            start_channel: start_index.into(),
        };

        for i in 0..len {
            self.channels[i + start_index] = Some(patched_fixture.channels[i].clone());
        }

        self.fixtures.push(patched_fixture);

        Ok(start_index.into())
    }
}

#[cfg(test)]
mod test {
    use crate::fixture::{DmxUniverse, FixtureType};

    use super::FixtureUniverse;

    #[test]
    fn test_out() {
        let fixture: FixtureType =
            serde_json::from_str(include_str!("../../../led-nano-par.json")).unwrap();

        let mut universe = FixtureUniverse::empty(DmxUniverse(1));
        {
            universe.patch(&fixture, 0).unwrap();
            universe.patch(&fixture, 0).unwrap();
            universe.patch(&fixture, 0).unwrap();
        }

        println!("{:#?}", universe);
        // assert!(false);
    }
}
