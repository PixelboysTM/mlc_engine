pub mod feature {
    use feature_tile_to_raw as to_raw;
    use mlc_common::fixture::FaderAddress;
    use mlc_common::patched::feature::{
        Dimmer, FeatureSetRequest, FeatureTile, FixtureFeature, PanTilt, Rgb, Rotation,
    };

    use crate::runtime::{RuntimeData, ToFaderValue};

    pub trait ApplyFeature {
        async fn apply(&self, req: FeatureSetRequest, runtime_data: &RuntimeData);
    }

    impl ApplyFeature for Vec<FixtureFeature> {
        async fn apply(&self, req: FeatureSetRequest, runtime_data: &RuntimeData) {
            match req {
                FeatureSetRequest::Dimmer { value } => {
                    if let Some(d) = find_dimmer(self) {
                        update_values(&[(d.dimmer, value)], runtime_data).await;
                    }
                }
                FeatureSetRequest::White { value } => {
                    if let Some(d) = find_white(self) {
                        update_values(&[(d.dimmer, value)], runtime_data).await;
                    }
                }
                FeatureSetRequest::Amber { value } => {
                    if let Some(d) = find_amber(self) {
                        update_values(&[(d.dimmer, value)], runtime_data).await;
                    }
                }
                FeatureSetRequest::Rgb { red, green, blue } => {
                    if let Some(rgb) = find_rgb(self) {
                        update_values(
                            &[(rgb.red, red), (rgb.blue, blue), (rgb.green, green)],
                            runtime_data,
                        )
                            .await;
                    }
                }
                FeatureSetRequest::Rotation { value } => {
                    if let Some(rot) = find_rotation(self) {
                        if value > 0.0 {
                            update_values(&[(rot.cw, value.abs())], runtime_data).await;
                        }
                        if value < 0.0 {
                            update_values(&[(rot.ccw, value.abs())], runtime_data).await;
                        }
                    }
                }
                FeatureSetRequest::PanTilt { pan, tilt } => {
                    if let Some(pantilt) = find_pantilt(self) {
                        update_values(&[(pantilt.pan, pan), (pantilt.tilt, tilt)], runtime_data).await;
                    }
                }
                FeatureSetRequest::GetAvailableFeatures => {
                    eprintln!("Something is not working with your code dumb ass")
                }
            }
        }
    }

    async fn update_values(ts: &[(FeatureTile, f32)], runtime: &RuntimeData) {
        let mut universes = vec![];
        let mut channels = vec![];
        let mut values = vec![];

        for (tile, raw_v) in ts {
            let v = to_raw(tile, raw_v);
            for (f, v) in v {
                universes.push(f.universe);
                channels.push(f.address);
                values.push(v);
            }
        }

        if universes.is_empty() {
            return;
        }

        if universes.len() == 1 {
            runtime
                .set_value(universes[0], channels[0], values[0])
                .await;
            return;
        }

        runtime.set_values(universes, channels, values).await;
    }

    fn find_dimmer(features: &[FixtureFeature]) -> Option<Dimmer> {
        for f in features {
            match f {
                FixtureFeature::Dimmer(d) => return Some(d.clone()),
                _ => continue,
            }
        }

        None
    }

    fn find_rotation(features: &[FixtureFeature]) -> Option<Rotation> {
        for f in features {
            match f {
                FixtureFeature::Rotation(d) => return Some(d.clone()),
                _ => continue,
            }
        }

        None
    }

    fn find_white(features: &[FixtureFeature]) -> Option<Dimmer> {
        for f in features {
            match f {
                FixtureFeature::White(d) => return Some(d.clone()),
                _ => continue,
            }
        }

        None
    }

    fn find_amber(features: &[FixtureFeature]) -> Option<Dimmer> {
        for f in features {
            match f {
                FixtureFeature::Amber(d) => return Some(d.clone()),
                _ => continue,
            }
        }

        None
    }

    fn find_rgb(features: &[FixtureFeature]) -> Option<Rgb> {
        for f in features {
            match f {
                FixtureFeature::Rgb(d) => return Some(d.clone()),
                _ => continue,
            }
        }

        None
    }

    fn find_pantilt(features: &[FixtureFeature]) -> Option<PanTilt> {
        for f in features {
            match f {
                FixtureFeature::PanTilt(d) => return Some(d.clone()),
                _ => continue,
            }
        }

        None
    }

    pub fn feature_tile_to_raw(tile: &FeatureTile, val: &f32) -> Vec<(FaderAddress, u8)> {
        match tile {
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

    pub mod finder {
        use mlc_common::config::{DmxColor, DmxRange, FixtureCapability, FixtureChannel, FixtureMode, FixtureType, RotationSpeed};
        use mlc_common::fixture::FaderAddress;
        use mlc_common::patched::{UniverseAddress, UniverseId};
        use mlc_common::patched::feature::{Dimmer, FeatureChannel, FeatureTile, FixtureFeature, PanTilt, Rgb, Rotation};

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
    }
}
