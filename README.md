<!--
SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE

SPDX-License-Identifier: EPL-2.0
-->

# BaSyx Rust SDK

[![Crates.io](https://img.shields.io/crates/l/basyx-rs.svg)](https://crates.io/crates/basyx-rs)
[![Crates.io](https://img.shields.io/crates/v/basyx-rs.svg)](https://crates.io/crates/basyx-rs)
[![Documentation](https://docs.rs/basyx-rs/badge.svg)](https://docs.rs/basyx-rs)
[![Minimum Stable Rust Version](https://img.shields.io/badge/Rust-1.54.0%2B-blue?color=fc8d62&logo=rust)](https://rustup.rs/)
[![ClearlyDefined Score](https://img.shields.io/clearlydefined/score/crate/cratesio/-/basyx-rs/0.1.0?label=ClearlyDefined%20Score)](https://clearlydefined.io/definitions/crate/cratesio/-/basyx-rs/0.1.0)
[![Dependency Status](https://deps.rs/repo/github/eclipse-basyx/basyx-rust-sdk/status.svg)](https://deps.rs/repo/github/eclipse-basyx/basyx-rust-sdk)
[![Build Status](https://img.shields.io/github/workflow/status/eclipse-basyx/basyx-rust-sdk/Run%20CI/main)](https://github.com)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A Rust library to work with Asset Administration Shells (AAS).

This library supports version 3.0-RC01 of the ["Details of the Asset Administration Shell"](https://www.plattform-i40.de/IP/Redaktion/EN/Downloads/Publikation/Details_of_the_Asset_Administration_Shell_Part1_V3.html). At the moment, only JSON (de-)serialization is supported.

The Eclipse BaSyx project provides a [wiki](https://wiki.eclipse.org/BaSyx).

For install information, see [this page](https://wiki.eclipse.org/BaSyx_/_Download).
There are introductory examples provided [here](https://wiki.eclipse.org/BaSyx_/_Introductory_Examples).
Additionally, the API is described [here](https://wiki.eclipse.org/BaSyx_/_Documentation_/_API).
If you would like to contribute, the overall process is described [here](https://wiki.eclipse.org/BaSyx_/_Developer_/_Contributing).

## Checking JSON Files

Installing this crate with `cargo install --path .` provides access to the `aascheck` binary utility:

```bash
aascheck --mode Submodel <INPUT>
```

or for a complete AAS environment:

```bash
aascheck --mode AAS <INPUT>
```

## Funding Acknowledgment

Eclipse BaSyx was started in 2017 for the implementation of the results
of the project "BaSys 4.0", funded by the German Federal Ministry
of Education and Research (BMBF), grant number 01IS16022.

## Declared Project Licenses

This program and the accompanying materials are made available under the terms of the Eclipse Public License v. 2.0 which is available at
https://www.eclipse.org/org/documents/epl-2.0/EPL-2.0.html

SPDX-License-Identifier: EPL-2.0