use bevy::{
    asset::{oi::Reader, AssertLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::HashMap,
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LocTableData(pub HashMap<String, String>);