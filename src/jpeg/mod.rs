//! Decoding and Encoding of JPEG Images
//!
//! JPEG (Joint Photographic Experts Group) is an image format that supports lossy compression.
//! This module implements the Baseline JPEG standard.
//!
//! # Related Links
//! * <http://www.w3.org/Graphics/JPEG/itu-t81.pdf> - The JPEG specification
//!

pub use self::decoder::JPEGDecoder;
pub use self::encoder::JPEGEncoder;

mod encoder;
mod decoder;
mod entropy;
mod transform;
