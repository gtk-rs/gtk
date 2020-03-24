use gtk_sys;

use libc::c_int;
use std::mem;

use glib::translate::*;

use glib::subclass::prelude::*;
use glib::ObjectClass;

use crate::DragResult;
use crate::Inhibit;
use crate::Orientation;
use crate::SelectionData;
use crate::TextDirection;
use cairo;
use cairo_sys;
use Allocation;
use SizeRequestMode;
use Widget;
use WidgetClass;
use WidgetExt;

pub trait WidgetImpl: WidgetImplExt + ObjectImpl + 'static {
    fn adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32) {
        self.parent_adjust_baseline_allocation(widget, baseline)
    }

    fn adjust_baseline_request(
        &self,
        widget: &Widget,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    ) {
        self.parent_adjust_baseline_request(widget, minimum_baseline, natural_baseline)
    }

    fn adjust_size_allocation(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        self.parent_adjust_size_allocation(
            widget,
            orientation,
            minimum_size,
            natural_size,
            allocated_pos,
            allocated_size,
        )
    }

    fn adjust_size_request(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        self.parent_adjust_size_request(widget, orientation, minimum_size, natural_size)
    }

    fn button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_press_event(widget, event)
    }

    fn button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        self.parent_button_release_event(widget, event)
    }

    fn child_notify(&self, widget: &Widget, child_property: &glib::ParamSpec) {
        self.parent_child_notify(widget, child_property)
    }

    fn composited_changed(&self, widget: &Widget) {
        self.parent_composited_changed(widget)
    }

    fn compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        self.parent_compute_expand(widget, hexpand_p, vexpand_p)
    }

    fn configure_event(&self, widget: &Widget, event: &gdk::EventConfigure) -> Inhibit {
        self.parent_configure_event(widget, event)
    }

    fn damage_event(&self, widget: &Widget, event: &gdk::EventExpose) -> Inhibit {
        self.parent_damage_event(widget, event)
    }

    fn delete_event(&self, widget: &Widget, event: &gdk::Event) -> Inhibit {
        self.parent_delete_event(widget, event)
    }

    fn destroy(&self, widget: &Widget) {
        self.parent_destroy(widget)
    }

    fn destroy_event(&self, widget: &Widget, event: &gdk::Event) -> Inhibit {
        self.parent_destroy_event(widget, event)
    }

    fn direction_changed(&self, widget: &Widget, previous_direction: TextDirection) {
        self.parent_direction_changed(widget, previous_direction)
    }

    fn dispatch_child_properties_changed(&self, widget: &Widget, pspecs: &[glib::ParamSpec]) {
        self.parent_dispatch_child_properties_changed(widget, pspecs)
    }

    fn drag_begin(&self, widget: &Widget, context: &gdk::DragContext) {
        self.parent_drag_begin(widget, context)
    }

    fn drag_data_delete(&self, widget: &Widget, context: &gdk::DragContext) {
        self.parent_drag_data_delete(widget, context)
    }

    fn drag_data_get(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        self.parent_drag_data_get(widget, context, selection_data, info, time)
    }

    fn drag_data_received(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        self.parent_drag_data_received(widget, context, x, y, selection_data, info, time)
    }

    fn drag_drop(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        self.parent_drag_drop(widget, context, x, y, time)
    }

    fn drag_end(&self, widget: &Widget, context: &gdk::DragContext) {
        self.parent_drag_end(widget, context)
    }

    fn drag_failed(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        result: DragResult,
    ) -> Inhibit {
        self.parent_drag_failed(widget, context, result)
    }

    fn drag_leave(&self, widget: &Widget, context: &gdk::DragContext, time: u32) {
        self.parent_drag_leave(widget, context, time)
    }

    fn drag_motion(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        self.parent_drag_motion(widget, context, x, y, time)
    }

    fn draw(&self, widget: &Widget, cr: &cairo::Context) -> Inhibit {
        self.parent_draw(widget, cr)
    }

    // fn can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
    //     self.parent_can_activate_accel(widget, signal_id)
    // }

    fn get_request_mode(&self, widget: &Widget) -> SizeRequestMode {
        self.parent_get_request_mode(widget)
    }

    fn get_preferred_width(&self, widget: &Widget) -> (i32, i32) {
        self.parent_get_preferred_width(widget)
    }

    fn get_preferred_width_for_height(&self, widget: &Widget, height: i32) -> (i32, i32) {
        self.parent_get_preferred_width_for_height(widget, height)
    }

    fn get_preferred_height(&self, widget: &Widget) -> (i32, i32) {
        self.parent_get_preferred_height(widget)
    }

    fn get_preferred_height_for_width(&self, widget: &Widget, width: i32) -> (i32, i32) {
        self.parent_get_preferred_height_for_width(widget, width)
    }

    fn size_allocate(&self, widget: &Widget, allocation: &Allocation) {
        self.parent_size_allocate(widget, allocation)
    }

    fn realize(&self, widget: &Widget) {
        self.parent_realize(widget);
    }

    fn unrealize(&self, widget: &Widget) {
        self.parent_unrealize(widget);
    }
    fn map(&self, widget: &Widget) {
        self.parent_map(widget);
    }

    fn unmap(&self, widget: &Widget) {
        self.parent_unmap(widget);
    }

    fn motion_notify_event(&self, widget: &Widget, event: &gdk::EventMotion) -> Inhibit {
        self.parent_motion_notify_event(widget, event)
    }

    fn scroll_event(&self, widget: &Widget, event: &gdk::EventScroll) -> Inhibit {
        self.parent_scroll_event(widget, event)
    }
}

