// Copyright (c) 2015 Takeru Ohta <phjgt308@gmail.com>
//
// This software is released under the MIT License,
// see the LICENSE file at the top-level directory.

extern crate dawg;

use std::env;
use std::process;
use std::io;
use std::io::BufRead;
use dawg::binary_tree::Builder as BinaryTreeBuilder;
use dawg::double_array::Builder as DoubleArrayBuilder;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} OUTPUT_INDEX_FILE", args[0]);
        process::exit(1);
    }

    let stdin = io::stdin();
    let output_file = &args[1];

    let mut builder = BinaryTreeBuilder::new();
    for line in stdin.lock().lines() {
        match line {
            Err(e) => {
                println!("[ERROR] Can't read line: reason={}", e);
                process::exit(1);
            }
            Ok(line) => {
                if let Err(e) = builder.insert(line.bytes()) {
                    println!("[ERROR] Can't insert a word {:?}: reason={}", line, e);
                    process::exit(1);
                }
            }
        }
    }
    let trie = DoubleArrayBuilder::new().build(builder.finish());
    if let Err(e) = trie.save(output_file) {
        println!("[ERROR] Can't save dawg index: path={}, reason={}",
                 output_file,
                 e);
        process::exit(1);
    }

    println!("DONE");
}
