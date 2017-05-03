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

extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

mod helloworld;
mod helloworld_grpc;

pub use self::helloworld::*;
pub use self::helloworld_grpc::*;
