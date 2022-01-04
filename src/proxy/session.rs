use std::time::Duration;

#[cfg(not(feature = "azync"))]
use zbus::blocking::Connection;
#[cfg(not(feature = "azync"))]
use zbus::blocking::Proxy;
#[cfg(feature = "azync")]
use zbus::Connection;
#[cfg(feature = "azync")]
use zbus::Proxy;
use zbus::Result;

use crate::{
    generated::session,
    types::{Device, IntoSessionPath, SeatPath, SessionClass, SessionState, SessionType, UserPath},
    DEFAULT_DEST,
};

/// Proxy wrapper for the logind `Session` dbus interface
///
/// All `get_*` methods are property getters
///
/// # Example
/// ```rust
/// use logind_zbus::ManagerProxy;
/// use logind_zbus::SessionProxy;
/// use zbus::blocking::Connection;
///
/// let connection = Connection::system().unwrap();
/// let manager = ManagerProxy::new(&connection).unwrap();
/// let sessions = manager.list_sessions().unwrap();
/// let session = SessionProxy::new(&connection, &sessions[0]).unwrap();
///
/// let time1 = session.get_timestamp().unwrap();
/// assert!(time1.as_secs() > 0);
///
/// let time2 = session.get_timestamp_monotonic().unwrap();
/// assert!(time2.as_secs() > 0);
/// ```
///
/// # Notes
/// All `connect_*` functions are signals and each of these functions
/// names reflect the underlying generated Proxy call. If desired the wrapped function
/// can be bypassed with:
/// ```ignore
/// *<SessionProxy>.connect_<function name>()
/// ```
#[cfg(not(feature = "azync"))]
pub struct SessionProxy<'a>(session::SessionProxyBlocking<'a>);

#[cfg(feature = "azync")]
pub struct SessionProxy<'a>(session::SessionProxy<'a>);

impl<'a> std::ops::Deref for SessionProxy<'a> {
    type Target = Proxy<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for SessionProxy<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> std::convert::AsRef<Proxy<'a>> for SessionProxy<'a> {
    fn as_ref(&self) -> &Proxy<'a> {
        &*self
    }
}

impl<'a> std::convert::AsMut<Proxy<'a>> for SessionProxy<'a> {
    fn as_mut(&mut self) -> &mut Proxy<'a> {
        &mut *self
    }
}

