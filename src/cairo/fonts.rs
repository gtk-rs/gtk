use libc::{c_ulong, c_int, c_double};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::ops::Drop;
use std::c_str::CString;
use std::mem::transmute;

use cairo;
use cairo::enums::{
    Antialias,
    SubpixelOrder,
    HintStyle,
    HintMetrics,

    FontType,
    FontWeight,
    FontSlant,
    TextClusterFlags,
};
use cairo::matrices::Matrix;
use cairo::ffi;
use cairo::ffi::{
    cairo_font_options_t,
    cairo_font_face_t,
    cairo_scaled_font_t
};


pub struct TextCluster {
    num_bytes: c_int,
    num_glyphs: c_int
}

pub struct Glyph{
    index: c_ulong,
    x: c_double,
    y: c_double
}

/* TODO
 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.


impl Glyph{

    //pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *Glyph;

    //pub fn cairo_glyph_free(glyphs: *Glyph);
}

 Allocates an array of cairo_glyph_t's. This function is only useful in
 implementations of cairo_user_scaled_font_text_to_glyphs_func_t where the user
 needs to allocate an array of glyphs that cairo will free. For all other uses,
 user can use their own allocation method for glyphs.

impl TextCluster{
    //pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *TextCluster;

    //pub fn cairo_text_cluster_free(clusters: *TextCluster);
}
*/

pub struct FontExtents{
    pub ascent: c_double,
    pub descent: c_double,
    pub height: c_double,
    pub max_x_advance: c_double,
    pub max_y_advance: c_double,
}

pub struct TextExtents {
    pub x_bearing: c_double,
    pub y_bearing: c_double,
    pub width: c_double,
    pub height: c_double,
    pub x_advance: c_double,
    pub y_advance: c_double,
}


pub struct FontOptions(*mut cairo_font_options_t);

impl FontOptions{
    pub fn new() -> FontOptions{
        let font_options = unsafe{
            FontOptions(ffi::cairo_font_options_create())
        };
        font_options.ensure_status();
        font_options
    }

    pub fn get_ptr(&self) -> *mut cairo_font_options_t{
        let FontOptions(ptr) = *self;
        ptr
    }

