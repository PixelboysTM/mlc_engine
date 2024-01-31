pub mod apply;

use crate::fixture::{
    DmxColor, DmxRange, FaderAddress, FixtureCapability, FixtureChannel, FixtureMode, FixtureType,
    RotationSpeed,
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PanTilt {
    pub pan: FeatureTile,
    pub tilt: FeatureTile,
}

// Indexes are offsets from the start_index of the Fixture
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum FixtureFeature {
    Dimmer(Dimmer),
    White(Dimmer),
    Rgb(Rgb),
    Rotation(Rotation),
    PanTilt(PanTilt),
}

impl FixtureFeature {
    pub fn name(&self) -> &'static str {
        match self {
            FixtureFeature::Dimmer(_) => "Dimmer",
            FixtureFeature::White(_) => "White",
            FixtureFeature::Rgb(_) => "Rgb",
            FixtureFeature::Rotation(_) => "Rotation",
            FixtureFeature::PanTilt(_) => "PanTilt",
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum FeatureTile {
    Single {
        channel: FeatureChannel,
        fader: FaderAddress,
        range: DmxRange,
    },
    Double {
        channel: FeatureChannel,
        channel_fine: FeatureChannel,
        fader: FaderAddress,
        fader_fine: FaderAddress,
        range: DmxRange,
    },
    Tripple {
        channel: FeatureChannel,
        channel_fine: FeatureChannel,
        channel_grain: FeatureChannel,
        fader: FaderAddress,
        fader_fine: FaderAddress,
        fader_grain: FaderAddress,
        range: DmxRange,
    },
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
    let finders: Vec<&FeatureFinder> = vec![
        &search_dimmer,
        &search_rgb,
        &search_white,
        &search_rotation,
        &search_pantilt,
    ];

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
            if caps.pixel_key.is_some() {
                continue;
            }

            for cap in &caps.capabilities {
                if let FixtureCapability::Intensity(d) = &cap.detail {
                    let dimmer = make_feature_tile(
                        caps,
                        start_index,
                        universe_id,
                        i,
                        &cap.dmx_range,
                        channels,
                    );
                    return Some(FixtureFeature::Dimmer(Dimmer { dimmer }));
                }
            }
        }
    }

    None
}

fn make_feature_tile(
    channel: &FixtureChannel,
    start_index: UniverseAddress,
    universe_id: UniverseId,
    i: usize,
    range: &DmxRange,
    channels: &[String],
) -> FeatureTile {
    let fine = if channel.fine_channel_aliases.len() == 0 {
        None
    } else {
        channels
            .iter()
            .position(|f| f == &channel.fine_channel_aliases[0])
    };
    let grain = if fine.is_none() || channel.fine_channel_aliases.len() == 1 {
        None
    } else {
        channels
            .iter()
            .position(|f| f == &channel.fine_channel_aliases[1])
    };

    if fine.is_none() {
        FeatureTile::Single {
            channel: FeatureChannel(i),
            fader: FaderAddress {
                universe: universe_id,
                address: start_index + i,
            },
            range: *range,
        }
    } else {
        let fine = fine.expect("Must be");
        if grain.is_none() {
            FeatureTile::Double {
                channel: FeatureChannel(i),
                channel_fine: FeatureChannel(fine),
                fader: FaderAddress {
                    universe: universe_id,
                    address: start_index + i,
                },
                fader_fine: FaderAddress {
                    universe: universe_id,
                    address: start_index + fine,
                },
                range: *range,
            }
        } else {
            let grain = grain.expect("Must be");
            FeatureTile::Tripple {
                channel: FeatureChannel(i),
                channel_fine: FeatureChannel(fine),
                channel_grain: FeatureChannel(grain),
                fader: FaderAddress {
                    universe: universe_id,
                    address: start_index + i,
                },
                fader_fine: FaderAddress {
                    universe: universe_id,
                    address: start_index + fine,
                },
                fader_grain: FaderAddress {
                    universe: universe_id,
                    address: start_index + grain,
                },
                range: *range,
            }
        }
    }
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
                if caps.pixel_key.is_some() {
                    continue;
                }
                if let FixtureCapability::ColorIntensity(d) = &cap.detail {
                    if d.color == DmxColor::White {
                        return Some(FixtureFeature::White(Dimmer {
                            dimmer: make_feature_tile(
                                caps,
                                start_index,
                                universe_id,
                                i,
                                &cap.dmx_range,
                                channels,
                            ),
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
                if caps.pixel_key.is_some() {
                    continue;
                }
                if let FixtureCapability::ColorIntensity(c) = &cap.detail {
                    match c.color {
                        DmxColor::Red if red.is_none() => {
                            red = Some(make_feature_tile(
                                caps,
                                start_index,
                                universe_id,
                                i,
                                &cap.dmx_range,
                                channels,
                            ))
                        }
                        DmxColor::Green if green.is_none() => {
                            green = Some(make_feature_tile(
                                caps,
                                start_index,
                                universe_id,
                                i,
                                &cap.dmx_range,
                                channels,
                            ))
                        }
                        DmxColor::Blue if blue.is_none() => {
                            blue = Some(make_feature_tile(
                                caps,
                                start_index,
                                universe_id,
                                i,
                                &cap.dmx_range,
                                channels,
                            ))
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
                if caps.pixel_key.is_some() {
                    continue;
                }
                if let FixtureCapability::Rotation(d) = &cap.detail {
                    if ((matches!(d.speed_start, RotationSpeed::SlowCw)
                        && matches!(d.speed_end, RotationSpeed::FastCw))
                        || (matches!(d.speed_start, RotationSpeed::FastCw)
                            && matches!(d.speed_end, RotationSpeed::SlowCw)))
                        && cw.is_none()
                    {
                        cw = Some(make_feature_tile(
                            caps,
                            start_index,
                            universe_id,
                            i,
                            &cap.dmx_range,
                            channels,
                        ))
                    }

                    if ((matches!(d.speed_start, RotationSpeed::SlowCcw)
                        && matches!(d.speed_end, RotationSpeed::FastCcw))
                        || (matches!(d.speed_start, RotationSpeed::FastCcw)
                            && matches!(d.speed_end, RotationSpeed::SlowCcw)))
                        && ccw.is_none()
                    {
                        ccw = Some(make_feature_tile(
                            caps,
                            start_index,
                            universe_id,
                            i,
                            &cap.dmx_range,
                            channels,
                        ))
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

fn search_pantilt(
    fixture: &FixtureType,
    channels: &[String],
    universe_id: UniverseId,
    start_index: UniverseAddress,
) -> Option<FixtureFeature> {
    let mut pan = None;
    let mut tilt = None;

    for (i, channel) in channels.iter().enumerate() {
        let caps = fixture.get_available_channels().get(channel);
        if let Some(caps) = caps {
            if caps.pixel_key.is_some() {
                continue;
            }
            for cap in &caps.capabilities {
                match &cap.detail {
                    FixtureCapability::Pan(_) if pan.is_none() => {
                        pan = Some(make_feature_tile(
                            caps,
                            start_index,
                            universe_id,
                            i,
                            &cap.dmx_range,
                            channels,
                        ));
                    }
                    FixtureCapability::Tilt(_) if tilt.is_none() => {
                        tilt = Some(make_feature_tile(
                            caps,
                            start_index,
                            universe_id,
                            i,
                            &cap.dmx_range,
                            channels,
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    if pan.is_some() && tilt.is_some() {
        Some(FixtureFeature::PanTilt(PanTilt {
            pan: pan.expect("Must be"),
            tilt: tilt.expect("Must be"),
        }))
    } else {
        None
    }
}
