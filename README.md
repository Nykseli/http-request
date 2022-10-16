# System Calls as a Service

Blazingly fast system calls as a service!

## Requirements

* rust >=1.59
* nasm (optional)
  * used to compile examples

## How to run

First on a terminal start the server
```
cd server
cargo run
```

Then on a other terminal, you can run programs with the vm
```
cd vm
cargo run path-to-executable
```

### Running example read and writes

First start the server.

Then on another terminal:

```sh
# Manual test use this location for fs related things
mkdir -p /tmp/http-syscall/
cd tests/asm
make read_write
cd ../../vm
cargo run `pwd`/../tests/asm/read_write.out
```

Then you can confirm that the server wrote to `write-file.txt` and `write-file2.txt`

```sh
ls -l /tmp/http-syscall/
cat /tmp/http-syscall/write-file.txt
cat /tmp/http-syscall/write-file2.txt
```
