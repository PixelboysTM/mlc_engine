pub mod apply;

use crate::fixture::{
    DmxColor, DmxRange, FaderAddress, FixtureCapability, FixtureMode, FixtureType,
};

use super::{UniverseAddress, UniverseId};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Dimmer {
    pub dimmer: FeatureTile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Rgb {
    pub red: FeatureTile,
    pub green: FeatureTile,
    pub blue: FeatureTile,
}

// Indexes are offsets from the start_index of the Fixture
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum FixtureFeature {
    Dimmer(Dimmer),
    Rgb(Rgb),
}

impl FixtureFeature {
    pub fn name(&self) -> &'static str {
        match self {
            FixtureFeature::Dimmer(_) => "Dimmer",
            FixtureFeature::Rgb(_) => "Rgb",
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct FeatureTile {
    channel: FeatureChannel,
    fader: FaderAddress,
    range: DmxRange,
}

/// The Offset of channelss from the start of the Fixture Fader = start_index + self
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct FeatureChannel(usize);

pub fn find_features(
    fixture: &FixtureType,
    mode: &FixtureMode,
    universe: UniverseId,
    start_index: UniverseAddress,
) -> Vec<FixtureFeature> {
    let finders: Vec<&FeatureFinder> = vec![&search_dimmer, &search_rgb];

    let mut features = vec![];

    let channels = mode.get_channels();

    for finder in finders {
        if let Some(feature) = finder(fixture, channels, universe, start_index) {
            features.push(feature);
        }
    }

    features
}

type FeatureFinder =
    dyn Fn(&FixtureType, &[String], UniverseId, UniverseAddress) -> Option<FixtureFeature>;

fn search_dimmer(
    fixture: &FixtureType,
    channels: &[String],
    universe_id: UniverseId,
    start_index: UniverseAddress,
) -> Option<FixtureFeature> {
    for (i, channel) in channels.iter().enumerate() {
        let caps = fixture.get_available_channels().get(channel);
        if let Some(caps) = caps {
            for cap in &caps.capabilities {
                if let FixtureCapability::Intensity(d) = cap {
                    return Some(FixtureFeature::Dimmer(Dimmer {
                        dimmer: FeatureTile {
                            channel: FeatureChannel(i),
                            fader: FaderAddress {
                                universe: universe_id,
                                address: start_index + i,
                            },
                            range: d.dmx_range,
                        },
                    }));
                }
            }
        }
    }

    None
}

fn search_rgb(
    fixture: &FixtureType,
    channels: &[String],
    universe_id: UniverseId,
    start_index: UniverseAddress,
) -> Option<FixtureFeature> {
    let mut red = None;
    let mut green = None;
    let mut blue = None;

    for (i, channel) in channels.iter().enumerate() {
        let caps = fixture.get_available_channels().get(channel);
        if let Some(caps) = caps {
            for cap in &caps.capabilities {
                if let FixtureCapability::ColorIntensity(c) = cap {
                    match c.color {
                        DmxColor::Red if red.is_none() => {
                            red = Some(FeatureTile {
                                channel: FeatureChannel(i),
                                fader: FaderAddress {
                                    universe: universe_id,
                                    address: start_index + i,
                                },
                                range: c.dmx_range,
                            })
                        }
                        DmxColor::Green if green.is_none() => {
                            green = Some(FeatureTile {
                                channel: FeatureChannel(i),
                                fader: FaderAddress {
                                    universe: universe_id,
                                    address: start_index + i,
                                },
                                range: c.dmx_range,
                            })
                        }
                        DmxColor::Blue if blue.is_none() => {
                            blue = Some(FeatureTile {
                                channel: FeatureChannel(i),
                                fader: FaderAddress {
                                    universe: universe_id,
                                    address: start_index + i,
                                },
                                range: c.dmx_range,
                            })
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if red.is_some() && green.is_some() && blue.is_some() {
        Some(FixtureFeature::Rgb(Rgb {
            red: red.expect(""),
            green: green.expect(""),
            blue: blue.expect(""),
        }))
    } else {
        None
    }
}
