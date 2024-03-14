use crate::config::{
    DmxColor, DmxRange, FixtureCapability, FixtureChannel, FixtureMode, FixtureType, RotationSpeed,
};
use crate::fixture::FaderAddress;
use crate::patched::{UniverseAddress, UniverseId};
use crate::ToFaderValue;
use get_size::GetSize;
use schemars::JsonSchema;
use std::fmt::{Display, Formatter};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct Dimmer {
    pub dimmer: FeatureTile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct Rgb {
    pub red: FeatureTile,
    pub green: FeatureTile,
    pub blue: FeatureTile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct Rotation {
    pub cw: FeatureTile,
    pub ccw: FeatureTile,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct PanTilt {
    pub pan: FeatureTile,
    pub tilt: FeatureTile,
}

// Indexes are offsets from the start_index of the Fixture
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub enum FixtureFeature {
    Dimmer(Dimmer),
    White(Dimmer),
    Amber(Dimmer),
    Rgb(Rgb),
    Rotation(Rotation),
    PanTilt(PanTilt),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, get_size::GetSize)]
pub enum FixtureFeatureType {
    Dimmer,
    White,
    Rgb,
    Rotation,
    PanTilt,
    Amber,
}

impl Display for FixtureFeatureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FixtureFeatureType::Dimmer => "Dimmer",
            FixtureFeatureType::White => "White",
            FixtureFeatureType::Rgb => "Rgb",
            FixtureFeatureType::Rotation => "Rotation",
            FixtureFeatureType::PanTilt => "PanTilt",
            FixtureFeatureType::Amber => "Amber",
        })
    }
}

impl FixtureFeature {
    pub fn name(&self) -> FixtureFeatureType {
        match self {
            FixtureFeature::Dimmer(_) => FixtureFeatureType::Dimmer,
            FixtureFeature::White(_) => FixtureFeatureType::White,
            FixtureFeature::Rgb(_) => FixtureFeatureType::Rgb,
            FixtureFeature::Rotation(_) => FixtureFeatureType::Rotation,
            FixtureFeature::PanTilt(_) => FixtureFeatureType::PanTilt,
            FixtureFeature::Amber(_) => FixtureFeatureType::Amber,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
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

impl FeatureTile {
    pub fn to_raw(&self, val: &f32) -> Vec<(FaderAddress, u8)> {
        match self {
            FeatureTile::Single { fader, range, .. } => {
                let v = val.to_fader_value_range(range);
                vec![(*fader, v)]
            }
            FeatureTile::Double {
                fader,
                fader_fine,
                range,
                ..
            } => {
                let (v, f) = val.to_fader_value_range_fine(range);
                vec![(*fader, v), (*fader_fine, f)]
            }
            FeatureTile::Tripple {
                fader,
                fader_fine,
                fader_grain,
                range,
                ..
            } => {
                let (v, f, g) = val.to_fader_value_range_grain(range);
                vec![(*fader, v), (*fader_fine, f), (*fader_grain, g)]
            }
        }
    }
}

/// The Offset of channelss from the start of the Fixture Fader = start_index + self
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, get_size::GetSize, JsonSchema)]
pub struct FeatureChannel(usize);

//TODO: Probably should not be here
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
        &search_amber,
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum FeatureSetRequest {
    // 0.0 -> 1.0
    Dimmer { value: f32 },
    // (0.0, 0.0, 0.0) -> (1.0, 1.0, 1.0)
    Rgb { red: f32, green: f32, blue: f32 },
    // 0.0 -> 1.0
    White { value: f32 },
    // 0.0 -> 1.0
    Amber { value: f32 },
    // -1.0 -> 1.0  TODO: Needs an update in naming etc. is not clear what is meant (is it speed, value, ...)
    Rotation { value: f32 },
    // (0.0, 0.0) -> (1.0, 1.0)
    PanTilt { pan: f32, tilt: f32 },
    GetAvailableFeatures,
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
                if let FixtureCapability::Intensity(_) = &cap.detail {
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
    let fine = if channel.fine_channel_aliases.is_empty() {
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

    if let (Some(red), Some(green), Some(blue)) = (red, green, blue) {
        Some(FixtureFeature::Rgb(Rgb { red, green, blue }))
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

    if let (Some(cw), Some(ccw)) = (cw, ccw) {
        Some(FixtureFeature::Rotation(Rotation { cw, ccw }))
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

    if let (Some(pan), Some(tilt)) = (pan, tilt) {
        Some(FixtureFeature::PanTilt(PanTilt { pan, tilt }))
    } else {
        None
    }
}

fn search_amber(
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
                    if d.color == DmxColor::Amber {
                        return Some(FixtureFeature::Amber(Dimmer {
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
