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

//! Displays a calendar and allows the user to select a date

use libc::{c_uint, c_int};

use gtk::enums::CalendarDisplayOptions;
use utils::cast::GTK_CALENDAR;
use ffi;
use gtk::traits;
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
struct_Widget!(Calendar)

impl Calendar {
    pub fn new() -> Option<Calendar> {
        let tmp_pointer = unsafe { ffi::gtk_calendar_new() };
        check_pointer!(tmp_pointer, Calendar)
    }

    pub fn select_month(&mut self, month: u32, year: u32) -> () {
        unsafe {
            ffi::gtk_calendar_select_month(GTK_CALENDAR(self.pointer), month as c_uint, year as c_uint)
        }
    }

    pub fn select_day(&mut self, day: u32) -> () {
        unsafe {
            ffi::gtk_calendar_select_day(GTK_CALENDAR(self.pointer), day as c_uint)
        }
    }

    pub fn mark_day(&mut self, day: u32) -> () {
        unsafe {
            ffi::gtk_calendar_mark_day(GTK_CALENDAR(self.pointer), day as c_uint)
        }
    }

    pub fn unmark_day(&mut self, day: u32) -> () {
        unsafe {
            ffi::gtk_calendar_unmark_day(GTK_CALENDAR(self.pointer), day as c_uint)
        }
    }

    pub fn get_day_is_marked(&self, day: u32) -> bool {
        match unsafe { ffi::gtk_calendar_get_day_is_marked(GTK_CALENDAR(self.pointer), day as c_uint) } {
            ffi::Gfalse => false,
            _           => true
        }
    }

    pub fn clear_marks(&mut self) -> () {
        unsafe {
            ffi::gtk_calendar_clear_marks(GTK_CALENDAR(self.pointer));
        }
    }

    pub fn get_display_options(&self) -> CalendarDisplayOptions {
        unsafe {
            ffi::gtk_calendar_get_display_options(GTK_CALENDAR(self.pointer))
        }
    }

    pub fn set_display_options(&mut self, flags: CalendarDisplayOptions) -> () {
        unsafe {
            ffi::gtk_calendar_set_display_options(GTK_CALENDAR(self.pointer), flags)
        }
    }

    pub fn get_date(&self) -> (u32, u32, u32) {
        let year = 0;
        let month = 0;
        let day = 0;
        unsafe {
            ffi::gtk_calendar_get_date(GTK_CALENDAR(self.pointer), &year, &month, &day);
        }
        (year, month, day)
    }

    pub fn get_detail_with_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_width_chars(GTK_CALENDAR(self.pointer))
        }
    }

    pub fn set_detail_with_chars(&mut self, chars: i32) -> () {
        unsafe {
            ffi::gtk_calendar_set_detail_width_chars(GTK_CALENDAR(self.pointer), chars as c_int)
        }
    }

    pub fn get_detail_height_rows(&self) -> i32 {
        unsafe {
            ffi::gtk_calendar_get_detail_height_rows(GTK_CALENDAR(self.pointer)) as i32
        }
    }

    pub fn set_detail_heigth_rows(&mut self, rows: i32) -> () {
        unsafe {
            ffi::gtk_calendar_set_detail_height_rows(GTK_CALENDAR(self.pointer), rows as c_int)
        }
    }

}

impl_GtkWidget!(Calendar)





