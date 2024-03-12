use std::fmt::Debug;
use std::ops::Add;

use get_size::GetSize;
use rocket::request::FromParam;
use serde::{de::Visitor, Deserialize, Serialize};
use serde::de::Error;

pub mod feature;


