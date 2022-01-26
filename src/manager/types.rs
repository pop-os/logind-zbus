use serde::{Deserialize, Serialize};
use zvariant::{OwnedObjectPath, OwnedValue, Signature, Structure, Type, Value};

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
pub enum InhibitThis {
    Shutdown,
    Sleep,
    Idle,
    HandlePowerKey,
    HandleSuspendKey,
    HandleHibernateKey,
    HandleLidSwitch,
    Invalid,
}

impl From<&str> for InhibitThis {
    fn from(s: &str) -> Self {
        match s.trim() {
            "shutdown" => Self::Shutdown,
            "sleep" => Self::Sleep,
            "idle" => Self::Idle,
            "handle-power-key" => Self::HandlePowerKey,
            "handle-suspend-key" => Self::HandleSuspendKey,
            "handle-hibernate-key" => Self::HandleHibernateKey,
            "handle-lid-switch" => Self::HandleLidSwitch,
            _ => Self::Invalid,
        }
    }
}

impl From<&InhibitThis> for &str {
    fn from(s: &InhibitThis) -> Self {
        match s {
            InhibitThis::Shutdown => "shutdown",
            InhibitThis::Sleep => "sleep",
            InhibitThis::Idle => "idle",
            InhibitThis::HandlePowerKey => "handle-power-key",
            InhibitThis::HandleSuspendKey => "handle-suspend-key",
            InhibitThis::HandleHibernateKey => "handle-hibernate-key",
            InhibitThis::HandleLidSwitch => "handle-lid-switch",
            InhibitThis::Invalid => "invalid",
        }
    }
}

impl From<InhibitThis> for &str {
    fn from(s: InhibitThis) -> Self {
        <&str>::from(&s)
    }
}

impl Serialize for InhibitThis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for InhibitThis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(InhibitThis::from(s.as_str()))
    }
}

impl Type for InhibitThis {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

#[derive(Debug, PartialEq, Clone, Type, Value, OwnedValue, Serialize, Deserialize)]
pub struct InhibitLock {
    /// What this lock is inhibiting
    what: String,
    /// The name or ID of what is inhibiting, for example the applicaiton name creating this lock
    who: String,
    /// A description of why the lock was created
    why: String,
    /// The lock behaviour
    mode: String,
}

impl InhibitLock {
    pub fn new(
        what: InhibitThis,
        who: String,
        why: String,
        mode: Mode,
    ) -> Self {
        Self {
            what: <&str>::from(what).to_string(),
            who,
            why,
            mode: <&str>::from(mode).to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct InhibitorLock {
    /// What this lock is inhibiting
    what: InhibitThis,
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
#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub enum Mode {
    /// Inhibitor is mandatory
    Block,
    /// Inhibitor delays to a certain time
    Delay,
}

impl From<Mode> for &str {
    fn from(m: Mode) -> Self {
        match m {
            Mode::Block => "block",
            Mode::Delay => "delay",
        }
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
