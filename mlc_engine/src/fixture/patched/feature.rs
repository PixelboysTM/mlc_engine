use crate::fixture::{FixtureMode, FixtureType};

// Indexes are offsets from the start_index of the Fixture
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum FixtureFeature {
    Dimmer {
        channel_index: usize,
    },
    Rgb {
        red_channel_index: usize,
        green_channel_index: usize,
        blue_channel_index: usize,
    },
}

pub fn find_features(fixture: &FixtureType, mode: &FixtureMode) -> Vec<FixtureFeature> {
    let mut features = vec![];

    let channels = mode.get_channels();

    features
}

fn search_dimmer(fixture: &FixtureType, channels: &[String]) -> Option<FixtureFeature> {
    for channel in channels {
        let cap = fixture.get_available_channels().get(channel);
        if let Some(cap) = cap {
            for c in cap.capabilities {
                c.
            }
        }
    }
}
