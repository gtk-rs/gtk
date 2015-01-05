// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version _3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! Enumeration used with pango types

/// The PangoGravity type represents the orientation of glyphs in a segment of text. This is useful when rendering vertical text
/// layouts. In those situations, the layout is rotated using a non-identity PangoMatrix, and then glyph orientation is controlled
/// using PangoGravity. Not every value in this enumeration makes sense for every usage of PangoGravity; for example, PANGO_GRAVITY_AUTO
/// only can be passed to pango_context_set_base_gravity() and can only be returned by pango_context_get_base_gravity().
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum Gravity {
    /// Glyphs stand upright (default)
    South,
    /// Glyphs are rotated 90 degrees clockwise
    East,
    /// Glyphs are upside-down
    North,
    /// Glyphs are rotated 90 degrees counter-clockwise
    West,
    /// Gravity is resolved from the context matrix
    Auto
}

/// The PangoGravityHint defines how horizontal scripts should behave in a vertical context. That is, English excerpt in a
/// vertical paragraph for example.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum GravityHint {
    /// scripts will take their natural gravity based on the base gravity and the script. This is the default.
    Natural,
    /// always use the base gravity set, regardless of the script.
    Strong,
    /// for scripts not in their natural direction (eg. Latin in East gravity), choose per-script gravity such that every script
    /// respects the line progression. This means, Latin and Arabic will take opposite gravities and both flow top-to-bottom for example.
    Line
}

/// The PangoScript enumeration identifies different writing systems. The values correspond to the names as defined in the Unicode standard. Note
/// that new types may be added in the future.
/// Applications should be ready to handle unknown values. This enumeration is interchangeable with GUnicodeScript. See Unicode Standard Annex
/// 24: Script names.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum Script {
    /// a value never returned from pango_script_for_unichar()
    InvalidCode,
    /// a character used by multiple different scripts
    Common,
    /// a mark glyph that takes its script from the base glyph to which it is attached
    Inherited,
    /// Arabic
    Arabic,
    /// Armenian,
    Armenian,
    /// Bengali,
    Bengali,
    /// Bopomofo
    Bopomofo,
    /// Cherokee,
    Cherokee,
    /// Coptic,
    Coptic,
    /// Cyrillic
    Cyrillic,
    /// Deseret,
    Deseret,
    /// Devanagari
    Devanagari,
    /// Ethiopic
    Ethiopic,
    /// Georgian
    Georgian,
    /// Gothic
    Gothic,
    /// Greek
    Greek,
    /// Gujarati
    Gujarati,
    /// Gurmukhi
    Gurmukhi,
    /// Han
    Han,
    /// Hangul
    Hangul,
    /// Hebrew
    Hebrew,
    /// Hiragana
    Hiragana,
    /// Kannada
    Kannada,
    /// Katakana
    Katakana,
    /// Khmer
    Khmer,
    /// Lao
    Lao,
    /// Latin
    Latin,
    /// Malayalam
    Malayalam,
    /// Mongolian
    Mongolian,
    /// Myanmar
    Myanmar,
    /// Ogham
    Ogham,
    /// Old Italic
    OldItalic,
    /// Oriya
    Oriya,
    /// Runic
    Runic,
    /// Sinhala
    Sinhala,
    /// Syriac
    Syriac,
    /// Tamil
    Tamil,
    /// Telugu
    Telugu,
    /// Thaana
    Thaana,
    /// Thai
    Thai,
    /// Tibetan
    Tibetan,
    /// Canadian Aboriginal
    CanadianAboriginal,
    /// Yi
    Yi,
    /// Tagalog
    Tagalog,
    /// Hanunoo
    Hanunoo,
    /// Buhid
    Buhid,
    /// Tagbanwa
    Tagbanwa,
    /// Braille
    Braille,
    /// Cypriot
    Cypriot,
    /// Limbu
    Limbu,
    /// Osmanya
    Osmanya,
    /// Shavian
    Shavian,
    /// Linear B
    LinearB,
    /// Tai Le
    TaiLe,
    /// Ugaritic
    Ugaritic,
    /// New Tai Lue. Since 1.10
    NewTaiLue,
    /// Buginese. Since 1.10
    Buginese,
    /// Glagolitic. Since 1.10
    Glagolitic,
    /// Tifinagh. Since 1.10
    Tifinagh,
    /// Syloti Nagri. Since 1.10
    SylotiNagri,
    /// Old Persian. Since 1.10
    OldPersian,
    /// Kharoshthi. Since 1.10
    Kharoshthi,
    /// an unassigned code point. Since 1.14
    Unknown,
    /// Balinese. Since 1.14
    Balinese,
    /// Cuneiform. Since 1.14
    Cuneiform,
    /// Phoenician. Since 1.14
    Phoenician,
    /// Phags-pa. Since 1.14
    PhagsPa,
    /// N'Ko. Since 1.14
    NKo,
    /// Kayah Li. Since 1.20.1
    KayahLi,
    /// Lepcha. Since 1.20.1
    Lepcha,
    /// Rejang. Since 1.20.1
    Rejang,
    /// Sundanese. Since 1.20.1
    Sundanese,
    /// Saurashtra. Since 1.20.1
    Saurashtra,
    /// Cham. Since 1.20.1
    Cham,
    /// Ol Chiki. Since 1.20.1
    OlChiki,
    /// Vai. Since 1.20.1
    Vai,
    /// Carian. Since 1.20.1
    Carian,
    /// Lycian. Since 1.20.1
    Lycian,
    /// Lydian. Since 1.20.1
    Lydian,
    /// Batak. Since 1.32
    Batak,
    /// Brahmi. Since 1.32
    Brahmi,
    /// Mandaic. Since 1.32
    Mandaic,
    /// Chakma. Since: 1.32
    Chakma,
    /// Meroitic Cursive. Since: 1.32
    MeroiticCursive,
    /// Meroitic Hieroglyphs. Since: 1.32
    MeroiticHieroglyphs,
    /// Miao. Since: 1.32
    Miao,
    /// Sharada. Since: 1.32
    Sharada,
    /// Sora Sompeng. Since: 1.32
    SoraSompeng,
    /// Takri. Since: 1.32
    Takri
}

