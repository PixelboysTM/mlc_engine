use crate::fixture::fixture_parser::units::{Brightness, RotationAngle, Unit};
use mlc_common::config::{
    ColorIntensity, DmxRange, FixtureCapability, FixtureCapabilityCommon, FixtureChannel,
    FixtureMode, FixtureType, Intensity, Percentage, ValueResolution,
};
use serde_json::{Map, Value};
use std::collections::HashMap;

trait NoneLogger {
    fn log<F>(self, f: F) -> Self
    where
        F: FnOnce();
}

impl<T> NoneLogger for Option<T> {
    fn log<F>(self, f: F) -> Self
    where
        F: FnOnce(),
    {
        if self.is_none() {
            f();
        }
        self
    }
}

pub fn parse_fixture(json: &str) -> Result<Vec<FixtureType>, String> {
    let json: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("Error parsing ofl:\n{e:?}"))?;
    let arr = if let Some(arr) = json.as_array() {
        arr.clone()
    } else {
        vec![json]
    };

    let fixtures: Vec<_> = arr.iter().filter_map(parse_value_root).collect();

    Ok(fixtures)
}

fn parse_value_root(val: &serde_json::Value) -> Option<FixtureType> {
    let obj = val.as_object()?;

    let name = obj
        .get("name")
        .log(|| println!("'name' must be present in Fixture object"))?
        .as_str()
        .log(|| println!("'name' must be a String"))?;
    let short_name = obj
        .get("shortName")
        .map(|n| {
            n.as_str()
                .log(|| println!("'shortName' must be a String"))
                .expect("")
        })
        .unwrap_or(name);
    let key = name.to_lowercase().replace(' ', "-");
    let categories = obj
        .get("categories")
        .map(|v| {
            v.as_array()
                .log(|| println!("'categrories' must be an array of strings"))
                .expect("")
        })
        .map(|a| {
            a.iter()
                .map(|v| v.as_str())
                .filter(|f| f.is_some())
                .map(|f| f.expect("Filtered").to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![]);

    let modes = parse_modes(obj.get("modes"))?;
    let channels = parse_channels(obj.get("availableChannels"))?;

    Some(FixtureType {
        id: uuid::Uuid::new_v4(),
        name: name.to_string(),
        short_name: short_name.to_string(),
        fixture_key: key,
        categories,
        modes,
        available_channels: channels,
    })
}

fn parse_channels(channels_raw: Option<&Value>) -> Option<HashMap<String, FixtureChannel>> {
    let mut channels = HashMap::new();

    if let Some(avail) = channels_raw.and_then(|v| {
        v.as_object()
            .log(|| println!("'availableChannels' must be an object"))
    }) {
        for (k, v) in avail {
            if let Some(channel) = parse_channel(v) {
                channels.insert(k.clone(), channel);
            }
        }
    }

    Some(channels)
}

fn parse_channel(channel_raw: &Value) -> Option<FixtureChannel> {
    let raw = channel_raw
        .as_object()
        .log(|| println!("Channel must be an object"))?;

    let fine_aliases = raw
        .get("fineChannelAliases")
        .and_then(|a| a.as_array())
        .map(|v| {
            v.iter()
                .map(|a| a.as_str().expect("Must be").to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![]);

    let resolution = match fine_aliases.len() {
        0 => ValueResolution::U8,
        1 => ValueResolution::U16,
        2 => ValueResolution::U24,
        _ => {
            println!("Too many fine ChannelAliases at most 2");
            return None;
        }
    };
    let value_resolution = raw
        .get("dmxValueResolution")
        .map(|v| v.as_str().expect("must be"))
        .map(|v| match v {
            "8bit" => ValueResolution::U8,
            "16bit" => ValueResolution::U16,
            "24bit" => ValueResolution::U24,
            r => {
                println!("unknown value resolution: {r}");
                resolution
            }
        })
        .unwrap_or(resolution);

    let default_value = raw
        .get("defaultValue")
        .and_then(|v| v.as_u64())
        .map(|v| Percentage::from_dmx(v as usize, value_resolution))
        .unwrap_or(Percentage::new(0.0));

    let capabilities = parse_capabilities(raw, value_resolution)?;

    Some(FixtureChannel {
        fine_channel_aliases: fine_aliases,
        default_value,
        pixel_key: None,
        capabilities,
    })
}

fn parse_capabilities(
    raw: &Map<String, Value>,
    value_resolution: ValueResolution,
) -> Option<Vec<FixtureCapabilityCommon>> {
    if let Some(raw_cap) = raw.get("capability").and_then(|c| c.as_object()) {
        let detail = parse_detail_capability(raw_cap)?;

        return Some(vec![FixtureCapabilityCommon {
            dmx_range: DmxRange::full(),
            detail,
        }]);
    }

    let raw_caps = raw.get("capabilities").and_then(|v| v.as_array())?;

    let mut caps = vec![];
    for raw_cap in raw_caps {
        if let Some(raw_cap) = raw_cap.as_object() {
            let range = parse_dmx_range(raw_cap.get("dmxRange"), value_resolution);
            if let (Some(detail), Some(dmx_range)) = (parse_detail_capability(raw_cap), range) {
                caps.push(FixtureCapabilityCommon { dmx_range, detail });
            }
        }
    }

    Some(caps)
}

fn parse_dmx_range(
    dmx_range: Option<&Value>,
    value_resolution: ValueResolution,
) -> Option<DmxRange> {
    let range = dmx_range?
        .as_array()?
        .iter()
        .filter_map(|e| e.as_u64())
        .collect::<Vec<_>>();
    if range.len() != 2 {
        println!("Dmx Range must contain exactly 2 elements");
        return None;
    }

    let start = range[0];
    let end = range[1];

    Some(DmxRange {
        start: Percentage::from_dmx(start as usize, value_resolution),
        end: Percentage::from_dmx(end as usize, value_resolution),
    })
}

fn parse_detail_capability(raw_cap: &Map<String, Value>) -> Option<FixtureCapability> {
    let t = raw_cap.get("type")?.as_str()?;
    match t {
        "NoFunction" => Some(FixtureCapability::NoFunction),
        "Intensity" => {
            if let Some(i) = Unit::<Brightness>::parse(raw_cap, false) {
                Some(FixtureCapability::Intensity(i))
            } else {
                use mlc_common::config::Brightness as Br;
                Some(FixtureCapability::Intensity(Intensity {
                    brightness_start: Br::Percentage(Percentage::new(0.0)),
                    brightness_end: Br::Percentage(Percentage::new(1.0)),
                }))
            }
        }
        "ColorIntensity" => {
            if let Some(raw_color) = raw_cap.get("color") {
                Some(FixtureCapability::ColorIntensity(ColorIntensity {
                    color: serde_json::from_value(raw_color.clone()).expect("Must be"),
                }))
            } else {
                println!("No color specified");
                None
            }
        }
        "Pan" => Unit::<RotationAngle>::parse(raw_cap, false).map(FixtureCapability::Pan),
        "Tilt" => Unit::<RotationAngle>::parse(raw_cap, false).map(FixtureCapability::Tilt),
        s => {
            println!("Unknown Capability type: {s}");
            Some(FixtureCapability::Unimplemented)
        }
    }
}

fn parse_modes(modes: Option<&Value>) -> Option<Vec<FixtureMode>> {
    let modes = modes
        .log(|| println!("'modes' must be present"))?
        .as_array()
        .log(|| println!("'modes' must be an array"))?;

    let mut mds = vec![];
    for mode in modes {
        if let Some(mode) = mode
            .as_object()
            .log(|| println!("A mode must be an object"))
        {
            let name = mode
                .get("name")
                .log(|| println!("mode must have a name"))
                .and_then(|v| v.as_str())
                .map(|v| v.to_string());
            let short_name = mode
                .get("shortName")
                .log(|| println!("mode must have a name"))
                .and_then(|v| v.as_str())
                .map(|v| v.to_string());
            let channels = mode
                .get("channels")
                .log(|| println!("mode must have channels array of strings"))
                .and_then(|c| {
                    c.as_array().map(|v| {
                        v.iter()
                            .map(|x| x.as_str())
                            .filter(|x| x.is_some())
                            .map(|x| x.expect("").to_string())
                            .collect::<Vec<_>>()
                    })
                });

            if let (Some(name), Some(short_name), Some(channels)) = (name, short_name, channels) {
                mds.push(FixtureMode {
                    name,
                    short_name,
                    channels,
                });
            } else {
                println!("Building mode failed: {mode:?}");
            }
        }
    }

    Some(mds)
}

mod units {
    use mlc_common::config::{Intensity, PanTilt, PanTiltRotation, Percentage};
    use serde_json::{Map, Value};
    use std::marker::PhantomData;
    use std::str::FromStr;

    pub trait UnitProvider {
        type R;
        fn get_units() -> Vec<&'static str>;
        fn get_name() -> &'static str;
        fn get_aliases() -> Vec<(&'static str, f32)>;
        fn assemble(start: f32, end: f32, unit: &str) -> Self::R;
    }

    pub struct Unit<T: UnitProvider>(PhantomData<T>);

    impl<T: UnitProvider> Unit<T> {
        pub fn parse(raw: &Map<String, Value>, must_be_stepped: bool) -> Option<T::R> {
            if let Some(val) = raw.get(T::get_name()) {
                if let Some((v, u)) = Self::create_val(val) {
                    return Some(T::assemble(v, v, u));
                }

                return None;
            }

            if must_be_stepped {
                return None;
            }

            let start_val = if let Some(val) = raw.get(&format!("{}Start", T::get_name())) {
                if let Some((v, u)) = Self::create_val(val) {
                    Some((v, u))
                } else {
                    None
                }
            } else {
                None
            };

            let end_val = if let Some(val) = raw.get(&format!("{}End", T::get_name())) {
                if let Some((v, u)) = Self::create_val(val) {
                    Some((v, u))
                } else {
                    None
                }
            } else {
                None
            };

            if let (Some(start), Some(end)) = (start_val, end_val) {
                if start.1 != end.1 {
                    println!("Units dont match!");
                    return None;
                }

                Some(T::assemble(start.0, end.0, start.1))
            } else {
                None
            }
        }

        fn create_val(raw: &Value) -> Option<(f32, &'static str)> {
            let val = raw.as_str()?;

            for (a, v) in T::get_aliases() {
                if a == val {
                    return Some((v, "%"));
                }
            }

            for unit in T::get_units() {
                if val.ends_with(unit) {
                    let raw_num = val.trim_end_matches(unit);
                    let num = f32::from_str(raw_num);
                    if let Ok(n) = num {
                        return Some((n, unit));
                    }
                }
            }

            None
        }
    }

    pub struct RotationAngle;

    impl UnitProvider for RotationAngle {
        type R = PanTilt;

        fn get_units() -> Vec<&'static str> {
            vec!["deg", "%"]
        }

        fn get_name() -> &'static str {
            "angle"
        }

        fn get_aliases() -> Vec<(&'static str, f32)> {
            vec![]
        }

        fn assemble(start: f32, end: f32, unit: &str) -> Self::R {
            let (s, e) = match unit {
                "deg" => (
                    PanTiltRotation::Angle(start as u32),
                    PanTiltRotation::Angle(end as u32),
                ),
                "%" => (
                    PanTiltRotation::Percentage(Percentage::new(start as f64)),
                    PanTiltRotation::Percentage(Percentage::new(end as f64)),
                ),
                _ => unreachable!("Why here units does not match"),
            };

            PanTilt {
                angle_start: s,
                angle_end: e,
            }
        }
    }

    pub struct Brightness;

    impl UnitProvider for Brightness {
        type R = Intensity;

        fn get_units() -> Vec<&'static str> {
            vec!["lm", "%"]
        }

        fn get_name() -> &'static str {
            "brightness"
        }

        fn get_aliases() -> Vec<(&'static str, f32)> {
            vec![("off", 0.0), ("dark", 0.01), ("bright", 1.0)]
        }

        fn assemble(start: f32, end: f32, unit: &str) -> Self::R {
            use mlc_common::config::Brightness as Br;

            let (s, e) = match unit {
                "lm" => (Br::Lumen(start), Br::Lumen(end)),
                "%" => (
                    Br::Percentage(Percentage::new(start as f64)),
                    Br::Percentage(Percentage::new(end as f64)),
                ),
                _ => unreachable!("Why here units does not match"),
            };

            Intensity {
                brightness_start: s,
                brightness_end: e,
            }
        }
    }
}
