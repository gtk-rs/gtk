use std::marker::PhantomFn;
use translate::{FromGlib, ToGlib};
use ffi;

/// A GLib or GLib-based library type
#[derive(Copy, Debug, PartialEq, Eq)]
pub enum Type {
    /// An invalid `Type` used as error return value in some functions
    Invalid,
    /// The fundamental type corresponding to the unit type `()`
    Unit,
    /// The fundamental type corresponding to `i8`
    I8,
    /// The fundamental type corresponding to `u8`
    U8,
    /// The fundamental type corresponding to `bool`
    Bool,
    /// The fundamental type corresponding to `i32`
    I32,
    /// The fundamental type corresponding to `u32`
    U32,
    /// The fundamental type corresponding to `isize`
    ISize,
    /// The fundamental type corresponding to `usize`
    USize,
    /// The fundamental type corresponding to `i64`
    I64,
    /// The fundamental type corresponding to `u64`
    U64,
    /// The fundamental type corresponding to `f32`
    F32,
    /// The fundamental type corresponding to `f64`
    F64,
    /// The fundamental type corresponding to `String`
    String,
    /// The fundamental type corresponding to a pointer
    Pointer,
    /// The fundamental type of GVariant
    Variant,
    /// The fundamental type from which all interfaces are derived
    BaseInterface,
    /// The fundamental type from which all enumeration types are derived
    BaseEnum,
    /// The fundamental type from which all flags types are derived
    BaseFlags,
    /// The fundamental type from which all boxed types are derived
    BaseBoxed,
    /// The fundamental type from which all `GParamSpec` types are derived
    BaseParamSpec,
    /// The fundamental type from which all objects are derived
    BaseObject,
    /// A non-fundamental type identified by value of type `usize`
    Other(usize),
}

pub trait GetType: PhantomFn<Self> {
    fn get_type() -> Type;
}

impl FromGlib for Type {
    type GlibType = ffi::GType;

    fn conv(val: ffi::GType) -> Type {
        use self::Type::*;
        match val {
            ffi::G_TYPE_INVALID => Invalid,
            ffi::G_TYPE_NONE => Unit,
            ffi::G_TYPE_INTERFACE => BaseInterface,
            ffi::G_TYPE_CHAR => I8,
            ffi::G_TYPE_UCHAR => U8,
            ffi::G_TYPE_BOOLEAN => Bool,
            ffi::G_TYPE_INT => I32,
            ffi::G_TYPE_UINT => U32,
            ffi::G_TYPE_LONG => ISize,
            ffi::G_TYPE_ULONG => USize,
            ffi::G_TYPE_INT64 => I64,
            ffi::G_TYPE_UINT64 => U64,
            ffi::G_TYPE_ENUM => BaseEnum,
            ffi::G_TYPE_FLAGS => BaseFlags,
            ffi::G_TYPE_FLOAT => F32,
            ffi::G_TYPE_DOUBLE => F64,
            ffi::G_TYPE_STRING => String,
            ffi::G_TYPE_POINTER => Pointer,
            ffi::G_TYPE_BOXED => BaseBoxed,
            ffi::G_TYPE_PARAM => BaseParamSpec,
            ffi::G_TYPE_OBJECT => BaseObject,
            ffi::G_TYPE_VARIANT => Variant,
            x => Other(x as usize),
        }
    }
}

impl ToGlib for Type {
    type GlibType = ffi::GType;

    fn to_glib(&self) -> ffi::GType {
        use self::Type::*;
        match *self {
            Invalid => ffi::G_TYPE_INVALID,
            Unit => ffi::G_TYPE_NONE,
            BaseInterface => ffi::G_TYPE_INTERFACE,
            I8 => ffi::G_TYPE_CHAR,
            U8 => ffi::G_TYPE_UCHAR,
            Bool => ffi::G_TYPE_BOOLEAN,
            I32 => ffi::G_TYPE_INT,
            U32 => ffi::G_TYPE_UINT,
            ISize => ffi::G_TYPE_LONG,
            USize => ffi::G_TYPE_ULONG,
            I64 => ffi::G_TYPE_INT64,
            U64 => ffi::G_TYPE_UINT64,
            BaseEnum => ffi::G_TYPE_ENUM,
            BaseFlags => ffi::G_TYPE_FLAGS,
            F32 => ffi::G_TYPE_FLOAT,
            F64 => ffi::G_TYPE_DOUBLE,
            String => ffi::G_TYPE_STRING,
            Pointer => ffi::G_TYPE_POINTER,
            BaseBoxed => ffi::G_TYPE_BOXED,
            BaseParamSpec => ffi::G_TYPE_PARAM,
            BaseObject => ffi::G_TYPE_OBJECT,
            Variant => ffi::G_TYPE_VARIANT,
            Other(x) => x as ffi::GType,
        }
    }
}