    pub fn ensure_status(&self){
        let status = unsafe{
            ffi::cairo_font_options_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn merge(&mut self, other: &mut FontOptions){
        unsafe{
            ffi::cairo_font_options_merge(self.get_ptr(), other.get_ptr())
        }
    }

    pub fn hash(&self) -> u64{
        unsafe{
            ffi::cairo_font_options_hash(self.get_ptr())
        }
    }

    pub fn set_antialias(&self, antialias: Antialias){
        unsafe{
            ffi::cairo_font_options_set_antialias(self.get_ptr(), antialias)
        }
    }

    pub fn get_antialias(&self) -> Antialias{
        unsafe{
            ffi::cairo_font_options_get_antialias(self.get_ptr())
        }
    }

    pub fn set_subpixel_order(&self, order: SubpixelOrder){
        unsafe{
            ffi::cairo_font_options_set_subpixel_order(self.get_ptr(), order)
        }
    }

    pub fn get_subpixel_order(&self) -> SubpixelOrder{
        unsafe{
            ffi::cairo_font_options_get_subpixel_order(self.get_ptr())
        }
    }

    pub fn set_hint_style(&self, hint_style: HintStyle){
        unsafe{
            ffi::cairo_font_options_set_hint_style(self.get_ptr(), hint_style)
        }
    }

    pub fn get_hint_style(&self) -> HintStyle{
        unsafe{
            ffi::cairo_font_options_get_hint_style(self.get_ptr())
        }
    }

    pub fn set_hint_metrics(&self, hint_metrics: HintMetrics){
        unsafe{
            ffi::cairo_font_options_set_hint_metrics(self.get_ptr(), hint_metrics)
        }
    }

    pub fn get_hint_metrics(&self) -> HintMetrics{
        unsafe{
            ffi::cairo_font_options_get_hint_metrics(self.get_ptr())
        }
    }
}

impl PartialEq for FontOptions{
    fn eq(&self, other: &FontOptions) -> bool{
        unsafe{
            ffi::cairo_font_options_equal(self.get_ptr(), other.get_ptr()).as_bool()
        }
    }
}

impl Clone for FontOptions{
    fn clone(&self) -> FontOptions{
        unsafe{
            FontOptions(ffi::cairo_font_options_copy(self.get_ptr()))
        }
    }
}

impl Drop for FontOptions{
    fn drop(&mut self){
        unsafe{
            ffi::cairo_font_options_destroy(self.get_ptr())
        }
    }
}




pub struct FontFace(*mut cairo_font_face_t);

impl FontFace{
    pub fn get_ptr(&self) -> *mut cairo_font_face_t{
        let FontFace(ptr) = *self;
        ptr
    }

    pub fn toy_create<S: ToCStr>(family: S, slant: FontSlant, weight: FontWeight) -> FontFace{
        let font_face = FontFace(unsafe{
            family.with_c_str(|family|{
                ffi::cairo_toy_font_face_create(family, slant, weight)
            })
        });
        font_face.ensure_status();
        font_face
    }

    pub fn toy_get_family(&self) -> String{
        unsafe{
            let ptr = ffi::cairo_toy_font_face_get_family(self.get_ptr());
            let c_str = CString::new(ptr, false);
            c_str.as_str().unwrap().to_string()
        }
    }

    pub fn toy_get_slant(&self) -> FontSlant{
        unsafe{
            ffi::cairo_toy_font_face_get_slant(self.get_ptr())
        }
    }

    pub fn toy_get_weight(&self) -> FontWeight{
        unsafe{
            ffi::cairo_toy_font_face_get_weight(self.get_ptr())
        }
    }

    pub fn ensure_status(&self){
        let status = unsafe{
            ffi::cairo_font_face_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType{
        unsafe{
            ffi::cairo_font_face_get_type(self.get_ptr())
        }
    }

    pub fn get_reference_count(&self) -> uint{
        unsafe{
            ffi::cairo_font_face_get_reference_count(self.get_ptr()) as uint
        }
    }
}

impl Clone for FontFace{
    fn clone(&self) -> FontFace{
        unsafe{
            FontFace(ffi::cairo_font_face_reference(self.get_ptr()))
        }
    }
}

impl Drop for FontFace{
    fn drop(&mut self){
        unsafe{
            ffi::cairo_font_face_destroy(self.get_ptr())
        }
    }
}



pub struct ScaledFont(*mut cairo_scaled_font_t);

impl ScaledFont{
    pub fn get_ptr(&self) -> *mut cairo_scaled_font_t{
        let ScaledFont(ptr) = *self;
        ptr
    }

    pub fn new(font_face: FontFace, font_matrix: &mut Matrix, ctm: &mut Matrix, options: FontOptions) -> ScaledFont{
        let scaled_font = unsafe{
            ScaledFont(ffi::cairo_scaled_font_create(font_face.get_ptr(), font_matrix, ctm, options.get_ptr()))
        };
        scaled_font.ensure_status();
        scaled_font
    }

    pub fn ensure_status(&self){
        let status = unsafe{
            ffi::cairo_scaled_font_status(self.get_ptr())
        };
        status.ensure_valid()
    }

    pub fn get_type(&self) -> FontType{
        unsafe{
            ffi::cairo_scaled_font_get_type(self.get_ptr())
        }
    }

    pub fn get_reference_count(&self) -> uint{
        unsafe{
            ffi::cairo_scaled_font_get_reference_count(self.get_ptr()) as uint
        }
    }

    //pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut cairo_font_extents_t);

    //                    cairo_text_extents_t;
    //pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *mut char, extents: *mut cairo_text_extents_t);

    //pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *mut Glyph, num_glyphs: c_int, extents: *mut cairo_text_extents_t);

    //pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: c_double, y: c_double, utf8: *mut char, utf8_len: c_int, glyphs: **mut Glyph, num_glyphs: *mut c_int, clusters: **mut TextCluster, num_clusters: *mut c_int, cluster_flags: *mut TextClusterFlags) -> Status;

    //pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;

    //pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t);

    //pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut cairo_matrix_t);

    //pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut cairo_matrix_t);

    //pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *cairo_matrix_t);

}

impl Clone for ScaledFont{
    fn clone(&self) -> ScaledFont{
        unsafe{
            ScaledFont(ffi::cairo_scaled_font_reference(self.get_ptr()))
        }
    }
}

impl Drop for ScaledFont{
    fn drop(&mut self){
        unsafe{
            ffi::cairo_scaled_font_destroy(self.get_ptr())
        }
    }
}



impl cairo::Context{
    pub fn select_font_face<S: ToCStr>(&self, family: S, slant: FontSlant, weight: FontWeight){
        unsafe{
            family.with_c_str(|family|{
                ffi::cairo_select_font_face(self.get_ptr(), family, slant, weight)
            })
        }
    }

    pub fn set_font_size(&self, size: f64){
        unsafe{
            ffi::cairo_set_font_size(self.get_ptr(), size)
        }
    }

    //FIXME probably needs a heap allocation
    pub fn set_font_matrix(&self, matrix: Matrix){
        unsafe{
            ffi::cairo_set_font_matrix(self.get_ptr(), &matrix)
        }
    }

    pub fn get_font_matrix(&self) -> Matrix{
        let mut matrix = Matrix::null();
        unsafe{
            ffi::cairo_get_font_matrix(self.get_ptr(), &mut matrix);
        }
        matrix
    }

    pub fn set_font_options(&self, options: FontOptions){
        unsafe{
            ffi::cairo_set_font_options(self.get_ptr(), options.get_ptr())
        }
    }

    pub fn get_font_options(&self) -> FontOptions{
        let out = FontOptions::new();
        unsafe{
            ffi::cairo_get_font_options(self.get_ptr(), out.get_ptr());
        }
        out
    }

    pub fn set_font_face(&self, font_face: FontFace){
        unsafe{
            ffi::cairo_set_font_face(self.get_ptr(), font_face.get_ptr())
        }
    }

    pub fn get_font_face(&self) -> FontFace{
        unsafe{
            FontFace(ffi::cairo_get_font_face(self.get_ptr()))
        }
    }

    pub fn set_scaled_font(&self, scaled_font: ScaledFont){
        unsafe{
            ffi::cairo_set_scaled_font(self.get_ptr(), scaled_font.get_ptr())
        }
    }

    pub fn get_scaled_font(&self) -> ScaledFont{
        unsafe{
            ScaledFont(ffi::cairo_get_scaled_font(self.get_ptr()))
        }
    }

    pub fn show_text<S: ToCStr>(&self, text: S){
        unsafe{
            text.with_c_str(|text|{
                ffi::cairo_show_text(self.get_ptr(), text)
            })
        }
    }

    pub fn show_glyphs<V: Vector<Glyph>>(&self, vec: V){
        unsafe{
            let slice: &[Glyph] = vec.as_slice();
            ffi::cairo_show_glyphs(self.get_ptr(), slice.as_ptr(), slice.len() as c_int)
        }
    }

    pub fn show_text_glyphs<S: ToCStr,
                            V: Vector<Glyph>,
                            W: Vector<TextCluster>>(&self,
                                       text: S,
                                       glyph_vec: V,
                                       cluster_vec: W,
                                       cluster_flags: TextClusterFlags){
        unsafe{
            let glyphs: &[Glyph] = glyph_vec.as_slice();
            let clusters: &[TextCluster] = cluster_vec.as_slice();

            text.with_c_str(|text| {
                ffi::cairo_show_text_glyphs(self.get_ptr(),
                                       text,
                                       -1 as c_int, //NUL terminated
                                       glyphs.as_ptr(),
                                       glyphs.len() as c_int,
                                       clusters.as_ptr(),
                                       clusters.len() as c_int,
                                       cluster_flags)
            })
        }
    }

    pub fn font_extents(&self) -> FontExtents{
        let mut extents = FontExtents{
            ascent: 0.0,
            descent: 0.0,
            height: 0.0,
            max_x_advance: 0.0,
            max_y_advance: 0.0,
        };

        unsafe{
            ffi::cairo_font_extents(self.get_ptr(), &mut extents);
        }

        extents
    }

    pub fn text_extents<S: ToCStr>(&self, text: S) -> TextExtents{
        let mut extents = TextExtents{
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        text.with_c_str(|text|{
            unsafe{
                ffi::cairo_text_extents(self.get_ptr(), text, &mut extents)
            }
        });

        extents
    }

    pub fn glyph_extents<G: Vector<Glyph>>(&self, glyph_vec: G) -> TextExtents{
        let mut extents = TextExtents{
            x_bearing: 0.0,
            y_bearing: 0.0,
            width: 0.0,
            height: 0.0,
            x_advance: 0.0,
            y_advance: 0.0,
        };

        let glyphs = glyph_vec.as_slice();

        unsafe{
            ffi::cairo_glyph_extents(self.get_ptr(), glyphs.as_ptr(), glyphs.len() as c_int, &mut extents);
        }

        extents
    }
}