pub trait WidgetImplExt {
    fn parent_adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32);
    fn parent_adjust_baseline_request(
        &self,
        widget: &Widget,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    );
    fn parent_adjust_size_allocation(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    );
    fn parent_adjust_size_request(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    );
    fn parent_button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit;
    fn parent_button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit;
    // fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool;
    fn parent_child_notify(&self, widget: &Widget, child_property: &glib::ParamSpec);
    fn parent_composited_changed(&self, widget: &Widget);
    fn parent_compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool);
    fn parent_configure_event(&self, widget: &Widget, event: &gdk::EventConfigure) -> Inhibit;
    fn parent_damage_event(&self, widget: &Widget, event: &gdk::EventExpose) -> Inhibit;
    fn parent_delete_event(&self, widget: &Widget, event: &gdk::Event) -> Inhibit;
    fn parent_destroy(&self, widget: &Widget);
    fn parent_destroy_event(&self, widget: &Widget, event: &gdk::Event) -> Inhibit;
    fn parent_direction_changed(&self, widget: &Widget, previous_direction: TextDirection);
    fn parent_dispatch_child_properties_changed(&self, widget: &Widget, pspecs: &[glib::ParamSpec]);
    fn parent_drag_begin(&self, widget: &Widget, context: &gdk::DragContext);
    fn parent_drag_data_delete(&self, widget: &Widget, context: &gdk::DragContext);
    fn parent_drag_data_get(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    );
    fn parent_drag_data_received(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    );
    fn parent_drag_drop(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit;
    fn parent_drag_end(&self, widget: &Widget, context: &gdk::DragContext);
    fn parent_drag_failed(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        result: DragResult,
    ) -> Inhibit;
    fn parent_drag_leave(&self, widget: &Widget, context: &gdk::DragContext, time: u32);
    fn parent_drag_motion(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit;
    fn parent_draw(&self, widget: &Widget, cr: &cairo::Context) -> Inhibit;
    fn parent_get_request_mode(&self, widget: &Widget) -> SizeRequestMode;
    fn parent_get_preferred_width(&self, widget: &Widget) -> (i32, i32);
    fn parent_get_preferred_width_for_height(&self, widget: &Widget, height: i32) -> (i32, i32);
    fn parent_get_preferred_height(&self, widget: &Widget) -> (i32, i32);
    fn parent_get_preferred_height_for_width(&self, widget: &Widget, width: i32) -> (i32, i32);
    fn parent_size_allocate(&self, widget: &Widget, allocation: &Allocation);
    fn parent_realize(&self, widget: &Widget);
    fn parent_unrealize(&self, widget: &Widget);
    fn parent_map(&self, widget: &Widget);
    fn parent_unmap(&self, widget: &Widget);
    fn parent_motion_notify_event(&self, widget: &Widget, event: &gdk::EventMotion) -> Inhibit;
    fn parent_scroll_event(&self, widget: &Widget, event: &gdk::EventScroll) -> Inhibit;
}

impl<T: WidgetImpl + ObjectImpl> WidgetImplExt for T {
    fn parent_adjust_baseline_allocation(&self, widget: &Widget, baseline: &mut i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_allocation
                .expect("No parent class impl for \"adjust_baseline_allocation\"");
            f(widget.to_glib_none().0, baseline)
        }
    }

    fn parent_adjust_baseline_request(
        &self,
        widget: &Widget,
        minimum_baseline: &mut i32,
        natural_baseline: &mut i32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_baseline_request
                .expect("No parent class impl for \"adjust_baseline_request\"");
            f(widget.to_glib_none().0, minimum_baseline, natural_baseline)
        }
    }

    fn parent_adjust_size_allocation(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
        allocated_pos: &mut i32,
        allocated_size: &mut i32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_allocation
                .expect("No parent class impl for \"adjust_size_allocation\"");
            f(
                widget.to_glib_none().0,
                orientation.to_glib(),
                minimum_size,
                natural_size,
                allocated_pos,
                allocated_size,
            )
        }
    }

    fn parent_adjust_size_request(
        &self,
        widget: &Widget,
        orientation: Orientation,
        minimum_size: &mut i32,
        natural_size: &mut i32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .adjust_size_request
                .expect("No parent class impl for \"adjust_size_request\"");
            f(
                widget.to_glib_none().0,
                orientation.to_glib(),
                minimum_size as *mut i32,
                natural_size as *mut i32,
            )
        }
    }

    fn parent_button_press_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).button_press_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_button_release_event(&self, widget: &Widget, event: &gdk::EventButton) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).button_release_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
            } else {
                Inhibit(false)
            }
        }
    }

    // fn parent_can_activate_accel(&self, widget: &Widget, signal_id: u32) -> bool {
    //     unsafe {
    //         let data = self.get_type_data();
    //         let parent_class =
    //             data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
    //         let f = (*parent_class)
    //             .can_activate_accel
    //             .expect("No parent class impl for \"can_activate_accel\"");
    //         f(widget.to_glib_none().0, signal_id) != 0
    //     }
    // }

    fn parent_child_notify(&self, widget: &Widget, child_property: &glib::ParamSpec) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).child_notify {
                let pspec_glib = glib::translate::mut_override(child_property.to_glib_none().0);
                f(widget.to_glib_none().0, pspec_glib)
            }
        }
    }

    fn parent_composited_changed(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).composited_changed {
                f(widget.to_glib_none().0)
            }
        }
    }

    fn parent_compute_expand(&self, widget: &Widget, hexpand_p: &mut bool, vexpand_p: &mut bool) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).compute_expand {
                let mut h: i32 = hexpand_p.to_glib();
                let mut v: i32 = vexpand_p.to_glib();
                f(widget.to_glib_none().0, &mut h, &mut v);
                *hexpand_p = from_glib(h);
                *vexpand_p = from_glib(v);
            } else {
                // Fill the booleans so the compiler will be happy
                // and do nothing else since the vmenthod is NULL
                *hexpand_p = widget.get_hexpand();
                *vexpand_p = widget.get_vexpand();
            }
        }
    }

    fn parent_configure_event(&self, widget: &Widget, event: &gdk::EventConfigure) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).configure_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_damage_event(&self, widget: &Widget, event: &gdk::EventExpose) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).damage_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_delete_event(&self, widget: &Widget, event: &gdk::Event) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).delete_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_destroy(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).destroy {
                f(widget.to_glib_none().0)
            }
        }
    }

    fn parent_destroy_event(&self, widget: &Widget, event: &gdk::Event) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).destroy_event {
                let ev_glib = glib::translate::mut_override(event.to_glib_none().0);
                Inhibit(from_glib(f(widget.to_glib_none().0, ev_glib)))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_direction_changed(&self, widget: &Widget, previous_direction: TextDirection) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).direction_changed {
                f(widget.to_glib_none().0, previous_direction.to_glib())
            }
        }
    }

    fn parent_dispatch_child_properties_changed(
        &self,
        widget: &Widget,
        pspecs: &[glib::ParamSpec],
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).dispatch_child_properties_changed {
                let mut pspecs_array = pspecs
                    .iter()
                    .map(|p| p.to_glib_none().0)
                    .collect::<Vec<_>>();
                let pspecs_ptr = pspecs_array.as_mut_ptr();
                f(widget.to_glib_none().0, pspecs.len() as u32, pspecs_ptr)
            }
        }
    }

    fn parent_drag_begin(&self, widget: &Widget, context: &gdk::DragContext) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_begin {
                f(widget.to_glib_none().0, context.to_glib_none().0)
            }
        }
    }

    fn parent_drag_data_delete(&self, widget: &Widget, context: &gdk::DragContext) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_delete {
                f(widget.to_glib_none().0, context.to_glib_none().0)
            }
        }
    }

    fn parent_drag_data_get(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_get {
                let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
                f(
                    widget.to_glib_none().0,
                    context.to_glib_none().0,
                    selection_mut,
                    info,
                    time,
                )
            }
        }
    }

    fn parent_drag_data_received(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        selection_data: &SelectionData,
        info: u32,
        time: u32,
    ) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_data_received {
                let selection_mut = glib::translate::mut_override(selection_data.to_glib_none().0);
                f(
                    widget.to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    selection_mut,
                    info,
                    time,
                )
            }
        }
    }

    fn parent_drag_drop(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_drop {
                Inhibit(from_glib(f(
                    widget.to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    time,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_drag_end(&self, widget: &Widget, context: &gdk::DragContext) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_end {
                f(widget.to_glib_none().0, context.to_glib_none().0)
            }
        }
    }

    fn parent_drag_failed(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        result: DragResult,
    ) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_failed {
                Inhibit(from_glib(f(
                    widget.to_glib_none().0,
                    context.to_glib_none().0,
                    result.to_glib(),
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_drag_leave(&self, widget: &Widget, context: &gdk::DragContext, time: u32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_leave {
                f(widget.to_glib_none().0, context.to_glib_none().0, time)
            }
        }
    }

    fn parent_drag_motion(
        &self,
        widget: &Widget,
        context: &gdk::DragContext,
        x: i32,
        y: i32,
        time: u32,
    ) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).drag_motion {
                Inhibit(from_glib(f(
                    widget.to_glib_none().0,
                    context.to_glib_none().0,
                    x,
                    y,
                    time,
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_draw(&self, widget: &Widget, cr: &cairo::Context) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).draw {
                Inhibit(from_glib(f(widget.to_glib_none().0, cr.to_glib_none().0)))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_get_request_mode(&self, widget: &Widget) -> SizeRequestMode {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_request_mode.unwrap();
            from_glib(f(widget.to_glib_none().0))
        }
    }

    fn parent_get_preferred_width(&self, widget: &Widget) -> (i32, i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_width.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_get_preferred_width_for_height(&self, widget: &Widget, height: i32) -> (i32, i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_width_for_height.unwrap();

            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.to_glib_none().0,
                height,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_get_preferred_height(&self, widget: &Widget) -> (i32, i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_height.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.to_glib_none().0,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }
    fn parent_get_preferred_height_for_width(&self, widget: &Widget, width: i32) -> (i32, i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class).get_preferred_height_for_width.unwrap();
            let mut minimum_size = mem::MaybeUninit::uninit();
            let mut natural_size = mem::MaybeUninit::uninit();
            f(
                widget.to_glib_none().0,
                width,
                minimum_size.as_mut_ptr(),
                natural_size.as_mut_ptr(),
            );
            (minimum_size.assume_init(), natural_size.assume_init())
        }
    }

    fn parent_size_allocate(&self, widget: &Widget, allocation: &Allocation) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            let f = (*parent_class)
                .size_allocate
                .expect("No parent class impl for \"size_allocate\"");
            f(
                widget.to_glib_none().0,
                mut_override(allocation.to_glib_none().0),
            );
        }
    }

    fn parent_realize(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).realize {
                f(widget.to_glib_none().0);
            }
        }
    }

    fn parent_unrealize(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).unrealize {
                f(widget.to_glib_none().0);
            }
        }
    }

    fn parent_map(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).map {
                f(widget.to_glib_none().0);
            }
        }
    }

    fn parent_unmap(&self, widget: &Widget) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).unmap {
                f(widget.to_glib_none().0);
            }
        }
    }

    fn parent_motion_notify_event(&self, widget: &Widget, event: &gdk::EventMotion) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).motion_notify_event {
                Inhibit(from_glib(f(
                    widget.to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                )))
            } else {
                Inhibit(false)
            }
        }
    }

    fn parent_scroll_event(&self, widget: &Widget, event: &gdk::EventScroll) -> Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWidgetClass;
            if let Some(f) = (*parent_class).scroll_event {
                Inhibit(from_glib(f(
                    widget.to_glib_none().0,
                    mut_override(event.to_glib_none().0),
                )))
            } else {
                Inhibit(false)
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + WidgetImpl> IsSubclassable<T> for WidgetClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkWidgetClass);
            klass.adjust_baseline_allocation = Some(widget_adjust_baseline_allocation::<T>);
            klass.adjust_baseline_request = Some(widget_adjust_baseline_request::<T>);
            klass.adjust_size_allocation = Some(widget_adjust_size_allocation::<T>);
            klass.adjust_size_request = Some(widget_adjust_size_request::<T>);
            klass.button_press_event = Some(widget_button_press_event::<T>);
            klass.button_release_event = Some(widget_button_release_event::<T>);
            // klass.can_activate_accel = Some(widget_can_activate_accel::<T>);
            klass.child_notify = Some(widget_child_notify::<T>);
            klass.composited_changed = Some(widget_composited_changed::<T>);
            klass.compute_expand = Some(widget_compute_expand::<T>);
            klass.configure_event = Some(widget_configure_event::<T>);
            klass.damage_event = Some(widget_damage_event::<T>);
            klass.delete_event = Some(widget_delete_event::<T>);
            klass.destroy = Some(widget_destroy::<T>);
            klass.destroy_event = Some(widget_destroy_event::<T>);
            klass.direction_changed = Some(widget_direction_changed::<T>);
            klass.dispatch_child_properties_changed =
                Some(widget_dispatch_child_properties_changed::<T>);
            klass.drag_begin = Some(widget_drag_begin::<T>);
            klass.drag_data_delete = Some(widget_drag_data_delete::<T>);
            klass.drag_data_get = Some(widget_drag_data_get::<T>);
            klass.drag_data_received = Some(widget_drag_data_received::<T>);
            klass.drag_drop = Some(widget_drag_drop::<T>);
            klass.drag_end = Some(widget_drag_end::<T>);
            klass.drag_failed = Some(widget_drag_failed::<T>);
            klass.drag_leave = Some(widget_drag_leave::<T>);
            klass.drag_motion = Some(widget_drag_motion::<T>);
            klass.draw = Some(widget_draw::<T>);
            klass.get_request_mode = Some(widget_get_request_mode::<T>);
            klass.get_preferred_width = Some(widget_get_preferred_width::<T>);
            klass.get_preferred_height_for_width = Some(widget_get_preferred_height_for_width::<T>);
            klass.get_preferred_height = Some(widget_get_preferred_height::<T>);
            klass.get_preferred_width_for_height = Some(widget_get_preferred_width_for_height::<T>);
            klass.size_allocate = Some(widget_size_allocate::<T>);
            klass.realize = Some(widget_realize::<T>);
            klass.unrealize = Some(widget_unrealize::<T>);
            klass.map = Some(widget_map::<T>);
            klass.unmap = Some(widget_unmap::<T>);
            klass.motion_notify_event = Some(widget_motion_notify_event::<T>);
            klass.scroll_event = Some(widget_scroll_event::<T>);
        }
    }
}

