// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::subclass::prelude::*;

use glib::translate::*;

use super::container::ContainerImpl;
use crate::Inhibit;
use ContainerClass;
use Socket;
use SocketClass;

pub trait SocketImpl: SocketImplExt + ContainerImpl {
    fn plug_added(&self, socket: &Socket) {
        self.parent_plug_added(socket)
    }

    fn plug_removed(&self, socket: &Socket) -> Inhibit {
        self.parent_plug_removed(socket)
    }
}

pub trait SocketImplExt {
    fn parent_plug_added(&self, socket: &Socket);
    fn parent_plug_removed(&self, socket: &Socket) -> Inhibit;
}

impl<T: SocketImpl> SocketImplExt for T {
    fn parent_plug_added(&self, socket: &Socket) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkSocketClass;
            if let Some(f) = (*parent_class).plug_added {
                f(socket.to_glib_none().0)
            }
        }
    }

    fn parent_plug_removed(&self, socket: &Socket) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkSocketClass;
            if let Some(f) = (*parent_class).plug_removed {
                Inhibit(from_glib(f(socket.to_glib_none().0)))
            } else {
                Inhibit(false)
            }
        }
    }
}

unsafe impl<T: SocketImpl> IsSubclassable<T> for SocketClass {
    fn override_vfuncs(&mut self) {
        <ContainerClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkSocketClass);
            klass.plug_added = Some(socket_plug_added::<T>);
            klass.plug_removed = Some(socket_plug_removed::<T>);
        }
    }
}

unsafe extern "C" fn socket_plug_added<T: SocketImpl>(ptr: *mut gtk_sys::GtkSocket) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Socket> = from_glib_borrow(ptr);

    imp.plug_added(&wrap)
}

unsafe extern "C" fn socket_plug_removed<T: SocketImpl>(
    ptr: *mut gtk_sys::GtkSocket,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Socket> = from_glib_borrow(ptr);

    imp.plug_removed(&wrap).to_glib()
}