impl<'a> SessionProxy<'a> {
    pub fn new<S>(connection: &Connection, session: &'a S) -> Result<Self>
    where
        S: IntoSessionPath,
    {
        #[cfg(feature = "azync")]
        let s = session::SessionProxy::builder(&connection);

        #[cfg(not(feature = "azync"))]
        let s = session::SessionProxyBlocking::builder(&connection);

        Ok(Self(
            s.destination(DEFAULT_DEST)?
                .path(session.into_path_ref())?
                .build()?,
        ))
    }

    /// Borrow the underlying `SessionProxy` for use with zbus directly
    #[inline]
    #[cfg(feature = "azync")]
    pub fn get_proxy(&self) -> &session::SessionProxy {
        &self.0
    }

    /// Borrow the underlying `SessionProxy` for use with zbus directly
    #[inline]
    #[cfg(not(feature = "azync"))]
    pub fn get_proxy(&self) -> &session::SessionProxyBlocking {
        &self.0
    }

    /// Bring session to foreground
    #[inline]
    pub fn activate(&self) -> zbus::Result<()> {
        self.0.activate()
    }

    /// Send a signal to all processes of the user
    #[inline]
    pub fn kill(&self, who: UserPath, signal: i32) -> zbus::Result<()> {
        self.0.kill(&who.uid().to_string(), signal)
    }

    /// Ask session to activate its screen lock
    #[inline]
    pub fn lock(&self) -> zbus::Result<()> {
        self.0.lock()
    }

    /// Allows a session controller to synchronously pause a device after
    /// receiving a PauseDevice("pause") signal
    #[inline]
    pub fn pause_device_complete(&self, major: u32, minor: u32) -> zbus::Result<()> {
        self.0.pause_device_complete(major, minor)
    }

    /// Drops control of a given session. Closing the D-Bus connection implicitly
    /// releases control as well. This method also releases all devices for which
    /// the controller requested
    #[inline]
    pub fn release_control(&self) -> zbus::Result<()> {
        self.0.release_control()
    }

    /// Release a device (after TakeDevice). This is also implicitly done by
    /// `release_control()` or when closing the D-Bus connection.
    #[inline]
    pub fn release_device(&self, major: u32, minor: u32) -> zbus::Result<()> {
        self.0.release_device(major, minor)
    }

    /// Used to set the display brightness. This is intended to be used
    /// by the desktop environment and allows unprivileged programs to access
    /// hardware settings in a controlled way.
    #[inline]
    pub fn set_brightness(&self, subsystem: &str, name: &str, brightness: u32) -> zbus::Result<()> {
        self.0.set_brightness(subsystem, name, brightness)
    }

    /// SetIdleHint() is called by the session object to update the idle state
    /// of the session whenever it changes
    #[inline]
    pub fn set_idle_hint(&self, idle: bool) -> zbus::Result<()> {
        self.0.set_idle_hint(idle)
    }

    #[inline]
    pub fn set_locked_hint(&self, locked: bool) -> zbus::Result<()> {
        self.0.set_locked_hint(locked)
    }

    /// Allows the type of the session to be changed dynamically. It can only be
    /// called by session's current controller. If `take_control()` has not been
    /// called, this method will fail. In addition, the session type will be
    /// reset to its original value once control is released, either by calling
    /// `release_control()` or closing the D-Bus connection.
    #[inline]
    pub fn set_type(&self, type_: &str) -> zbus::Result<()> {
        self.0.set_type(type_)
    }

    /// Allows a process to take exclusive managed device access-control for that session
    #[inline]
    pub fn take_control(&self, force: bool) -> zbus::Result<()> {
        self.0.take_control(force)
    }

    /// Get a file descriptor for a specific device. Pass in the major and minor
    /// numbers of the character device and systemd-logind will return a file
    /// descriptor for the device.
    #[inline]
    pub fn take_device(&self, major: u32, minor: u32) -> zbus::Result<Device> {
        self.0.take_device(major, minor)
    }

    /// Forcibly terminate this session
    #[inline]
    pub fn terminate(&self) -> zbus::Result<()> {
        self.0.terminate()
    }

    /// Ask this session to deactivate its lock screen
    #[inline]
    pub fn unlock(&self) -> zbus::Result<()> {
        self.0.unlock()
    }

    /// Property: Is session is active, i.e. currently in the foreground.
    /// This field is semi-redundant due to State (`get_state()`).
    #[inline]
    pub fn get_active(&self) -> zbus::Result<bool> {
        self.0.active()
    }

    /// Property: the Kernel Audit session ID of the session if auditing is available.
    #[inline]
    pub fn get_audit(&self) -> zbus::Result<u32> {
        self.0.audit()
    }

    /// Property: The class of Session
    #[inline]
    pub fn get_class(&self) -> zbus::Result<SessionClass> {
        self.0.class().map(|v| v.as_str().into())
    }

    /// Property: Describes the desktop environment running in the session (if known)
    #[inline]
    pub fn get_desktop(&self) -> zbus::Result<String> {
        self.0.desktop()
    }

    /// The X11 display name if this is a graphical login. If not, this is an empty string.
    #[inline]
    pub fn get_display(&self) -> zbus::Result<String> {
        self.0.display()
    }

    /// Property: Session ID
    #[inline]
    pub fn get_id(&self) -> zbus::Result<String> {
        self.0.id()
    }

    #[inline]
    pub fn get_is_idle_hint(&self) -> zbus::Result<bool> {
        self.0.idle_hint()
    }

    #[inline]
    pub fn get_is_idle_since_hint(&self) -> zbus::Result<u64> {
        self.0.idle_since_hint()
    }

    #[inline]
    pub fn get_is_idle_since_hint_monotonic(&self) -> zbus::Result<u64> {
        self.0.idle_since_hint_monotonic()
    }

    /// Property: PID of the process that registered the session
    #[inline]
    pub fn get_leader(&self) -> zbus::Result<u32> {
        self.0.leader()
    }

    /// Property: shows the locked hint state of this session
    #[inline]
    pub fn get_locked_hint(&self) -> zbus::Result<bool> {
        self.0.locked_hint()
    }

    /// Property: The `User` name
    #[inline]
    pub fn get_name(&self) -> zbus::Result<String> {
        self.0.name()
    }

    /// Property: local or remote
    #[inline]
    pub fn get_is_remote(&self) -> zbus::Result<bool> {
        self.0.remote()
    }

    /// Property: None if not remote
    #[inline]
    pub fn get_remote_host(&self) -> zbus::Result<Option<String>> {
        self.0.remote_host().map(|s| {
            if s.is_empty() {
                return Some(s);
            }
            None
        })
    }

    /// Property: None if not remote
    #[inline]
    pub fn get_remote_user(&self) -> zbus::Result<Option<String>> {
        self.0.remote_user().map(|s| {
            if s.is_empty() {
                return Some(s);
            }
            None
        })
    }

    /// Property: systemd scope unit name of this session
    #[inline]
    pub fn get_scope(&self) -> zbus::Result<String> {
        self.0.scope()
    }

    /// Property: seat this session belongs to if there is any
    #[inline]
    pub fn get_seat(&self) -> zbus::Result<SeatPath> {
        self.0.seat()
    }

    /// Property: PAM service name that registered the session
    #[inline]
    pub fn get_service(&self) -> zbus::Result<String> {
        self.0.service()
    }

    /// Property: `State` of the session
    #[inline]
    pub fn get_state(&self) -> zbus::Result<SessionState> {
        self.0.state().map(|v| v.as_str().into())
    }

    /// Property: kernel TTY path of the session if this is a text login.
    /// If not this None.
    #[inline]
    pub fn get_tty(&self) -> zbus::Result<Option<String>> {
        self.0.tty().map(|s| {
            if s.is_empty() {
                return Some(s);
            }
            None
        })
    }

    /// Property: Get time since session was created (realtime)
    #[inline]
    pub fn get_timestamp(&self) -> zbus::Result<Duration> {
        self.0.timestamp().map(Duration::from_micros)
    }

    /// Property: Get time since session was created (wal time)
    #[inline]
    pub fn get_timestamp_monotonic(&self) -> zbus::Result<Duration> {
        self.0.timestamp_monotonic().map(Duration::from_micros)
    }

    /// Property: Session type
    #[inline]
    pub fn get_type(&self) -> zbus::Result<SessionType> {
        self.0.type_().map(|v| v.as_str().into())
    }

    /// Property: User the session belongs to
    #[inline]
    pub fn get_user(&self) -> zbus::Result<UserPath> {
        self.0.user()
    }

    /// Property: Virtual terminal number of the session if there is any, 0 otherwise.
    #[inline]
    pub fn get_vtnr(&self) -> zbus::Result<u32> {
        self.0.vtnr()
    }

    receive_signal_name!(receive_lock, session::LockStream, session::LockIterator);
    receive_signal_name!(
        receive_unlock,
        session::UnlockStream,
        session::UnlockIterator
    );
    receive_signal_name!(
        receive_pause_device,
        session::PauseStream,
        session::PauseDeviceIterator
    );
    receive_signal_name!(
        receive_resume_device,
        session::ResumeDeviceStream,
        session::ResumeDeviceIterator
    );
}

#[cfg(test)]
mod tests {
    use crate::ManagerProxy;
    use crate::SessionProxy;
    use core::panic;
    use zbus::blocking::Connection;

