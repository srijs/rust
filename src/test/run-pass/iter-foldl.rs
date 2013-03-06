// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn add(x: &float, y: &uint) -> float { *x + ((*y) as float) }

pub fn main() {
    fail_unless!([1u, 3u].foldl(20f, add) == 24f);
    fail_unless!([].foldl(20f, add) == 20f);
    fail_unless!(iter::foldl(&None::<uint>, 20f, add) == 20f);
    fail_unless!(iter::foldl(&Some(1u), 20f, add) == 21f);
    fail_unless!(iter::foldl(&Some(2u), 20f, add) == 22f);
}
