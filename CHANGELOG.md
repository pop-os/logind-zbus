# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
# [0.5.0] - 2021-03-15
### Changed
- Add deref, deref_mut, as_ref, as_mut to all proxy to coeerce to `Proxy`
- Rename all `<Name>Interface` to `<Name>Proxy` as correct

# [0.4.0] - 2021-03-15
### Changed
- Begin using zbus 2.0
- Remove wrapper `disconnect_<name>` functions due to no-longer being generated

# [0.3.0] - 2021-03-15
### Changed
- Pass closures to wrapped signals correctly
- Return wrapped proxy as `<Type>Proxy` instead of trait object `Proxy`
- Enable more signals
- Add basic signal example