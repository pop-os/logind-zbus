use std::str::FromStr;
use serde::{Deserialize, Serialize};
use zbus::fdo;
use zvariant::{OwnedObjectPath, OwnedValue, Structure, Type};

use crate::{enum_impl_serde_str, enum_impl_str_conv, IntoPath};

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

#[derive(Debug, PartialEq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum IsSupported {
    NA,
    Yes,
    No,
    Challenge,
}
enum_impl_serde_str!(IsSupported);
enum_impl_str_conv!(IsSupported, {
    "na": NA,
    "yes": Yes,
    "no": No,
    "challenge": Challenge,
});

#[derive(Debug, PartialEq, Clone, Type)]
#[zvariant(signature = "s")]
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

impl From<InhibitTypes> for String {
    fn from(s: InhibitTypes) -> Self {
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
enum_impl_serde_str!(InhibitType);
enum_impl_str_conv!(InhibitType, {
    "shutdown": Shutdown,
    "sleep": Sleep,
    "idle": Idle,
    "handle-power-key": HandlePowerKey,
    "handle-suspend-key": HandleSuspendKey,
    "handle-hibernate-key": HandleHibernateKey,
    "handle-lid-switch": HandleLidSwitch,
});

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
#[derive(Debug, PartialEq, Copy, Clone, Type)]
#[zvariant(signature = "s")]
pub enum Mode {
    /// Inhibitor is mandatory
    Block,
    /// Inhibitor delays to a certain time
    Delay,
}
enum_impl_serde_str!(Mode);
enum_impl_str_conv!(Mode, {
    "block": Block,
    "delay": Delay,
});

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
