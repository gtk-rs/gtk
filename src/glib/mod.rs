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
Bindings and wrappers for __GLib__
*/

use libc::c_char;

pub use self::list::{List, Elem, RevElem};
pub use self::slist::{SList, SElem};
pub use self::glib_container::GlibContainer;
pub use self::error::{Error};
pub use self::permission::Permission;
pub use self::ffi::GType;
pub use self::traits::{FFIGObject, GObject};

mod list;
mod slist;
pub mod ffi;
pub mod glib_container;
mod error;
mod permission;
pub mod traits;

// An opaque structure used as the base of all interface types.
pub struct TypeInterface;

// An opaque structure used as the base of all type instances.
pub struct TypeInstance;

// An opaque structure used as the base of all classes.
pub struct TypeClass;

//FIXME: Check if this is actually correct (maybe not since ParamFlags is deprecated)
pub enum ParamFlags{
    Readable,
    Writable,
    ReadWrite,
    Construct,
    ConstructOnly,
    LaxValidation,
    StaticName,
    Private,
    StaticNick,
    StaticBlurb,
    Deprecated
}

pub struct ParamSpec {
    g_type_instance: TypeInstance,
    name: *mut c_char,
    flags: ParamFlags,
    value_type: GType,
    owner_type: GType
}
