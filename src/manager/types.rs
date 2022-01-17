use serde::{Serialize, Deserialize};
use zvariant::{Type, OwnedValue, Structure, Signature};


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


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

impl TryFrom<OwnedValue> for InhibitThis {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let value = <String>::try_from(value)?;
        return Ok(Self::from(value.as_str()));
    }
}

impl Type for InhibitThis {
    fn signature() -> zvariant::Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

#[derive(Debug, PartialEq, Clone, Type, Serialize, Deserialize)]
pub struct Inhibitors {
    what: InhibitThis,
    who: String,
    why: String,
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
