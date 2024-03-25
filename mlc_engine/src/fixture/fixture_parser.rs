use std::collections::HashMap;
use reqwest::get;
use serde_json::{Map, Value};
use mlc_common::config::{ColorIntensity, DmxRange, FixtureCapability, FixtureCapabilityCommon, FixtureChannel, FixtureMode, FixtureType, Intensity, Percentage, ValueResolution};

trait NoneLogger {
    fn log<F>(self, f: F) -> Self where F: FnOnce();
}

impl<T> NoneLogger for Option<T> {
    fn log<F>(self, f: F) -> Self where F: FnOnce() {
        if self.is_none() {
            f();
        }
        self
    }
}

pub fn parse_fixture(json: &str) -> Result<Vec<FixtureType>, String> {
    // let mut data: Wrapper =
    //     serde_json::from_str(json).map_err(|e| format!("Error parsing ofl:\n{e:#?}"))?;
    // for t in &mut data.fixtures {
    //     t.id = Uuid::new_v4()
    // }
    let json: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("Error parsing ofl:\n{e:?}"))?;
    let arr = if let Some(arr) = json.as_array() {
        arr.clone()
    } else {
        vec![json]
    };

    let fixtures: Vec<_> = arr.iter().map(parse_value_root).filter(|f| f.is_some()).map(|f| f.expect("Was filtered!")).collect();

    Ok(fixtures)
}

fn parse_value_root(val: &serde_json::Value) -> Option<FixtureType> {
    let obj = val.as_object()?;

    let name = obj.get("name").log(|| println!("'name' must be present in Fixture object"))?.as_str().log(|| println!("'name' must be a String"))?;
    let short_name = obj.get("shortName").map(|n| n.as_str().log(|| println!("'shortName' must be a String")).expect("")).unwrap_or(name);
    let key = name.to_lowercase().replace(" ", "-");
    let categories = obj.get("categories")
        .map(|v| v.as_array().log(|| println!("'categrories' must be an array of strings")).expect(""))
        .map(|a| a.iter().map(|v| v.as_str()).filter(|f| f.is_some()).map(|f| f.expect("Filtered").to_owned()).collect::<Vec<_>>())
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

    if let Some(avail) = channels_raw.map(|v| v.as_object().log(|| println!("'availableChannels' must be an object"))).flatten() {
        for (k, v) in avail {
            if let Some(channel) = parse_channel(v) {
                channels.insert(k.clone(), channel);
            }
        }
    }

    Some(channels)
}

fn parse_channel(channel_raw: &Value) -> Option<FixtureChannel> {
    let raw = channel_raw.as_object().log(|| println!("Channel must be an object"))?;

    let fine_aliases = raw.get("fineChannelAliases")
        .map(|a| a.as_array())
        .flatten()
        .map(|v| v.iter().map(|a| a.as_str().expect("Must be").to_string()).collect::<Vec<_>>())
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
    let value_resolution = raw.get("dmxValueResolution")
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

    let default_value = raw.get("defaultValue")
        .map(|v| v.as_u64())
        .flatten()
        .map(|v| Percentage::dmx(v as usize, value_resolution))
        .unwrap_or(Percentage::new(0.0));


    let capabilities = parse_capabilities(raw, value_resolution)?;

    Some(FixtureChannel {
        fine_channel_aliases: fine_aliases,
        default_value,
        pixel_key: None,
        capabilities,
    })
}

fn parse_capabilities(raw: &Map<String, Value>, value_resolution: ValueResolution) -> Option<Vec<FixtureCapabilityCommon>> {
    if let Some(raw_cap) = raw.get("capability").map(|c| c.as_object()).flatten() {
        let detail = parse_detail_capability(raw_cap)?;

        return Some(vec![FixtureCapabilityCommon {
            dmx_range: DmxRange {
                start: Percentage::new(0.0),
                end: Percentage::new(1.0),
            },
            detail,
        }]);
    }

    let raw_caps = raw.get("capabilities").map(|v| v.as_array()).flatten()?;

    let mut caps = vec![];
    for raw_cap in raw_caps {
        if let Some(raw_cap) = raw_cap.as_object() {
            let range = parse_dmx_range(raw_cap.get("dmxRange"), value_resolution);
            if let (Some(detail), Some(dmx_range)) = (parse_detail_capability(raw_cap), range) {
                caps.push(FixtureCapabilityCommon {
                    dmx_range,
                    detail,
                });
            }
        }
    }

    Some(caps)
}

fn parse_dmx_range(dmx_range: Option<&Value>, value_resolution: ValueResolution) -> Option<DmxRange> {
    let range = dmx_range?.as_array()?.iter().map(|e| e.as_u64()).filter(|e| e.is_some()).map(|e| e.expect("must be")).collect::<Vec<_>>();
    if range.len() != 2 {
        println!("Dmx Range must contain exactly 2 elements");
        return None;
    }

    let start = range[0];
    let end = range[1];

    Some(DmxRange {
        start: Percentage::dmx(start as usize, value_resolution),
        end: Percentage::dmx(end as usize, value_resolution),
    })
}

fn parse_detail_capability(raw_cap: &Map<String, Value>) -> Option<FixtureCapability> {
    let t = raw_cap.get("type")?.as_str()?;
    match t {
        "NoFunction" => {
            Some(FixtureCapability::NoFunction)
        }
        "Intensity" => {
            Some(FixtureCapability::Intensity(Intensity {}))
        }
        "ColorIntensity" => {
            if let Some(raw_color) = raw_cap.get("color") {
                Some(FixtureCapability::ColorIntensity(ColorIntensity {
                    color: serde_json::from_value(raw_color.clone()).expect("Must be")
                }))
            } else {
                println!("No color specified");
                None
            }
        }
        s => {
            println!("Unknown Capability type: {s}");
            Some(FixtureCapability::Unimplemented)
        }
    }
}

fn parse_modes(modes: Option<&Value>) -> Option<Vec<FixtureMode>> {
    let modes = modes.log(|| println!("'modes' must be present"))?.as_array().log(|| println!("'modes' must be an array"))?;

    let mut mds = vec![];
    for mode in modes {
        if let Some(mode) = mode.as_object().log(|| println!("A mode must be an object")) {
            let name = mode.get("name").log(|| println!("mode must have a name")).map(|v| v.as_str()).flatten().map(|v| v.to_string());
            let short_name = mode.get("shortName").log(|| println!("mode must have a name")).map(|v| v.as_str()).flatten().map(|v| v.to_string());
            let channels = mode.get("channels").log(|| println!("mode must have channels array of strings"))
                .map(|c| c.as_array().map(|v| v.iter().map(|x| x.as_str()).filter(|x| x.is_some()).map(|x| x.expect("").to_string()).collect::<Vec<_>>())).flatten();

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
