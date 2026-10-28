//! Output vocabulary: `wl_output.transform`, which `casement-wayland`, the desktop client and
//! the shell all name.

use super::code::wire_enum;

wire_enum! {
    /// How an output's content is rotated (counter-clockwise) and flipped before scanout
    /// (`wl_output.transform`).
    Transform {
        /// As drawn.
        Normal = 0,
        /// Rotated 90 degrees.
        R90 = 1,
        /// Rotated 180 degrees.
        R180 = 2,
        /// Rotated 270 degrees.
        R270 = 3,
        /// Flipped about the vertical axis.
        Flipped = 4,
        /// Flipped, then rotated 90 degrees.
        Flipped90 = 5,
        /// Flipped, then rotated 180 degrees.
        Flipped180 = 6,
        /// Flipped, then rotated 270 degrees.
        Flipped270 = 7,
    }
}
