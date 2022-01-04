use logind_zbus::{
    types::{SessionClass, SessionType},
    ManagerProxy, SessionProxy,
};
use zbus::blocking::Connection;

// Type, Class, Active
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system()?;
    let manager = ManagerProxy::new(&connection)?;
    let sessions = manager.list_sessions()?;

    for session in sessions {
        let session_proxy = SessionProxy::new(&connection, &session)?;

        session_proxy.get_class().and_then(|class| {
            if class == SessionClass::User {
                session_proxy.get_type().and_then(|typ| {
                    match typ {
                        SessionType::X11 | SessionType::Wayland | SessionType::MIR => {
                            session_proxy.get_active().and_then(|active| {
                                if active {
                                    println!("Active graphical session found");
                                } else {
                                    println!("Inactive graphical session found");
                                }
                                Ok(())
                            })?;
                        }
                        SessionType::TTY | SessionType::Unspecified => {}
                        SessionType::Invalid => panic!("session type response was bad"),
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })?;
    }
    Ok(())
}
