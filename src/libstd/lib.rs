// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
//! It is linked to all Rust crates by default.
//!
//! ## Intrinsic types and operations
//!
//! The [`ptr`](ptr/index.html), [`mem`](mem/index.html),
//! and [`cast`](cast/index.html) modules deal with unsafe pointers,
//! memory manipulation, and coercion.
//! [`kinds`](kinds/index.html) defines the special built-in traits,
//! and [`raw`](raw/index.html) the runtime representation of Rust types.
//! These are some of the lowest-level building blocks of Rust
//! abstractions.
//!
//! ## Math on primitive types and math traits
//!
//! Although basic operations on primitive types are implemented
//! directly by the compiler, the standard library additionally
//! defines many common operations through traits defined in
//! mod [`num`](num/index.html).
//!
//! ## Pervasive types
//!
//! The [`option`](option/index.html) and [`result`](result/index.html)
//! modules define optional and error-handling types, `Option` and `Result`.
//! [`iter`](iter/index.html) defines Rust's iterator protocol
//! along with a wide variety of iterators.
//! [`Cell` and `RefCell`](cell/index.html) are for creating types that
//! manage their own mutability.
//!
//! ## Vectors, slices and strings
//!
//! The common container type, `Vec`, a growable vector backed by an
//! array, lives in the [`vec`](vec/index.html) module. References to
//! arrays, `&[T]`, more commonly called "slices", are built-in types
//! for which the [`slice`](slice/index.html) module defines many
//! methods.
//!
//! UTF-8 strings, `~str` and `&str`, are built-in types, and the
//! standard library defines methods for them on a variety of traits
//! in the [`str`](str/index.html) module. Rust strings are immutable;
//! use the `StrBuf` type defined in [`strbuf`](strbuf/index.html)
//! for a mutable string builder.
//!
//! For converting to strings use the [`format!`](fmt/index.html)
//! macro, and for converting from strings use the
//! [`FromStr`](from_str/index.html) trait.
//!
//! ## Platform abstractions
//!
//! Besides basic data types, the standard library is largely concerned
//! with abstracting over differences in common platforms, most notably
//! Windows and Unix derivatives. The [`os`](os/index.html) module
//! provides a number of basic functions for interacting with the
//! operating environment, including program arguments, environment
//! variables, and directory navigation. The [`path`](path/index.html)
//! module encapsulates the platform-specific rules for dealing
//! with file paths.
//!
//! `std` also includes modules for interoperating with the
//! C language: [`c_str`](c_str/index.html) and
//! [`c_vec`](c_vec/index.html).
//!
//! ## Concurrency, I/O, and the runtime
//!
//! The [`task`](task/index.html) module contains Rust's threading abstractions,
//! while [`comm`](comm/index.html) contains the channel types for message
//! passing. [`sync`](sync/index.html) contains further, primitive, shared
//! memory types, including [`atomics`](sync/atomics/index.html).
//!
//! Common types of I/O, including files, TCP, UPD, pipes, Unix domain sockets,
//! timers, and process spawning, are defined in the [`io`](io/index.html).
//!
//! Rust's I/O and concurrency depends on a small runtime interface
//! that lives, along with its support code, in mod [`rt`](rt/index.html).
//! While a notable part of the standard library's architecture, this
//! module is not intended for public use.
//!
//! ## The Rust prelude and macros
//!
//! Finally, the [`prelude`](prelude/index.html) defines a set of
//! common set of traits, types, and functions that are made available
//! to all code by default. [`macros`](macros/index.html) contains
//! all the standard macros, such as `assert!`, `fail!`, `println!`.

#![crate_id = "std#0.11-pre"]
#![comment = "The Rust standard library"]
#![license = "MIT/ASL2"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://static.rust-lang.org/doc/master")]
#![feature(macro_rules, globs, asm, managed_boxes, thread_local, link_args,
           simd, linkage, default_type_params, phase, concat_idents, quad_precision_float)]

// Don't link to std. We are std.
#![no_std]

#![deny(missing_doc)]

