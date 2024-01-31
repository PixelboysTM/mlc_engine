pub mod apply;

use crate::fixture::{
    DmxColor, DmxRange, FaderAddress, FixtureCapability, FixtureMode, FixtureType, RotationSpeed,
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Rotation {
    pub cw: FeatureTile,
    pub ccw: FeatureTile,
}

// Indexes are offsets from the start_index of the Fixture
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum FixtureFeature {
    Dimmer(Dimmer),
    White(Dimmer),
    Rgb(Rgb),
    Rotation(Rotation),
}

impl FixtureFeature {
    pub fn name(&self) -> &'static str {
        match self {
            FixtureFeature::Dimmer(_) => "Dimmer",
            FixtureFeature::White(_) => "White",
            FixtureFeature::Rgb(_) => "Rgb",
            FixtureFeature::Rotation(_) => "Rotation",
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
    let finders: Vec<&FeatureFinder> =
        vec![&search_dimmer, &search_rgb, &search_white, &search_rotation];

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

fn search_white(
    fixture: &FixtureType,
    channels: &[String],
    universe_id: UniverseId,
    start_index: UniverseAddress,
) -> Option<FixtureFeature> {
    for (i, channel) in channels.iter().enumerate() {
        let caps = fixture.get_available_channels().get(channel);
        if let Some(caps) = caps {
            for cap in &caps.capabilities {
                if let FixtureCapability::ColorIntensity(d) = cap {
                    if d.color == DmxColor::White {
                        return Some(FixtureFeature::White(Dimmer {
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

fn search_rotation(
    fixture: &FixtureType,
    channels: &[String],
    universe_id: UniverseId,
    start_index: UniverseAddress,
) -> Option<FixtureFeature> {
    let mut cw: Option<FeatureTile> = None;
    let mut ccw: Option<FeatureTile> = None;

    for (i, channel) in channels.iter().enumerate() {
        let caps = fixture.get_available_channels().get(channel);
        if let Some(caps) = caps {
            for cap in &caps.capabilities {
                if let FixtureCapability::Rotation(d) = cap {
                    if ((matches!(d.speed_start, RotationSpeed::SlowCw)
                        && matches!(d.speed_end, RotationSpeed::FastCw))
                        || (matches!(d.speed_start, RotationSpeed::FastCw)
                            && matches!(d.speed_end, RotationSpeed::SlowCw)))
                        && cw.is_none()
                    {
                        cw = Some(FeatureTile {
                            channel: FeatureChannel(i),
                            fader: FaderAddress {
                                universe: universe_id,
                                address: start_index + i,
                            },
                            range: d.dmx_range,
                        })
                    }

                    if ((matches!(d.speed_start, RotationSpeed::SlowCcw)
                        && matches!(d.speed_end, RotationSpeed::FastCcw))
                        || (matches!(d.speed_start, RotationSpeed::FastCcw)
                            && matches!(d.speed_end, RotationSpeed::SlowCcw)))
                        && ccw.is_none()
                    {
                        ccw = Some(FeatureTile {
                            channel: FeatureChannel(i),
                            fader: FaderAddress {
                                universe: universe_id,
                                address: start_index + i,
                            },
                            range: d.dmx_range,
                        })
                    }
                }
            }
        }
    }

    if cw.is_some() && ccw.is_some() {
        Some(FixtureFeature::Rotation(Rotation {
            cw: cw.expect("Must be"),
            ccw: ccw.expect("Must be"),
        }))
    } else {
        None
    }
}
