use uuid::Uuid;

use super::FixtureType;

pub fn parse_fixture(json: &str) -> Result<Vec<FixtureType>, String> {
    let mut data: Wrapper =
        serde_json::from_str(json).map_err(|e| format!("Error parsing ofl:\n{e:#?}"))?;
    for t in &mut data.fixtures {
        t.id = Uuid::new_v4()
    }
    Ok(data.fixtures)
}

#[derive(Debug, serde::Deserialize)]
struct Wrapper {
    fixtures: Vec<FixtureType>,
}
