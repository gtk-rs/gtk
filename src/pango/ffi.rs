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

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::{c_int, c_uint, c_char, c_double};
use gtk::ffi::{Gboolean};
use pango;

#[repr(C)]
#[derive(Copy)]
pub struct C_PangoContext;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoAttrList;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoAttrIterator;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoItem;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontMap;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontDescription;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoLanguage;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoMatrix;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFont;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontset;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontMetrics;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoFontFamily;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoAnalysis;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoLogAttr;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoGlyphString;
#[repr(C)]
#[derive(Copy)]
pub struct C_PangoScript;

extern "C" {
    //=========================================================================
    // PangoItem                                                         NOT OK
    //=========================================================================
    pub fn pango_item_free                (item: *mut C_PangoItem);
    pub fn pango_item_copy                (item: *mut C_PangoItem) -> *mut C_PangoItem;
    pub fn pango_item_new                 () -> *mut C_PangoItem;
    pub fn pango_item_split               (item: *mut C_PangoItem, split_index: c_int, split_offset: c_int) -> *mut C_PangoItem;

    //=========================================================================
    // PangoContext                                                      NOT OK
    //=========================================================================
    pub fn pango_context_new              () -> *mut C_PangoContext;
    pub fn pango_context_changed          (context: *mut C_PangoContext);
    pub fn pango_context_get_serial       (context: *mut C_PangoContext) -> c_uint;
    pub fn pango_context_set_font_map     (context: *mut C_PangoContext, font_map: *mut C_PangoFontMap);
    pub fn pango_context_get_font_map     (context: *mut C_PangoContext) -> *mut C_PangoFontMap;
    pub fn pango_context_get_font_description(context: *mut C_PangoContext) -> *mut C_PangoFontDescription;
    pub fn pango_context_set_font_description(context: *mut C_PangoContext, desc: *const C_PangoFontDescription);
    pub fn pango_context_get_language     (context: *mut C_PangoContext) -> *mut C_PangoLanguage;
    pub fn pango_context_set_language     (context: *mut C_PangoContext, language: *mut C_PangoLanguage);
    pub fn pango_context_get_base_dir     (context: *mut C_PangoContext) -> pango::Direction;
    pub fn pango_context_set_base_dir     (context: *mut C_PangoContext, direction: pango::Direction);
    pub fn pango_context_get_base_gravity (context: *mut C_PangoContext) -> pango::Gravity;
    pub fn pango_context_set_base_gravity (context: *mut C_PangoContext, gravity: pango::Gravity);
    pub fn pango_context_get_gravity      (context: *mut C_PangoContext) -> pango::Gravity;
    pub fn pango_context_get_gravity_hint (context: *mut C_PangoContext) -> pango::GravityHint;
    pub fn pango_context_set_gravity_hint (context: *mut C_PangoContext, hint: pango::GravityHint);
    pub fn pango_context_get_matrix       (context: *mut C_PangoContext) -> *const C_PangoMatrix;
    pub fn pango_context_set_matrix       (context: *mut C_PangoContext, matrix: *const C_PangoMatrix);
    pub fn pango_context_load_font        (context: *mut C_PangoContext, desc: *const C_PangoFontDescription) -> *mut C_PangoFont;
    pub fn pango_context_load_fontset     (context: *mut C_PangoContext, desc: *const C_PangoFontDescription, language: *mut C_PangoLanguage) -> *mut C_PangoFontset;
    pub fn pango_context_get_metrics      (context: *mut C_PangoContext, desc: *const C_PangoFontDescription, language: *mut C_PangoLanguage) -> *mut C_PangoFontMetrics;
    pub fn pango_context_list_families    (context: *mut C_PangoContext, families: *mut *mut *mut C_PangoFontFamily, n_families: *mut c_int);

    //=========================================================================
    // PangoMatrix                                                       NOT OK
    //=========================================================================
    pub fn pango_gravity_get_for_matrix   (matrix: *const C_PangoMatrix) -> pango::Gravity;

    //=========================================================================
    // PangoScript                                                       NOT OK
    //=========================================================================
    pub fn pango_gravity_get_for_script     (script: pango::Script, base_gravity: pango::Gravity, hint: pango::GravityHint) -> pango::Gravity;
    pub fn pango_gravity_get_for_script_and_width(script: pango::Script, wide: Gboolean, base_gravity: pango::Gravity,
        hint: pango::GravityHint) -> pango::Gravity;

    //=========================================================================
    // PangoGravity                                                      NOT OK
    //=========================================================================
    pub fn pango_gravity_to_rotation        (gravity: pango::Gravity) -> c_double;

    //=========================================================================
    // PangoDirection                                                    NOT OK
    //=========================================================================
    pub fn pango_unichar_direction          (ch: u32) -> pango::Direction;
    pub fn pango_find_base_dir              (text: *const c_char, length: c_int) -> pango::Direction;

    //=========================================================================
    // PangoBidiType                                                     NOT OK
    //=========================================================================
    pub fn pango_bidi_type_for_unichar      (ch: u32) -> pango::BidiType;

    //pub fn pango_itemize                  (context: *mut C_PangoContext, text: *const c_char, start_index: c_int, length: c_int,
    //    attrs: *mut C_PangoAttrList, cached_iter: *mut PangoAttrIterator) -> *mut GList;
    //pub fn pango_itemize_with_base_dir    (context: *mut C_PangoContext, direction: pango::Direction, text: *const c_char, start_index: c_int, length: c_int,
    //    attrs: *mut C_PangoAttrList, cached_iter: *mut PangoAttrIterator) -> *mut GList;
    //pub fn pango_reorder_items            (logical_items: *mut GList) -> *mut GList;
    pub fn pango_break                    (text: *const c_char, length: c_int, analysis: *mut C_PangoAnalysis, attrs: *mut C_PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_get_log_attrs            (text: *const c_char, length: c_int, level: c_int, language: *mut C_PangoLanguage, log_attrs: *mut C_PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_find_paragraph_boundary  (text: *const c_char, length: c_int, paragraph_delimiter_index: *mut c_int,
        next_paragraph_start: *mut c_int);
    pub fn pango_default_break            (text: *const c_char, length: c_int, analysis: *mut C_PangoAnalysis, attrs: *mut C_PangoLogAttr,
        attrs_len: c_int);
    pub fn pango_shape                    (text: *const c_char, length: c_int, analysis: *const C_PangoAnalysis, glyphs: *mut C_PangoGlyphString);
    pub fn pango_shape_full               (item_text: *const c_char, item_length: c_int, paragraph_text: *const c_char, paragraph_length: c_int,
        analysis: *const C_PangoAnalysis, glyphs: *mut C_PangoGlyphString);
}