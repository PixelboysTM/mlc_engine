use super::{FixtureType, PatchedChannel, PatchedFixture, UniverseAddress, UniverseId};

pub const UNIVERSE_SIZE: usize = 512;

#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct FixtureUniverse {
    id: UniverseId,
    #[serde_as(as = "[_;UNIVERSE_SIZE]")]
    channels: [Option<PatchedChannelIndex>; UNIVERSE_SIZE],
    fixtures: Vec<PatchedFixture>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PatchedChannelIndex {
    fixture_index: usize,
    channel_index: usize,
}

impl FixtureUniverse {
    const INIT: Option<PatchedChannelIndex> = None;

    pub fn empty(id: UniverseId) -> Self {
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

        false
    }

    pub fn patch(
        &mut self,
        fixture: &FixtureType,
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
            config: fixture.clone(),
            num_channels: len as u8,
            channels: (0..len)
                .map(|i| PatchedChannel {
                    config: fixture.get_available_channels()
                        [&fixture.get_modes()[mode_index].get_channels()[i]]
                        .clone(),
                    channel_address: (start_index + i).into(),
                })
                .collect(),
            start_channel: start_index.into(),
            name: format!(
                "{} / {}",
                fixture.get_name(),
                fixture.get_modes()[mode_index].get_name()
            ),
        };

        let fixture_index = self.fixtures.len();
        self.fixtures.push(patched_fixture);
        for i in 0..len {
            self.channels[i + start_index] = Some(PatchedChannelIndex {
                fixture_index,
                channel_index: i,
            });
        }

        Ok(start_index.into())
    }
}

#[cfg(test)]
mod test {
    use crate::fixture::{FixtureType, UniverseId};

    use super::FixtureUniverse;

    #[test]
    fn test_out() {
        let fixture: FixtureType =
            serde_json::from_str(include_str!("../../../test_fixtures/led_par_56.json")).unwrap();

        let mut universe = FixtureUniverse::empty(UniverseId(1));
        {
            universe.patch(&fixture, 0).unwrap();
            universe.patch(&fixture, 0).unwrap();
            universe.patch(&fixture, 0).unwrap();
        }

        println!("{:#?}", universe);
        // assert!(false);
    }
}
