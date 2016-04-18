# mpirs - An implementation of MPI in Rust

This is an implementation of the MPI standard in Rust. Unlike other projects
that provide bindings to C (or other) implementations, mpirs is entirely
self-contained and completely written in Rust. This comes with advantages of
being able to use convenience and expressiveness of Rust but at the same time
the library is not complete and as heavily tested as compared to more mature
implementations.

__NOTE:__ This project is still in its alpha version and does not support all
functions mentioned in the MPI standard. However, it does provide the base and
framework to implement these easily.

## Usage Notes
Build you programs using the library mpirs. Once the binary is compiled, do
not use `cargo run` in order to execute your programs. mpirs comes with a
runner that takes care of setting and spawning processes (this is located in
the `bin/` directory). To run the program, use:

`./target/debug/mpirun -n <num_procs> <path/to/rust/executable>`

Alternatively, to make it easier to use, you could also set up a symlink to
the mpirun. (`ln -s $(PWD)/target/debug/mpirun /usr/bin/`)

### Example:

`./target/debug/mpirun -n 8 ./target/debug/token`

## Examples
Examples can be found in the [examples/](./examples) directory

## Building
mpirs can be built using a standard rust tool chain using `cargo build`.

## License

mpirs is dual-licensed under:
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

Use under either one of the above listed licenses is acceptable.
