// Copyright (c) 2011 The RevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.
//
// Slice is a simple structure containing a pointer into some external
// storage and a size.  The user of a Slice must ensure that the slice
// is not used after the corresponding external storage has been
// deallocated.
//
// Multiple threads can invoke const methods on a Slice without
// external synchronization, but if any of the threads may call a
// non-const method, all threads accessing the same Slice must use
// external synchronization.


pub struct Slice<'a> {
    data: &'a str
}


impl Drop for Slice {
    fn drop(&mut self) {
        //do nothing
    }
}

impl Slice {
    // Create a slice that refers to d[0,n-1].
    pub fn new(data: &str) -> Self {
        Slice {
            data: &data
        }
    }


    pub fn data(&self) -> &str {
        self.data
    }

    // Return the length (in bytes) of the referenced data
    pub fn size(&self) -> usize {
        self.data.len()
    }


    // Change this slice to refer to an empty array
    pub fn clear(&mut self) {
        self.data = "";
    }


    // Drop the first "n" bytes from this slice.
    pub fn remove_prefix(&mut self, n: usize) {
        self.data = &self.data[n..];
    }


    // Return true iff the length of the referenced data is zero
    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

/*

// Return the ith byte in the referenced data.
// REQUIRES: n < size()
char operator[](size_t n) const {
assert(n < size());
return data_[n];
}






// Return a string that contains the copy of the referenced data.
std::string ToString() const { return std::string(data_, size_); }

// Three-way comparison.  Returns value:
//   <  0 iff "*this" <  "b",
//   == 0 iff "*this" == "b",
//   >  0 iff "*this" >  "b"
int compare(const Slice& b) const;

// Return true iff "x" is a prefix of "*this"
bool starts_with(const Slice& x) const {
return ((size_ >= x.size_) && (memcmp(data_, x.data_, x.size_) == 0));
}




inline bool operator==(const Slice& x, const Slice& y) {
return ((x.size() == y.size()) &&
(memcmp(x.data(), y.data(), x.size()) == 0));
}

inline bool operator!=(const Slice& x, const Slice& y) { return !(x == y); }

inline int Slice::compare(const Slice& b) const {
const size_t min_len = (size_ < b.size_) ? size_ : b.size_;
int r = memcmp(data_, b.data_, min_len);
if (r == 0) {
if (size_ < b.size_)
r = -1;
else if (size_ > b.size_)
r = +1;
}
return r;
}

}  // namespace leveldb

#endif  // STORAGE_LEVELDB_INCLUDE_SLICE_H_
*/
