//! # DBus interface proxy for: `org.freedesktop.login1.Manager`

#![allow(non_snake_case)]

use zbus::dbus_proxy;

use crate::types::{
    Inhibitors, IsSupported, Mode, ScheduledShutdown, SeatPath, SessionInfo, UserInfo,
};

#[dbus_proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait Manager {
    /// ActivateSession method
    #[inline]
    fn activate_session(&self, session_id: &str) -> zbus::Result<()>;

    /// ActivateSessionOnSeat method
    #[inline]
    fn activate_session_on_seat(&self, session_id: &str, seat_id: &str) -> zbus::Result<()>;

    /// AttachDevice method
    #[inline]
    fn attach_device(&self, seat_id: &str, sysfs_path: &str, interactive: bool)
        -> zbus::Result<()>;

    /// CanHalt method
    #[inline]
    fn can_halt(&self) -> zbus::Result<IsSupported>;

    /// CanHibernate method
    #[inline]
    fn can_hibernate(&self) -> zbus::Result<IsSupported>;

    /// CanHybridSleep method
    #[inline]
    fn can_hybrid_sleep(&self) -> zbus::Result<IsSupported>;

    /// CanPowerOff method
    #[inline]
    fn can_power_off(&self) -> zbus::Result<IsSupported>;

    /// CanReboot method
    #[inline]
    fn can_reboot(&self) -> zbus::Result<IsSupported>;

    /// CanRebootParameter method
    #[inline]
    fn can_reboot_parameter(&self) -> zbus::Result<IsSupported>;

    /// CanRebootToBootLoaderEntry method
    #[inline]
    fn can_reboot_to_boot_loader_entry(&self) -> zbus::Result<IsSupported>;

    /// CanRebootToBootLoaderMenu method
    #[inline]
    fn can_reboot_to_boot_loader_menu(&self) -> zbus::Result<IsSupported>;

    /// CanRebootToFirmwareSetup method
    #[inline]
    fn can_reboot_to_firmware_setup(&self) -> zbus::Result<IsSupported>;

    /// CanSuspend method
    #[inline]
    fn can_suspend(&self) -> zbus::Result<IsSupported>;

    /// CanSuspendThenHibernate method
    #[inline]
    fn can_suspend_then_hibernate(&self) -> zbus::Result<IsSupported>;

    /// CancelScheduledShutdown method
    #[inline]
    fn cancel_scheduled_shutdown(&self) -> zbus::Result<bool>;

    // /// CreateSession method
    // #[inline]
    // fn create_session(
    //     &self,
    //     session: SessionCreate,
    //     properties: &[(&str, zbus::zvariant::Value<'_>)],
    // ) -> zbus::Result<(
    //     String,
    //     zvariant::OwnedObjectPath,
    //     String,
    //     std::os::unix::io::RawFd,
    //     u32,
    //     String,
    //     u32,
    //     bool,
    // )>;

    /// FlushDevices method
    #[inline]
    fn flush_devices(&self, interactive: bool) -> zbus::Result<()>;

    /// GetSeat method
    #[inline]
    fn get_seat(&self, seat_id: &str) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// GetSession method
    #[inline]
    fn get_session(&self, session_id: &str) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// GetSessionByPID method
    #[inline]
    fn get_session_by_PID(&self, pid: u32) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// GetUser method
    #[inline]
    fn get_user(&self, uid: u32) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// GetUserByPID method
    #[inline]
    fn get_user_by_PID(&self, pid: u32) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// Halt method
    #[inline]
    fn halt(&self, interactive: bool) -> zbus::Result<()>;

    /// Hibernate method
    #[inline]
    fn hibernate(&self, interactive: bool) -> zbus::Result<()>;

    /// HybridSleep method
    #[inline]
    fn hybrid_sleep(&self, interactive: bool) -> zbus::Result<()>;

    /// Inhibit method
    #[inline]
    fn inhibit(
        &self,
        what: &str,
        who: &str,
        why: &str,
        mode: Mode,
    ) -> zbus::Result<std::os::unix::io::RawFd>;

    /// KillSession method
    #[inline]
    fn kill_session(&self, session_id: &str, who: &str, signal_number: i32) -> zbus::Result<()>;

    /// KillUser method
    #[inline]
    fn kill_user(&self, uid: u32, signal_number: i32) -> zbus::Result<()>;

    /// ListInhibitors method
    #[inline]
    fn list_inhibitors(&self) -> zbus::Result<Vec<Inhibitors>>;

    /// ListSeats method
    #[inline]
    fn list_seats(&self) -> zbus::Result<Vec<SeatPath>>;

    /// ListSessions method
    #[inline]
    fn list_sessions(&self) -> zbus::Result<Vec<SessionInfo>>;

    /// ListUsers method
    #[inline]
    fn list_users(&self) -> zbus::Result<Vec<UserInfo>>;

    /// LockSession method
    #[inline]
    fn lock_session(&self, session_id: &str) -> zbus::Result<()>;

    /// LockSessions method
    #[inline]
    fn lock_sessions(&self) -> zbus::Result<()>;

    /// PowerOff method
    #[inline]
    fn power_off(&self, interactive: bool) -> zbus::Result<()>;

    /// Reboot method
    #[inline]
    fn reboot(&self, interactive: bool) -> zbus::Result<()>;

    /// ReleaseSession method
    #[inline]
    fn release_session(&self, session_id: &str) -> zbus::Result<()>;

    /// ScheduleShutdown method
    #[inline]
    fn schedule_shutdown(&self, r#type: &str, usec: u64) -> zbus::Result<()>;

    /// SetRebootParameter method
    #[inline]
    fn set_reboot_parameter(&self, parameter: &str) -> zbus::Result<()>;

    /// SetRebootToBootLoaderEntry method
    #[inline]
    fn set_reboot_to_boot_loader_entry(&self, boot_loader_entry: &str) -> zbus::Result<()>;

    /// SetRebootToBootLoaderMenu method
    #[inline]
    fn set_reboot_to_boot_loader_menu(&self, timeout: u64) -> zbus::Result<()>;

    /// SetRebootToFirmwareSetup method
    #[inline]
    fn set_reboot_to_firmware_setup(&self, enable: bool) -> zbus::Result<()>;

    /// SetUserLinger method
    #[inline]
    fn set_user_linger(&self, uid: u32, enable: bool, interactive: bool) -> zbus::Result<()>;

    /// SetWallMessage method
    #[inline]
    fn set_wall_message(&self, wall_message: &str, enable: bool) -> zbus::Result<()>;

    /// Suspend method
    #[inline]
    fn suspend(&self, interactive: bool) -> zbus::Result<()>;

    /// SuspendThenHibernate method
    #[inline]
    fn suspend_then_hibernate(&self, interactive: bool) -> zbus::Result<()>;

    /// TerminateSeat method
    #[inline]
    fn terminate_seat(&self, seat_id: &str) -> zbus::Result<()>;

    /// TerminateSession method
    #[inline]
    fn terminate_session(&self, session_id: &str) -> zbus::Result<()>;

    /// TerminateUser method
    #[inline]
    fn terminate_user(&self, uid: u32) -> zbus::Result<()>;

    /// UnlockSession method
    #[inline]
    fn unlock_session(&self, session_id: &str) -> zbus::Result<()>;

    /// UnlockSessions method
    #[inline]
    fn unlock_sessions(&self) -> zbus::Result<()>;

    /// PrepareForShutdown signal
    #[dbus_proxy(signal)]
    #[inline]
    fn prepare_for_shutdown(&self, start: bool) -> zbus::Result<()>;

    /// PrepareForSleep signal
    #[dbus_proxy(signal)]
    #[inline]
    fn prepare_for_sleep(&self, start: bool) -> zbus::Result<()>;

    /// SeatNew signal
    #[dbus_proxy(signal)]
    #[inline]
    fn seat_new(&self, seat_id: &str, object_path: zvariant::OwnedObjectPath) -> zbus::Result<()>;

    /// SeatRemoved signal
    #[dbus_proxy(signal)]
    #[inline]
    fn seat_removed(
        &self,
        seat_id: &str,
        object_path: zvariant::OwnedObjectPath,
    ) -> zbus::Result<()>;

    /// SessionNew signal
    #[dbus_proxy(signal)]
    #[inline]
    fn session_new(
        &self,
        session_id: &str,
        object_path: zvariant::OwnedObjectPath,
    ) -> zbus::Result<()>;

    /// SessionRemoved signal
    #[dbus_proxy(signal)]
    #[inline]
    fn session_removed(
        &self,
        session_id: &str,
        object_path: zvariant::OwnedObjectPath,
    ) -> zbus::Result<()>;

    /// UserNew signal
    #[dbus_proxy(signal)]
    #[inline]
    fn user_new(&self, uid: u32, object_path: zvariant::OwnedObjectPath) -> zbus::Result<()>;

    /// UserRemoved signal
    #[dbus_proxy(signal)]
    #[inline]
    fn user_removed(&self, uid: u32, object_path: zvariant::OwnedObjectPath) -> zbus::Result<()>;

    /// BlockInhibited property
    #[dbus_proxy(property)]
    #[inline]
    fn block_inhibited(&self) -> zbus::Result<String>;

    /// BootLoaderEntries property
    #[dbus_proxy(property)]
    #[inline]
    fn boot_loader_entries(&self) -> zbus::Result<Vec<String>>;

    /// DelayInhibited property
    #[dbus_proxy(property)]
    #[inline]
    fn delay_inhibited(&self) -> zbus::Result<String>;

    /// Docked property
    #[dbus_proxy(property)]
    #[inline]
    fn docked(&self) -> zbus::Result<bool>;

    /// EnableWallMessages property
    #[dbus_proxy(property)]
    #[inline]
    fn enable_wall_messages(&self) -> zbus::Result<bool>;

    #[DBusProxy(property)]
    #[inline]
    fn set_enable_wall_messages(&self, value: bool) -> zbus::Result<()>;

    /// HandleHibernateKey property
    #[dbus_proxy(property)]
    #[inline]
    fn handle_hibernate_key(&self) -> zbus::Result<String>;

    /// HandleLidSwitch property
    #[dbus_proxy(property)]
    #[inline]
    fn handle_lid_switch(&self) -> zbus::Result<String>;

    /// HandleLidSwitchDocked property
    #[dbus_proxy(property)]
    #[inline]
    fn handle_lid_switch_docked(&self) -> zbus::Result<String>;

    /// HandleLidSwitchExternalPower property
    #[dbus_proxy(property)]
    #[inline]
    fn handle_lid_switch_external_power(&self) -> zbus::Result<String>;

    /// HandlePowerKey property
    #[dbus_proxy(property)]
    #[inline]
    fn handle_power_key(&self) -> zbus::Result<String>;

    /// HandleSuspendKey property
    #[dbus_proxy(property)]
    #[inline]
    fn handle_suspend_key(&self) -> zbus::Result<String>;

    /// HoldoffTimeoutUSec property
    #[dbus_proxy(property)]
    #[inline]
    fn holdoff_timeout_USec(&self) -> zbus::Result<u64>;

    /// IdleAction property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_action(&self) -> zbus::Result<String>;

    /// IdleActionUSec property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_action_USec(&self) -> zbus::Result<u64>;

    /// IdleHint property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_hint(&self) -> zbus::Result<bool>;

    /// IdleSinceHint property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint(&self) -> zbus::Result<u64>;

    /// IdleSinceHintMonotonic property
    #[dbus_proxy(property)]
    #[inline]
    fn idle_since_hint_monotonic(&self) -> zbus::Result<u64>;

    /// InhibitDelayMaxUSec property
    #[dbus_proxy(property)]
    #[inline]
    fn inhibit_delay_max_USec(&self) -> zbus::Result<u64>;

    /// InhibitorsMax property
    #[dbus_proxy(property)]
    #[inline]
    fn inhibitors_max(&self) -> zbus::Result<u64>;

    /// KillExcludeUsers property
    #[dbus_proxy(property)]
    #[inline]
    fn kill_exclude_users(&self) -> zbus::Result<Vec<String>>;

    /// KillOnlyUsers property
    #[dbus_proxy(property)]
    #[inline]
    fn kill_only_users(&self) -> zbus::Result<Vec<String>>;

    /// KillUserProcesses property
    #[dbus_proxy(property)]
    #[inline]
    fn kill_user_processes(&self) -> zbus::Result<bool>;

    /// LidClosed property
    #[dbus_proxy(property)]
    #[inline]
    fn lid_closed(&self) -> zbus::Result<bool>;

    /// NAutoVTs property
    #[dbus_proxy(property)]
    #[inline]
    fn NAuto_VTs(&self) -> zbus::Result<u32>;

    /// NCurrentInhibitors property
    #[dbus_proxy(property)]
    #[inline]
    fn NCurrent_inhibitors(&self) -> zbus::Result<u64>;

    /// NCurrentSessions property
    #[dbus_proxy(property)]
    #[inline]
    fn NCurrent_sessions(&self) -> zbus::Result<u64>;

    /// OnExternalPower property
    #[dbus_proxy(property)]
    #[inline]
    fn on_external_power(&self) -> zbus::Result<bool>;

    /// PreparingForShutdown property
    #[dbus_proxy(property)]
    #[inline]
    fn preparing_for_shutdown(&self) -> zbus::Result<bool>;

    /// PreparingForSleep property
    #[dbus_proxy(property)]
    #[inline]
    fn preparing_for_sleep(&self) -> zbus::Result<bool>;

    /// RebootParameter property
    #[dbus_proxy(property)]
    #[inline]
    fn reboot_parameter(&self) -> zbus::Result<String>;

    /// RebootToBootLoaderEntry property
    #[dbus_proxy(property)]
    #[inline]
    fn reboot_to_boot_loader_entry(&self) -> zbus::Result<String>;

    /// RebootToBootLoaderMenu property
    #[dbus_proxy(property)]
    #[inline]
    fn reboot_to_boot_loader_menu(&self) -> zbus::Result<u64>;

    /// RebootToFirmwareSetup property
    #[dbus_proxy(property)]
    #[inline]
    fn reboot_to_firmware_setup(&self) -> zbus::Result<bool>;

    /// RemoveIPC property
    #[dbus_proxy(property)]
    #[inline]
    fn remove_IPC(&self) -> zbus::Result<bool>;

    /// RuntimeDirectoryInodesMax property
    #[dbus_proxy(property)]
    #[inline]
    fn runtime_directory_inodes_max(&self) -> zbus::Result<u64>;

    /// RuntimeDirectorySize property
    #[dbus_proxy(property)]
    #[inline]
    fn runtime_directory_size(&self) -> zbus::Result<u64>;

    /// ScheduledShutdown property
    #[dbus_proxy(property)]
    #[inline]
    fn scheduled_shutdown(&self) -> zbus::Result<ScheduledShutdown>;

    /// SessionsMax property
    #[dbus_proxy(property)]
    #[inline]
    fn sessions_max(&self) -> zbus::Result<u64>;

    /// UserStopDelayUSec property
    #[dbus_proxy(property)]
    #[inline]
    fn user_stop_delay_USec(&self) -> zbus::Result<u64>;

    /// WallMessage property
    #[dbus_proxy(property)]
    #[inline]
    fn wall_message(&self) -> zbus::Result<String>;
}

