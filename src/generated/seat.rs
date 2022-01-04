//! # DBus interface proxy for: `org.freedesktop.login1.Seat`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/login1/seat/seat0' from service 'org.freedesktop.login1' on system bus`.
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

#![allow(non_snake_case)]

use zbus::dbus_proxy;

use crate::types::SessionPath;

#[dbus_proxy(
    interface = "org.freedesktop.login1.Seat",
    default_service = "org.freedesktop.login1"
)]
trait Seat {
    /// ActivateSession method
    #[inline]
    fn activate_session(&self, session_id: &str) -> zbus::Result<()>;

    /// SwitchTo method
    #[inline]
    fn switch_to(&self, vtnr: u32) -> zbus::Result<()>;

    /// SwitchToNext method
    #[inline]
    fn switch_to_next(&self) -> zbus::Result<()>;

    /// SwitchToPrevious method
    #[inline]
    fn switch_to_previous(&self) -> zbus::Result<()>;

    /// Terminate method
    #[inline]
    fn terminate(&self) -> zbus::Result<()>;

    /// ActiveSession property
    #[dbus_proxy(property)]
    #[inline]
    fn active_session(&self) -> zbus::Result<SessionPath>;

    /// CanGraphical property
    #[dbus_proxy(property)]
    #[inline]
    fn can_graphical(&self) -> zbus::Result<bool>;

    /// CanTTY property
    #[dbus_proxy(property)]
    #[inline]
    fn can_TTY(&self) -> zbus::Result<bool>;

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

    /// Sessions property
    #[dbus_proxy(property)]
    #[inline]
    fn sessions(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;
}