unsafe extern "C" fn widget_adjust_baseline_allocation<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    baseptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.adjust_baseline_allocation(&wrap, &mut *baseptr)
}

unsafe extern "C" fn widget_adjust_baseline_request<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    minptr: *mut i32,
    natptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.adjust_baseline_request(&wrap, &mut *minptr, &mut *natptr)
}

unsafe extern "C" fn widget_adjust_size_allocation<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    orientation: gtk_sys::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
    posptr: *mut i32,
    sizeptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let wrap_orientation: Orientation = from_glib(orientation);

    imp.adjust_size_allocation(
        &wrap,
        wrap_orientation,
        &mut *minptr,
        &mut *natptr,
        &mut *posptr,
        &mut *sizeptr,
    )
}

unsafe extern "C" fn widget_adjust_size_request<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    orientation: gtk_sys::GtkOrientation,
    minptr: *mut i32,
    natptr: *mut i32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let wrap_orientation: Orientation = from_glib(orientation);

    imp.adjust_size_request(&wrap, wrap_orientation, &mut *minptr, &mut *natptr)
}

unsafe extern "C" fn widget_button_press_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    btnptr: *mut gdk_sys::GdkEventButton,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventButton = from_glib_borrow(btnptr);

    imp.button_press_event(&wrap, &evwrap).to_glib()
}

