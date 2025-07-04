#![forbid(unsafe_code)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    rust_2018_idioms
)]
#![allow(clippy::too_many_arguments)]
// TODO: fix error variants too long
#![allow(clippy::result_large_err)]
// TODO: disable unwraps:
//  https://github.com/informalsystems/hermes/issues/987
// #![cfg_attr(not(test), deny(clippy::unwrap_used))]

//! IBC Relayer implementation as a library.
//!
//! For the IBC relayer binary, please see [Hermes] (`ibc-relayer-cli` crate).
//!
//! [Hermes]: https://docs.rs/ibc-relayer-cli/1.13.2/

extern crate alloc;

pub mod account;
pub mod cache;
pub mod chain;
pub mod channel;
pub mod client_state;
pub mod config;
pub mod connection;
pub mod consensus_state;
pub mod denom;
pub mod error;
pub mod event;
pub mod extension_options;
pub mod foreign_client;
pub mod keyring;
pub mod light_client;
pub mod link;
pub mod misbehaviour;
pub mod object;
pub mod path;
pub mod registry;
pub mod rest;
pub mod sdk_error;
pub mod spawn;
pub mod supervisor;
pub mod telemetry;
pub mod transfer;
pub mod upgrade_chain;
pub mod util;
pub mod worker;

pub const HERMES_VERSION: &str = "1.13.2";
