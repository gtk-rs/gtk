pub use self::context::Context;
pub use self::paths::{
	Path,
	PathIterator,
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
	Mesh
};

pub mod enums;
pub mod types;

mod context;
mod paths;
mod transformations;
mod patterns;

mod ffi;