/// The PangoDirection type represents a direction in the Unicode bidirectional algorithm; not every value in this enumeration makes sense for
/// every usage of PangoDirection; for example, the return value of pango_unichar_direction() and pango_find_base_dir() cannot be PANGO_DIRECTION_WEAK_LTR
/// or PANGO_DIRECTION_WEAK_RTL, since every character is either neutral or has a strong direction; on the other hand PANGO_DIRECTION_NEUTRAL doesn't
/// make sense to pass to pango_itemize_with_base_dir().
/// 
/// The PANGO_DIRECTION_TTB_LTR, PANGO_DIRECTION_TTB_RTL values come from an earlier interpretation of this enumeration as the writing direction of
/// a block of text and are no longer used; See PangoGravity for how vertical text is handled in Pango.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum Direction {
    /// A strong left-to-right direction
    StrongLeftToRight,
    /// A strong right-to-left direction
    StrognRightToLeft,
    /// Deprecated value; treated the same as PANGO_DIRECTION_RTL.
    TTB_LeftToRight,
    /// Deprecated value; treated the same as PANGO_DIRECTION_LTR
    TTB_RightToLeft,
    /// A weak left-to-right direction
    WeakLeftToRight,
    /// A weak right-to-left direction
    WeakRightToLeft,
    /// No direction specified
    Neutral
}

/// The PangoBidiType type represents the bidirectional character type of a Unicode character as specified by the
/// [Unicode bidirectional algorithm](http://www.unicode.org/reports/tr9/).
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum BidiType {
    /// Left-to-Right
    LeftToRight,
    /// Left-to-Right Embedding
    LeftToRightEmbedding,
    /// Left-to-Right Override
    LeftToRightOverride,
    /// Right-to-Left
    RightToLeft,
    /// Right-to-Left Arabic
    RightToLeftArabic,
    /// Right-to-Left Embedding
    RightToLeftEmbedding,
    /// Right-to-Left Override
    RightToLeftOverride,
    /// Pop Directional Format
    PopDirectionalFormat,
    /// European Number
    EuropeanNumber,
    /// European Number Separator
    EuropeanNumberSeparator,
    /// European Number Terminator
    EuropeanNumberTerminator,
    /// Arabic Number
    ArabicNumber,
    /// Common Number Separator
    CommonNumberSeparator,
    /// Nonspacing Mark
    NonspacingMark,
    /// Boundary Neutral
    BoundaryNeutral,
    /// Paragraph Separator
    ParagraphSeparator,
    /// Segment Separator
    SegmentSeparator,
    /// Whitespace
    Whitespace,
    /// Other Neutrals
    OtherNeutrals
}

/// An enumeration specifying the various slant styles possible for a font.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum Style {
    /// the font is upright.
    Normal,
    /// the font is slanted, but in a roman style.
    Oblique,
    /// the font is slanted in an italic style.
    Italic
}

/// An enumeration specifying the weight (boldness) of a font. This is a numerical value ranging from 100 to 1000, but there are some
/// predefined values:
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum Weight {
    /// the thin weight (= 100; Since: 1.24)
    Thin,
    /// the ultralight weight (= 200)
    UltraLight,
    /// the light weight (= 300)
    Light,
    /// the semilight weight (= 350; Since: 1.36.7)
    Semilight,
    /// the book weight (= 380; Since: 1.24)
    Book,
    /// the default weight (= 400)
    Normal,
    /// the normal weight (= 500; Since: 1.24)
    Medium,
    /// the semibold weight (= 600)
    Semibold,
    /// the bold weight (= 700)
    Bold,
    /// the ultrabold weight (= 800)
    Ultrabold,
    /// the heavy weight (= 900)
    Heavy,
    /// the ultraheavy weight (= 1000; Since: 1.24)
    Ultraheavy
}

/// An enumeration specifying capitalization variant of the font.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum Variant {
    /// A normal font.
    Normal,
    /// A font with the lower case characters replaced by smaller variants of the capital characters.
    SmallCaps
}

/// An enumeration specifying the width of the font relative to other designs within a family.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum Stretch {
    /// ultra condensed width
    Condensed,
    /// extra condensed width
    ExtraCondensed,
    /// condensed width
    StretchCondensed,
    /// semi condensed width
    SemiCondensed,
    /// the normal width
    Normal,
    /// semi expanded width
    SemiExpanded,
    /// expanded width
    StretchExpanded,
    /// extra expanded width
    ExtraExpanded,
    /// ultra expanded width
    UltraExpanded
}

/// The bits in a PangoFontMask correspond to fields in a PangoFontDescription that have been set.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Show, Copy)]
pub enum FontMask {
    /// the font family is specified.
    Family,
    /// the font style is specified.
    Style,
    /// the font variant is specified.
    Variant,
    /// the font weight is specified.
    Weight,
    /// the font stretch is specified.
    Stretch,
    /// the font size is specified.
    Size,
    /// the font gravity is specified (Since: 1.16.)
    Gravity
}