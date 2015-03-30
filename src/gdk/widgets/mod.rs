// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

pub use self::app_launch_context::AppLaunchContext;
pub use self::atom::Atom;
pub use gdk_ffi::C_GdkColor as Color;
pub use self::cursor::Cursor;
pub use self::device::Device;
pub use self::device_manager::DeviceManager;
pub use self::display::Display;
pub use self::display_manager::DisplayManager;
pub use self::drag_context::DragContext;
#[cfg(feature = "GTK_3_8")]
pub use self::frame_clock::FrameClock;
#[cfg(feature = "GTK_3_8")]
pub use self::frame_timings::FrameTimings;
pub use self::pixbuf::Pixbuf;
pub use self::point::Point;
pub use self::rectangle::Rectangle;
pub use self::rgba::RGBA;
pub use self::screen::Screen;
pub use self::visual::Visual;
pub use self::window::{WindowAttr, Window};

mod app_launch_context;
mod atom;
mod cursor;
mod device;
mod device_manager;
mod display;
mod display_manager;
mod drag_context;
#[cfg(feature = "GTK_3_8")]
mod frame_clock;
#[cfg(feature = "GTK_3_8")]
mod frame_timings;
mod pixbuf;
mod point;
mod rectangle;
mod rgba;
mod screen;
mod visual;
mod window;
