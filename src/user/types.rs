use serde::{Deserialize, Serialize};
use zvariant::{OwnedObjectPath, OwnedValue, Signature, Structure, Type};

/// State of a User. If `UserState::Invalid` then the response from
/// logind was not well defined.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum UserState {
    Online,
    Offline,
    Lingering,
    Active,
    Closing,
    Invalid,
}

impl From<&str> for UserState {
    fn from(s: &str) -> Self {
        match s.trim() {
            "online" => Self::Online,
            "offline" => Self::Offline,
            "lingering" => Self::Lingering,
            "active" => Self::Active,
            "closing" => Self::Closing,
            _ => Self::Invalid,
        }
    }
}

impl TryFrom<OwnedValue> for UserState {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <String>::try_from(value)?;
        return Ok(UserState::from(value.as_str()));
    }
}

impl Type for UserState {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct UserPath {
    id: String,
    /// Name of session user
    path: OwnedObjectPath,
}

impl UserPath {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl TryFrom<OwnedValue> for UserPath {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <Structure>::try_from(value)?;
        return Ok(Self {
            id: <String>::try_from(value.fields()[0].clone())?,
            path: <OwnedObjectPath>::try_from(value.fields()[1].clone())?,
        });
    }
}
