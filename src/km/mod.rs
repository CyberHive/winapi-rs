// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers for kernel mode
#[cfg(feature = "d3dkmthk")]
pub mod d3dkmthk;

//make relax and take slow
#[cfg(feature = "km_util")]
pub type MISS_TYPE_PTR = *const ::shared::ntdef::PVOID;

#[cfg(feature = "fltkernel")]
pub mod fltkernel;
#[cfg(feature = "fwp")]
pub mod fwp;
#[cfg(feature = "ndis")]
pub mod ndis;
#[cfg(feature = "ntifs")]
pub mod ntifs;
#[cfg(feature = "wdm")]
pub mod wdm;