#[cfg(test)]
mod tests {
    use futures_lite::future;

    use crate::generated::manager::ManagerProxy;
    use crate::generated::manager::ManagerProxyBlocking;

    #[test]
    fn timestamps() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();

        assert!(manager.can_suspend().is_ok());
    }

    #[test]
    fn properties() {
        let connection = zbus::blocking::Connection::system().unwrap();
        let manager = ManagerProxyBlocking::new(&connection).unwrap();

        assert!(manager.block_inhibited().is_ok());
        assert!(manager.boot_loader_entries().is_ok());
        assert!(manager.delay_inhibited().is_ok());
        assert!(manager.docked().is_ok());
        assert!(manager.handle_hibernate_key().is_ok());
        assert!(manager.handle_lid_switch().is_ok());
        assert!(manager.handle_lid_switch_docked().is_ok());
        assert!(manager.handle_lid_switch_external_power().is_ok());
        assert!(manager.handle_power_key().is_ok());
        assert!(manager.handle_suspend_key().is_ok());
        assert!(manager.holdoff_timeout_USec().is_ok());
        assert!(manager.idle_action().is_ok());
        assert!(manager.idle_action_USec().is_ok());
        assert!(manager.idle_hint().is_ok());
        assert!(manager.idle_since_hint().is_ok());
        assert!(manager.idle_since_hint_monotonic().is_ok());
        assert!(manager.inhibit_delay_max_USec().is_ok());
        assert!(manager.inhibitors_max().is_ok());
        assert!(manager.kill_exclude_users().is_ok());
        assert!(manager.kill_only_users().is_ok());
        assert!(manager.kill_user_processes().is_ok());
        assert!(manager.lid_closed().is_ok());
        assert!(manager.NAuto_VTs().is_ok());
        assert!(manager.NCurrent_inhibitors().is_ok());
        assert!(manager.NCurrent_sessions().is_ok());
        assert!(manager.on_external_power().is_ok());
        assert!(manager.preparing_for_shutdown().is_ok());
        assert!(manager.preparing_for_sleep().is_ok());
        assert!(manager.reboot_parameter().is_ok());
        assert!(manager.reboot_to_boot_loader_entry().is_ok());
        assert!(manager.reboot_to_boot_loader_menu().is_ok());
        assert!(manager.reboot_to_firmware_setup().is_ok());
        assert!(manager.remove_IPC().is_ok());
        assert!(manager.runtime_directory_inodes_max().is_ok());
        assert!(manager.runtime_directory_size().is_ok());
        assert!(manager.scheduled_shutdown().is_ok());
        assert!(manager.get_seat("seat0").is_ok());

        // Requires a valid session ID
        // assert!(manager.get_session("c4").is_ok());
        // Requires knowing a process ID owned by a session
        // assert!(manager.get_session_by_pid(5455).is_ok());

        assert!(manager.sessions_max().is_ok());
        // Requires a valid logged-in user ID
        // assert!(manager.get_user(1000).is_ok());
        // Requires knowing a process ID
        // assert!(manager.get_user_by_pid(25813).is_ok());
        assert!(manager.user_stop_delay_USec().is_ok());
        assert!(manager.wall_message().is_ok());
    }

    #[test]
    fn timestamps_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();

            assert!(manager.can_suspend().await.is_ok());
        });
    }

    #[test]
    fn properties_async() {
        future::block_on(async {
            let connection = zbus::Connection::system().await.unwrap();
            let manager = ManagerProxy::new(&connection).await.unwrap();

            assert!(manager.block_inhibited().await.is_ok());
            assert!(manager.boot_loader_entries().await.is_ok());
            assert!(manager.delay_inhibited().await.is_ok());
            assert!(manager.docked().await.is_ok());
            assert!(manager.handle_hibernate_key().await.is_ok());
            assert!(manager.handle_lid_switch().await.is_ok());
            assert!(manager.handle_lid_switch_docked().await.is_ok());
            assert!(manager.handle_lid_switch_external_power().await.is_ok());
            assert!(manager.handle_power_key().await.is_ok());
            assert!(manager.handle_suspend_key().await.is_ok());
            assert!(manager.holdoff_timeout_USec().await.is_ok());
            assert!(manager.idle_action().await.is_ok());
            assert!(manager.idle_action_USec().await.is_ok());
            assert!(manager.idle_hint().await.is_ok());
            assert!(manager.idle_since_hint().await.is_ok());
            assert!(manager.idle_since_hint_monotonic().await.is_ok());
            assert!(manager.inhibit_delay_max_USec().await.is_ok());
            assert!(manager.inhibitors_max().await.is_ok());
            assert!(manager.kill_exclude_users().await.is_ok());
            assert!(manager.kill_only_users().await.is_ok());
            assert!(manager.kill_user_processes().await.is_ok());
            assert!(manager.lid_closed().await.is_ok());
            assert!(manager.NAuto_VTs().await.is_ok());
            assert!(manager.NCurrent_inhibitors().await.is_ok());
            assert!(manager.NCurrent_sessions().await.is_ok());
            assert!(manager.on_external_power().await.is_ok());
            assert!(manager.preparing_for_shutdown().await.is_ok());
            assert!(manager.preparing_for_sleep().await.is_ok());
            assert!(manager.reboot_parameter().await.is_ok());
            assert!(manager.reboot_to_boot_loader_entry().await.is_ok());
            assert!(manager.reboot_to_boot_loader_menu().await.is_ok());
            assert!(manager.reboot_to_firmware_setup().await.is_ok());
            assert!(manager.remove_IPC().await.is_ok());
            assert!(manager.runtime_directory_inodes_max().await.is_ok());
            assert!(manager.runtime_directory_size().await.is_ok());
            assert!(manager.scheduled_shutdown().await.is_ok());
            assert!(manager.get_seat("seat0").await.is_ok());

            // Requires a valid session ID
            // assert!(manager.get_session("c4").await.is_ok());
            // Requires knowing a process ID owned by a session
            // assert!(manager.get_session_by_pid(5455).await.is_ok());

            assert!(manager.sessions_max().await.is_ok());
            // Requires a valid logged-in user ID
            // assert!(manager.get_user(1000).await.is_ok());
            // Requires knowing a process ID
            // assert!(manager.get_user_by_pid(25813).await.is_ok());
            assert!(manager.user_stop_delay_USec().await.is_ok());
            assert!(manager.wall_message().await.is_ok());
        });
    }
}
