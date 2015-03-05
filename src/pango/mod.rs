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

/*!

Bindings and wrappers for __PANGO__

*/

pub use self::widgets::{
    Item,
    Rectangle,
    Matrix,
    GlyphString
};

pub use self::enums::{
    Gravity,
    GravityHint,
    Script,
    Direction,
    BidiType,
    Style,
    Weight,
    Variant,
    Stretch,
    FontMask
};

pub mod widgets;
pub mod enums;

#[doc(hidden)]
pub mod ffi;