    #[test]
    fn timestamps() {
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();
        let session = SessionProxy::new(&connection, &sessions[0]).unwrap();

        let time1 = session.get_timestamp().unwrap();
        assert!(time1.as_secs() > 0);

        let time2 = session.get_timestamp_monotonic().unwrap();
        assert!(time2.as_secs() > 0);
    }

    #[test]
    fn list_active_session_types() {
        use crate::types::SessionType;
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();

        for session in sessions {
            let session_proxy = SessionProxy::new(&connection, &session).unwrap();
            if session_proxy.get_active().unwrap() {
                let st = session_proxy.get_type().unwrap();
                match st {
                    crate::types::SessionType::X11 => assert_eq!(st, SessionType::X11),
                    crate::types::SessionType::Wayland => assert_eq!(st, SessionType::Wayland),
                    crate::types::SessionType::MIR => assert_eq!(st, SessionType::MIR),
                    crate::types::SessionType::TTY => assert_eq!(st, SessionType::TTY),
                    crate::types::SessionType::Unspecified => {
                        assert_eq!(st, SessionType::Unspecified)
                    }
                    crate::types::SessionType::Invalid => panic!("session type response was bad"),
                }
            }
        }
    }

    #[test]
    fn properties() {
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();
        let session = SessionProxy::new(&connection, &sessions[0]).unwrap();

        assert!(session.get_active().is_ok());
        assert!(session.get_audit().is_ok());
        assert!(session.get_class().is_ok());
        assert!(session.get_desktop().is_ok());
        assert!(session.get_display().is_ok());
        assert!(session.get_id().is_ok());
        assert!(session.get_is_idle_hint().is_ok());
        assert!(session.get_is_idle_since_hint().is_ok());
        assert!(session.get_is_idle_since_hint_monotonic().is_ok());
        assert!(session.get_is_remote().is_ok());
        assert!(session.get_leader().is_ok());
        assert!(session.get_locked_hint().is_ok());
        assert!(session.get_name().is_ok());
        assert!(session.get_remote_host().is_ok());
        assert!(session.get_remote_user().is_ok());
        assert!(session.get_scope().is_ok());
        assert!(session.get_seat().is_ok());
        assert!(session.get_service().is_ok());
        assert!(session.get_state().is_ok());
        assert!(session.get_timestamp().is_ok());
        assert!(session.get_timestamp_monotonic().is_ok());
        // Special case
        //assert!(session_proxy.get_tty().is_ok());
        assert!(session.get_type().is_ok());
        assert!(session.get_user().is_ok());
        // Special case
        //assert!(session_proxy.get_vtnr().is_ok());
    }
}
