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
// Generated from SVD A, with svd2pac 0.8.0 on Fri, 25 Sep 2026 21:30:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common;
#[allow(unused_imports)]
use crate::common::{
    AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
    Write as _,
};
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
    pub fn pcgcctl(&self) -> &'static self::PcgcctlT {
        unsafe { self::PcgcctlT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }
}

#[doc = "power and clock gating control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcgcctl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Pcgcctl {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct PcgcctlT;
unsafe impl crate::common::AsPtr for PcgcctlT {}
impl crate::common::Reg<Pcgcctl> for PcgcctlT {}

unsafe impl crate::common::Read<Pcgcctl> for PcgcctlT {}
unsafe impl crate::common::Write<Pcgcctl> for PcgcctlT {}
impl Pcgcctl {
    #[doc = "Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Power clamp"]
    #[inline(always)]
    pub fn pwrclmp(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Power down modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "PHY Suspended"]
    #[inline(always)]
    pub fn physusp(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable sleep clock gating"]
    #[inline(always)]
    pub fn enable_l1gating(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "PHY is in sleep mode"]
    #[inline(always)]
    pub fn physleep(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "PHY is in deep sleep"]
    #[inline(always)]
    pub fn deepsleep(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Reset after suspend"]
    #[inline(always)]
    pub fn resetaftersusp(self) -> crate::common::RegisterFieldBool<8, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Restore mode"]
    #[inline(always)]
    pub fn restoremode(self) -> crate::common::RegisterFieldBool<9, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable extended hibernation"]
    #[inline(always)]
    pub fn enextndedhiber(self) -> crate::common::RegisterFieldBool<10, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Extended hibernation clamp"]
    #[inline(always)]
    pub fn extndedhibernationclamp(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Extended hibernation switch"]
    #[inline(always)]
    pub fn extndedhibernationswitch(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Essential register values restored"]
    #[inline(always)]
    pub fn essregrestored(self) -> crate::common::RegisterFieldBool<13, 1, 0, Pcgcctl, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pcgcctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Restore value"]
    #[inline(always)]
    pub fn restore_value(
        self,
    ) -> crate::common::RegisterField<14, 0x3ffff, 1, 0, u32, u32, Pcgcctl, common::RW> {
        crate::common::RegisterField::<14,0x3ffff,1,0,u32,u32,Pcgcctl,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Pcgcctl> for PcgcctlT {
    #[inline(always)]
    fn reset_value(&self) -> Pcgcctl {
        Pcgcctl::new(537624576)
    }
}
