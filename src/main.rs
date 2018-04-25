use std::io::{self, Write};
use std::env;
mod resources;
use resources::*;

fn main () {
    let usage = b"
Usage: ./huffman_v1 [OPTION] [INPUT FILE] [OUTPUT PATH]

-p   --pack      compress the file passed
-u   --unpack    decompress the file passed
            
Written by Andrea Galletti

\n";
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        let mut out_path = &String::from("./");
        if args.len() > 3 {
            out_path = &args[3]
        }
        if args[1] == "-p" || args[1] == "--pack" {
            pack(&args[2], out_path);
        } else if args[1] == "-u" || args[1] == "--unpack" {
            unpack(&args[2], out_path);
        } else {
            io::stdout().write(usage).unwrap();
        }
    } else {
        io::stdout().write(usage).unwrap();
    }
}

