# huffman-rust
Huffman-rust is a simple application of huffman code **lossless compression** logic written in Rust.

Installation
------------
To compile the binary you must have installed Cargo and Rust compiler:
```console
$ git clone https://github.com/bomboclat/huffman-rust.git
$ cd huffman-rust
$ cargo build --release
```

Usage
------
To compress:
```console
$ huffman_compression -p /path/to/file.txt
```

To decompress:
```console
$ huffman_compression -u /path/to/file.txt.comp
```

`.comp` is the extension of compressed files.
