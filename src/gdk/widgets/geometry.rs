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
The GdkGeometry struct gives the window manager information about a window’s geometry constraints. Normally you would set these on the 
GTK+ level using gtk_window_set_geometry_hints(). GtkWindow then sets the hints on the GdkWindow it creates.

gdk_window_set_geometry_hints() expects the hints to be fully valid already and simply passes them to the window manager; in contrast, 
gtk_window_set_geometry_hints() performs some interpretation. For example, GtkWindow will apply the hints to the geometry widget instead 
of the toplevel window, if you set a geometry widget. Also, the min_width /min_height /max_width /max_height fields may be set to -1, and 
GtkWindow will substitute the size request of the window or geometry widget. If the minimum size hint is not provided, GtkWindow will use 
its requisition as the minimum size. If the minimum size is provided and a geometry widget is set, GtkWindow will take the minimum size 
as the minimum size of the geometry widget rather than the entire window. The base size is treated similarly.

The canonical use-case for gtk_window_set_geometry_hints() is to get a terminal widget to resize properly. Here, the terminal text area 
should be the geometry widget; GtkWindow will then automatically set the base size to the size of other widgets in the terminal window, 
such as the menubar and scrollbar. Then, the width_inc and height_inc fields should be set to the size of one character in the terminal. 
Finally, the base size should be set to the size of one character. The net effect is that the minimum size of the terminal will have a 
1x1 character terminal area, and only terminal sizes on the “character grid” will be allowed.
*/

use libc::c_int;
use gdk;

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub struct Geometry {
    /// minimum width of window (or -1 to use requisition, with GtkWindow only)
    pub min_width: c_int,
    /// minimum height of window (or -1 to use requisition, with GtkWindow only)
    pub min_height: c_int,
    /// maximum width of window (or -1 to use requisition, with GtkWindow only)
    pub max_width: c_int,
    /// maximum height of window (or -1 to use requisition, with GtkWindow only)
    pub max_height: c_int,
    /// allowed window widths are base_width + width_inc * N where N is any integer (-1 allowed with GtkWindow)
    pub base_width: c_int,
    /// allowed window widths are base_height + height_inc * N where N is any integer (-1 allowed with GtkWindow)
    pub base_height: c_int,
    /// width resize increment
    pub width_inc: c_int,
    /// height resize increment
    pub height_inc: c_int,
    /// minimum width/height ratio
    pub min_aspect: f64,
    /// maximum width/height ratio
    pub max_aspect: f64,
    /// window gravity, see gtk_window_set_gravity()
    pub win_gravity: gdk::Gravity
}