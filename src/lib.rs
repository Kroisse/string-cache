// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "string_cache"]
#![crate_type = "rlib"]

#![feature(plugin, unsafe_no_drop_flag)]
#![feature(core, collections, alloc, hash)]
#![deny(warnings)]
#![cfg_attr(test, feature(test, std_misc))]
#![cfg_attr(bench, feature(rand))]
#![plugin(phf_macros, string_cache_macros)]

#[cfg(test)]
extern crate test;

#[no_link]
extern crate phf_macros;
extern crate phf;

#[macro_use]
extern crate lazy_static;

extern crate rand;
extern crate xxhash;

#[no_link] #[macro_use]
extern crate string_cache_macros;

#[cfg(feature = "log-events")]
extern crate serialize;

pub use atom::Atom;
pub use namespace::{Namespace, QualName};

#[cfg(feature = "log-events")]
pub mod event;

pub mod atom;
pub mod namespace;

// A private module so that macro-expanded idents like
// `::string_cache::atom::Atom` will also work in this crate.
//
// `libstd` uses the same trick.
#[doc(hidden)]
mod string_cache {
    pub use atom;
    pub use namespace;
}
