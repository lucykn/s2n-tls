// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

// Ensure memory is correctly managed in tests
// tests invoked using the checkers::test macro have additional
// memory sanity checks that occur
#[cfg(test)]
#[global_allocator]
static ALLOCATOR: checkers::Allocator = checkers::Allocator::system();

#[macro_use]
pub mod error;

pub mod callbacks;
pub mod cert_chain;
pub mod client_hello;
pub mod config;
pub mod connection;
pub mod enums;
#[cfg(feature = "unstable-fingerprint")]
pub mod fingerprint;
pub mod init;
pub mod pool;
pub mod security;

pub use s2n_tls_sys as ffi;

#[cfg(test)]
mod testing;
