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

#![macro_escape]

macro_rules! check_pointer(
    ($tmp_pointer:ident, $gtk_struct:ident) => (
        if $tmp_pointer.is_null() {
            None
        } else {
            Some($gtk_struct {
                pointer:            $tmp_pointer,
                can_drop:           true,
                signal_handlers:    Vec::new()
            })
        }
    );
)

macro_rules! impl_GtkWidget(
    ($gtk_struct:ident) => (
        impl GtkWidget for $gtk_struct {
            fn get_widget(&self) -> *ffi::C_GtkWidget {
                self.pointer
            }

            fn wrap_widget(widget: *ffi::C_GtkWidget) -> $gtk_struct {
                $gtk_struct {
                    pointer:         widget,
                    can_drop:        false,
                    signal_handlers: Vec::new()
                }
            }
        }
    );
)

macro_rules! redirect_callback(
    ($gtk_struct:ident) => (
        extern fn redirect_callback(widget: *ffi::C_GtkWidget, user_data: *c_void) -> () {
            let mut button = $gtk_struct { pointer: widget, can_drop: false, signal_handlers: Vec::new()};
            let sighandler = unsafe {(user_data as *SignalHandler).to_option().unwrap()};
            let func = sighandler.function.unwrap();
            func(&mut button, sighandler.user_data);
        }
    );
)

macro_rules! redirect_callback_widget(
    ($gtk_struct:ident) => (
        extern fn redirect_callback_widget(widget: *ffi::C_GtkWidget, user_data: *c_void) -> () {
            let mut button = $gtk_struct { pointer: widget, can_drop: false, signal_handlers: Vec::new()};
            let sighandler = unsafe {(user_data as *SignalHandler).to_option().unwrap()};
            let user_data = if !user_data.is_null() { 
                Some(unsafe { 
                    let tmp: &mut $gtk_struct = std::mem::transmute(sighandler.user_data as *mut $gtk_struct);
                    tmp as &mut GtkWidget
                })
            }  else {
                None
            };
            let func = sighandler.function_widget.unwrap();
            func(&mut button, user_data);
        }
    );
)

macro_rules! struct_signal(
    ($gtk_struct:ident) => (
        #[doc(hidden)]
        pub struct SignalHandler {
            function: Option<fn(&mut $gtk_struct, *c_void)>,
            function_widget: Option<fn(&mut $gtk_struct, Option<&mut GtkWidget>)>,
            user_data: *c_void
        }
    );
)


macro_rules! impl_signals(
    ($gtk_struct:ident) => (
        impl Signal for $gtk_struct {
            fn connect(&mut self, signal: &str, function: fn()) -> () {
                unsafe { 
                    signal.with_c_str(|c_str| {
                        ffi::signal_connect(self.pointer, c_str, Some(function))
                    })
                }
            }

            fn connect_2p<B>(&mut self, 
                             signal: &str, 
                             function: fn(&mut $gtk_struct, *c_void), 
                             user_data: Option<&B>) -> () {
                unsafe {
                    let tmp_sighandler_ptr: *c_void = std::mem::transmute(box SignalHandler {
                        function: Some(function),
                        function_widget: None,
                        user_data: std::mem::transmute(user_data.unwrap())
                    });
                    signal.with_c_str(|c_str| {
                        ffi::signal_connect_2params(self.pointer, 
                                                    c_str, 
                                                    Some(redirect_callback), 
                                                    tmp_sighandler_ptr) 
                    });
                    self.signal_handlers.push(std::mem::transmute(tmp_sighandler_ptr));
                }
            }

            fn connect_2p_widget<B: GtkWidget>(&mut self, 
                                               signal: &str, 
                                               function: fn(&mut $gtk_struct, Option<&mut GtkWidget>), 
                                               user_data: Option<&B>) -> () {
                unsafe{ 
                    let tmp_sighandler_ptr: *c_void = std::mem::transmute(box SignalHandler {
                        function: None, 
                        function_widget: Some(function),
                        user_data: if user_data.is_some() { std::mem::transmute(user_data.unwrap()) } else { ptr::null() }
                    });

                    signal.with_c_str(|c_str| {
                        ffi::signal_connect_2params(self.pointer, 
                                                    c_str, 
                                                    Some(redirect_callback_widget), 
                                                    tmp_sighandler_ptr)
                    });

                    self.signal_handlers.push(std::mem::transmute(tmp_sighandler_ptr));
                }
            }

        }
    );
)