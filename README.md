# Passing files between adjacent crates

This is a short example how to use package metadata to pass copy files from a
depended crate to the dependent crate.  (Whether you should do that in the
first place is another matter.)

```console
$ cd my-lib
$ cargo build
   Compiling my-lib v0.1.0 (.../copy-static-between-crates/my-lib)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
$ cd ../my-bin
$ cargo run
   Compiling my-lib v0.1.0 (.../copy-static-between-crates/my-lib)
   Compiling my-bin v0.1.0 (.../copy-static-between-crates/my-bin)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/my-bin`
Hello, world!
```
