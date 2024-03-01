use get_size::GetSize;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, get_size::GetSize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureMode {
    name: String,
    short_name: String,
    channels: Vec<String>,
}

impl FixtureMode {
    pub fn get_channels(&self) -> &[String] {
        &self.channels
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}