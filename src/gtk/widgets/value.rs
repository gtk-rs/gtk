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

use libc::c_char;

pub trait GValue {
    type Value;

    fn get(&self) -> &<Self as GValue>::Value {
        *self
    }
}

pub trait GValuePrivate {
    type Ret;

    fn get_gvalue(&self) -> *const <Self as GValuePrivate>::Ret {
        self
    }
}

impl GValuePrivate for String {
    type Ret = c_char;
    
    fn get_gvalue(&self) -> *const c_char {
        self.to_c_str().as_str()
    }
}

impl GValuePrivate for i32 {}
impl GValuePrivate for i64 {}
impl GValuePrivate for f32 {}
impl GValuePrivate for f64 {}
impl GValuePrivate for u32 {}
impl GValuePrivate for u64 {}

impl GValue for String {}
impl GValue for i32 {}
impl GValue for i64 {}
impl GValue for f32 {}
impl GValue for f64 {}
impl GValue for u32 {}
impl GValue for u64 {}