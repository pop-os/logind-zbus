use std::time::Duration;

use zbus::Result;
#[cfg(feature = "azync")]
use zbus::azync::Connection;
#[cfg(not(feature = "azync"))]
use zbus::Connection;

use crate::{DEFAULT_DEST, generated::user, types::{DbusPath, UserInfo, UserState}};
/// Proxy wrapper for the logind `User` dbus interface
///
/// All `get_*` methods are property getters
///
/// # Example
/// ```rust
/// use logind_zbus::ManagerInterface;
/// use logind_zbus::UserInterface;
/// use zbus::Connection;
///
/// let connection = Connection::new_system().unwrap();
/// let manager = ManagerInterface::new(&connection).unwrap();
/// let users = manager.list_users().unwrap();
/// let user = UserInterface::new(&connection, &users[0]).unwrap();
/// 
/// let time1 = user.get_timestamp().unwrap();
/// assert!(time1.as_secs() > 0);
/// 
/// let time2 = user.get_timestamp_monotonic().unwrap();
/// assert!(time2.as_secs() > 0);
/// ```
pub struct UserInterface<'a> {
    _inner: user::UserProxy<'a>,
}

impl<'a> UserInterface<'a> {
    #[inline]
    pub fn new(connection: &Connection, user: &'a UserInfo) -> Result<Self> {
        Ok(Self {
            _inner: user::UserProxy::new_for(&connection, DEFAULT_DEST, user.path())?,
        })
    }

    /// Borrow the underlying `UserProxy` for use with zbus directly
    pub fn get_proxy(&self) -> &user::UserProxy {
        &self._inner
    }

    #[inline]
    pub fn kill(&self, signal_number: i32) -> Result<()> {
        self._inner.kill(signal_number)
    }

    /// They'll be back
    #[inline]
    pub fn terminate(&self) -> Result<()> {
        self._inner.terminate()
    }

    /// Property: which graphical session should be used as the primary UI
    /// display for the user. It is a structure encoding the session ID and
    /// the object path of the session to use.
    #[inline]
    pub fn get_display(&self) -> Result<DbusPath> {
        self._inner.display()
    }

    /// Property: primary GID of the user
    #[inline]
    pub fn get_gid(&self) -> Result<u32> {
        self._inner.gid()
    }

    /// Property: idle hint state of the user
    #[inline]
    pub fn get_is_idle_hint(&self) -> zbus::Result<bool> {
        self._inner.idle_hint()
    }

    /// Property: idle hint state of the user
    #[inline]
    pub fn get_is_idle_since_hint(&self) -> zbus::Result<Duration> {
        self._inner.idle_since_hint().map(|t| Duration::from_micros(t))
    }

    /// Property: idle hint state of the user
    #[inline]
    pub fn get_is_idle_since_hint_monotonic(&self) -> zbus::Result<Duration> {
        self._inner.idle_since_hint_monotonic().map(|t| Duration::from_micros(t))
    }

    /// Property: shows whether lingering is enabled for this user
    #[inline]
    pub fn get_linger(&self) -> Result<bool> {
        self._inner.linger()
    }

    /// Property: Users name
    #[inline]
    pub fn get_name(&self) -> Result<String> {
        self._inner.name()
    }

    // TODO: convert to OS path
    /// Property:  runtime path of the user, i.e. `$XDG_RUNTIME_DIR`
    #[inline]
    pub fn get_runtime_path(&self) -> Result<String> {
        self._inner.runtime_path()
    }

    /// Property: the unit name of the user systemd service of this user.
    /// Each logged in user is assigned a user service that runs a user
    /// systemd instance. This is usually an instance of `user@.service`.
    #[inline]
    pub fn get_service(&self) -> Result<String> {
        self._inner.service()
    }

    // #[inline]
    // pub fn get_sessions(&self) -> Result<Vec<String>> {
    //     self._inner.sessions()
    // }

    /// Property: unit name of the user systemd slice of this user. Each logged
    /// in user gets a private slice.
    #[inline]
    pub fn get_slice(&self) -> Result<String> {
        self._inner.slice()
    }

    /// Property: Users state
    #[inline]
    pub fn get_state(&self) -> Result<UserState> {
        self._inner.state().map(|s| s.as_str().into())
    }

    /// Property: login time of the user in microseconds since the epoch (realtime)
    #[inline]
    pub fn get_timestamp(&self) -> zbus::Result<Duration> {
        self._inner.timestamp().map(|t| Duration::from_micros(t))
    }

    /// Property: login time of the user in microseconds since the epoch (walltime)
    #[inline]
    pub fn get_timestamp_monotonic(&self) -> zbus::Result<Duration> {
        self._inner.timestamp_monotonic().map(|t| Duration::from_micros(t))
    }

    /// Property: Unix UID of the user
    #[inline]
    pub fn get_uid(&self) -> zbus::Result<u32> {
        self._inner.uid()
    }
}

#[cfg(test)]
mod tests {
    use crate::ManagerInterface;
    use crate::UserInterface;
    use zbus::Connection;

    #[test]
    fn timestamps() {
        let connection = Connection::new_system().unwrap();
        let manager = ManagerInterface::new(&connection).unwrap();
        let users = manager.list_users().unwrap();
        let user = UserInterface::new(&connection, &users[0]).unwrap();

        let time1 = user.get_timestamp().unwrap();
        assert!(time1.as_secs() > 0);

        let time2 = user.get_timestamp_monotonic().unwrap();
        assert!(time2.as_secs() > 0);
    }

    #[test]
    fn properties() {
        let connection = Connection::new_system().unwrap();
        let manager = ManagerInterface::new(&connection).unwrap();
        let users = manager.list_users().unwrap();
        let user = UserInterface::new(&connection, &users[0]).unwrap();

        assert!(user.get_display().is_ok());
        // Special case. Not exists on first user
        //assert!(user_proxy.get_gid().is_ok());
        assert!(user.get_is_idle_hint().is_ok());
        assert!(user.get_is_idle_since_hint().is_ok());
        assert!(user.get_is_idle_since_hint_monotonic().is_ok());
        assert!(user.get_linger().is_ok());
        assert!(user.get_name().is_ok());
        assert!(user.get_runtime_path().is_ok());
        assert!(user.get_service().is_ok());
        assert!(user.get_slice().is_ok());
        //assert!(user.get_sessions().is_ok());
        assert!(user.get_state().is_ok());
        assert!(user.get_timestamp().is_ok());
        assert!(user.get_timestamp_monotonic().is_ok());
        // Special case. Not exists on first user
        //assert!(user_proxy.get_uid().is_ok());
    }
}