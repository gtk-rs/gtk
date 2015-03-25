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

#![feature(core)]
#![feature(unsafe_destructor)]

extern crate libc;
extern crate glib_sys as glib_ffi;

pub use glib_ffi as ffi;

use libc::c_char;

pub use self::list::{List, Elem, RevElem};
pub use self::slist::{SList, SElem};
pub use self::glib_container::GlibContainer;
pub use self::error::{Error};
pub use self::permission::Permission;
pub use self::traits::{FFIGObject, Connect};
pub use self::value::{Value, ValuePublic};
pub use type_::Type;

mod list;
mod slist;
pub mod glib_container;
mod error;
mod permission;
pub mod traits;
pub mod translate;
mod value;
pub mod type_;

pub fn to_gboolean(b: bool) -> ffi::Gboolean {
    match b {
        true => ffi::GTRUE,
        false => ffi::GFALSE
    }
}

pub fn to_bool(b: ffi::Gboolean) -> bool {
    b != ffi::GFALSE
}

// An opaque structure used as the base of all interface types.
#[derive(Copy)]
pub struct TypeInterface;

// An opaque structure used as the base of all type instances.
#[derive(Copy)]
pub struct TypeInstance;

// An opaque structure used as the base of all classes.
#[derive(Copy)]
pub struct TypeClass;

//FIXME: Check if this is actually correct (maybe not since ParamFlags is deprecated)
#[derive(Copy)]
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

#[derive(Copy)]
#[repr(C)]
pub struct ParamSpec {
    g_type_instance: TypeInstance,
    name: *mut c_char,
    flags: ParamFlags,
    value_type: ffi::GType,
    owner_type: ffi::GType,
}
