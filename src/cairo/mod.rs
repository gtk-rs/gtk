
/*!
Bindings and wrappers for __Cairo__
*/

pub use self::context::{
    Context,
    Rectangle
};

pub use self::paths::{
    Path,
    PathSegments,
    PathSegment
};

pub use self::enums::{
    Status,
    Antialias,
    FillRule,
    LineCap,
    LineJoin,
    Operator,
    PathDataType
};

pub use self::patterns::{
    //Traits
    Pattern,
    Gradient,

    //Structs
    LinearGradient,
    RadialGradient,
    SolidPattern,
    SurfacePattern,
    Mesh,

    //Enum
    MeshCorner,
};

pub use self::fonts::{
    FontFace,
    ScaledFont,
    FontOptions,

    Glyph,
    FontExtents,
    TextExtents,
    TextCluster,
};

pub use self::matrices::Matrix;

pub mod enums;
pub mod ffi;

mod fonts;
mod context;
mod paths;
mod patterns;
mod matrices;