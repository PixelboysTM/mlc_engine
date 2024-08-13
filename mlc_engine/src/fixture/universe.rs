use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Deref;

use rocket::request::FromParam;
use schemars::JsonSchema;

use mlc_common::config::{FixtureChannel, FixtureMode, FixtureType, ValueResolution};
use mlc_common::patched::{PatchedChannel, PatchedFixture, UniverseAddress, UniverseId};
use mlc_common::universe::{FixtureUniverse, PatchedChannelIndex};

use crate::fixture::feature::finder::find_features;

pub fn can_patch(universe: &FixtureUniverse, fixture: &FixtureType, mode_index: usize) -> bool {
    let mode = &fixture.get_modes()[mode_index];

    let len = mode.channels.len();

    let mut i = 0;
    for channel in &universe.channels {
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
    universe: &mut FixtureUniverse,
    fixture: &FixtureType,
    mode_index: usize,
) -> Result<UniverseAddress, String> {
    if !can_patch(universe, fixture, mode_index) {
        return Err("Can't fit the fixture in the Universe.".to_string());
    }

    let mode = &fixture.get_modes()[mode_index];

    let len = mode.channels.len();

    let mut i = 0;
    let mut start_index = 0;
    for channel in &universe.channels {
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
        create_patched_fixture(universe, fixture, len, mode_index, start_index, mode)?;

    let fixture_index = universe.fixtures.len();
    universe.fixtures.push(patched_fixture);
    for i in 0..len {
        universe.channels[i + start_index] = Some(PatchedChannelIndex {
            fixture_index,
            channel_index: i,
        });
    }

    Ok(start_index.into())
}

fn create_patched_fixture(
    universe: &mut FixtureUniverse,
    fixture: &FixtureType,
    len: usize,
    mode_index: usize,
    start_index: usize,
    mode: &FixtureMode,
) -> Result<PatchedFixture, String> {
    let mut resolution: ValueResolution = ValueResolution::U8;
    let mut cs = (0..len).map(|i| -> Result<_, String> {
        let c = fixture.get_available_channels().find(
            &fixture.get_modes()[mode_index].channels[i],
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
            fixture.name,
            fixture.get_modes()[mode_index].name
        ),
        mode: mode_index,
        features: find_features(fixture, mode, universe.id, start_index.into()),
        id: uuid::Uuid::new_v4(),
    })
}

trait FindChannelConfig {
    fn find(&self, name: &str, fine: &mut ValueResolution) -> Result<FixtureChannel, String>;
}

impl FindChannelConfig for HashMap<String, FixtureChannel> {
    fn find(&self, name: &str, fine: &mut ValueResolution) -> Result<FixtureChannel, String> {
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
                    return Err("More than two fine channels! Unknown resolution".to_string());
                }
            };
            Ok(d.clone())
        } else {
            Err(format!("Unknown Channel Name: {name}"))
        }
    }
}

#[derive(JsonSchema)]
pub struct UniverseIdParam(UniverseId);

impl FromParam<'_> for UniverseIdParam {
    type Error = ParseIntError;

    fn from_param(param: &'_ str) -> Result<Self, Self::Error> {
        param.parse::<u16>().map(UniverseId).map(UniverseIdParam)
    }
}

impl Deref for UniverseIdParam {
    type Target = UniverseId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
