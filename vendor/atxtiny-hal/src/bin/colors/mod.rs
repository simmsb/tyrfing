/// Import HSV color support
pub mod hsv;

/// Import RGB color support
pub mod rgb;

/// Import RGBA color support
pub mod rgba;

/// Exports the color functions/types in the `prelude` namespace.
pub mod prelude {
    pub use crate::colors::hsv::*;
    pub use crate::colors::rgb::*;
    pub use crate::colors::rgba::*;
}
