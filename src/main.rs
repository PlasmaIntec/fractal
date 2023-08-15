// Copyright (c) 2021 Joone Hur <joone@kldp.org> All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

extern crate glfw;

mod actor;
mod geometry;
mod koch;

use crate::koch::launch_koch;

#[tokio::main]
async fn main() {
	launch_koch().await
}
