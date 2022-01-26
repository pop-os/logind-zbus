use std::str::FromStr;

use serde::{Deserialize, Serialize};
use zbus::fdo;
use zvariant::{OwnedObjectPath, OwnedValue, Signature, Structure, Type};

use crate::IntoPath;

/// Basic user information
#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct UserInfo {
    /// User ID
    uid: u32,
    /// User name
    name: String,
    /// DBUS path to this user
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

impl IntoPath for UserInfo {
    fn into_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    fn into_path_ref(&self) -> &OwnedObjectPath {
        &self.path
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct ScheduledShutdown {
    /// Name of the shutdown
    id: String,
    /// Time in micros
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
            "challenge" => Self::Challenge,
            _ => Self::Invalid,
        }
    }
}

impl From<String> for IsSupported {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl TryFrom<OwnedValue> for IsSupported {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <String>::try_from(value)?;
        Ok(Self::from(value))
    }
}

impl Type for IsSupported {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InhibitTypes(Vec<InhibitType>);

impl FromStr for InhibitTypes {
    type Err = fdo::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = Vec::new();
        for chunk in s.split(':') {
            buf.push(InhibitType::from_str(chunk)?);
        }
        Ok(Self(buf))
    }
}

impl From<&InhibitTypes> for String {
    fn from(s: &InhibitTypes) -> Self {
        let mut string = String::new();
        for (i, inhibit) in s.0.iter().enumerate() {
            if i > 0 && i < s.0.len() {
                string.push(':');
            }
            string.push_str((*inhibit).into());
        }
        string
    }
}

impl From<InhibitTypes> for String {
    fn from(s: InhibitTypes) -> Self {
        String::from(&s)
    }
}

impl Serialize for InhibitTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(String::from(self).as_str())
    }
}

impl<'de> Deserialize<'de> for InhibitTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        InhibitTypes::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl Type for InhibitTypes {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Type)]
#[zvariant(signature = "s")]
pub enum InhibitType {
    Shutdown,
    Sleep,
    Idle,
    HandlePowerKey,
    HandleSuspendKey,
    HandleHibernateKey,
    HandleLidSwitch,
}

impl FromStr for InhibitType {
    type Err = fdo::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s.trim() {
            "shutdown" => Self::Shutdown,
            "sleep" => Self::Sleep,
            "idle" => Self::Idle,
            "handle-power-key" => Self::HandlePowerKey,
            "handle-suspend-key" => Self::HandleSuspendKey,
            "handle-hibernate-key" => Self::HandleHibernateKey,
            "handle-lid-switch" => Self::HandleLidSwitch,
            _ => return Err(fdo::Error::IOError(format!("{} is an invalid variant", s))),
        };
        Ok(res)
    }
}

impl From<InhibitType> for &str {
    fn from(s: InhibitType) -> Self {
        match s {
            InhibitType::Shutdown => "shutdown",
            InhibitType::Sleep => "sleep",
            InhibitType::Idle => "idle",
            InhibitType::HandlePowerKey => "handle-power-key",
            InhibitType::HandleSuspendKey => "handle-suspend-key",
            InhibitType::HandleHibernateKey => "handle-hibernate-key",
            InhibitType::HandleLidSwitch => "handle-lid-switch",
        }
    }
}

impl Serialize for InhibitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str((*self).into())
    }
}

impl<'de> Deserialize<'de> for InhibitType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        InhibitType::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct Inhibitor {
    /// What this lock is inhibiting
    what: InhibitTypes,
    /// The name or ID of what is inhibiting, for example the applicaiton name creating this lock
    who: String,
    /// A description of why the lock was created
    why: String,
    /// The lock behaviour
    mode: Mode,
    user_id: u32,
    process_id: u32,
}

/// Used to determine behaviour of inhibitors
#[derive(Debug, PartialEq, Clone)]
pub enum Mode {
    /// Inhibitor is mandatory
    Block,
    /// Inhibitor delays to a certain time
    Delay,
}

impl FromStr for Mode {
    type Err = fdo::Error;

    fn from_str(m: &str) -> Result<Self, Self::Err> {
        let res = match m {
            "block" => Mode::Block,
            "delay" => Mode::Delay,
            _ => return Err(fdo::Error::IOError(format!("{} is an invalid variant", m))),
        };
        Ok(res)
    }
}

impl From<&Mode> for &str {
    fn from(m: &Mode) -> Self {
        match m {
            Mode::Block => "block",
            Mode::Delay => "delay",
        }
    }
}

impl From<Mode> for &str {
    fn from(s: Mode) -> Self {
        <&str>::from(&s)
    }
}

impl Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Mode::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl Type for Mode {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

#[derive(Debug, PartialEq, Type, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    sid: String,
    /// User ID
    uid: u32,
    /// Name of session user
    user: String,
    /// The session seat label
    seat: String,
    /// DBUS path for this session
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

impl IntoPath for SessionInfo {
    fn into_path(&self) -> OwnedObjectPath {
        self.path.clone()
    }

    fn into_path_ref(&self) -> &OwnedObjectPath {
        &self.path
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct SeatPath {
    /// The seat label
    id: String,
    /// DBUS path for this seat
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
