# picmaker
This is the beginning of a Rust graphics engine.

## Rust Stuff

###
Installing

For Ubuntu 16.04 Xenial, running the shell script command provided at https://www.rust-lang.org/en-US/install.html and proceeding with the default installation (option 1) should install it.

For MacOS, you also run the shell script, but I ran into an error with the PATH, where rust tried to install it to the file ~/.profile instead of something like ~/.bashrc or ~/.zshrc. Adding the PATH to one of those files should make it work.

###
Building

As for building the project, rust comes with a default compiler called rustc. However, rust also comes with a project build tool called cargo which can build the project with cargo build or run the project (auto-compiling if there was a change, similar to make) with cargo run.

There is a Makefile provided, it should work, provided that rust was installed as described above, with cargo.