//! Reference <https://www.freedesktop.org/software/systemd/man/org.freedesktop.login1.html>
mod generated;
/// Logind dbus interface proxies
#[macro_use]
mod proxy;
/// Types that some logind responses can be parsed to
pub mod types;

pub use proxy::manager::ManagerProxy;
pub use proxy::seat::SeatProxy;
pub use proxy::session::SessionProxy;
pub use generated::user;

const DEFAULT_DEST: &str = "org.freedesktop.login1";

#[cfg(test)]
mod tests {
    use crate::ManagerProxy;
    use crate::SessionProxy;
    use zbus::blocking::Connection;

    #[test]
    fn basic_test() {
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();
        let sessions = manager.list_sessions().unwrap();
        let session_proxy = SessionProxy::new(&connection, &sessions[0]).unwrap();

        assert!(session_proxy.get_seat().is_ok());
    }
}
