pub use self::status::Status;

pub mod status{
    #[repr(C)]
    #[deriving(PartialEq)]
    pub enum Status {
        Success = 0,

        NoMemory,
        InvalidRestore,
        InvalidPopGroup,
        NoCurrentPoint,
        InvalidMatrix,
        InvalidStatus,
        NullPointer,
        InvalidString,
        InvalidPathData,
        ReadError,
        WriteError,
        SurfaceFinished,
        SurfaceTypeMismatch,
        PatternTypeMismatch,
        InvalidContent,
        InvalidFormat,
        InvalidVisual,
        FileNotFound,
        InvalidDash,
        InvalidDscComment,
        InvalidIndex,
        ClipNotRepresentable,
        TempFileError,
        InvalidStride,
        FontTypeMismatch,
        UserFontImmutable,
        UserFontError,
        NegativeCount,
        InvalidClusters,
        InvalidSlant,
        InvalidWeight,
        InvalidSize,
        UserFontNotImplemented,
        DeviceTypeMismatch,
        DeviceError,
        InvalidMeshConstruction,
        DeviceFinished,

        LastStatus
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
