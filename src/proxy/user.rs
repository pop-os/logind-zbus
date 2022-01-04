use std::time::Duration;

#[cfg(not(feature = "non_blocking"))]
use zbus::blocking::Connection;
#[cfg(not(feature = "non_blocking"))]
use zbus::blocking::Proxy;
#[cfg(feature = "non_blocking")]
use zbus::Connection;
#[cfg(feature = "non_blocking")]
use zbus::Proxy;
use zbus::Result;

use crate::{
    generated::user,
    types::{DbusPath, IntoUserPath, UserState},
    DEFAULT_DEST,
};
/// Proxy wrapper for the logind `User` dbus interface
///
/// All `get_*` methods are property getters
///
/// # Example
/// ```rust
/// use logind_zbus::ManagerProxy;
/// use logind_zbus::UserProxy;
/// use zbus::blocking::Connection;
///
/// let connection = Connection::system().unwrap();
/// let manager = ManagerProxy::new(&connection).unwrap();
/// let users = manager.list_users().unwrap();
///
/// let user = UserProxy::new(&connection, &users[0]).unwrap();
///
/// let time1 = user.get_timestamp().unwrap();
/// assert!(time1.as_secs() > 0);
///
/// let time2 = user.get_timestamp_monotonic().unwrap();
/// assert!(time2.as_secs() > 0);
/// ```
#[cfg(not(feature = "non_blocking"))]
pub struct UserProxy<'a>(user::UserProxyBlocking<'a>);

#[cfg(feature = "non_blocking")]
pub struct UserProxy<'a>(user::UserProxy<'a>);

impl<'a> std::ops::Deref for UserProxy<'a> {
    type Target = Proxy<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for UserProxy<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> std::convert::AsRef<Proxy<'a>> for UserProxy<'a> {
    fn as_ref(&self) -> &Proxy<'a> {
        &*self
    }
}

impl<'a> std::convert::AsMut<Proxy<'a>> for UserProxy<'a> {
    fn as_mut(&mut self) -> &mut Proxy<'a> {
        &mut *self
    }
}

impl<'a> UserProxy<'a> {
    #[inline]
    pub fn new<U>(connection: &Connection, user: &'a U) -> Result<Self>
    where
        U: IntoUserPath,
    {
        #[cfg(feature = "non_blocking")]
        let s = user::UserProxy::builder(&connection);

        #[cfg(not(feature = "non_blocking"))]
        let s = user::UserProxyBlocking::builder(&connection);

        Ok(Self(
            s.destination(DEFAULT_DEST)?
                .path(user.into_path_ref())?
                .build()?,
        ))
    }

    #[inline]
    pub fn kill(&self, signal_number: i32) -> Result<()> {
        self.0.kill(signal_number)
    }

    /// They'll be back
    #[inline]
    pub fn terminate(&self) -> Result<()> {
        self.0.terminate()
    }

    /// Property: which graphical session should be used as the primary UI
    /// display for the user. It is a structure encoding the session ID and
    /// the object path of the session to use.
    #[inline]
    pub fn get_display(&self) -> Result<DbusPath> {
        self.0.display()
    }

    /// Property: primary GID of the user
    #[inline]
    pub fn get_gid(&self) -> Result<u32> {
        self.0.GID()
    }

    /// Property: idle hint state of the user
    #[inline]
    pub fn get_is_idle_hint(&self) -> zbus::Result<bool> {
        self.0.idle_hint()
    }

    /// Property: idle hint state of the user
    #[inline]
    pub fn get_is_idle_since_hint(&self) -> zbus::Result<Duration> {
        self.0.idle_since_hint().map(Duration::from_micros)
    }

    /// Property: idle hint state of the user
    #[inline]
    pub fn get_is_idle_since_hint_monotonic(&self) -> zbus::Result<Duration> {
        self.0
            .idle_since_hint_monotonic()
            .map(Duration::from_micros)
    }

    /// Property: shows whether lingering is enabled for this user
    #[inline]
    pub fn get_linger(&self) -> Result<bool> {
        self.0.linger()
    }

    /// Property: Users name
    #[inline]
    pub fn get_name(&self) -> Result<String> {
        self.0.name()
    }

    // TODO: convert to OS path
    /// Property:  runtime path of the user, i.e. `$XDG_RUNTIME_DIR`
    #[inline]
    pub fn get_runtime_path(&self) -> Result<String> {
        self.0.runtime_path()
    }

    /// Property: the unit name of the user systemd service of this user.
    /// Each logged in user is assigned a user service that runs a user
    /// systemd instance. This is usually an instance of `user@.service`.
    #[inline]
    pub fn get_service(&self) -> Result<String> {
        self.0.service()
    }

    #[inline]
    pub fn get_sessions(&self) -> Result<Vec<DbusPath>> {
        let tmp = self.0.sessions()?;
        let mut sessions = Vec::with_capacity(tmp.len());
        for t in tmp {
            sessions.push(DbusPath::new(t.0, t.1))
        }
        Ok(sessions)
    }

    /// Property: unit name of the user systemd slice of this user. Each logged
    /// in user gets a private slice.
    #[inline]
    pub fn get_slice(&self) -> Result<String> {
        self.0.slice()
    }

    /// Property: Users state
    #[inline]
    pub fn get_state(&self) -> Result<UserState> {
        self.0.state().map(|s| s.as_str().into())
    }

    /// Property: login time of the user in microseconds since the epoch (realtime)
    #[inline]
    pub fn get_timestamp(&self) -> zbus::Result<Duration> {
        self.0.timestamp().map(Duration::from_micros)
    }

    /// Property: login time of the user in microseconds since the epoch (walltime)
    #[inline]
    pub fn get_timestamp_monotonic(&self) -> zbus::Result<Duration> {
        self.0.timestamp_monotonic().map(Duration::from_micros)
    }

    /// Property: Unix UID of the user
    #[inline]
    pub fn get_uid(&self) -> zbus::Result<u32> {
        self.0.UID()
    }
}

#[cfg(test)]
mod tests {
    use crate::ManagerProxy;
    use crate::UserProxy;
    use zbus::blocking::Connection;

    #[test]
    fn timestamps() {
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let users = manager.list_users().unwrap();
        let user = UserProxy::new(&connection, &users[0]).unwrap();

        let time1 = user.get_timestamp().unwrap();
        assert!(time1.as_secs() > 0);

        let time2 = user.get_timestamp_monotonic().unwrap();
        assert!(time2.as_secs() > 0);
    }

    #[test]
    fn properties() {
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let users = manager.list_users().unwrap();
        let user = UserProxy::new(&connection, &users[1]).unwrap();

        assert!(user.get_display().is_ok());
        // Special case. Exists only on users
        assert!(user.get_gid().is_ok());
        assert!(user.get_is_idle_hint().is_ok());
        assert!(user.get_is_idle_since_hint().is_ok());
        assert!(user.get_is_idle_since_hint_monotonic().is_ok());
        assert!(user.get_linger().is_ok());
        assert!(user.get_name().is_ok());
        assert!(user.get_runtime_path().is_ok());
        assert!(user.get_service().is_ok());
        assert!(user.get_slice().is_ok());
        assert!(user.get_sessions().is_ok());
        assert!(user.get_state().is_ok());
        assert!(user.get_timestamp().is_ok());
        assert!(user.get_timestamp_monotonic().is_ok());
        // Special case. Exists only on users
        assert!(user.get_uid().is_ok());
    }
}
