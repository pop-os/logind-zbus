//! Reference <https://www.freedesktop.org/software/systemd/man/org.freedesktop.login1.html>
mod generated;
/// Logind dbus interface proxies
mod proxy;
/// Types that some logind responses can be parsed to
pub mod types;

pub use proxy::manager::ManagerInterface;
pub use proxy::session::SessionInterface;
pub use proxy::seat::SeatInterface;
pub use proxy::user::UserInterface;

const DEFAULT_DEST: &str = "org.freedesktop.login1";

#[cfg(test)]
mod tests {
    use crate::ManagerInterface;
    use crate::SessionInterface;
    use zbus::Connection;

    #[test]
    fn basic_test() {
        let connection = Connection::new_system().unwrap();
        let manager = ManagerInterface::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();
        let session_proxy = SessionInterface::new(&connection, &sessions[0]).unwrap();

        assert!(session_proxy.get_seat().is_ok());
    }
}
