//! # DBus interface proxy for: `org.freedesktop.login1.Seat`

#![allow(non_snake_case)]

use zbus::dbus_proxy;

use crate::types::{SessionPath, TimeStamp};

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
    fn idle_since_hint(&self) -> zbus::Result<TimeStamp>;

    /// IdleSinceHintMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint_monotonic(&self) -> zbus::Result<TimeStamp>;

    /// Sessions property
    #[dbus_proxy(property)]
    #[inline]
    fn sessions(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;
}

#[cfg(test)]
mod tests {
    use crate::generated::seat::SeatProxy;
    use crate::generated::seat::SeatProxyBlocking;
    use crate::manager::ManagerProxy;
    use crate::manager::ManagerProxyBlocking;
    use futures_lite::future;

    #[test]
    fn timestamps() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let seats = manager.list_seats().unwrap();
        let seat = SeatProxyBlocking::builder(&connection)
            .path(seats[0].path())
            .unwrap()
            .build()
            .unwrap();

        assert!(seat.active_session().is_ok());
    }

    #[test]
    fn properties() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let seats = manager.list_seats().unwrap();
        let seat = SeatProxyBlocking::builder(&connection)
            .path(seats[0].path())
            .unwrap()
            .build()
            .unwrap();

        assert!(seat.active_session().is_ok());
        assert!(seat.can_graphical().is_ok());
        assert!(seat.can_TTY().is_ok());
        assert!(seat.id().is_ok());
        assert!(seat.idle_hint().is_ok());
        assert!(seat.idle_since_hint().is_ok());
        assert!(seat.idle_since_hint_monotonic().is_ok());
        assert!(seat.sessions().is_ok());
    }

    #[test]
    fn timestamps_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();
            let seats = manager.list_seats().await.unwrap();
            let seat = SeatProxy::builder(&connection)
                .path(seats[0].path())
                .unwrap()
                .build()
                .await
                .unwrap();

            assert!(seat.active_session().await.is_ok());
        })
    }

    #[test]
    fn properties_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();
            let seats = manager.list_seats().await.unwrap();
            let seat = SeatProxy::builder(&connection)
                .path(seats[0].path())
                .unwrap()
                .build()
                .await
                .unwrap();

            assert!(seat.active_session().await.is_ok());
            assert!(seat.can_graphical().await.is_ok());
            assert!(seat.can_TTY().await.is_ok());
            assert!(seat.id().await.is_ok());
            assert!(seat.idle_hint().await.is_ok());
            assert!(seat.idle_since_hint().await.is_ok());
            assert!(seat.idle_since_hint_monotonic().await.is_ok());
            assert!(seat.sessions().await.is_ok());
        })
    }
}
