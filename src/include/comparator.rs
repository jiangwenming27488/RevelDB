
// Copyright (c) 2011 The RevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

// A Comparator object provides a total order across slices that are
// used as keys in an stable or a database.  A Comparator implementation
// must be thread-safe since leveldb may invoke its methods concurrently
// from multiple threads.

use std::os::unix::raw::ino_t;
use crate::include::slice::Slice;

trait Comparator {
    // Three-way comparison.  Returns value:
    //   < 0 iff "a" < "b",
    //   == 0 iff "a" == "b",
    //   > 0 iff "a" > "b"
    fn compare(&self, a: &Slice, b: &Slice) -> i32;


    // The name of the comparator.  Used to check for comparator
    // mismatches (i.e., a DB created with one comparator is
    // accessed using a different comparator.
    //
    // The client of this package should switch to a new name whenever
    // the comparator implementation changes in a way that will cause
    // the relative ordering of any two keys to change.
    //
    // Names starting with "reveldb." are reserved and should not be used
    // by any clients of this package.
    fn name(&self) -> &str;


    // Advanced functions: these are used to reduce the space requirements
    // for internal data structures like index blocks.

    // If *start < limit, changes *start to a short string in [start,limit).
    // Simple comparator implementations may return with *start unchanged,
    // i.e., an implementation of this method that does nothing is correct.
    fn find_shortest_separator(&self, start: &String, limit: &Slice);


    // Changes *key to a short string >= *key.
    // Simple comparator implementations may return with *key unchanged,
    // i.e., an implementation of this method that does nothing is correct.
    fn find_short_successor(&self, key: &String);

    // Return a builtin comparator that uses lexicographic byte-wise
    // ordering.  The result remains the property of this module and
    // must not be deleted.
    fn byte_wise_comparator(&self) -> impl Comparator;
}

