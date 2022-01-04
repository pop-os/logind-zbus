use logind_zbus::{ManagerProxy, SessionProxy};
use zbus::blocking::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system()?;
    let manager = ManagerProxy::new(&connection)?;
    let sessions = manager.list_sessions()?;
    dbg!(&sessions);

    let mut seat = 0;

    for (i, s) in sessions.iter().enumerate() {
        if s.uid() >= 1000 {
            seat = i;
            break;
        }
    }

    let session = SessionProxy::new(&connection, &sessions[seat])?;

    if let Ok(mut sig_iter) = session.get_proxy().receive_unlock() {
        while let Some(_) = sig_iter.next() {
            println!("Unlocked");
            break;
        }
    }

    Ok(())
}
