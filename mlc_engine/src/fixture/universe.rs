use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Deref;

use rocket::request::FromParam;
use schemars::JsonSchema;
use mlc_common::config::{FixtureChannel, FixtureMode, ValueResolution};

use mlc_common::patched::{UniverseAddress, UniverseId};
use mlc_common::universe::{FixtureUniverse, UNIVERSE_SIZE};


#[derive(JsonSchema)]
pub struct Wrapper(UniverseId);

impl FromParam<'_> for Wrapper {
    type Error = ParseIntError;

    fn from_param(param: &'_ str) -> Result<Self, Self::Error> {
        param.parse::<u16>().map(UniverseId).map(Wrapper)
    }
}

impl Deref for Wrapper {
    type Target = UniverseId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use mlc_common::config::FixtureType;
    use mlc_common::patched::UniverseId;

    use super::FixtureUniverse;

    #[test]
    fn test_out() {
        let fixture: FixtureType =
            serde_json::from_str(include_str!("../../../test_fixtures/led_par_56.json")).unwrap();

        let mut universe = FixtureUniverse::empty(UniverseId(1));
        {
            universe.patch(&fixture, 0).unwrap();
            universe.patch(&fixture, 0).unwrap();
            universe.patch(&fixture, 0).unwrap();
        }

        println!("{:#?}", universe);
        // assert!(false);
    }
}
