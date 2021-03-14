use std::time::Duration;

use zbus::{Proxy, Result};
#[cfg(feature = "azync")]
use zbus::azync::Connection;
#[cfg(not(feature = "azync"))]
use zbus::Connection;
use zvariant::OwnedObjectPath;

use crate::{generated::manager, types::{DbusPath, IsSupported, ScheduledShutdown, SessionInfo, ShutdownType, UserInfo}};

pub(crate) type CallbackBool = fn(bool) -> std::result::Result<(), zbus::Error>;
pub(crate) type CallbackStrPath = fn(&str, OwnedObjectPath) -> std::result::Result<(), zbus::Error>;
pub(crate) type CallbackU32Path = fn(u32, OwnedObjectPath) -> std::result::Result<(), zbus::Error>;

/// Proxy wrapper for the logind `Manager` dbus interface
///
/// # Example
/// ```rust
/// use logind_zbus::ManagerInterface;
/// use zbus::Connection;
///
/// let connection = Connection::new_system().unwrap();
/// let manager = ManagerInterface::new(&connection).unwrap();
/// 
/// assert!(manager.can_suspend().is_ok());
/// ```
pub struct ManagerInterface<'a> {
    _inner: manager::ManagerProxy<'a>,
}

impl<'a> ManagerInterface<'a> {
    #[inline]
    pub fn new(connection: &Connection) -> Result<Self> {
        Ok(Self {
            _inner: manager::ManagerProxy::new(&connection)?,
        })
    }

    /// Borrow the underlying `Proxy` for use with zbus directly
    pub fn get_proxy(&self) -> &Proxy {
        &self._inner
    }

    /// Brings the session with the specified ID into the foreground
    #[inline]
    pub fn activate_session(&self, session_id: &str) -> zbus::Result<()> {
        self._inner.activate_session(session_id)
    }

