/*
    Copyright © 2021 StarBrilliant <coder@poorlab.com>
    This work is free. You can redistribute it and/or modify it under the
    terms of the Do What The Fuck You Want To Public License, Version 2,
    as published by Sam Hocevar. See the COPYING file for more details.
*/

#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

#[cfg(not(feature = "allocator_api"))]
mod allocator_api_off;

#[cfg(feature = "allocator_api")]
mod allocator_api_on;

mod tests;

#[cfg(not(feature = "allocator_api"))]
pub use allocator_api_off::*;

#[cfg(feature = "allocator_api")]
pub use allocator_api_on::*;
