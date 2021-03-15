use std::{sync::{Arc, Mutex}, thread::sleep, time::Duration};

use logind_zbus::{ManagerInterface, SessionInterface};
use zbus::{Connection, SignalReceiver};

fn print_unlocked() -> std::result::Result<(), zbus::Error> {
    println!("Session unlocked fn call");
    Ok(())
}

fn main() {
    let connection = Connection::new_system().unwrap();
    let manager = ManagerInterface::new(&connection).unwrap();
    let sessions = manager.list_sessions().unwrap();
    dbg!(&sessions);
    let session = SessionInterface::new(&connection, &sessions[0]).unwrap();

    let end = Arc::new(Mutex::new(false));
    let end2= end.clone();

    session.get_proxy().connect_unlock(print_unlocked).unwrap();

    session.connect_unlock(move || {
        println!("Session unlocked");
        if let Ok(mut lock) = end2.lock() {
            *lock = true;
        }
        Ok(())
    }).unwrap();

    let mut signals = SignalReceiver::new(connection);
    signals.receive_for(session.get_proxy());

    loop {
        signals.next_signal().unwrap();
        if let Ok(lock) = end.lock() {
            if *lock {
                break;
            }
        }
        sleep(Duration::from_millis(100));
    }
}