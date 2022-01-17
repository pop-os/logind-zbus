//! # DBus interface proxy for: `org.freedesktop.login1.Session`

#![allow(non_snake_case)]

use zbus::dbus_proxy;

use crate::types::{Device, SeatPath, SessionClass, SessionState, SessionType, TimeStamp, User};

#[dbus_proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1"
)]
trait Session {
    /// Activate method
    #[inline]
    fn activate(&self) -> zbus::Result<()>;

    /// Kill method
    #[inline]
    fn kill(&self, who: &str, signal_number: i32) -> zbus::Result<()>;

    /// Lock method
    #[inline]
    fn lock(&self) -> zbus::Result<()>;

    /// PauseDeviceComplete method
    #[inline]
    fn pause_device_complete(&self, major: u32, minor: u32) -> zbus::Result<()>;

    /// ReleaseControl method
    #[inline]
    fn release_control(&self) -> zbus::Result<()>;

    /// ReleaseDevice method
    #[inline]
    fn release_device(&self, major: u32, minor: u32) -> zbus::Result<()>;

    /// SetBrightness method
    #[inline]
    fn set_brightness(&self, subsystem: &str, name: &str, brightness: u32) -> zbus::Result<()>;

    /// SetIdleHint method
    #[inline]
    fn set_idle_hint(&self, idle: bool) -> zbus::Result<()>;

    /// SetLockedHint method
    #[inline]
    fn set_locked_hint(&self, locked: bool) -> zbus::Result<()>;

    /// SetType method
    #[inline]
    fn set_type(&self, type_: &str) -> zbus::Result<()>;

    /// TakeControl method
    #[inline]
    fn take_control(&self, force: bool) -> zbus::Result<()>;

    /// TakeDevice method
    #[inline]
    fn take_device(&self, major: u32, minor: u32) -> zbus::Result<Device>;

    /// Terminate method
    #[inline]
    fn terminate(&self) -> zbus::Result<()>;

    /// Unlock method
    #[inline]
    fn unlock(&self) -> zbus::Result<()>;

    /// Lock signal
    #[dbus_proxy(signal)]
    #[inline]
    fn lock(&self) -> zbus::Result<()>;

    /// PauseDevice signal
    #[dbus_proxy(signal)]
    #[inline]
    fn pause_device(&self, major: u32, minor: u32, type_: &str) -> zbus::Result<()>;

    /// ResumeDevice signal
    #[dbus_proxy(signal)]
    #[inline]
    fn resume_device(
        &self,
        major: u32,
        minor: u32,
        fd: std::os::unix::io::RawFd,
    ) -> zbus::Result<()>;

    /// Unlock signal
    #[dbus_proxy(signal)]
    #[inline]
    fn unlock(&self) -> zbus::Result<()>;

    /// Active property
    #[dbus_proxy(property)]
    #[inline]
    fn active(&self) -> zbus::Result<bool>;

    /// Audit property
    #[dbus_proxy(property)]
    #[inline]
    fn audit(&self) -> zbus::Result<u32>;

    /// Class property
    #[dbus_proxy(property)]
    #[inline]
    fn class(&self) -> zbus::Result<SessionClass>;

    /// Desktop property
    #[dbus_proxy(property)]
    #[inline]
    fn desktop(&self) -> zbus::Result<String>;

    /// Display property
    #[dbus_proxy(property)]
    #[inline]
    fn display(&self) -> zbus::Result<String>;

    /// Id property
    #[dbus_proxy(property)]
    #[inline]
    fn id(&self) -> zbus::Result<String>;

    /// IdleHint property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_hint(&self) -> zbus::Result<bool>;

    /// IdleSinceHint property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint(&self) -> zbus::Result<TimeStamp>;

    /// IdleSinceHintMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint_monotonic(&self) -> zbus::Result<TimeStamp>;

    /// Leader property
    #[dbus_proxy(property)]
    #[inline]
    fn leader(&self) -> zbus::Result<u32>;

    /// LockedHint property
    #[dbus_proxy(property)]
    #[inline]
    fn locked_hint(&self) -> zbus::Result<bool>;

    /// Name property
    #[dbus_proxy(property)]
    #[inline]
    fn name(&self) -> zbus::Result<String>;

    /// Remote property
    #[dbus_proxy(property)]
    #[inline]
    fn remote(&self) -> zbus::Result<bool>;

    /// RemoteHost property
    #[dbus_proxy(property)]
    #[inline]
    fn remote_host(&self) -> zbus::Result<String>;

    /// RemoteUser property
    #[dbus_proxy(property)]
    #[inline]
    fn remote_user(&self) -> zbus::Result<String>;

    /// Scope property
    #[dbus_proxy(property)]
    #[inline]
    fn scope(&self) -> zbus::Result<String>;

    /// Seat property
    #[dbus_proxy(property)]
    #[inline]
    fn seat(&self) -> zbus::Result<SeatPath>;

    /// Service property
    #[dbus_proxy(property)]
    #[inline]
    fn service(&self) -> zbus::Result<String>;

    /// State property
    #[dbus_proxy(property)]
    #[inline]
    fn state(&self) -> zbus::Result<SessionState>;

    /// TTY property
    #[dbus_proxy(property)]
    #[inline]
    fn TTY(&self) -> zbus::Result<String>;

    /// Timestamp property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp(&self) -> zbus::Result<TimeStamp>;

    /// TimestampMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp_monotonic(&self) -> zbus::Result<TimeStamp>;

