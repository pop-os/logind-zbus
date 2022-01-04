use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use zvariant::{OwnedObjectPath, OwnedValue, Structure};
use zvariant_derive::Type;

/// If `IsSupported::Invalid` then the response from
/// logind was not well defined.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum IsSupported {
    NA,
    Yes,
    No,
    Challenge,
    Invalid,
}

impl From<&str> for IsSupported {
    fn from(s: &str) -> Self {
        match s.trim() {
            "na" => Self::NA,
            "yes" => Self::Yes,
            "no" => Self::No,
            _ => Self::NA,
        }
    }
}

/// If `ShutdownType::Invalid` then the response from
/// logind was not well defined.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ShutdownType {
    PowerOff,
    DryPowerOff,
    Reboot,
    DryReboot,
    Halt,
    DryHalt,
    Invalid,
}

impl From<&str> for ShutdownType {
    fn from(s: &str) -> Self {
        match s.trim() {
            "poweroff" => Self::PowerOff,
            "dry-poweroff" => Self::DryPowerOff,
            "reboot" => Self::Reboot,
            "dry-reboot" => Self::DryReboot,
            "halt" => Self::Halt,
            "dry-halt" => Self::DryHalt,
            _ => Self::Invalid,
        }
    }
}

impl From<ShutdownType> for &str {
    fn from(s: ShutdownType) -> Self {
        match s {
            ShutdownType::PowerOff => "poweroff",
            ShutdownType::DryPowerOff => "dry-poweroff",
            ShutdownType::Reboot => "reboot",
            ShutdownType::DryReboot => "dry-reboot",
            ShutdownType::Halt => "halt",
            ShutdownType::DryHalt => "dry-halt",
            ShutdownType::Invalid => "invalid",
        }
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

pub trait IntoSessionPath {
    fn into_path(&self) -> OwnedObjectPath;
    fn into_path_ref(&self) -> &OwnedObjectPath;
}

#[derive(Debug, PartialEq, Type, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    sid: String,
    /// User ID
    uid: u32,
    /// Name of session user
    user: String,
    seat: String,
    path: OwnedObjectPath,
}

impl SessionInfo {
    pub fn sid(&self) -> &str {
        &self.sid
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn seat(&self) -> &str {
        &self.seat
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl IntoSessionPath for SessionInfo {
    fn into_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    fn into_path_ref(&self) -> &OwnedObjectPath {
        &self.path
    }
}

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

impl IntoSessionPath for SessionPath {
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

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct ScheduledShutdown {
    id: String,
    /// Name of session user
    time: u64,
}

impl ScheduledShutdown {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn time(&self) -> u64 {
        self.time
    }
}

impl TryFrom<OwnedValue> for ScheduledShutdown {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <Structure>::try_from(value)?;
        return Ok(Self {
            id: <String>::try_from(value.fields()[0].clone())?,
            time: <u64>::try_from(value.fields()[1].clone())?,
        });
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct DbusPath {
    id: String,
    /// Name of session user
    path: OwnedObjectPath,
}

impl DbusPath {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl TryFrom<OwnedValue> for DbusPath {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <Structure>::try_from(value)?;
        return Ok(Self {
            id: <String>::try_from(value.fields()[0].clone())?,
            path: <OwnedObjectPath>::try_from(value.fields()[1].clone())?,
        });
    }
}

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

pub trait IntoUserPath {
    fn into_path(&self) -> OwnedObjectPath;
    fn into_path_ref(&self) -> &OwnedObjectPath;
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct UserInfo {
    uid: u32,
    name: String,
    /// Name of session user
    path: OwnedObjectPath,
}

impl UserInfo {
    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &OwnedObjectPath {
        &self.path
    }
}

impl IntoUserPath for UserInfo {
    fn into_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    fn into_path_ref(&self) -> &OwnedObjectPath {
        &self.path
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct UserPath {
    uid: u32,
    /// Name of session user
    path: OwnedObjectPath,
}

impl UserPath {
    pub fn uid(&self) -> u32 {
        self.uid
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
            uid: <u32>::try_from(value.fields()[0].clone())?,
            path: <OwnedObjectPath>::try_from(value.fields()[1].clone())?,
        });
    }
}

impl IntoUserPath for UserPath {
    fn into_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    fn into_path_ref(&self) -> &OwnedObjectPath {
        &self.path
    }
}

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
