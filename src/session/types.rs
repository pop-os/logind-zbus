use serde::{Deserialize, Serialize};
use zvariant::{OwnedObjectPath, OwnedValue, Signature, Structure, Type};

use crate::IntoPath;

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

/// The type of Session. If `State::Invalid` then the response from
/// logind was not well defined.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum SessionType {
    X11,
    Wayland,
    MIR,
    TTY,
    Unspecified,
    Invalid,
}

impl From<&str> for SessionType {
    fn from(s: &str) -> Self {
        match s {
            "wayland" => SessionType::Wayland,
            "x11" => SessionType::X11,
            "mir" => SessionType::MIR,
            "tty" => SessionType::TTY,
            "unspecified" => SessionType::Unspecified,
            _ => SessionType::Invalid,
        }
    }
}

impl TryFrom<OwnedValue> for SessionType {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <String>::try_from(value)?;
        return Ok(Self::from(value.as_str()));
    }
}

impl Type for SessionType {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

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

/// Class of Session. If `SessionClass::Invalid` then the response from
/// logind was not well defined.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum SessionClass {
    User,
    Greeter,
    LockScreen,
    Invalid,
}

impl From<&str> for SessionClass {
    fn from(s: &str) -> Self {
        match s.trim() {
            "user" => Self::User,
            "greeter" => Self::Greeter,
            "lock-screen" => Self::LockScreen,
            _ => Self::Invalid,
        }
    }
}

impl TryFrom<OwnedValue> for SessionClass {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <String>::try_from(value)?;
        return Ok(Self::from(value.as_str()));
    }
}

/// State of a session. If `SessionState::Invalid` then the response from
/// logind was not well defined.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum SessionState {
    Online,
    Active,
    Closing,
    Invalid,
}

impl From<&str> for SessionState {
    fn from(s: &str) -> Self {
        match s.trim() {
            "online" => Self::Online,
            "active" => Self::Active,
            "closing" => Self::Closing,
            _ => Self::Invalid,
        }
    }
}

impl TryFrom<OwnedValue> for SessionState {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <String>::try_from(value)?;
        return Ok(Self::from(value.as_str()));
    }
}

impl Type for SessionState {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}
