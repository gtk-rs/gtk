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

use std::libc::c_void;

use traits::GtkWidget;


pub trait Signal: GtkWidget {
    fn connect(&mut self, signal: &str, function: fn()) -> ();
    fn connect_2p<B>(&mut self, signal: &str, function: fn(&mut Self, *c_void), user_data: Option<&B>) -> () ;
    fn connect_2p_widget<B: GtkWidget>(&mut self, signal: &str, function: fn(&mut Self, Option<&mut GtkWidget>), user_data: Option<&B>) -> () ;
}
