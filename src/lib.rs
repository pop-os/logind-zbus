//! Reference <https://www.freedesktop.org/software/systemd/man/org.freedesktop.login1.html>
pub mod manager;
pub mod session;
pub mod seat;
pub mod user;
/// Types that some logind responses can be parsed to
pub mod types;

//const DEFAULT_DEST: &str = "org.freedesktop.login1";

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
