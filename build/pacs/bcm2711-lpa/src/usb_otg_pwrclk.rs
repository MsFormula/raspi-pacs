/*
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org>

*/
// Generated from SVD A, with svd2pac 0.7.0 on Thu, 17 Sep 2026 02:44:17 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"USB on the go high speed power control"]
unsafe impl ::core::marker::Send for super::UsbOtgPwrclk {}
unsafe impl ::core::marker::Sync for super::UsbOtgPwrclk {}
impl super::UsbOtgPwrclk {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "power and clock gating control"]
    #[inline(always)]
    pub const fn pcgcctl(
        &self,
    ) -> &'static crate::common::Reg<self::Pcgcctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcgcctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcgcctl_SPEC;
impl crate::sealed::RegSpec for Pcgcctl_SPEC {
    type DataType = u32;
}

#[doc = "power and clock gating control"]
pub type Pcgcctl = crate::RegValueT<Pcgcctl_SPEC>;

impl NoBitfieldReg<Pcgcctl_SPEC> for Pcgcctl {}
impl ::core::default::Default for Pcgcctl {
    #[inline(always)]
    fn default() -> Pcgcctl {
        <crate::RegValueT<Pcgcctl_SPEC> as RegisterValue<_>>::new(537624576)
    }
}