// When testing libstd, bring in libuv as the I/O backend so tests can print
// things and all of the std::io tests have an I/O interface to run on top
// of
#[cfg(test)] extern crate rustuv;
#[cfg(test)] extern crate native;
#[cfg(test)] extern crate green;
#[cfg(test)] #[phase(syntax, link)] extern crate log;

// Make and rand accessible for benchmarking/testcases
#[cfg(test)] extern crate rand;

// we wrap some libc stuff
extern crate libc;

// Make std testable by not duplicating lang items. See #2912
#[cfg(test)] extern crate realstd = "std";
#[cfg(test)] pub use kinds = realstd::kinds;
#[cfg(test)] pub use ops = realstd::ops;
#[cfg(test)] pub use cmp = realstd::cmp;
#[cfg(test)] pub use ty = realstd::ty;

// Run tests with libgreen instead of libnative.
//
// FIXME: This egregiously hacks around starting the test runner in a different
//        threading mode than the default by reaching into the auto-generated
//        '__test' module.
#[cfg(test)] #[start]
fn start(argc: int, argv: **u8) -> int {
    green::start(argc, argv, rustuv::event_loop, __test::main)
}

/* Exported macros */

pub mod macros;
pub mod bitflags;

mod rtdeps;

/* The Prelude. */

pub mod prelude;


/* Primitive types */

#[path = "num/float_macros.rs"] mod float_macros;
#[path = "num/int_macros.rs"]   mod int_macros;
#[path = "num/uint_macros.rs"]  mod uint_macros;

#[path = "num/int.rs"]  pub mod int;
#[path = "num/i8.rs"]   pub mod i8;
#[path = "num/i16.rs"]  pub mod i16;
#[path = "num/i32.rs"]  pub mod i32;
#[path = "num/i64.rs"]  pub mod i64;

#[path = "num/uint.rs"] pub mod uint;
#[path = "num/u8.rs"]   pub mod u8;
#[path = "num/u16.rs"]  pub mod u16;
#[path = "num/u32.rs"]  pub mod u32;
#[path = "num/u64.rs"]  pub mod u64;

#[path = "num/f32.rs"]   pub mod f32;
#[path = "num/f64.rs"]   pub mod f64;

pub mod unit;
pub mod bool;
pub mod char;
pub mod tuple;

pub mod slice;
pub mod vec;
pub mod str;
pub mod strbuf;

pub mod ascii;

pub mod ptr;
pub mod owned;
mod managed;
mod reference;
pub mod rc;
pub mod gc;


/* Core language traits */

#[cfg(not(test))] pub mod kinds;
#[cfg(not(test))] pub mod ops;
#[cfg(not(test))] pub mod cmp;
#[cfg(not(test))] pub mod ty;


/* Common traits */

pub mod from_str;
pub mod num;
pub mod iter;
pub mod to_str;
pub mod clone;
pub mod hash;
pub mod container;
pub mod default;
pub mod any;

/* Common data structures */

pub mod option;
pub mod result;
pub mod cell;


/* Tasks and communication */

pub mod task;
pub mod comm;
pub mod local_data;
pub mod sync;


/* Runtime and platform support */

pub mod c_str;
pub mod c_vec;
pub mod os;
pub mod io;
pub mod path;
pub mod cast;
pub mod fmt;
pub mod cleanup;
pub mod mem;


/* Unsupported interfaces */

#[unstable]
pub mod repr;
#[unstable]
pub mod reflect;

// Private APIs
#[unstable]
pub mod unstable;
#[experimental]
pub mod intrinsics;
#[experimental]
pub mod raw;

/* For internal use, not exported */

mod unicode;

// FIXME #7809: This shouldn't be pub, and it should be reexported under 'unstable'
// but name resolution doesn't work without it being pub.
#[unstable]
pub mod rt;

// A curious inner-module that's not exported that contains the binding
// 'std' so that macro-expanded references to std::error and such
// can be resolved within libstd.
#[doc(hidden)]
mod std {
    pub use clone;
    pub use cmp;
    pub use comm;
    pub use fmt;
    pub use hash;
    pub use io;
    pub use kinds;
    pub use local_data;
    pub use option;
    pub use os;
    pub use rt;
    pub use str;
    pub use to_str;
    pub use ty;
    pub use unstable;
    pub use vec;
}