unsafe extern "C" fn widget_button_release_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    btnptr: *mut gdk_sys::GdkEventButton,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventButton = from_glib_borrow(btnptr);

    imp.button_release_event(&wrap, &evwrap).to_glib()
}

// unsafe extern "C" fn widget_can_activate_accel<T: ObjectSubclass>(
//     ptr: *mut gtk_sys::GtkWidget,
//     signal_id: u32,
// ) -> glib_sys::gboolean
//     where T: WidgetImpl
// {
//     let instance = &*(ptr as *mut T::Instance);
//     let imp = instance.get_impl();
//     let wrap: Widget = from_glib_borrow(ptr);

//     imp.can_activate_accel(&wrap, signal_id) as glib_sys::gboolean
// }

unsafe extern "C" fn widget_child_notify<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    paramptr: *mut gobject_sys::GParamSpec,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let paramwrap: glib::ParamSpec = from_glib_borrow(paramptr);

    imp.child_notify(&wrap, &paramwrap)
}

unsafe extern "C" fn widget_composited_changed<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWidget)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.composited_changed(&wrap)
}

unsafe extern "C" fn widget_compute_expand<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    hexpand_ptr: *mut glib_sys::gboolean,
    vexpand_ptr: *mut glib_sys::gboolean,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let mut hexpand_p: bool = from_glib(*hexpand_ptr);
    let mut vexpand_p: bool = from_glib(*vexpand_ptr);

    imp.compute_expand(&wrap, &mut hexpand_p, &mut vexpand_p);
    *hexpand_ptr = hexpand_p.to_glib();
    *vexpand_ptr = vexpand_p.to_glib();
}

