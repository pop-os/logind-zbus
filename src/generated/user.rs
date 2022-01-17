//! # DBus interface proxy for: `org.freedesktop.login1.User`

#![allow(non_snake_case)]

use zbus::dbus_proxy;

use crate::types::{DbusPath, TimeStamp, UserState};

#[dbus_proxy(
    interface = "org.freedesktop.login1.User",
    default_service = "org.freedesktop.login1"
)]
trait User {
    /// Kill method
    #[inline]
    fn kill(&self, signal_number: i32) -> zbus::Result<()>;

    /// Terminate method
    #[inline]
    fn terminate(&self) -> zbus::Result<()>;

    /// Display property
    #[dbus_proxy(property)]
    #[inline]
    fn display(&self) -> zbus::Result<DbusPath>;

    /// GID property
    #[dbus_proxy(property)]
    #[inline]
    fn GID(&self) -> zbus::Result<u32>;

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

    /// Linger property
    #[dbus_proxy(property)]
    #[inline]
    fn linger(&self) -> zbus::Result<bool>;

    /// Name property
    #[dbus_proxy(property)]
    #[inline]
    fn name(&self) -> zbus::Result<String>;

    /// RuntimePath property
    #[dbus_proxy(property)]
    #[inline]
    fn runtime_path(&self) -> zbus::Result<String>;

    /// Service property
    #[dbus_proxy(property)]
    #[inline]
    fn service(&self) -> zbus::Result<String>;

    /// Sessions property
    #[dbus_proxy(property)]
    #[inline]
    fn sessions(&self) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// Slice property
    #[dbus_proxy(property)]
    #[inline]
    fn slice(&self) -> zbus::Result<String>;

    /// State property
    #[dbus_proxy(property)]
    #[inline]
    fn state(&self) -> zbus::Result<UserState>;

    /// Timestamp property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp(&self) -> zbus::Result<TimeStamp>;

    /// TimestampMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn timestamp_monotonic(&self) -> zbus::Result<TimeStamp>;

    /// UID property
    #[dbus_proxy(property)]
    #[inline]
    fn UID(&self) -> zbus::Result<u32>;
}

#[cfg(test)]
mod tests {
    use crate::generated::user::UserProxy;
    use crate::generated::user::UserProxyBlocking;
    use crate::manager::ManagerProxy;
    use crate::manager::ManagerProxyBlocking;
    use crate::types::IntoUserPath;
    use futures_lite::future;

    #[test]
    fn timestamps() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let users = manager.list_users().unwrap();
        let user = UserProxyBlocking::builder(&connection)
            .path(users[0].into_path_ref())
            .unwrap()
            .build()
            .unwrap();

        let time1 = user.timestamp().unwrap();
        assert!(time1.as_secs() > 0);

        let time2 = user.timestamp_monotonic().unwrap();
        assert!(time2.as_secs() > 0);
    }

    #[test]
    fn properties() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();
        let users = manager.list_users().unwrap();
        let user = UserProxyBlocking::builder(&connection)
            .path(users[0].into_path_ref())
            .unwrap()
            .build()
            .unwrap();

        assert!(user.display().is_ok());
        // Special case. Exists only on users
        assert!(user.GID().is_ok());
        assert!(user.idle_hint().is_ok());
        assert!(user.idle_since_hint().is_ok());
        assert!(user.idle_since_hint_monotonic().is_ok());
        assert!(user.linger().is_ok());
        assert!(user.name().is_ok());
        assert!(user.runtime_path().is_ok());
        assert!(user.service().is_ok());
        assert!(user.slice().is_ok());
        assert!(user.sessions().is_ok());
        assert!(user.state().is_ok());
        assert!(user.timestamp().is_ok());
        assert!(user.timestamp_monotonic().is_ok());
        // Special case. Exists only on users
        assert!(user.UID().is_ok());
    }

    #[test]
    fn timestamps_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();
            let users = manager.list_users().await.unwrap();
            let user = UserProxy::builder(&connection)
                .path(users[0].into_path_ref())
                .unwrap()
                .build()
                .await
                .unwrap();

            let time1 = user.timestamp().await.unwrap();
            assert!(time1.as_secs() > 0);

            let time2 = user.timestamp_monotonic().await.unwrap();
            assert!(time2.as_secs() > 0);
        });
    }

    #[test]
    fn properties_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();
            let users = manager.list_users().await.unwrap();
            let user = UserProxy::builder(&connection)
                .path(users[0].into_path_ref())
                .unwrap()
                .build()
                .await
                .unwrap();

            assert!(user.display().await.is_ok());
            // Special case. Exists only on users
            assert!(user.GID().await.is_ok());
            assert!(user.idle_hint().await.is_ok());
            assert!(user.idle_since_hint().await.is_ok());
            assert!(user.idle_since_hint_monotonic().await.is_ok());
            assert!(user.linger().await.is_ok());
            assert!(user.name().await.is_ok());
            assert!(user.runtime_path().await.is_ok());
            assert!(user.service().await.is_ok());
            assert!(user.slice().await.is_ok());
            assert!(user.sessions().await.is_ok());
            assert!(user.state().await.is_ok());
            assert!(user.timestamp().await.is_ok());
            assert!(user.timestamp_monotonic().await.is_ok());
            // Special case. Exists only on users
            assert!(user.UID().await.is_ok());
        });
    }
}
