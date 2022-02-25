// A galaxy simulator made in Rust.
// Copyright (C) 2022 NovaliX
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use anyhow::Context;

use common::vec2f::Vec2F;
use sdl2::pixels::Color;
use simulation::object;

use crate::simulation::object::macros::create_object_value_checked;
use crate::simulation::object::Object;

mod app;
mod common;
mod renderer;
mod simulation;

fn main() -> anyhow::Result<()> {
    // -------------------------------------------------------------------------
    // Objects creation and configuration
    // -------------------------------------------------------------------------

    let bh1 = create_object_value_checked!(1000.0, Vec2F::new(300.0, 300.0), false, Color::RED);
    let bh2 = create_object_value_checked!(1000.0, Vec2F::new(500.0, 300.0), false, Color::RED);

    let mut s1 = create_object_value_checked!(50.0, Vec2F::new(400.0, 200.0), true, Color::CYAN);

    object::add_orbital_velocity(&mut s1, &vec![bh1, bh2], object::VelocityDirection::Left);

    let objects = vec![bh1, bh2, s1];

    // launch the app
    app::run(objects, 720000.0)
}