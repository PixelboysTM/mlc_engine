use std::{clone, collections::HashMap};

use super::{
    feature::find_features, FixtureChannel, FixtureType, PatchedChannel, PatchedFixture,
    UniverseAddress, UniverseId, ValueResolution,
};

pub const UNIVERSE_SIZE: usize = 512;

#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct FixtureUniverse {
    id: UniverseId,
    #[serde_as(as = "[_;UNIVERSE_SIZE]")]
    channels: [Option<PatchedChannelIndex>; UNIVERSE_SIZE],
    fixtures: Vec<PatchedFixture>,
}

impl FixtureUniverse {
    pub fn get_fixtures(&self) -> &Vec<PatchedFixture> {
        &self.fixtures
    }
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

        let patched_fixture =
            self.create_patched_fixture(fixture, len, mode_index, start_index, mode)?;

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

    fn create_patched_fixture(
        &mut self,
        fixture: &FixtureType,
        len: usize,
        mode_index: usize,
        start_index: usize,
        mode: &super::FixtureMode,
    ) -> Result<PatchedFixture, &'static str> {
        let mut resolution: ValueResolution = ValueResolution::U8;
        let mut cs = (0..len).map(|i| -> Result<_, &'static str> {
            let c = fixture.get_available_channels().find(
                &fixture.get_modes()[mode_index].get_channels()[i],
                &mut resolution,
            )?;

            Ok(PatchedChannel {
                config: c,
                channel_address: (start_index + i).into(),
                resolution,
            })
        });

        if cs.any(|f| f.is_err()) {
            let _ = cs.find(|f| f.is_err()).expect("Must be")?;
        }

        Ok(PatchedFixture {
            config: fixture.clone(),
            num_channels: len as u8,
            channels: cs.map(|f| f.expect("Must be")).collect(),
            start_channel: start_index.into(),
            name: format!(
                "{} / {}",
                fixture.get_name(),
                fixture.get_modes()[mode_index].get_name()
            ),
            mode: mode_index,
            features: find_features(fixture, mode, self.id, start_index.into()),
            id: uuid::Uuid::new_v4(),
        })
    }
}

trait FindChannelConfig {
    fn find(&self, name: &str, fine: &mut ValueResolution) -> Result<FixtureChannel, &'static str>;
}

impl FindChannelConfig for HashMap<String, FixtureChannel> {
    fn find(&self, name: &str, fine: &mut ValueResolution) -> Result<FixtureChannel, &'static str> {
        if let Some(d) = self.get(name) {
            *fine = ValueResolution::U8;
            Ok(d.clone())
        } else if let Some(d) = self
            .values()
            .find(|c| c.fine_channel_aliases.contains(&name.to_string()))
        {
            let i = d
                .fine_channel_aliases
                .iter()
                .position(|c| c == name)
                .expect("Must be");
            *fine = match i {
                0 => ValueResolution::U16,
                1 => ValueResolution::U24,
                _ => {
                    return Err("More than two fine channels! Unknown resolution");
                }
            };
            Ok(d.clone())
        } else {
            Err("Unknown Channel Name")
        }
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
