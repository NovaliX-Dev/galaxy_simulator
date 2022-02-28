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

use crate::common::vec2::{Vec2, Vec2F};

pub struct Viewport {
    pub scale: f64,
    initial_scale: f64,
    pub scaled_shift: Vec2F,
    absolute_shift: Vec2F,
}

impl Viewport {
    pub fn new(scale: f64, shift: Vec2F) -> Self {
        Self {
            scale,
            initial_scale: scale,
            scaled_shift: shift,
            absolute_shift: shift,
        }
    }

    pub fn zoom(&mut self, y: i32, window_size: (u32, u32)) {
        // compute new scale
        let delta_scale = y as f64 * 0.01 * self.initial_scale;
        self.scale += delta_scale;

        // compute the center of the window
        let win_vec = Vec2::new(window_size.0, window_size.1).convert(|v| v as f64);
        let center = win_vec / 2.0;

        self.scaled_shift -= (center + self.absolute_shift) * delta_scale;
    }

    pub fn move_(&mut self, x: i32, y: i32) {
        self.scaled_shift += Vec2F::new(x.into(), y.into());

        self.absolute_shift -= Vec2F::new(x.into(), y.into()) / self.scale;
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            scale: 1.0,
            initial_scale: 1.0,
            scaled_shift: Vec2::new_null(),
            absolute_shift: Vec2::new_null(),
        }
    }
}