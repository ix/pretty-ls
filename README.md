pretty-ls (pls)
===============

pretty-ls is a "minimalist" `ls` implementation written in Rust.
It is often faster than GNU `ls`, while providing much more eye-pleasing output.

![pls in use](/meta/pls.png)

#TODO
- [X] Improve the output for single file listings.
- [X] Improve or drop support for Windows (dropped, see issue #3).
- [X] Remove the dependency on the `term` crate.
- [X] Optimize the program's speed.
  - [ ] Optimize it even more.
