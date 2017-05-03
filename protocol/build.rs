/*
 * whistled_protocol: gRPC procotol for whistled used to relay rule information
 * Copyright (C) 2017 Aleksa Sarai <cyphar@cyphar.com>
 *
 * whistled_protocol is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * whistled_protocol is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with whistled_protocol.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate glob;

use glob::glob;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    /* Get the list of *.proto files. */
    let protos = glob("proto/*.proto").unwrap()
                                      .map(|x| x.unwrap())
                                      .collect::<Vec<PathBuf>>();

    /* Generate protobuf code. */
    let status = Command::new("protoc")
                         .arg("--rust_out=src")
                         .args(&protos)
                         .status()
                         .expect("failed to start protoc");
    if !status.success() {
        std::process::exit(status.code().unwrap());
    }

    /* Generate gRPC wrappers. */
    let status = Command::new("protoc")
                         .arg("--rust-grpc_out=src")
                         .args(&protos)
                         .status()
                         .expect("failed to start protoc");
    if !status.success() {
        std::process::exit(status.code().unwrap());
    }
}