    /// Brings the session with the specified ID into the foreground if the seat ID matches
    #[inline]
    pub fn activate_session_on_seat(&self, session_id: &str, seat_id: &str) -> zbus::Result<()> {
        self._inner.activate_session_on_seat(session_id, seat_id)
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
        self._inner.attach_device(seat_id, sysfs_path, interactive)
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_halt(&self) -> zbus::Result<IsSupported> {
        self._inner.can_halt().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_hibernate(&self) -> zbus::Result<IsSupported> {
        self._inner.can_hibernate().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_hybrid_sleep(&self) -> zbus::Result<IsSupported> {
        self._inner.can_hybrid_sleep().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_power_off(&self) -> zbus::Result<IsSupported> {
        self._inner.can_power_off().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot(&self) -> zbus::Result<IsSupported> {
        self._inner.can_reboot().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_parameter(&self) -> zbus::Result<IsSupported> {
        self._inner
            .can_reboot_parameter()
            .map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_to_boot_loader_entry(&self) -> zbus::Result<IsSupported> {
        self._inner
            .can_reboot_to_boot_loader_entry()
            .map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_to_boot_loader_menu(&self) -> zbus::Result<IsSupported> {
        self._inner
            .can_reboot_to_boot_loader_menu()
            .map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_reboot_to_firmware_setup(&self) -> zbus::Result<IsSupported> {
        self._inner
            .can_reboot_to_firmware_setup()
            .map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_suspend(&self) -> zbus::Result<IsSupported> {
        self._inner.can_suspend().map(|v| v.as_str().into())
    }

    /// Check if supported and the calling user is allowed to execute it
    #[inline]
    pub fn can_suspend_then_hibernate(&self) -> zbus::Result<IsSupported> {
        self._inner
            .can_suspend_then_hibernate()
            .map(|v| v.as_str().into())
    }

    #[inline]
    pub fn cancel_scheduled_shutdown(&self) -> zbus::Result<bool> {
        self._inner.cancel_scheduled_shutdown()
    }

    #[inline]
    pub fn flush_devices(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.flush_devices(interactive)
    }

    #[inline]
    pub fn get_seat(&self, seat_id: &str) -> zbus::Result<zvariant::OwnedObjectPath> {
        self._inner.get_seat(seat_id)
    }


    #[inline]
    pub fn get_session(&self, session_id: &str) -> zbus::Result<zvariant::OwnedObjectPath> {
        self._inner.get_session(session_id)
    }

    #[inline]
    pub fn get_session_by_pid(&self, pid: u32) -> zbus::Result<zvariant::OwnedObjectPath> {
        self._inner.get_session_by_pid(pid)
    }

    #[inline]
    pub fn get_user(&self, uid: u32) -> zbus::Result<zvariant::OwnedObjectPath> {
        self._inner.get_user(uid)
    }

    #[inline]
    pub fn get_user_by_pid(&self, pid: u32) -> zbus::Result<zvariant::OwnedObjectPath> {
        self._inner.get_user_by_pid(pid)
    }

    #[inline]
    pub fn halt(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.halt(interactive)
    }

    #[inline]
    pub fn hibernate(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.hibernate(interactive)
    }

    #[inline]
    pub fn inhibit(
        &self,
        what: &str,
        who: &str,
        why: &str,
        mode: &str,
    ) -> zbus::Result<std::os::unix::io::RawFd> {
        self._inner.inhibit(what, who, why, mode)
    }

    #[inline]
    pub fn hybrid_sleep(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.hybrid_sleep(interactive)
    }

    #[inline]
    pub fn kill_session(&self, session_id: &str, who: &str, signal_number: i32) -> zbus::Result<()>{
        self._inner.kill_session(session_id, who, signal_number)
    }

    #[inline]
    pub fn kill_user(&self, uid: u32, signal_number: i32) -> zbus::Result<()> {
        self._inner.kill_user(uid, signal_number)
    }

    #[inline]
    pub fn list_inhibitors(&self) -> zbus::Result<Vec<(String, String, String, String, u32, u32)>> {
        self._inner.list_inhibitors()
    }

    #[inline]
    pub fn list_seats(&self) -> zbus::Result<Vec<DbusPath>> {
        self._inner.list_seats()
    }

    #[inline]
    pub fn list_sessions(&self) -> zbus::Result<Vec<SessionInfo>> {
        self._inner.list_sessions()
    }

    #[inline]
    pub fn list_users(&self) -> zbus::Result<Vec<UserInfo>> {
        self._inner.list_users()
    }

    #[inline]
    pub fn lock_session(&self, session_id: &str) -> zbus::Result<()> {
        self._inner.lock_session(session_id)
    }

    #[inline]
    pub fn lock_sessions(&self) -> zbus::Result<()> {
        self._inner.lock_sessions()
    }

    #[inline]
    pub fn power_off(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.power_off(interactive)
    }

    #[inline]
    pub fn reboot(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.reboot(interactive)
    }

    #[inline]
    pub fn release_session(&self, session_id: &str) -> zbus::Result<()> {
        self._inner.release_session(session_id)
    }

    #[inline]
    pub fn schedule_shutdown(&self, shutdown_type: ShutdownType, micros: Duration) -> zbus::Result<()> {
        self._inner.schedule_shutdown(shutdown_type.into(), micros.as_micros() as u64)
    }

    #[inline]
    pub fn set_reboot_parameter(&self, parameter: &str) -> zbus::Result<()> {
        self._inner.set_reboot_parameter(parameter)
    }

    #[inline]
    pub fn set_reboot_to_boot_loader_entry(&self, boot_loader_entry: &str) -> zbus::Result<()> {
        self._inner.set_reboot_to_boot_loader_entry(boot_loader_entry)
    }

    #[inline]
    pub fn set_reboot_to_boot_loader_menu(&self, timeout: Duration) -> zbus::Result<()> {
        self._inner.set_reboot_to_boot_loader_menu(timeout.as_secs())
    }

    #[inline]
    pub fn set_reboot_to_firmware_setup(&self, enable: bool) -> zbus::Result<()> {
        self._inner.set_reboot_to_firmware_setup(enable)
    }

    #[inline]
    pub fn set_user_linger(&self, uid: u32, enable: bool, interactive: bool) -> zbus::Result<()> {
        self._inner.set_user_linger(uid, enable, interactive)
    }

    #[inline]
    pub fn set_wall_message(&self, wall_message: &str, enable: bool) -> zbus::Result<()> {
        self._inner.set_wall_message(wall_message, enable)
    }

    #[inline]
    pub fn suspend(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.suspend(interactive)
    }

    #[inline]
    pub fn suspend_then_hibernate(&self, interactive: bool) -> zbus::Result<()> {
        self._inner.suspend_then_hibernate(interactive)
    }

    #[inline]
    pub fn terminate_seat(&self, seat_id: &str) -> zbus::Result<()> {
        self._inner.terminate_seat(seat_id)
    }

    #[inline]
    pub fn terminate_session(&self, session_id: &str) -> zbus::Result<()> {
        self._inner.terminate_session(session_id)
    }

    #[inline]
    pub fn terminate_user(&self, uid: u32) -> zbus::Result<()> {
        self._inner.terminate_user(uid)
    }

    #[inline]
    pub fn unlock_session(&self, session_id: &str) -> zbus::Result<()> {
        self._inner.unlock_session(session_id)
    }

    #[inline]
    pub fn unlock_sessions(&self) -> zbus::Result<()> {
        self._inner.unlock_sessions()
    }

    ///////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn get_block_inhibited(&self) -> zbus::Result<String> {
        self._inner.block_inhibited()
    }

    #[inline]
    pub fn get_boot_loader_entries(&self) -> zbus::Result<Vec<String>> {
        self._inner.boot_loader_entries()
    }

    #[inline]
    pub fn get_delay_inhibited(&self) -> zbus::Result<String> {
        self._inner.delay_inhibited()
    }

    #[inline]
    pub fn get_docked(&self) -> zbus::Result<bool> {
        self._inner.docked()
    }

    #[inline]
    pub fn set_enable_wall_messages(&self, value: bool) -> zbus::Result<()> {
        self._inner.set_enable_wall_messages(value)
    }

    #[inline]
    pub fn get_handle_hibernate_key(&self) -> zbus::Result<String> {
        self._inner.handle_hibernate_key()
    }

    #[inline]
    pub fn get_handle_lid_switch(&self) -> zbus::Result<String> {
        self._inner.handle_lid_switch()
    }

    #[inline]
    pub fn get_handle_lid_switch_docked(&self) -> zbus::Result<String> {
        self._inner.handle_lid_switch_docked()
    }

    #[inline]
    pub fn get_handle_lid_switch_external_power(&self) -> zbus::Result<String> {
        self._inner.handle_lid_switch_external_power()
    }

    #[inline]
    pub fn get_handle_power_key(&self) -> zbus::Result<String> {
        self._inner.handle_power_key()
    }

    #[inline]
    pub fn get_handle_suspend_key(&self) -> zbus::Result<String> {
        self._inner.handle_suspend_key()
    }

    #[inline]
    pub fn get_holdoff_timeout_usec(&self) -> zbus::Result<Duration> {
        self._inner.holdoff_timeout_usec().map(|usec| Duration::from_micros(usec))
    }

    #[inline]
    pub fn get_idle_action(&self) -> zbus::Result<String> {
        self._inner.idle_action()
    }

    #[inline]
    pub fn get_idle_action_usec(&self) -> zbus::Result<Duration> {
        self._inner.idle_action_usec().map(|usec| Duration::from_micros(usec))
    }

    #[inline]
    pub fn get_idle_hint(&self) -> zbus::Result<bool> {
        self._inner.idle_hint()
    }

    #[inline]
    pub fn get_idle_since_hint(&self) -> zbus::Result<Duration> {
        self._inner.idle_since_hint().map(|usec| Duration::from_micros(usec))
    }

    #[inline]
    pub fn get_idle_since_hint_monotonic(&self) -> zbus::Result<Duration> {
        self._inner.idle_since_hint_monotonic().map(|usec| Duration::from_micros(usec))
    }

    #[inline]
    pub fn get_inhibit_delay_max_usec(&self) -> zbus::Result<Duration> {
        self._inner.inhibit_delay_max_usec().map(|usec| Duration::from_micros(usec))
    }

    #[inline]
    pub fn get_inhibitors_max(&self) -> zbus::Result<u64> {
        self._inner.inhibitors_max()
    }

    #[inline]
    pub fn get_kill_exclude_users(&self) -> zbus::Result<Vec<String>> {
        self._inner.kill_exclude_users()
    }

    #[inline]
    pub fn get_kill_only_users(&self) -> zbus::Result<Vec<String>> {
        self._inner.kill_only_users()
    }

    #[inline]
    pub fn get_kill_user_processes(&self) -> zbus::Result<bool> {
        self._inner.kill_user_processes()
    }

    #[inline]
    pub fn get_lid_closed(&self) -> zbus::Result<bool> {
        self._inner.lid_closed()
    }

    #[inline]
    pub fn get_nauto_vts(&self) -> zbus::Result<u32> {
        self._inner.nauto_vts()
    }

    #[inline]
    pub fn get_ncurrent_inhibitors(&self) -> zbus::Result<u64> {
        self._inner.ncurrent_inhibitors()
    }

    #[inline]
    pub fn get_ncurrent_sessions(&self) -> zbus::Result<u64> {
        self._inner.ncurrent_sessions()
    }

    #[inline]
    pub fn get_on_external_power(&self) -> zbus::Result<bool> {
        self._inner.on_external_power()
    }

    #[inline]
    pub fn get_preparing_for_shutdown(&self) -> zbus::Result<bool> {
        self._inner.preparing_for_shutdown()
    }

    #[inline]
    pub fn get_preparing_for_sleep(&self) -> zbus::Result<bool> {
        self._inner.preparing_for_sleep()
    }

    #[inline]
    pub fn get_reboot_parameter(&self) -> zbus::Result<String> {
        self._inner.reboot_parameter()
    }

    #[inline]
    pub fn get_reboot_to_boot_loader_entry(&self) -> zbus::Result<String> {
        self._inner.reboot_to_boot_loader_entry()
    }

    #[inline]
    pub fn get_reboot_to_boot_loader_menu(&self) -> zbus::Result<u64> {
        self._inner.reboot_to_boot_loader_menu()
    }

    #[inline]
    pub fn get_reboot_to_firmware_setup(&self) -> zbus::Result<bool> {
        self._inner.reboot_to_firmware_setup()
    }

    #[inline]
    pub fn get_remove_ipc(&self) -> zbus::Result<bool> {
        self._inner.remove_ipc()
    }

    #[inline]
    pub fn get_runtime_directory_inodes_max(&self) -> zbus::Result<u64> {
        self._inner.runtime_directory_inodes_max()
    }

    #[inline]
    pub fn get_runtime_directory_size(&self) -> zbus::Result<u64> {
        self._inner.runtime_directory_size()
    }

    #[inline]
    pub fn get_scheduled_shutdown(&self) -> zbus::Result<ScheduledShutdown> {
        self._inner.scheduled_shutdown()
    }

    #[inline]
    pub fn get_sessions_max(&self) -> zbus::Result<u64> {
        self._inner.sessions_max()
    }

    #[inline]
    pub fn get_user_stop_delay_usec(&self) -> zbus::Result<Duration> {
        self._inner.user_stop_delay_usec().map(|usec| Duration::from_micros(usec))
    }

    #[inline]
    pub fn get_wall_message(&self) -> zbus::Result<String> {
        self._inner.wall_message()
    }

    ///////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn connect_prepare_for_shutdown_signal(
        &self,
        callback: CallbackBool,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_prepare_for_shutdown(callback)
    }

    #[inline]
    pub fn disconnect_prepare_for_shutdown_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_prepare_for_shutdown()
    }

    #[inline]
    pub fn connect_prepare_for_sleep_signal(
        &self,
        callback: CallbackBool,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_prepare_for_sleep(callback)
    }

    #[inline]
    pub fn disconnect_prepare_for_sleep_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_prepare_for_sleep()
    }

    #[inline]
    pub fn connect_new_seat_signal(
        &self,
        callback: CallbackStrPath,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_seat_new(callback)
    }

    #[inline]
    pub fn disconnect_new_seat_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_seat_new()
    }

    #[inline]
    pub fn connect_seat_removed_signal(
        &self,
        callback: CallbackStrPath,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_seat_removed(callback)
    }

    #[inline]
    pub fn disconnect_seat_removed_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_seat_removed()
    }

    #[inline]
    pub fn connect_new_session_signal(
        &self,
        callback: CallbackStrPath,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_session_new(callback)
    }

    #[inline]
    pub fn disconnect_new_session_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_session_new()
    }

    #[inline]
    pub fn connect_session_removed_signal(
        &self,
        callback: CallbackStrPath,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_session_removed(callback)
    }

    #[inline]
    pub fn disconnect_session_removed_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_session_removed()
    }

    #[inline]
    pub fn connect_new_user_signal(
        &self,
        callback: CallbackU32Path,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_user_new(callback)
    }

    #[inline]
    pub fn disconnect_new_user_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_user_new()
    }

    #[inline]
    pub fn connect_user_removed_signal(
        &self,
        callback: CallbackU32Path,
    ) -> zbus::fdo::Result<()> {
        self._inner.connect_user_removed(callback)
    }

    #[inline]
    pub fn disconnect_user_removed_signal(&self) -> zbus::fdo::Result<bool> {
        self._inner.disconnect_user_removed()
    }
}

#[cfg(test)]
mod tests {
    use crate::ManagerInterface;
    use zbus::Connection;

