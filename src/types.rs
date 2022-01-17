use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use zvariant::{OwnedObjectPath, OwnedValue, Structure, Type};

pub trait IntoPath {
    fn into_path(&self) -> OwnedObjectPath;
    fn into_path_ref(&self) -> &OwnedObjectPath;
}

// /// If `ShutdownType::Invalid` then the response from
// /// logind was not well defined.
// #[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
// pub enum ShutdownType {
//     PowerOff,
//     DryPowerOff,
//     Reboot,
//     DryReboot,
//     Halt,
//     DryHalt,
//     Invalid,
// }

// impl From<&str> for ShutdownType {
//     fn from(s: &str) -> Self {
//         match s.trim() {
//             "poweroff" => Self::PowerOff,
//             "dry-poweroff" => Self::DryPowerOff,
//             "reboot" => Self::Reboot,
//             "dry-reboot" => Self::DryReboot,
//             "halt" => Self::Halt,
//             "dry-halt" => Self::DryHalt,
//             _ => Self::Invalid,
//         }
//     }
// }

// impl TryFrom<OwnedValue> for ShutdownType {
//     type Error = zbus::Error;

//     fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
//         let value = <String>::try_from(value)?;
//         return Ok(Self::from(value.as_str()));
//     }
// }

// impl Type for ShutdownType {
//     fn signature() -> zvariant::Signature<'static> {
//         Signature::from_str_unchecked("s")
//     }
// }

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct SessionCreate {
//     uid: u32,
//     pid: u32,
//     service: String,
//     r#type: SessionType,
//     class: SessionClass,
//     desktop: String,
//     seat_id: String,
//     vtnr: u32,
//     tty: String,
//     display: String,
//     remote: bool,
//     remote_user: String,
//     remote_host: String,
//     //properties: Vec<(String, zvariant::Value<'a>)>,
// }

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct SeatPath {
    id: String,
    /// Name of session user
    path: OwnedObjectPath,
}

impl SeatPath {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl TryFrom<OwnedValue> for SeatPath {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <Structure>::try_from(value)?;
        return Ok(Self {
            id: <String>::try_from(value.fields()[0].clone())?,
            path: <OwnedObjectPath>::try_from(value.fields()[1].clone())?,
        });
    }
}

pub struct TimeStamp(Duration);

impl Deref for TimeStamp {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TimeStamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<OwnedValue> for TimeStamp {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <u64>::try_from(value)?;
        return Ok(Self(Duration::from_micros(value)));
    }
}