unsafe extern "C" fn widget_configure_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    confptr: *mut gdk_sys::GdkEventConfigure,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventConfigure = from_glib_borrow(confptr);

    imp.configure_event(&wrap, &evwrap).to_glib()
}

unsafe extern "C" fn widget_damage_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    exposeptr: *mut gdk_sys::GdkEventExpose,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::EventExpose = from_glib_borrow(exposeptr);

    imp.damage_event(&wrap, &evwrap).to_glib()
}

unsafe extern "C" fn widget_delete_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    anyptr: *mut gdk_sys::GdkEventAny,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::Event = from_glib_borrow(anyptr);

    imp.delete_event(&wrap, &evwrap).to_glib()
}

unsafe extern "C" fn widget_destroy<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWidget)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.destroy(&wrap)
}

unsafe extern "C" fn widget_destroy_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    anyptr: *mut gdk_sys::GdkEventAny,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let evwrap: gdk::Event = from_glib_borrow(anyptr);

    imp.destroy_event(&wrap, &evwrap).to_glib()
}

unsafe extern "C" fn widget_direction_changed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    directnptr: gtk_sys::GtkTextDirection,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let dirwrap: TextDirection = from_glib(directnptr);

    imp.direction_changed(&wrap, dirwrap)
}

unsafe extern "C" fn widget_dispatch_child_properties_changed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    n_pspec_ptr: u32,
    pspecsptr: *mut *mut gobject_sys::GParamSpec,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let pspecs: Vec<glib::ParamSpec> =
        FromGlibContainer::from_glib_none_num(pspecsptr, n_pspec_ptr as usize);

    imp.dispatch_child_properties_changed(&wrap, &pspecs)
}

