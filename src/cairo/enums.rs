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

use std::fmt::{Error, Debug};
use cairo::ffi;

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Copy)]
pub enum Status {
    StatusSuccess = 0,

    StatusNoMemory,
    StatusInvalidRestore,
    StatusInvalidPopGroup,
    StatusNoCurrentPoint,
    StatusInvalidMatrix,
    StatusInvalidStatus,
    StatusNullPointer,
    StatusInvalidString,
    StatusInvalidPathData,
    StatusReadError,
    StatusWriteError,
    StatusSurfaceFinished,
    StatusSurfaceTypeMismatch,
    StatusPatternTypeMismatch,
    StatusInvalidContent,
    StatusInvalidFormat,
    StatusInvalidVisual,
    StatusFileNotFound,
    StatusInvalidDash,
    StatusInvalidDscComment,
    StatusInvalidIndex,
    StatusClipNotRepresentable,
    StatusTempFileError,
    StatusInvalidStride,
    StatusFontTypeMismatch,
    StatusUserFontImmutable,
    StatusUserFontError,
    StatusNegativeCount,
    StatusInvalidClusters,
    StatusInvalidSlant,
    StatusInvalidWeight,
    StatusInvalidSize,
    StatusUserFontNotImplemented,
    StatusDeviceTypeMismatch,
    StatusDeviceError,
    StatusInvalidMeshConstruction,
    StatusDeviceFinished,

    StatusLastStatus
}

impl Debug for Status {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), Error> {
        unsafe {
            let char_ptr = ffi::cairo_status_to_string(*self);
            let tmp = String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&char_ptr)).to_string();

            tmp.fmt(formatter)
        }
    }
}

impl Status {
    pub fn ensure_valid(&self) {
        if *self != Status::StatusSuccess {
            panic!("Cairo error {:?}", *self)
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Antialias {
    AntialiasDefault,

    /* method */
    AntialiasNone,
    AntialiasGray,
    AntialiasSubpixel,

    /* hints */
    AntialiasFast,
    AntialiasGood,
    AntialiasBest
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FillRule {
    FillRuleWinding,
    FillRuleEvenOdd
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineCap {
    LineCapButt,
    LineCapRound,
    LineCapSquare
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum LineJoin {
    LineJoinMiter,
    LineJoinRound,
    LineJoinBevel
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Operator {
    OperatorClear,

    OperatorSource,
    OperatorOver,
    OperatorIn,
    OperatorOut,
    OperatorAtop,

    OperatorDest,
    OperatorDestOver,
    OperatorDestIn,
    OperatorDestOut,
    OperatorDestAtop,

    OperatorXor,
    OperatorAdd,
    OperatorSaturate,

    OperatorMultiply,
    OperatorScreen,
    OperatorOverlay,
    OperatorDarken,
    OperatorLighten,
    OperatorColorDodge,
    OperatorColorBurn,
    OperatorHardLight,
    OperatorSoftLight,
    OperatorDifference,
    OperatorExclusion,
    OperatorHslHue,
    OperatorHslSaturation,
    OperatorHslColor,
    OperatorHslLuminosity
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PathDataType {
    PathMoveTo,
    PathLineTo,
    PathCurveTo,
    PathClosePath
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Content {
    ContentColor      = 0x1000,
    ContentAlpha      = 0x2000,
    ContentColorAlpha = 0x3000
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Extend {
    ExtendNone,
    ExtendRepeat,
    ExtendReflect,
    ExtendPad
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Filter {
    FilterFast,
    FilterGood,
    FilterBest,
    FilterNearest,
    FilterBilinear,
    FilterGaussian
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum PatternType {
    PatternTypeSolid,
    PatternTypeSurface,
    PatternTypeLinearGradient,
    PatternTypeRadialGradient,
    PatternTypeMesh,
    PatternTypeRasterSource
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    FontSlantNormal,
    FontSlantItalic,
    FontSlantOblique
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontWeight {
    FontWeightNormal,
    FontWeightBold
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum TextClusterFlags {
    TextClusterFlagNone     = 0x00000000,
    TextClusterFlagBackward = 0x00000001
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum FontType {
    FontTypeToy,
    FontTypeFt,
    FontTypeWin32,
    FontTypeQuartz,
    FontTypeUser
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum SubpixelOrder {
    SubpixelOrderDefault,
    SubpixelOrderRgb,
    SubpixelOrderBgr,
    SubpixelOrderVrgb,
    SubpixelOrderVbgr
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintStyle {
    HintStyleDefault,
    HintStyleNone,
    HintStyleSlight,
    HintStyleMedium,
    HintStyleFull
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum HintMetrics {
    HintMetricsDefault,
    HintMetricsOff,
    HintMetricsOn
}
