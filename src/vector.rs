macro_rules! impl_binary_ops {
    ($type:ident, $trait:ident, $fn:ident, $op:tt $(, $field:ident)*) => {
        impl std::ops::$trait<$type> for $type {
            type Output = $type;

            fn $fn(self, _rhs: $type) -> $type {
                $type {
                    $(
                        $field: self.$field $op _rhs.$field,
                    )*
                }
            }
        }
    };
}
macro_rules! impl_binary_assign_ops {
    ($type:ident, $trait:ident, $fn:ident, $op:tt $(, $field:ident)*) => {
        impl std::ops::$trait<$type> for $type {
            fn $fn(&mut self, other: $type) {
                $(self.$field $op other.$field;)*
            }
        }
    };
}
macro_rules! impl_neg {
    ($type:ident $(, $field:ident)*) => {
        impl std::ops::Neg for $type {
            type Output = $type;

            fn neg(self) -> $type {
                $type {
                    $(
                        $field: -self.$field,
                    )*
                }
            }
        }
    };
}
macro_rules! impl_unary {
    ($fn:ident, $type:ident $(, $field:ident)*) => {
        impl $type {
            pub fn $fn(self) -> $type {
                $type {
                    $(
                        $field: self.$field.$fn(),
                    )*
                }
            }
        }
    };
}
macro_rules! impl_vec {
    ($type:ident, $( $field:ident ),* ) => {
        impl_binary_ops!($type, Add, add, + $(, $field)*);
        impl_binary_ops!($type, Sub, sub, - $(, $field)*);
        impl_binary_ops!($type, Mul, mul, * $(, $field)*);
        impl_binary_ops!($type, Div, div, / $(, $field)*);
        impl_binary_ops!($type, Rem, rem, % $(, $field)*);
        impl_binary_assign_ops!($type, AddAssign, add_assign, += $(, $field)*);
        impl_binary_assign_ops!($type, SubAssign, sub_assign, -= $(, $field)*);
        impl_binary_assign_ops!($type, MulAssign, mul_assign, *= $(, $field)*);
        impl_binary_assign_ops!($type, DivAssign, div_assign, /= $(, $field)*);
        impl_binary_assign_ops!($type, RemAssign, rem_assign, %= $(, $field)*);
        impl_neg!($type $(, $field)*);
        impl_unary!(fract, $type $(, $field)*);
        impl_unary!(abs, $type $(, $field)*);
    };
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl_vec!(Vec2, x, y);
impl_vec!(Vec3, x, y, z);
impl_vec!(Vec4, x, y, z, w);

// maybe todo
// this * scalar
// scalar * this
// scalar product
// ones zeros, nth one
// probably just neg?
// then 1 per magnitude etc
// then 1 thats all for a type
// is the op infix needed or what
// then 1 thing that implements all for a type and its names
// impl ctor maybe. esp if all inner elements are same type
// yo if you used this with vec4s as inner type would you get a matrix4x4?

// also lol, IVec2 and stuff too yes?