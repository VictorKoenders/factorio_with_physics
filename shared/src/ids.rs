use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlayerId(pub Uuid);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GalaxyId(pub Uuid);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SolarSystemId(pub Uuid);
