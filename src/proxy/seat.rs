use std::time::Duration;

use zbus::Result;
#[cfg(feature = "azync")]
use zbus::azync::Connection;
#[cfg(not(feature = "azync"))]
use zbus::Connection;
#[cfg(feature = "azync")]
use zbus::azync::Proxy;
#[cfg(not(feature = "azync"))]
use zbus::Proxy;

use crate::{DEFAULT_DEST, generated::seat, types::{DbusPath}};

/// Proxy wrapper for the logind `Seat` dbus interface
///
/// # Example
/// ```rust
/// use logind_zbus::ManagerInterface;
/// use logind_zbus::SeatInterface;
/// use zbus::Connection;
///
/// let connection = Connection::new_system().unwrap();
/// let manager = ManagerInterface::new(&connection).unwrap();
/// let seats = manager.list_seats().unwrap();
/// let seat = SeatInterface::new(&connection, &seats[0]).unwrap();
/// 
/// assert!(seat.get_active_session().is_ok());
/// 
/// assert!(manager.can_suspend().is_ok());
/// ```
pub struct SeatProxy<'a>(seat::SeatProxy<'a>);

impl<'a> std::ops::Deref for SeatProxy<'a> {
    type Target = Proxy<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for SeatProxy<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> std::convert::AsRef<Proxy<'a>> for SeatProxy<'a> {
    fn as_ref(&self) -> &Proxy<'a> {
        &*self
    }
}

impl<'a> std::convert::AsMut<Proxy<'a>> for SeatProxy<'a> {
    fn as_mut(&mut self) -> &mut Proxy<'a> {
        &mut *self
    }
}

impl<'a> SeatProxy<'a> {
    #[inline]
    pub fn new(connection: &Connection, path: &'a DbusPath) -> Result<Self> {
        Ok(Self(seat::SeatProxy::new_for(&connection, DEFAULT_DEST, path.path())?))
    }

    /// Brings the session with the specified ID into the foreground if the
    /// session_id matches
    #[inline]
    pub fn activate_session(&self, session_id: &str) -> zbus::Result<()> {
        self.0.activate_session(session_id)
    }

    /// Switches to the session on the virtual terminal 
    #[inline]
    pub fn switch_to(&self, vtnr: u32) -> zbus::Result<()> {
        self.0.switch_to(vtnr)
    }

    /// Switches to next session on the virtual terminal
    ///
    /// If there is no active session, switches to the first session.
    #[inline]
    pub fn switch_to_next(&self) -> zbus::Result<()> {
        self.0.switch_to_next()
    }

    /// Switches to previous session on the virtual terminal
    ///
    /// If there is no active session, switches to the first session.
    #[inline]
    pub fn switch_to_previous(&self) -> zbus::Result<()> {
        self.0.switch_to_previous()
    }

    /// Nuke the seat
    #[inline]
    pub fn terminate(&self) -> zbus::Result<()> {
        self.0.terminate()
    }

    /// Property: currently active session if there is one
    #[inline]
    pub fn get_active_session(&self) -> zbus::Result<DbusPath> {
        self.0.active_session()
    }

    /// Property: the session is suitable for graphical logins
    #[inline]
    pub fn get_can_graphical(&self) -> zbus::Result<bool> {
        self.0.can_graphical()
    }

    /// Property: the session is suitable for text logins
    #[inline]
    pub fn get_can_tty(&self) -> zbus::Result<bool> {
        self.0.can_tty()
    }

    /// Property: seat ID
    #[inline]
    pub fn get_id(&self) -> zbus::Result<String> {
        self.0.id()
    }

    /// Property: Is the seat idle
    #[inline]
    pub  fn get_idle_hint(&self) -> zbus::Result<bool> {
        self.0.idle_hint()
    }

    /// Property: timestamp of the last change of the idle hint boolean (realtime)
    #[inline]
    pub  fn get_idle_since_hint(&self) -> zbus::Result<Duration> {
        self.0.idle_since_hint().map(|t| Duration::from_micros(t))
    }

    /// Property: timestamp of the last change of the idle hint boolean (walltime)
    #[inline]
    pub  fn get_idle_since_hint_monotonic(&self) -> zbus::Result<Duration> {
        self.0.idle_since_hint().map(|t| Duration::from_micros(t))
    }

    // /// Property: sessions on this seat
    // #[inline]
    // pub fn get_sessions(&self) -> zbus::Result<Vec<DbusPath>> {
    //     self.0.sessions()
    // }
}

#[cfg(test)]
mod tests {
    use crate::ManagerProxy;
    use crate::SeatProxy;
    use zbus::Connection;

    #[test]
    fn timestamps() {
        let connection = Connection::new_system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let seats = manager.list_seats().unwrap();
        let seat = SeatProxy::new(&connection, &seats[0]).unwrap();

        assert!(seat.get_active_session().is_ok());
    }

    #[test]
    fn properties() {
        let connection = Connection::new_system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let seats = manager.list_seats().unwrap();
        let seat = SeatProxy::new(&connection, &seats[0]).unwrap();

        assert!(seat.get_active_session().is_ok());
        assert!(seat.get_can_graphical().is_ok());
        //assert!(seat.get_can_tty().is_ok());
        assert!(seat.get_id().is_ok());
        assert!(seat.get_idle_hint().is_ok());
        assert!(seat.get_idle_since_hint().is_ok());
        assert!(seat.get_idle_since_hint_monotonic().is_ok());
        //assert!(seat.get_sessions().is_ok());
    }
}