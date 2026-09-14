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
// Generated from SVD A, with svd2pac 0.7.0 on Mon, 14 Sep 2026 00:08:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Three auxiliary peripherals"]
unsafe impl ::core::marker::Send for super::Aux {}
unsafe impl ::core::marker::Sync for super::Aux {}
impl super::Aux {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Interrupt status"]
    #[inline(always)]
    pub const fn irq(&self) -> &'static crate::common::Reg<self::Irq_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Irq_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Enable sub-peripherals"]
    #[inline(always)]
    pub const fn enables(
        &self,
    ) -> &'static crate::common::Reg<self::Enables_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Enables_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq_SPEC;
impl crate::sealed::RegSpec for Irq_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt status"]
pub type Irq = crate::RegValueT<Irq_SPEC>;

impl NoBitfieldReg<Irq_SPEC> for Irq {}
impl ::core::default::Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        <crate::RegValueT<Irq_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enables_SPEC;
impl crate::sealed::RegSpec for Enables_SPEC {
    type DataType = u32;
}

#[doc = "Enable sub-peripherals"]
pub type Enables = crate::RegValueT<Enables_SPEC>;

impl NoBitfieldReg<Enables_SPEC> for Enables {}
impl ::core::default::Default for Enables {
    #[inline(always)]
    fn default() -> Enables {
        <crate::RegValueT<Enables_SPEC> as RegisterValue<_>>::new(0)
    }
}
