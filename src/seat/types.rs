use serde::{Deserialize, Serialize};
use zvariant::{OwnedObjectPath, OwnedValue, Structure, Type};

use crate::IntoPath;

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct SessionPath {
    id: String,
    /// Name of session user
    path: OwnedObjectPath,
}

impl SessionPath {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl TryFrom<OwnedValue> for SessionPath {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <Structure>::try_from(value)?;
        return Ok(Self {
            id: <String>::try_from(value.fields()[0].clone())?,
            path: <OwnedObjectPath>::try_from(value.fields()[1].clone())?,
        });
    }
}

impl IntoPath for SessionPath {
    fn into_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    fn into_path_ref(&self) -> &OwnedObjectPath {
        &self.path
    }
}
