
use std::fmt::{Show, FormatError};
use cairo::ffi;
use std::c_str::CString;

#[repr(C)]
#[deriving(PartialEq)]
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

impl Show for Status{
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), FormatError>{
        let c_str = unsafe{
            let char_ptr = ffi::cairo_status_to_string(*self);
            CString::new(char_ptr, false) //FIXME I'm not sure if we actually own the str send in by cairo
        };
        c_str.as_str().unwrap().fmt(formatter)
    }
}

impl Status{
    pub fn ensure_valid(&self){
        if *self != StatusSuccess {
            fail!("Cairo error {}", *self)
        }
    }
}

#[repr(C)]
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
pub enum FillRule {
    FillRuleWinding,
    FillRuleEvenOdd
}

#[repr(C)]
pub enum LineCap {
    LineCapButt,
    LineCapRound,
    LineCapSquare
}

#[repr(C)]
pub enum LineJoin {
    LineJoinMiter,
    LineJoinRound,
    LineJoinBevel
}

#[repr(C)]
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
pub enum PathDataType{
    PathMoveTo,
    PathLineTo,
    PathCurveTo,
    PathClosePath
}

#[repr(C)]
pub enum Content{
    ContentColor      = 0x1000,
    ContentAlpha      = 0x2000,
    ContentColorAlpha = 0x3000
}

#[repr(C)]
pub enum Extend{
    ExtendNone,
    ExtendRepeat,
    ExtendReflect,
    ExtendPad
}

#[repr(C)]
pub enum Filter{
    FilterFast,
    FilterGood,
    FilterBest,
    FilterNearest,
    FilterBilinear,
    FilterGaussian
}

#[repr(C)]
pub enum PatternType{
    PatternTypeSolid,
    PatternTypeSurface,
    PatternTypeLinearGradient,
    PatternTypeRadialGradient,
    PatternTypeMesh,
    PatternTypeRasterSource
}

#[repr(C)]
pub enum FontSlant {
    FontSlantNormal,
    FontSlantItalic,
    FontSlantOblique
}

#[repr(C)]
pub enum FontWeight {
    FontWeightNormal,
    FontWeightBold
}

#[repr(C)]
pub enum TextClusterFlags {
    TextClusterFlagNone     = 0x00000000,
    TextClusterFlagBackward = 0x00000001
}

#[repr(C)]
pub enum FontType {
    FontTypeToy,
    FontTypeFt,
    FontTypeWin32,
    FontTypeQuartz,
    FontTypeUser
}

#[repr(C)]
pub enum SubpixelOrder {
    SubpixelOrderDefault,
    SubpixelOrderRgb,
    SubpixelOrderBgr,
    SubpixelOrderVrgb,
    SubpixelOrderVbgr
}

#[repr(C)]
pub enum HintStyle {
    HintStyleDefault,
    HintStyleNone,
    HintStyleSlight,
    HintStyleMedium,
    HintStyleFull
}

#[repr(C)]
pub enum HintMetrics {
    HintMetricsDefault,
    HintMetricsOff,
    HintMetricsOn
}