    #[test]
    fn timestamps() {
        let connection = Connection::new_system().unwrap();
        let manager = ManagerInterface::new(&connection).unwrap();

        assert!(manager.can_suspend().is_ok());
    }

    #[test]
    fn properties() {
        let connection = Connection::new_system().unwrap();
        let manager = ManagerInterface::new(&connection).unwrap();

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
        // UnknownProperty
        //assert!(manager.get_holdoff_timeout_usec().is_ok());
        assert!(manager.get_idle_action().is_ok());
        //assert!(manager.get_idle_action_usec().is_ok());
        assert!(manager.get_idle_hint().is_ok());
        assert!(manager.get_idle_since_hint().is_ok());
        assert!(manager.get_idle_since_hint_monotonic().is_ok());
        //assert!(manager.get_inhibit_delay_max_usec().is_ok());
        assert!(manager.get_inhibitors_max().is_ok());
        assert!(manager.get_kill_exclude_users().is_ok());
        assert!(manager.get_kill_only_users().is_ok());
        assert!(manager.get_kill_user_processes().is_ok());
        assert!(manager.get_lid_closed().is_ok());
        //assert!(manager.get_nauto_vts().is_ok());
        //assert!(manager.get_ncurrent_inhibitors().is_ok());
        //assert!(manager.get_ncurrent_sessions().is_ok());
        assert!(manager.get_on_external_power().is_ok());
        assert!(manager.get_preparing_for_shutdown().is_ok());
        assert!(manager.get_preparing_for_sleep().is_ok());
        assert!(manager.get_reboot_parameter().is_ok());
        assert!(manager.get_reboot_to_boot_loader_entry().is_ok());
        assert!(manager.get_reboot_to_boot_loader_menu().is_ok());
        assert!(manager.get_reboot_to_firmware_setup().is_ok());
        //assert!(manager.get_remove_ipc().is_ok());
        assert!(manager.get_runtime_directory_inodes_max().is_ok());
        assert!(manager.get_runtime_directory_size().is_ok());
        assert!(manager.get_scheduled_shutdown().is_ok());
        //assert!(manager.get_seat().is_ok());
        //assert!(manager.get_session().is_ok());
        //assert!(manager.get_session_by_pid().is_ok());
        assert!(manager.get_sessions_max().is_ok());
        //assert!(manager.get_user().is_ok());
        //assert!(manager.get_user_by_pid().is_ok());
        //assert!(manager.get_user_stop_delay_usec().is_ok());
        assert!(manager.get_wall_message().is_ok());
    }
}