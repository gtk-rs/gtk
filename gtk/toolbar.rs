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

//! Create bars of buttons and other widgets

use std::libc::c_void;
use std::{ptr, cast};

use traits::{GtkWidget, GtkToolShell, GtkOrientable, GtkContainer, Signal};
use ffi;
// use gtk::enums::GtkIconSize;

/** 
* Toolbar — Create bars of buttons and other widgets
*
* # Availables signals :
* * `focus-home-or-end` : Action
* * `orientation-changed` : Run First
* * `popup-context-menu` : Run Last
* * `style-changed` : Run First
*/
pub struct ToolBar {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl ToolBar {
  
}

impl_GtkWidget!(ToolBar)
redirect_callback!(ToolBar)
redirect_callback_widget!(ToolBar)
struct_signal!(ToolBar)
impl_signals!(ToolBar)

impl GtkContainer for ToolBar {}
impl GtkToolShell for ToolBar {}
impl GtkOrientable for ToolBar {}