use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use logind_zbus::{ManagerProxy, SessionProxy};
use zbus::{Connection, SignalReceiver};

fn print_unlocked() -> std::result::Result<(), zbus::Error> {
    println!("Session unlocked fn call");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::new_system()?;
    let manager = ManagerProxy::new(&connection)?;
    let sessions = manager.list_sessions()?;
    dbg!(&sessions);
    let session = SessionProxy::new(&connection, &sessions[0])?;

    let end = Arc::new(Mutex::new(false));
    let end2 = end.clone();

    session.get_proxy().connect_unlock(print_unlocked)?;

    session.connect_unlock(move || {
        println!("Session unlocked");
        if let Ok(mut lock) = end2.lock() {
            *lock = true;
        }
        Ok(())
    })?;

    let mut signals = SignalReceiver::new(connection);
    signals.receive_for(session.get_proxy());

    loop {
        signals.next_signal()?;
        if let Ok(lock) = end.lock() {
            if *lock {
                break;
            }
        }
        sleep(Duration::from_millis(100));
    }

    Ok(())
}
