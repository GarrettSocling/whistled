/*
 * whistled: generic GNU/Linux application firewall daemon
 * Copyright (C) 2017 Aleksa Sarai <cyphar@cyphar.com>
 *
 * whistled is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * whistled is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with whistled.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate whistled_protocol;
extern crate grpc;

use whistled_protocol::*;

fn main() {
    let client = GreeterClient::new("localhost", 50051, false).unwrap();

    let mut req = HelloRequest::new();
    req.set_name("mynameis".to_string());

    let resp = client.SayHello(req);
    println!("{:?}", resp);
}
