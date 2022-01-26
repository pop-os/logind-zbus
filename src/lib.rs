//! Reference <https://www.freedesktop.org/software/systemd/man/org.freedesktop.login1.html>

use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use zvariant::{OwnedObjectPath, OwnedValue};
pub mod manager;
pub mod seat;
pub mod session;
pub mod user;

//const DEFAULT_DEST: &str = "org.freedesktop.login1";

pub trait IntoPath {
    fn into_path(&self) -> OwnedObjectPath;
    fn into_path_ref(&self) -> &OwnedObjectPath;
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

#[cfg(test)]
mod tests {
    use crate::{manager::ManagerProxyBlocking, session::SessionProxyBlocking};

    #[test]
    fn basic_test() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();
        let session_proxy = SessionProxyBlocking::builder(&connection)
            .path(sessions[0].path())
            .unwrap()
            .build()
            .unwrap();

        assert!(session_proxy.seat().is_ok());
    }
}
