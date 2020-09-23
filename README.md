# cow-rc-str

CowRcStr optimization for Rust. Taken from https://github.com/servo/rust-cssparser/blob/master/src/cow_rc_str.rs.

Use CowRcStr in order to borrow from an original input string and avoid allocating/copying when possible.
