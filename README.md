# logind-zbus

A wrapper around the dbus interfaces provided by `systemd-logind`.

`logind-zbus` aims to provide a convenient API abstraction of the dbus interface
of logind in rust, where possible parsing responses to concrete structs and enums.

The crate is usable as is, but some of the interface properties are missing while
the associated issues are solved.

## Features

A single feature flag is available: `non_blocking`, which switches out `zbus::blocking::Connection`
for `zbus::Connection` allowing for async rust.

---

Q: `zbus-xmlgen` works fine, why not use the generated code directly?

A: It still needs massaging to fix small issues, and to provide convenient
helper types/structs/enums. The generated methods also don't show in IDE
auto-completion due to reasons.