    /// Type property
    #[dbus_proxy(property)]
    #[inline]
    fn type_(&self) -> zbus::Result<SessionType>;

    /// User property
    #[dbus_proxy(property)]
    #[inline]
    fn user(&self) -> zbus::Result<User>;

    /// VTNr property
    #[dbus_proxy(property)]
    #[inline]
    fn VTNr(&self) -> zbus::Result<u32>;
}

#[cfg(test)]
mod tests {
    use crate::manager::ManagerProxy;
    use crate::manager::ManagerProxyBlocking;
    use crate::session::SessionProxy;
    use crate::session::SessionProxyBlocking;
    use crate::types::SessionType;
    use core::panic;
    use futures_lite::future;

    #[test]
    fn timestamps() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();
        let session = SessionProxyBlocking::builder(&connection)
            .path(sessions[0].path())
            .unwrap()
            .build()
            .unwrap();

        let time1 = session.timestamp().unwrap();
        assert!(time1.as_secs() > 0);

        let time2 = session.timestamp_monotonic().unwrap();
        assert!(time2.as_secs() > 0);
    }

    #[test]
    fn list_active_session_types() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();

        for session in sessions {
            let session_proxy = SessionProxyBlocking::builder(&connection)
                .path(session.path())
                .unwrap()
                .build()
                .unwrap();
            if session_proxy.active().unwrap() {
                let st = session_proxy.type_().unwrap();
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
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();
        let session = SessionProxyBlocking::builder(&connection)
            .path(sessions[0].path())
            .unwrap()
            .build()
            .unwrap();

        assert!(session.active().is_ok());
        assert!(session.audit().is_ok());
        assert!(session.class().is_ok());
        assert!(session.desktop().is_ok());
        assert!(session.display().is_ok());
        assert!(session.id().is_ok());
        assert!(session.idle_hint().is_ok());
        assert!(session.idle_since_hint().is_ok());
        assert!(session.idle_since_hint_monotonic().is_ok());
        assert!(session.remote().is_ok());
        assert!(session.leader().is_ok());
        assert!(session.locked_hint().is_ok());
        assert!(session.name().is_ok());
        assert!(session.remote_host().is_ok());
        assert!(session.remote_user().is_ok());
        assert!(session.scope().is_ok());
        assert!(session.seat().is_ok());
        assert!(session.service().is_ok());
        assert!(session.state().is_ok());
        assert!(session.timestamp().is_ok());
        assert!(session.timestamp_monotonic().is_ok());
        assert!(session.TTY().is_ok());
        assert!(session.type_().is_ok());
        assert!(session.user().is_ok());
        assert!(session.VTNr().is_ok());
    }

    #[test]
    fn timestamps_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();
            let sessions = manager.list_sessions().await.unwrap();
            let session = SessionProxy::builder(&connection)
                .path(sessions[0].path())
                .unwrap()
                .build()
                .await
                .unwrap();

            let time1 = session.timestamp().await.unwrap();
            assert!(time1.as_secs() > 0);

            let time2 = session.timestamp_monotonic().await.unwrap();
            assert!(time2.as_secs() > 0);
        })
    }

    #[test]
    fn list_active_session_types_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();
            let sessions = manager.list_sessions().await.unwrap();

            for session in sessions {
                let session_proxy = SessionProxy::builder(&connection)
                    .path(session.path())
                    .unwrap()
                    .build()
                    .await
                    .unwrap();
                if session_proxy.active().await.unwrap() {
                    let st = session_proxy.type_().await.unwrap();
                    match st {
                        crate::types::SessionType::X11 => assert_eq!(st, SessionType::X11),
                        crate::types::SessionType::Wayland => assert_eq!(st, SessionType::Wayland),
                        crate::types::SessionType::MIR => assert_eq!(st, SessionType::MIR),
                        crate::types::SessionType::TTY => assert_eq!(st, SessionType::TTY),
                        crate::types::SessionType::Unspecified => {
                            assert_eq!(st, SessionType::Unspecified)
                        }
                        crate::types::SessionType::Invalid => {
                            panic!("session type response was bad")
                        }
                    }
                }
            }
        })
    }

    #[test]
    fn properties_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();
            let sessions = manager.list_sessions().await.unwrap();
            let session = SessionProxy::builder(&connection)
                .path(sessions[0].path())
                .unwrap()
                .build()
                .await
                .unwrap();

            assert!(session.active().await.is_ok());
            assert!(session.audit().await.is_ok());
            assert!(session.class().await.is_ok());
            assert!(session.desktop().await.is_ok());
            assert!(session.display().await.is_ok());
            assert!(session.id().await.is_ok());
            assert!(session.idle_hint().await.is_ok());
            assert!(session.idle_since_hint().await.is_ok());
            assert!(session.idle_since_hint_monotonic().await.is_ok());
            assert!(session.remote().await.is_ok());
            assert!(session.leader().await.is_ok());
            assert!(session.locked_hint().await.is_ok());
            assert!(session.name().await.is_ok());
            assert!(session.remote_host().await.is_ok());
            assert!(session.remote_user().await.is_ok());
            assert!(session.scope().await.is_ok());
            assert!(session.seat().await.is_ok());
            assert!(session.service().await.is_ok());
            assert!(session.state().await.is_ok());
            assert!(session.timestamp().await.is_ok());
            assert!(session.timestamp_monotonic().await.is_ok());
            assert!(session.TTY().await.is_ok());
            assert!(session.type_().await.is_ok());
            assert!(session.user().await.is_ok());
            assert!(session.VTNr().await.is_ok());
        })
    }
}
