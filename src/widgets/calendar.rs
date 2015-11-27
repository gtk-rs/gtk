// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_uint, c_int};

use CalendarDisplayOptions;
use cast::GTK_CALENDAR;
use ffi;
use glib::to_bool;

/**
* Calendar — Displays a calendar and allows the user to select a date
*
* # Available signals:
* * `day-selected` : Run First
* * `day-selected-double-click` : Run First
* * `month-changed` : Run First
* * `next-month` : Run First
* * `next-year` : Run First
* * `prev-month` : Run First
* * `prev-year` : Run First
*/
struct_Widget!(Calendar);

impl Calendar {
    pub fn new() -> Option<Calendar> {
        assert_initialized_main_thread!();
        let tmp_pointer = unsafe { ffi::gtk_calendar_new() };
        check_pointer!(tmp_pointer, Calendar)
    }

    pub fn select_month(&self, month: u32, year: u32) -> () {
        unsafe {
            ffi::gtk_calendar_select_month(GTK_CALENDAR(self.pointer), month as c_uint, year as c_uint)
        }
    }

    pub fn select_day(&self, day: u32) -> () {
        unsafe {
            ffi::gtk_calendar_select_day(GTK_CALENDAR(self.pointer), day as c_uint)
        }
    }

    pub fn mark_day(&self, day: u32) -> () {
        unsafe {
            ffi::gtk_calendar_mark_day(GTK_CALENDAR(self.pointer), day as c_uint)
        }
    }

    pub fn unmark_day(&self, day: u32) -> () {
        unsafe {
            ffi::gtk_calendar_unmark_day(GTK_CALENDAR(self.pointer), day as c_uint)
        }
    }

    pub fn get_day_is_marked(&self, day: u32) -> bool {
        unsafe { to_bool(ffi::gtk_calendar_get_day_is_marked(GTK_CALENDAR(self.pointer), day as c_uint)) }
    }

    pub fn clear_marks(&self) -> () {
        unsafe {
            ffi::gtk_calendar_clear_marks(GTK_CALENDAR(self.pointer));
        }
    }

    pub fn get_display_options(&self) -> CalendarDisplayOptions {
        unsafe {
            ffi::gtk_calendar_get_display_options(GTK_CALENDAR(self.pointer))
        }
    }

    pub fn set_display_options(&self, flags: CalendarDisplayOptions) -> () {
        unsafe {
            ffi::gtk_calendar_set_display_options(GTK_CALENDAR(self.pointer), flags)
        }
    }

    pub fn get_date(&self) -> (u32, u32, u32) {
        let mut year = 0;
        let mut month = 0;
        let mut day = 0;
        unsafe {
            ffi::gtk_calendar_get_date(GTK_CALENDAR(self.pointer), &mut year, &mut month, &mut day);
        }
        (year, month, day)
    }

    pub fn get_detail_with_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_width_chars(GTK_CALENDAR(self.pointer))
        }
    }

    pub fn set_detail_with_chars(&self, chars: i32) -> () {
        unsafe {
            ffi::gtk_calendar_set_detail_width_chars(GTK_CALENDAR(self.pointer), chars as c_int)
        }
    }

    pub fn get_detail_height_rows(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_height_rows(GTK_CALENDAR(self.pointer)) as i32
        }
    }

    pub fn set_detail_heigth_rows(&self, rows: i32) -> () {
        unsafe {
            ffi::gtk_calendar_set_detail_height_rows(GTK_CALENDAR(self.pointer), rows as c_int)
        }
    }

}

impl_drop!(Calendar);
impl_TraitWidget!(Calendar);
