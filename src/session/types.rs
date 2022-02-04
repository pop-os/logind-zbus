use std::str::FromStr;
use serde::{Deserialize, Serialize};
use zbus::fdo;
use zvariant::{OwnedObjectPath, OwnedValue, Structure, Type};

use crate::{enum_impl_serde_str, enum_impl_str_conv, IntoPath, impl_try_from_owned_as_str};

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct User {
    uid: u32,
    /// Name of session user
    path: OwnedObjectPath,
}

impl User {
    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl TryFrom<OwnedValue> for User {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <Structure>::try_from(value)?;
        return Ok(Self {
            uid: <u32>::try_from(value.fields()[0].clone())?,
            path: <OwnedObjectPath>::try_from(value.fields()[1].clone())?,
        });
    }
}

impl IntoPath for User {
    fn into_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    fn into_path_ref(&self) -> &OwnedObjectPath {
        &self.path
    }
}

/// The type of Session
#[derive(Debug, PartialEq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum SessionType {
    X11,
    Wayland,
    MIR,
    TTY,
    Unspecified,
}
enum_impl_serde_str!(SessionType);
impl_try_from_owned_as_str!(SessionType);
enum_impl_str_conv!(SessionType, {
    "wayland": Wayland,
    "x11": X11,
    "mir": MIR,
    "tty": TTY,
    "unspecified": Unspecified,
});

#[derive(Debug, PartialEq, Type, Serialize, Deserialize)]
pub struct Device {
    file_descriptor: std::os::unix::io::RawFd,
    inactive: bool,
}

impl Device {
    pub fn file_descriptor(&self) -> std::os::unix::io::RawFd {
        self.file_descriptor
    }

    pub fn inactive(&self) -> bool {
        self.inactive
    }
}

/// Class of Session
#[derive(Debug, PartialEq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum SessionClass {
    User,
    Greeter,
    LockScreen,
}
enum_impl_serde_str!(SessionClass);
impl_try_from_owned_as_str!(SessionClass);
enum_impl_str_conv!(SessionClass, {
    "user": User,
    "greeter": Greeter,
    "lock-screen": LockScreen,
});

/// State of a session
#[derive(Debug, PartialEq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum SessionState {
    Online,
    Active,
    Closing,
}
enum_impl_serde_str!(SessionState);
impl_try_from_owned_as_str!(SessionState);
enum_impl_str_conv!(SessionState, {
    "online": Online,
    "active": Active,
    "closing": Closing,
});
