use uuid::Uuid;

use serde_with::OneOrMany;
use mlc_common::config::FixtureType;

pub fn parse_fixture(json: &str) -> Result<Vec<FixtureType>, String> {
    let mut data: Wrapper =
        serde_json::from_str(json).map_err(|e| format!("Error parsing ofl:\n{e:#?}"))?;
    for t in &mut data.fixtures {
        t.id = Uuid::new_v4()
    }
    Ok(data.fixtures)
}

#[serde_with::serde_as]
#[derive(Debug, serde::Deserialize)]
struct Wrapper {
    #[serde_as(as = "OneOrMany<_>")]
    fixtures: Vec<FixtureType>,
}
