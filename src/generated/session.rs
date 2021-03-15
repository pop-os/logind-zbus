//! # DBus interface proxy for: `org.freedesktop.login1.Session`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/login1/session/_36' from service 'org.freedesktop.login1' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

use crate::types::{Device, DbusPath, UserSelf};

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
    fn take_device(&self, major: u32, minor: u32)
        -> zbus::Result<Device>;

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
    fn class(&self) -> zbus::Result<String>;

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
    fn idle_since_hint(&self) -> zbus::Result<u64>;

    /// IdleSinceHintMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint_monotonic(&self) -> zbus::Result<u64>;

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
    fn seat(&self) -> zbus::Result<DbusPath>;

    /// Service property
    #[dbus_proxy(property)]
    #[inline]
    fn service(&self) -> zbus::Result<String>;

    /// State property
    #[dbus_proxy(property)]
    #[inline]
    fn state(&self) -> zbus::Result<String>;

    /// TTY property
    #[dbus_proxy(property)]
    #[inline]
    fn tty(&self) -> zbus::Result<String>;

    /// Timestamp property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp(&self) -> zbus::Result<u64>;

    /// TimestampMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp_monotonic(&self) -> zbus::Result<u64>;

    /// Type property
    #[dbus_proxy(property)]
    #[inline]
    fn type_(&self) -> zbus::Result<String>;

    /// User property
    #[dbus_proxy(property)]
    #[inline]
    fn user(&self) -> zbus::Result<UserSelf>;

    /// VTNr property
    #[dbus_proxy(property)]
    #[inline]
    fn vtnr(&self) -> zbus::Result<u32>;
}
