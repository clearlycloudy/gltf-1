
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;
extern crate gltf_importer;

use gltf_importer::Import;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let import = Import::from_path(&path);
        match import.sync() {
            Ok(gltf) => println!("{:#?}", gltf),
            Err(err) => println!("Invalid glTF ({:?})", err),
        }
    } else {
        println!("usage: gltf-display <PATH>");
    }
}
