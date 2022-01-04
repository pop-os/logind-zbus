use std::time::Duration;

#[cfg(not(feature = "non_blocking"))]
use zbus::blocking::Connection;
#[cfg(not(feature = "non_blocking"))]
use zbus::blocking::Proxy;
#[cfg(feature = "non_blocking")]
use zbus::Connection;
#[cfg(feature = "non_blocking")]
use zbus::Proxy;
use zbus::Result;

use crate::{
    generated::manager,
    types::{IsSupported, ScheduledShutdown, SeatPath, SessionInfo, ShutdownType, UserInfo},
};

/// Proxy wrapper for the logind `Manager` dbus interface
///
/// # Example
/// ```rust
/// use logind_zbus::ManagerProxy;
/// use zbus::blocking::Connection;
///
/// let connection = Connection::system().unwrap();
/// let manager = ManagerProxy::new(&connection).unwrap();
///
/// assert!(manager.can_suspend().is_ok());
/// ```
///
/// # Notes
/// All `connect_*` functions are signals and each of these functions
/// names reflect the underlying generated Proxy call. If desired the wrapped function
/// can be bypassed with:
/// ```ignore
/// *<ManagerProxy>.connect_<function name>()
/// ```
#[cfg(not(feature = "non_blocking"))]
pub struct ManagerProxy<'a>(manager::ManagerProxyBlocking<'a>);

#[cfg(feature = "non_blocking")]
pub struct ManagerProxy<'a>(manager::ManagerProxy<'a>);

impl<'a> std::ops::Deref for ManagerProxy<'a> {
    type Target = Proxy<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for ManagerProxy<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> std::convert::AsRef<Proxy<'a>> for ManagerProxy<'a> {
    fn as_ref(&self) -> &Proxy<'a> {
        &*self
    }
}

impl<'a> std::convert::AsMut<Proxy<'a>> for ManagerProxy<'a> {
    fn as_mut(&mut self) -> &mut Proxy<'a> {
        &mut *self
    }
}

impl<'a> ManagerProxy<'a> {
    #[inline]
    pub fn new(connection: &Connection) -> Result<Self> {
        #[cfg(feature = "non_blocking")]
        return Ok(Self(manager::ManagerProxy::new(&connection)?));

        #[cfg(not(feature = "non_blocking"))]
        return Ok(Self(manager::ManagerProxyBlocking::new(&connection)?));
    }

    /// Brings the session with the specified ID into the foreground
    #[inline]
    pub fn activate_session(&self, session_id: &str) -> zbus::Result<()> {
        self.0.activate_session(session_id)
    }

    /// Brings the session with the specified ID into the foreground if the seat ID matches
    #[inline]
    pub fn activate_session_on_seat(&self, session_id: &str, seat_id: &str) -> zbus::Result<()> {
        self.0.activate_session_on_seat(session_id, seat_id)
    }

