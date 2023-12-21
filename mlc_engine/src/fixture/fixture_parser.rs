use super::FixtureType;

pub fn parse_fixture(json: &str) -> Result<Vec<FixtureType>, String> {
    let data: Wrapper =
        serde_json::from_str(json).map_err(|e| format!("Error parsing ofl:\n{e:#?}"))?;
    Ok(data.fixtures)
}

#[derive(Debug, serde::Deserialize)]
struct Wrapper {
    fixtures: Vec<FixtureType>,
}