unsafe extern "C" fn widget_drag_begin<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_begin(&wrap, &context)
}

unsafe extern "C" fn widget_drag_data_delete<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_data_delete(&wrap, &context)
}

unsafe extern "C" fn widget_drag_data_get<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    selectptr: *mut gtk_sys::GtkSelectionData,
    info: u32,
    time: u32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);
    let selection_data: SelectionData = from_glib_borrow(selectptr);

    imp.drag_data_get(&wrap, &context, &selection_data, info, time)
}

unsafe extern "C" fn widget_drag_data_received<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    selectptr: *mut gtk_sys::GtkSelectionData,
    info: u32,
    time: u32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);
    let selection_data: SelectionData = from_glib_borrow(selectptr);

    imp.drag_data_received(&wrap, &context, x, y, &selection_data, info, time)
}

unsafe extern "C" fn widget_drag_drop<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_drop(&wrap, &context, x, y, time).to_glib()
}

unsafe extern "C" fn widget_drag_end<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_end(&wrap, &context)
}

unsafe extern "C" fn widget_drag_failed<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    resultptr: gtk_sys::GtkDragResult,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);
    let result: DragResult = from_glib(resultptr);

    imp.drag_failed(&wrap, &context, result).to_glib()
}

unsafe extern "C" fn widget_drag_leave<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    time: u32,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_leave(&wrap, &context, time)
}

