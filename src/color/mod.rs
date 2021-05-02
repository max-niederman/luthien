pub mod average;
pub mod region;

pub use region::Region;

// Many trait bounds require a set whitepoint. This is aliased here.
pub type WhitePoint = palette::white_point::D65;
