use super::FixtureType;

pub fn parse_ofl_fixture(json: &str) -> Result<FixtureType, String> {
    let data: FixtureType =
        serde_json::from_str(json).map_err(|e| format!("Error parsing ofl:\n{e:#?}"))?;
    Ok(data)
}