unsafe extern "C" fn widget_drag_motion<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    ctxptr: *mut gdk_sys::GdkDragContext,
    x: i32,
    y: i32,
    time: u32,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let context: gdk::DragContext = from_glib_borrow(ctxptr);

    imp.drag_motion(&wrap, &context, x, y, time).to_glib()
}

unsafe extern "C" fn widget_draw<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    cr_ptr: *mut cairo_sys::cairo_t,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let cr: cairo::Context = from_glib_borrow(cr_ptr);

    imp.draw(&wrap, &cr).to_glib()
}

unsafe extern "C" fn widget_get_request_mode<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
) -> gtk_sys::GtkSizeRequestMode
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.get_request_mode(&wrap).to_glib()
}

unsafe extern "C" fn widget_get_preferred_height<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    let (min_size, nat_size) = imp.get_preferred_height(&wrap);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn widget_get_preferred_width_for_height<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    height: c_int,
    min_width_ptr: *mut c_int,
    nat_width_ptr: *mut c_int,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    let (min_width, nat_width) = imp.get_preferred_width_for_height(&wrap, height);
    if !min_width_ptr.is_null() {
        *min_width_ptr = min_width;
    }
    if !nat_width_ptr.is_null() {
        *nat_width_ptr = nat_width;
    }
}

unsafe extern "C" fn widget_get_preferred_width<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    minptr: *mut c_int,
    natptr: *mut c_int,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let (min_size, nat_size) = imp.get_preferred_width(&wrap);
    if !minptr.is_null() {
        *minptr = min_size;
    }
    if !natptr.is_null() {
        *natptr = nat_size;
    }
}

unsafe extern "C" fn widget_get_preferred_height_for_width<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    width: c_int,
    min_height_ptr: *mut c_int,
    nat_height_ptr: *mut c_int,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    let (min_height, nat_height) = imp.get_preferred_height_for_width(&wrap, width);
    if !min_height_ptr.is_null() {
        *min_height_ptr = min_height;
    }
    if !nat_height_ptr.is_null() {
        *nat_height_ptr = nat_height;
    }
}

unsafe extern "C" fn widget_size_allocate<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    allocation: *mut gtk_sys::GtkAllocation,
) where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let allocate: &Allocation = &from_glib_none(allocation);

    imp.size_allocate(&wrap, allocate);
}

pub unsafe extern "C" fn widget_realize<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWidget)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.realize(&wrap);
}

pub unsafe extern "C" fn widget_unrealize<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWidget)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);

    imp.unrealize(&wrap);
}

pub unsafe extern "C" fn widget_map<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWidget)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    imp.map(&wrap);
}

pub unsafe extern "C" fn widget_unmap<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWidget)
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    imp.unmap(&wrap);
}

pub unsafe extern "C" fn widget_motion_notify_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    mptr: *mut gdk_sys::GdkEventMotion,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let event: gdk::EventMotion = from_glib_borrow(mptr);

    imp.motion_notify_event(&wrap, &event).to_glib()
}

pub unsafe extern "C" fn widget_scroll_event<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWidget,
    mptr: *mut gdk_sys::GdkEventScroll,
) -> glib_sys::gboolean
where
    T: WidgetImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Widget = from_glib_borrow(ptr);
    let event: gdk::EventScroll = from_glib_borrow(mptr);

    imp.scroll_event(&wrap, &event).to_glib()
}
