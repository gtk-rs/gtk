// This file is part of rustgtk.
//
// rustgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rustgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rustgtk.  If not, see <http://www.gnu.org/licenses/>.

//! Manipulation of colors

/// The Color structure is used to describe a color, similar to the XColor struct used in the X11 drawing API.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct Color {
    /// For allocated colors, the pixel value used to draw this color on the screen. Not used anymore.
    pub pixel:  u32,
    /// The red component of the color. This is a value between 0 and 65535, with 65535 indicating full intensity
    pub red:    u16,
    /// The green component of the color
    pub green:  u16,
    /// The blue component of the color
    pub blue:   u16
}

/// The GdkRGBA structure is used to represent a (possibly translucent) color, in a way that is compatible with cairos notion of color.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct RGBA {
    /// The intensity of the red channel from 0.0 to 1.0 inclusive
    pub red: f64,
    /// The intensity of the green channel from 0.0 to 1.0 inclusive
    pub green: f64,
    /// The intensity of the blue channel from 0.0 to 1.0 inclusive
    pub blue: f64,
    /// The opacity of the color from 0.0 for completely translucent to 1.0 for opaque
    pub alpha: f64
}