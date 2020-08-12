# rust-mdb-demo
Proof of concept mdb module built in rust

## Example output

```
root - rustdev ~/src/mdb-test (git:HEAD) # file target/debug/librustmdb.so
target/debug/librustmdb.so:     ELF 64-bit LSB dynamic lib AMD64 Version 1, dynamically linked, not stripped
root - rustdev ~/src/mdb-test (git:HEAD) # mdb
> ::load target/debug/librustmdb.so
> ::help rust_dcmd

NAME
  rust_dcmd - hello from a rust mdb dcmd

SYNOPSIS
  ::rust_dcmd

ATTRIBUTES

  Target: proc
  Module: librustmdb
  Interface Stability: Unstable

>
```