    /// Used to assign a specific device to a specific seat. The device is
    /// identified by its /sys/ path and must be eligible for seat assignments
    #[inline]
    pub fn attach_device(
        &self,
        seat_id: &str,
        sysfs_path: &str,
        interactive: bool,
    ) -> zbus::Result<()> {
        self.0.attach_device(seat_id, sysfs_path, interactive)
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_halt(&self) -> zbus::Result<IsSupported> {
        self.0.can_halt().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_hibernate(&self) -> zbus::Result<IsSupported> {
        self.0.can_hibernate().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_hybrid_sleep(&self) -> zbus::Result<IsSupported> {
        self.0.can_hybrid_sleep().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_power_off(&self) -> zbus::Result<IsSupported> {
        self.0.can_power_off().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot(&self) -> zbus::Result<IsSupported> {
        self.0.can_reboot().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_parameter(&self) -> zbus::Result<IsSupported> {
        self.0.can_reboot_parameter().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_to_boot_loader_entry(&self) -> zbus::Result<IsSupported> {
        self.0
            .can_reboot_to_boot_loader_entry()
            .map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_to_boot_loader_menu(&self) -> zbus::Result<IsSupported> {
        self.0
            .can_reboot_to_boot_loader_menu()
            .map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_to_firmware_setup(&self) -> zbus::Result<IsSupported> {
        self.0
            .can_reboot_to_firmware_setup()
            .map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_suspend(&self) -> zbus::Result<IsSupported> {
        self.0.can_suspend().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_suspend_then_hibernate(&self) -> zbus::Result<IsSupported> {
        self.0
            .can_suspend_then_hibernate()
            .map(|v| v.as_str().into())
    }

    #[inline]
    pub fn cancel_scheduled_shutdown(&self) -> zbus::Result<bool> {
        self.0.cancel_scheduled_shutdown()
    }

    #[inline]
    pub fn flush_devices(&self, interactive: bool) -> zbus::Result<()> {
        self.0.flush_devices(interactive)
    }

    #[inline]
    pub fn get_seat(&self, seat_id: &str) -> zbus::Result<zvariant::OwnedObjectPath> {
        self.0.get_seat(seat_id)
    }

    #[inline]
    pub fn get_session(&self, session_id: &str) -> zbus::Result<zvariant::OwnedObjectPath> {
        self.0.get_session(session_id)
    }

    #[inline]
    pub fn get_session_by_pid(&self, pid: u32) -> zbus::Result<zvariant::OwnedObjectPath> {
        self.0.get_session_by_PID(pid)
    }

    #[inline]
    pub fn get_user(&self, uid: u32) -> zbus::Result<zvariant::OwnedObjectPath> {
        self.0.get_user(uid)
    }

    #[inline]
    pub fn get_user_by_pid(&self, pid: u32) -> zbus::Result<zvariant::OwnedObjectPath> {
        self.0.get_user_by_PID(pid)
    }

    #[inline]
    pub fn halt(&self, interactive: bool) -> zbus::Result<()> {
        self.0.halt(interactive)
    }

    #[inline]
    pub fn hibernate(&self, interactive: bool) -> zbus::Result<()> {
        self.0.hibernate(interactive)
    }

    #[inline]
    pub fn inhibit(
        &self,
        what: &str,
        who: &str,
        why: &str,
        mode: &str,
    ) -> zbus::Result<std::os::unix::io::RawFd> {
        self.0.inhibit(what, who, why, mode)
    }

    #[inline]
    pub fn hybrid_sleep(&self, interactive: bool) -> zbus::Result<()> {
        self.0.hybrid_sleep(interactive)
    }

    // #[inline]
    // pub fn create_session(
    //     &'a self,
    //     session: SessionCreate,
    //     properties: &[(&str, zvariant::Value<'a>)],
    // ) -> zbus::Result<(
    //     String,
    //     zvariant::OwnedObjectPath,
    //     String,
    //     std::os::unix::io::RawFd,
    //     u32,
    //     String,
    //     u32,
    //     bool,
    // )> {
    //     self.0.create_session(session, properties)
    // }

    #[inline]
    pub fn kill_session(
        &self,
        session_id: &str,
        who: &str,
        signal_number: i32,
    ) -> zbus::Result<()> {
        self.0.kill_session(session_id, who, signal_number)
    }

    #[inline]
    pub fn kill_user(&self, uid: u32, signal_number: i32) -> zbus::Result<()> {
        self.0.kill_user(uid, signal_number)
    }

    #[inline]
    pub fn list_inhibitors(&self) -> zbus::Result<Vec<(String, String, String, String, u32, u32)>> {
        self.0.list_inhibitors()
    }

    #[inline]
    pub fn list_seats(&self) -> zbus::Result<Vec<SeatPath>> {
        self.0.list_seats()
    }

    #[inline]
    pub fn list_sessions(&self) -> zbus::Result<Vec<SessionInfo>> {
        self.0.list_sessions()
    }

    #[inline]
    pub fn list_users(&self) -> zbus::Result<Vec<UserInfo>> {
        self.0.list_users()
    }

    #[inline]
    pub fn lock_session(&self, session_id: &str) -> zbus::Result<()> {
        self.0.lock_session(session_id)
    }

    #[inline]
    pub fn lock_sessions(&self) -> zbus::Result<()> {
        self.0.lock_sessions()
    }

    #[inline]
    pub fn power_off(&self, interactive: bool) -> zbus::Result<()> {
        self.0.power_off(interactive)
    }

    #[inline]
    pub fn reboot(&self, interactive: bool) -> zbus::Result<()> {
        self.0.reboot(interactive)
    }

    #[inline]
    pub fn release_session(&self, session_id: &str) -> zbus::Result<()> {
        self.0.release_session(session_id)
    }

    #[inline]
    pub fn schedule_shutdown(
        &self,
        shutdown_type: ShutdownType,
        micros: Duration,
    ) -> zbus::Result<()> {
        self.0
            .schedule_shutdown(shutdown_type.into(), micros.as_micros() as u64)
    }

    #[inline]
    pub fn set_reboot_parameter(&self, parameter: &str) -> zbus::Result<()> {
        self.0.set_reboot_parameter(parameter)
    }

    #[inline]
    pub fn set_reboot_to_boot_loader_entry(&self, boot_loader_entry: &str) -> zbus::Result<()> {
        self.0.set_reboot_to_boot_loader_entry(boot_loader_entry)
    }

    #[inline]
    pub fn set_reboot_to_boot_loader_menu(&self, timeout: Duration) -> zbus::Result<()> {
        self.0.set_reboot_to_boot_loader_menu(timeout.as_secs())
    }

    #[inline]
    pub fn set_reboot_to_firmware_setup(&self, enable: bool) -> zbus::Result<()> {
        self.0.set_reboot_to_firmware_setup(enable)
    }

    #[inline]
    pub fn set_user_linger(&self, uid: u32, enable: bool, interactive: bool) -> zbus::Result<()> {
        self.0.set_user_linger(uid, enable, interactive)
    }

    #[inline]
    pub fn set_wall_message(&self, wall_message: &str, enable: bool) -> zbus::Result<()> {
        self.0.set_wall_message(wall_message, enable)
    }

    #[inline]
    pub fn suspend(&self, interactive: bool) -> zbus::Result<()> {
        self.0.suspend(interactive)
    }

    #[inline]
    pub fn suspend_then_hibernate(&self, interactive: bool) -> zbus::Result<()> {
        self.0.suspend_then_hibernate(interactive)
    }

    #[inline]
    pub fn terminate_seat(&self, seat_id: &str) -> zbus::Result<()> {
        self.0.terminate_seat(seat_id)
    }

    #[inline]
    pub fn terminate_session(&self, session_id: &str) -> zbus::Result<()> {
        self.0.terminate_session(session_id)
    }

    #[inline]
    pub fn terminate_user(&self, uid: u32) -> zbus::Result<()> {
        self.0.terminate_user(uid)
    }

    #[inline]
    pub fn unlock_session(&self, session_id: &str) -> zbus::Result<()> {
        self.0.unlock_session(session_id)
    }

    #[inline]
    pub fn unlock_sessions(&self) -> zbus::Result<()> {
        self.0.unlock_sessions()
    }

    ///////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn get_block_inhibited(&self) -> zbus::Result<String> {
        self.0.block_inhibited()
    }

    #[inline]
    pub fn get_boot_loader_entries(&self) -> zbus::Result<Vec<String>> {
        self.0.boot_loader_entries()
    }

    #[inline]
    pub fn get_delay_inhibited(&self) -> zbus::Result<String> {
        self.0.delay_inhibited()
    }

    #[inline]
    pub fn get_docked(&self) -> zbus::Result<bool> {
        self.0.docked()
    }

    #[inline]
    pub fn set_enable_wall_messages(&self, value: bool) -> zbus::Result<()> {
        self.0.set_enable_wall_messages(value)
    }

    #[inline]
    pub fn get_handle_hibernate_key(&self) -> zbus::Result<String> {
        self.0.handle_hibernate_key()
    }

    #[inline]
    pub fn get_handle_lid_switch(&self) -> zbus::Result<String> {
        self.0.handle_lid_switch()
    }

    #[inline]
    pub fn get_handle_lid_switch_docked(&self) -> zbus::Result<String> {
        self.0.handle_lid_switch_docked()
    }

    #[inline]
    pub fn get_handle_lid_switch_external_power(&self) -> zbus::Result<String> {
        self.0.handle_lid_switch_external_power()
    }

    #[inline]
    pub fn get_handle_power_key(&self) -> zbus::Result<String> {
        self.0.handle_power_key()
    }

    #[inline]
    pub fn get_handle_suspend_key(&self) -> zbus::Result<String> {
        self.0.handle_suspend_key()
    }

    #[inline]
    pub fn get_holdoff_timeout_usec(&self) -> zbus::Result<Duration> {
        self.0.holdoff_timeout_USec().map(Duration::from_micros)
    }

    #[inline]
    pub fn get_idle_action(&self) -> zbus::Result<String> {
        self.0.idle_action()
    }

    #[inline]
    pub fn get_idle_action_usec(&self) -> zbus::Result<Duration> {
        self.0.idle_action_USec().map(Duration::from_micros)
    }

    #[inline]
    pub fn get_idle_hint(&self) -> zbus::Result<bool> {
        self.0.idle_hint()
    }

    #[inline]
    pub fn get_idle_since_hint(&self) -> zbus::Result<Duration> {
        self.0.idle_since_hint().map(Duration::from_micros)
    }

    #[inline]
    pub fn get_idle_since_hint_monotonic(&self) -> zbus::Result<Duration> {
        self.0
            .idle_since_hint_monotonic()
            .map(Duration::from_micros)
    }

    #[inline]
    pub fn get_inhibit_delay_max_usec(&self) -> zbus::Result<Duration> {
        self.0.inhibit_delay_max_USec().map(Duration::from_micros)
    }

    #[inline]
    pub fn get_inhibitors_max(&self) -> zbus::Result<u64> {
        self.0.inhibitors_max()
    }

    #[inline]
    pub fn get_kill_exclude_users(&self) -> zbus::Result<Vec<String>> {
        self.0.kill_exclude_users()
    }

    #[inline]
    pub fn get_kill_only_users(&self) -> zbus::Result<Vec<String>> {
        self.0.kill_only_users()
    }

    #[inline]
    pub fn get_kill_user_processes(&self) -> zbus::Result<bool> {
        self.0.kill_user_processes()
    }

    #[inline]
    pub fn get_lid_closed(&self) -> zbus::Result<bool> {
        self.0.lid_closed()
    }

    #[inline]
    pub fn get_nauto_vts(&self) -> zbus::Result<u32> {
        self.0.NAuto_VTs()
    }

    #[inline]
    pub fn get_ncurrent_inhibitors(&self) -> zbus::Result<u64> {
        self.0.NCurrent_inhibitors()
    }

    #[inline]
    pub fn get_ncurrent_sessions(&self) -> zbus::Result<u64> {
        self.0.NCurrent_sessions()
    }

    #[inline]
    pub fn get_on_external_power(&self) -> zbus::Result<bool> {
        self.0.on_external_power()
    }

    #[inline]
    pub fn get_preparing_for_shutdown(&self) -> zbus::Result<bool> {
        self.0.preparing_for_shutdown()
    }

    #[inline]
    pub fn get_preparing_for_sleep(&self) -> zbus::Result<bool> {
        self.0.preparing_for_sleep()
    }

    #[inline]
    pub fn get_reboot_parameter(&self) -> zbus::Result<String> {
        self.0.reboot_parameter()
    }

    #[inline]
    pub fn get_reboot_to_boot_loader_entry(&self) -> zbus::Result<String> {
        self.0.reboot_to_boot_loader_entry()
    }

    #[inline]
    pub fn get_reboot_to_boot_loader_menu(&self) -> zbus::Result<u64> {
        self.0.reboot_to_boot_loader_menu()
    }

    #[inline]
    pub fn get_reboot_to_firmware_setup(&self) -> zbus::Result<bool> {
        self.0.reboot_to_firmware_setup()
    }

    #[inline]
    pub fn get_remove_ipc(&self) -> zbus::Result<bool> {
        self.0.remove_IPC()
    }

    #[inline]
    pub fn get_runtime_directory_inodes_max(&self) -> zbus::Result<u64> {
        self.0.runtime_directory_inodes_max()
    }

    #[inline]
    pub fn get_runtime_directory_size(&self) -> zbus::Result<u64> {
        self.0.runtime_directory_size()
    }

    #[inline]
    pub fn get_scheduled_shutdown(&self) -> zbus::Result<ScheduledShutdown> {
        self.0.scheduled_shutdown()
    }

    #[inline]
    pub fn get_sessions_max(&self) -> zbus::Result<u64> {
        self.0.sessions_max()
    }

    #[inline]
    pub fn get_user_stop_delay_usec(&self) -> zbus::Result<Duration> {
        self.0.user_stop_delay_USec().map(Duration::from_micros)
    }

    #[inline]
    pub fn get_wall_message(&self) -> zbus::Result<String> {
        self.0.wall_message()
    }

    ///////////////////////////////////////////////////////////////////////////

    receive_signal_name!(
        receive_prepare_for_shutdown,
        manager::PrepareForShutdownStream,
        manager::PrepareForShutdownIterator
    );
    receive_signal_name!(
        receive_prepare_for_sleep,
        manager::PrepareForSleepStream,
        manager::PrepareForSleepIterator
    );
    receive_signal_name!(
        receive_seat_new,
        manager::SeatNewStream,
        manager::SeatNewIterator
    );
    receive_signal_name!(
        receive_seat_removed,
        manager::SeatRemovedStream,
        manager::SeatRemovedIterator
    );
    receive_signal_name!(
        receive_session_new,
        manager::SessionNewStream,
        manager::SessionNewIterator
    );
    receive_signal_name!(
        receive_session_removed,
        manager::SessionRemovedStream,
        manager::SessionRemovedIterator
    );
    receive_signal_name!(
        receive_user_new,
        manager::UserNewStream,
        manager::UserNewIterator
    );
    receive_signal_name!(
        receive_user_removed,
        manager::UserRemovedStream,
        manager::UserRemovedIterator
    );
}

#[cfg(test)]
mod tests {
    use crate::ManagerProxy;
    use zbus::blocking::Connection;

    #[test]
    fn timestamps() {
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();

        assert!(manager.can_suspend().is_ok());
    }

    #[test]
    fn properties() {
        let connection = Connection::system().unwrap();
        let manager = ManagerProxy::new(&connection).unwrap();

        assert!(manager.get_block_inhibited().is_ok());
        assert!(manager.get_boot_loader_entries().is_ok());
        assert!(manager.get_delay_inhibited().is_ok());
        assert!(manager.get_docked().is_ok());
        assert!(manager.get_handle_hibernate_key().is_ok());
        assert!(manager.get_handle_lid_switch().is_ok());
        assert!(manager.get_handle_lid_switch_docked().is_ok());
        assert!(manager.get_handle_lid_switch_external_power().is_ok());
        assert!(manager.get_handle_power_key().is_ok());
        assert!(manager.get_handle_suspend_key().is_ok());
        assert!(manager.get_holdoff_timeout_usec().is_ok());
        assert!(manager.get_idle_action().is_ok());
        assert!(manager.get_idle_action_usec().is_ok());
        assert!(manager.get_idle_hint().is_ok());
        assert!(manager.get_idle_since_hint().is_ok());
        assert!(manager.get_idle_since_hint_monotonic().is_ok());
        assert!(manager.get_inhibit_delay_max_usec().is_ok());
        assert!(manager.get_inhibitors_max().is_ok());
        assert!(manager.get_kill_exclude_users().is_ok());
        assert!(manager.get_kill_only_users().is_ok());
        assert!(manager.get_kill_user_processes().is_ok());
        assert!(manager.get_lid_closed().is_ok());
        assert!(manager.get_nauto_vts().is_ok());
        assert!(manager.get_ncurrent_inhibitors().is_ok());
        assert!(manager.get_ncurrent_sessions().is_ok());
        assert!(manager.get_on_external_power().is_ok());
        assert!(manager.get_preparing_for_shutdown().is_ok());
        assert!(manager.get_preparing_for_sleep().is_ok());
        assert!(manager.get_reboot_parameter().is_ok());
        assert!(manager.get_reboot_to_boot_loader_entry().is_ok());
        assert!(manager.get_reboot_to_boot_loader_menu().is_ok());
        assert!(manager.get_reboot_to_firmware_setup().is_ok());
        assert!(manager.get_remove_ipc().is_ok());
        assert!(manager.get_runtime_directory_inodes_max().is_ok());
        assert!(manager.get_runtime_directory_size().is_ok());
        assert!(manager.get_scheduled_shutdown().is_ok());
        assert!(manager.get_seat("seat0").is_ok());

        // Requires a valid session ID
        // assert!(manager.get_session("c4").is_ok());
        // Requires knowing a process ID owned by a session
        // assert!(manager.get_session_by_pid(5455).is_ok());

        assert!(manager.get_sessions_max().is_ok());
        // Requires a valid logged-in user ID
        // assert!(manager.get_user(1000).is_ok());
        // Requires knowing a process ID
        // assert!(manager.get_user_by_pid(25813).is_ok());
        assert!(manager.get_user_stop_delay_usec().is_ok());
        assert!(manager.get_wall_message().is_ok());
    }
}
