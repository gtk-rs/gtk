pub use self::context::Context;
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
    MeshCorner0,
    MeshCorner1,
    MeshCorner2,
    MeshCorner3
};

pub mod enums;
pub mod ffi;

mod context;
mod paths;
mod transformations;
mod patterns;