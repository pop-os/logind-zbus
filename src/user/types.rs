use std::str::FromStr;
use serde::{Deserialize, Serialize};
use zbus::fdo;
use zvariant::{OwnedValue, Type};
use crate::{enum_impl_serde_str, enum_impl_str_conv, impl_try_from_owned_as_str};

/// State of a User
#[derive(Debug, PartialEq, Clone, Copy, Type)]
#[zvariant(signature = "s")]
pub enum UserState {
    Online,
    Offline,
    Lingering,
    Active,
    Closing,
}
enum_impl_serde_str!(UserState);
impl_try_from_owned_as_str!(UserState);
enum_impl_str_conv!(UserState, {
    "online": Online,
    "offline": Offline,
    "lingering": Lingering,
    "active": Active,
    "closing": Closing,
});

