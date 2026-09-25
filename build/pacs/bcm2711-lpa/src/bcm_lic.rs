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
// Generated from SVD A, with svd2pac 0.8.0 on Fri, 25 Sep 2026 21:30:45 +0000

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
#[doc = r"Broadcom Legacy Interrupt Controller"]
unsafe impl ::core::marker::Send for super::BcmLic {}
unsafe impl ::core::marker::Sync for super::BcmLic {}
impl super::BcmLic {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Basic pending info"]
    #[inline(always)]
    pub fn basic_pending(&self) -> &'static self::BasicPendingT {
        unsafe { self::BasicPendingT::from_ptr(self._svd2pac_as_ptr().add(512usize)) }
    }

    #[doc = "Pending state for interrupts 1 - 31"]
    #[inline(always)]
    pub fn pending_1(&self) -> &'static self::Pending1T {
        unsafe { self::Pending1T::from_ptr(self._svd2pac_as_ptr().add(516usize)) }
    }

    #[doc = "Pending state for interrupts 32 - 63"]
    #[inline(always)]
    pub fn pending_2(&self) -> &'static self::Pending2T {
        unsafe { self::Pending2T::from_ptr(self._svd2pac_as_ptr().add(520usize)) }
    }

    #[doc = "FIQ control"]
    #[inline(always)]
    pub fn fiq_control(&self) -> &'static self::FiqControlT {
        unsafe { self::FiqControlT::from_ptr(self._svd2pac_as_ptr().add(524usize)) }
    }

    #[doc = "Enable interrupts 1 - 31"]
    #[inline(always)]
    pub fn enable_1(&self) -> &'static self::Enable1T {
        unsafe { self::Enable1T::from_ptr(self._svd2pac_as_ptr().add(528usize)) }
    }

    #[doc = "Enable interrupts 32 - 63"]
    #[inline(always)]
    pub fn enable_2(&self) -> &'static self::Enable2T {
        unsafe { self::Enable2T::from_ptr(self._svd2pac_as_ptr().add(532usize)) }
    }

    #[doc = "Enable basic interrupts"]
    #[inline(always)]
    pub fn enable_basic(&self) -> &'static self::EnableBasicT {
        unsafe { self::EnableBasicT::from_ptr(self._svd2pac_as_ptr().add(536usize)) }
    }

    #[doc = "Disable interrupts 1 - 31"]
    #[inline(always)]
    pub fn disable_1(&self) -> &'static self::Disable1T {
        unsafe { self::Disable1T::from_ptr(self._svd2pac_as_ptr().add(540usize)) }
    }

    #[doc = "Disable interrupts 32 - 63"]
    #[inline(always)]
    pub fn disable_2(&self) -> &'static self::Disable2T {
        unsafe { self::Disable2T::from_ptr(self._svd2pac_as_ptr().add(544usize)) }
    }

    #[doc = "Disable basic interrupts"]
    #[inline(always)]
    pub fn disable_basic(&self) -> &'static self::DisableBasicT {
        unsafe { self::DisableBasicT::from_ptr(self._svd2pac_as_ptr().add(548usize)) }
    }
}

#[doc = "Basic pending info"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BasicPending {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for BasicPending {
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
pub struct BasicPendingT;
unsafe impl crate::common::AsPtr for BasicPendingT {}
impl crate::common::Reg<BasicPending> for BasicPendingT {}

unsafe impl crate::common::Read<BasicPending> for BasicPendingT {}
impl BasicPending {
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn timer(self) -> crate::common::RegisterFieldBool<0, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn mailbox(self) -> crate::common::RegisterFieldBool<1, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(self) -> crate::common::RegisterFieldBool<2, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(self) -> crate::common::RegisterFieldBool<3, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(self) -> crate::common::RegisterFieldBool<4, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(self) -> crate::common::RegisterFieldBool<5, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "One or more bits are set in PENDING_1 (ignores 7, 9, 10, 18, 19)"]
    #[inline(always)]
    pub fn pending_1(self) -> crate::common::RegisterFieldBool<8, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "One or more bits are set in PENDING_2 (ignores 53 - 57, 62)"]
    #[inline(always)]
    pub fn pending_2(self) -> crate::common::RegisterFieldBool<9, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, BasicPending, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn int7(self) -> crate::common::RegisterFieldBool<10, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn int9(self) -> crate::common::RegisterFieldBool<11, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn int10(self) -> crate::common::RegisterFieldBool<12, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn int18(self) -> crate::common::RegisterFieldBool<13, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn int19(self) -> crate::common::RegisterFieldBool<14, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn int53(self) -> crate::common::RegisterFieldBool<15, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn int54(self) -> crate::common::RegisterFieldBool<16, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn int55(self) -> crate::common::RegisterFieldBool<17, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn int56(self) -> crate::common::RegisterFieldBool<18, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn int57(self) -> crate::common::RegisterFieldBool<19, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn int62(self) -> crate::common::RegisterFieldBool<20, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<BasicPending> for BasicPendingT {
    #[inline(always)]
    fn reset_value(&self) -> BasicPending {
        BasicPending::new(0)
    }
}

#[doc = "Pending state for interrupts 1 - 31"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Pending1 {
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
pub struct Pending1T;
unsafe impl crate::common::AsPtr for Pending1T {}
impl crate::common::Reg<Pending1> for Pending1T {}

unsafe impl crate::common::Read<Pending1> for Pending1T {}
impl Pending1 {
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn int0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn int1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn int2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn int3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn int4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn int5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn int6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn int7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 8"]
    #[inline(always)]
    pub fn int8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn int9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn int10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 11"]
    #[inline(always)]
    pub fn int11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 12"]
    #[inline(always)]
    pub fn int12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 13"]
    #[inline(always)]
    pub fn int13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 14"]
    #[inline(always)]
    pub fn int14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 15"]
    #[inline(always)]
    pub fn int15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 16"]
    #[inline(always)]
    pub fn int16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 17"]
    #[inline(always)]
    pub fn int17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn int18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn int19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 20"]
    #[inline(always)]
    pub fn int20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 21"]
    #[inline(always)]
    pub fn int21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 22"]
    #[inline(always)]
    pub fn int22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 23"]
    #[inline(always)]
    pub fn int23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 24"]
    #[inline(always)]
    pub fn int24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 25"]
    #[inline(always)]
    pub fn int25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 26"]
    #[inline(always)]
    pub fn int26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 27"]
    #[inline(always)]
    pub fn int27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 28"]
    #[inline(always)]
    pub fn int28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 29"]
    #[inline(always)]
    pub fn int29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 30"]
    #[inline(always)]
    pub fn int30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 31"]
    #[inline(always)]
    pub fn int31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Pending1, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Pending1> for Pending1T {
    #[inline(always)]
    fn reset_value(&self) -> Pending1 {
        Pending1::new(0)
    }
}

#[doc = "Pending state for interrupts 32 - 63"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Pending2 {
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
pub struct Pending2T;
unsafe impl crate::common::AsPtr for Pending2T {}
impl crate::common::Reg<Pending2> for Pending2T {}

unsafe impl crate::common::Read<Pending2> for Pending2T {}
impl Pending2 {
    #[doc = "Interrupt 32"]
    #[inline(always)]
    pub fn int32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 33"]
    #[inline(always)]
    pub fn int33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 34"]
    #[inline(always)]
    pub fn int34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 35"]
    #[inline(always)]
    pub fn int35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 36"]
    #[inline(always)]
    pub fn int36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 37"]
    #[inline(always)]
    pub fn int37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 38"]
    #[inline(always)]
    pub fn int38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 39"]
    #[inline(always)]
    pub fn int39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 40"]
    #[inline(always)]
    pub fn int40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 41"]
    #[inline(always)]
    pub fn int41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 42"]
    #[inline(always)]
    pub fn int42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 43"]
    #[inline(always)]
    pub fn int43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 44"]
    #[inline(always)]
    pub fn int44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 45"]
    #[inline(always)]
    pub fn int45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 46"]
    #[inline(always)]
    pub fn int46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 47"]
    #[inline(always)]
    pub fn int47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 48"]
    #[inline(always)]
    pub fn int48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 49"]
    #[inline(always)]
    pub fn int49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 50"]
    #[inline(always)]
    pub fn int50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 51"]
    #[inline(always)]
    pub fn int51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 52"]
    #[inline(always)]
    pub fn int52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn int53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn int54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn int55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn int56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn int57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 58"]
    #[inline(always)]
    pub fn int58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 59"]
    #[inline(always)]
    pub fn int59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 60"]
    #[inline(always)]
    pub fn int60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 61"]
    #[inline(always)]
    pub fn int61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn int62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Interrupt 63"]
    #[inline(always)]
    pub fn int63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Pending2, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Pending2> for Pending2T {
    #[inline(always)]
    fn reset_value(&self) -> Pending2 {
        Pending2::new(0)
    }
}

#[doc = "FIQ control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FiqControl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for FiqControl {
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
pub struct FiqControlT;
unsafe impl crate::common::AsPtr for FiqControlT {}
impl crate::common::Reg<FiqControl> for FiqControlT {}

unsafe impl crate::common::Read<FiqControl> for FiqControlT {}
unsafe impl crate::common::Write<FiqControl> for FiqControlT {}
impl FiqControl {
    #[doc = "FIQ Enable"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::RegisterFieldBool<7, 1, 0, FiqControl, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, FiqControl, common::RW>::from_register(self, 0)
    }

    #[doc = "FIQ Source"]
    #[inline(always)]
    pub fn source(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        fiq_control::Source,
        fiq_control::Source,
        FiqControl,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            fiq_control::Source,
            fiq_control::Source,
            FiqControl,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<FiqControl> for FiqControlT {
    #[inline(always)]
    fn reset_value(&self) -> FiqControl {
        FiqControl::new(0)
    }
}
pub mod fiq_control {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Source(u8);

    impl Source {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Source {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Source {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Source> for u64 {
        #[inline(always)]
        fn from(value: Source) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Source {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Source {
        #[doc = "Interrupt 0"]
        pub const INT_0: Self = Self(0);

        #[doc = "Interrupt 1"]
        pub const INT_1: Self = Self(1);

        #[doc = "Interrupt 2"]
        pub const INT_2: Self = Self(2);

        #[doc = "Interrupt 3"]
        pub const INT_3: Self = Self(3);

        #[doc = "Interrupt 4"]
        pub const INT_4: Self = Self(4);

        #[doc = "Interrupt 5"]
        pub const INT_5: Self = Self(5);

        #[doc = "Interrupt 6"]
        pub const INT_6: Self = Self(6);

        #[doc = "Interrupt 7"]
        pub const INT_7: Self = Self(7);

        #[doc = "Interrupt 8"]
        pub const INT_8: Self = Self(8);

        #[doc = "Interrupt 9"]
        pub const INT_9: Self = Self(9);

        #[doc = "Interrupt 10"]
        pub const INT_10: Self = Self(10);

        #[doc = "Interrupt 11"]
        pub const INT_11: Self = Self(11);

        #[doc = "Interrupt 12"]
        pub const INT_12: Self = Self(12);

        #[doc = "Interrupt 13"]
        pub const INT_13: Self = Self(13);

        #[doc = "Interrupt 14"]
        pub const INT_14: Self = Self(14);

        #[doc = "Interrupt 15"]
        pub const INT_15: Self = Self(15);

        #[doc = "Interrupt 16"]
        pub const INT_16: Self = Self(16);

        #[doc = "Interrupt 17"]
        pub const INT_17: Self = Self(17);

        #[doc = "Interrupt 18"]
        pub const INT_18: Self = Self(18);

        #[doc = "Interrupt 19"]
        pub const INT_19: Self = Self(19);

        #[doc = "Interrupt 20"]
        pub const INT_20: Self = Self(20);

        #[doc = "Interrupt 21"]
        pub const INT_21: Self = Self(21);

        #[doc = "Interrupt 22"]
        pub const INT_22: Self = Self(22);

        #[doc = "Interrupt 23"]
        pub const INT_23: Self = Self(23);

        #[doc = "Interrupt 24"]
        pub const INT_24: Self = Self(24);

        #[doc = "Interrupt 25"]
        pub const INT_25: Self = Self(25);

        #[doc = "Interrupt 26"]
        pub const INT_26: Self = Self(26);

        #[doc = "Interrupt 27"]
        pub const INT_27: Self = Self(27);

        #[doc = "Interrupt 28"]
        pub const INT_28: Self = Self(28);

        #[doc = "Interrupt 29"]
        pub const INT_29: Self = Self(29);

        #[doc = "Interrupt 30"]
        pub const INT_30: Self = Self(30);

        #[doc = "Interrupt 31"]
        pub const INT_31: Self = Self(31);

        #[doc = "Interrupt 32"]
        pub const INT_32: Self = Self(32);

        #[doc = "Interrupt 33"]
        pub const INT_33: Self = Self(33);

        #[doc = "Interrupt 34"]
        pub const INT_34: Self = Self(34);

        #[doc = "Interrupt 35"]
        pub const INT_35: Self = Self(35);

        #[doc = "Interrupt 36"]
        pub const INT_36: Self = Self(36);

        #[doc = "Interrupt 37"]
        pub const INT_37: Self = Self(37);

        #[doc = "Interrupt 38"]
        pub const INT_38: Self = Self(38);

        #[doc = "Interrupt 39"]
        pub const INT_39: Self = Self(39);

        #[doc = "Interrupt 40"]
        pub const INT_40: Self = Self(40);

        #[doc = "Interrupt 41"]
        pub const INT_41: Self = Self(41);

        #[doc = "Interrupt 42"]
        pub const INT_42: Self = Self(42);

        #[doc = "Interrupt 43"]
        pub const INT_43: Self = Self(43);

        #[doc = "Interrupt 44"]
        pub const INT_44: Self = Self(44);

        #[doc = "Interrupt 45"]
        pub const INT_45: Self = Self(45);

        #[doc = "Interrupt 46"]
        pub const INT_46: Self = Self(46);

        #[doc = "Interrupt 47"]
        pub const INT_47: Self = Self(47);

        #[doc = "Interrupt 48"]
        pub const INT_48: Self = Self(48);

        #[doc = "Interrupt 49"]
        pub const INT_49: Self = Self(49);

        #[doc = "Interrupt 50"]
        pub const INT_50: Self = Self(50);

        #[doc = "Interrupt 51"]
        pub const INT_51: Self = Self(51);

        #[doc = "Interrupt 52"]
        pub const INT_52: Self = Self(52);

        #[doc = "Interrupt 53"]
        pub const INT_53: Self = Self(53);

        #[doc = "Interrupt 54"]
        pub const INT_54: Self = Self(54);

        #[doc = "Interrupt 55"]
        pub const INT_55: Self = Self(55);

        #[doc = "Interrupt 56"]
        pub const INT_56: Self = Self(56);

        #[doc = "Interrupt 57"]
        pub const INT_57: Self = Self(57);

        #[doc = "Interrupt 58"]
        pub const INT_58: Self = Self(58);

        #[doc = "Interrupt 59"]
        pub const INT_59: Self = Self(59);

        #[doc = "Interrupt 60"]
        pub const INT_60: Self = Self(60);

        #[doc = "Interrupt 61"]
        pub const INT_61: Self = Self(61);

        #[doc = "Interrupt 62"]
        pub const INT_62: Self = Self(62);

        #[doc = "Interrupt 63"]
        pub const INT_63: Self = Self(63);

        #[doc = "ARMC Timer"]
        pub const TIMER: Self = Self(64);

        #[doc = "Mailbox"]
        pub const MAILBOX: Self = Self(65);

        #[doc = "Doorbell 0"]
        pub const DOORBELL_0: Self = Self(66);

        #[doc = "Doorbell 1"]
        pub const DOORBELL_1: Self = Self(67);

        #[doc = "VPU0 halted"]
        pub const VPU_0_HALTED: Self = Self(68);

        #[doc = "VPU1 halted"]
        pub const VPU_1_HALTED: Self = Self(69);

        #[doc = "ARM address error"]
        pub const ARM_ADDRESS_ERROR: Self = Self(70);

        #[doc = "ARM AXI error"]
        pub const ARM_AXI_ERROR: Self = Self(71);
    }
}

#[doc = "Enable interrupts 1 - 31"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Enable1 {
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
pub struct Enable1T;
unsafe impl crate::common::AsPtr for Enable1T {}
impl crate::common::Reg<Enable1> for Enable1T {}

unsafe impl crate::common::Read<Enable1> for Enable1T {}
unsafe impl crate::common::Write<Enable1> for Enable1T {}
impl Enable1 {
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn int0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn int1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn int2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn int3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn int4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn int5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn int6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn int7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 8"]
    #[inline(always)]
    pub fn int8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn int9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn int10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 11"]
    #[inline(always)]
    pub fn int11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 12"]
    #[inline(always)]
    pub fn int12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 13"]
    #[inline(always)]
    pub fn int13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 14"]
    #[inline(always)]
    pub fn int14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 15"]
    #[inline(always)]
    pub fn int15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 16"]
    #[inline(always)]
    pub fn int16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 17"]
    #[inline(always)]
    pub fn int17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn int18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn int19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 20"]
    #[inline(always)]
    pub fn int20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 21"]
    #[inline(always)]
    pub fn int21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 22"]
    #[inline(always)]
    pub fn int22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 23"]
    #[inline(always)]
    pub fn int23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 24"]
    #[inline(always)]
    pub fn int24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 25"]
    #[inline(always)]
    pub fn int25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 26"]
    #[inline(always)]
    pub fn int26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 27"]
    #[inline(always)]
    pub fn int27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 28"]
    #[inline(always)]
    pub fn int28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 29"]
    #[inline(always)]
    pub fn int29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 30"]
    #[inline(always)]
    pub fn int30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 31"]
    #[inline(always)]
    pub fn int31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Enable1> for Enable1T {
    #[inline(always)]
    fn reset_value(&self) -> Enable1 {
        Enable1::new(0)
    }
}

#[doc = "Enable interrupts 32 - 63"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Enable2 {
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
pub struct Enable2T;
unsafe impl crate::common::AsPtr for Enable2T {}
impl crate::common::Reg<Enable2> for Enable2T {}

unsafe impl crate::common::Read<Enable2> for Enable2T {}
unsafe impl crate::common::Write<Enable2> for Enable2T {}
impl Enable2 {
    #[doc = "Interrupt 32"]
    #[inline(always)]
    pub fn int32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 33"]
    #[inline(always)]
    pub fn int33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 34"]
    #[inline(always)]
    pub fn int34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 35"]
    #[inline(always)]
    pub fn int35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 36"]
    #[inline(always)]
    pub fn int36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 37"]
    #[inline(always)]
    pub fn int37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 38"]
    #[inline(always)]
    pub fn int38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 39"]
    #[inline(always)]
    pub fn int39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 40"]
    #[inline(always)]
    pub fn int40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 41"]
    #[inline(always)]
    pub fn int41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 42"]
    #[inline(always)]
    pub fn int42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 43"]
    #[inline(always)]
    pub fn int43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 44"]
    #[inline(always)]
    pub fn int44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 45"]
    #[inline(always)]
    pub fn int45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 46"]
    #[inline(always)]
    pub fn int46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 47"]
    #[inline(always)]
    pub fn int47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 48"]
    #[inline(always)]
    pub fn int48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 49"]
    #[inline(always)]
    pub fn int49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 50"]
    #[inline(always)]
    pub fn int50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 51"]
    #[inline(always)]
    pub fn int51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 52"]
    #[inline(always)]
    pub fn int52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn int53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn int54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn int55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn int56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn int57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 58"]
    #[inline(always)]
    pub fn int58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 59"]
    #[inline(always)]
    pub fn int59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 60"]
    #[inline(always)]
    pub fn int60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 61"]
    #[inline(always)]
    pub fn int61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn int62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 63"]
    #[inline(always)]
    pub fn int63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Enable2> for Enable2T {
    #[inline(always)]
    fn reset_value(&self) -> Enable2 {
        Enable2::new(0)
    }
}

#[doc = "Enable basic interrupts"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnableBasic {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for EnableBasic {
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
pub struct EnableBasicT;
unsafe impl crate::common::AsPtr for EnableBasicT {}
impl crate::common::Reg<EnableBasic> for EnableBasicT {}

unsafe impl crate::common::Read<EnableBasic> for EnableBasicT {}
unsafe impl crate::common::Write<EnableBasic> for EnableBasicT {}
impl EnableBasic {
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn timer(self) -> crate::common::RegisterFieldBool<0, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }

    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn mailbox(self) -> crate::common::RegisterFieldBool<1, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }

    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(self) -> crate::common::RegisterFieldBool<2, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }

    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(self) -> crate::common::RegisterFieldBool<3, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }

    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(self) -> crate::common::RegisterFieldBool<4, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }

    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(self) -> crate::common::RegisterFieldBool<5, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }

    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }

    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EnableBasic, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, EnableBasic, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<EnableBasic> for EnableBasicT {
    #[inline(always)]
    fn reset_value(&self) -> EnableBasic {
        EnableBasic::new(0)
    }
}

#[doc = "Disable interrupts 1 - 31"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disable1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Disable1 {
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
pub struct Disable1T;
unsafe impl crate::common::AsPtr for Disable1T {}
impl crate::common::Reg<Disable1> for Disable1T {}

unsafe impl crate::common::Read<Disable1> for Disable1T {}
unsafe impl crate::common::Write<Disable1> for Disable1T {}
impl Disable1 {
    #[doc = "Interrupt 0"]
    #[inline(always)]
    pub fn int0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 1"]
    #[inline(always)]
    pub fn int1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 2"]
    #[inline(always)]
    pub fn int2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 3"]
    #[inline(always)]
    pub fn int3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 4"]
    #[inline(always)]
    pub fn int4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 5"]
    #[inline(always)]
    pub fn int5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 6"]
    #[inline(always)]
    pub fn int6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 7"]
    #[inline(always)]
    pub fn int7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 8"]
    #[inline(always)]
    pub fn int8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 9"]
    #[inline(always)]
    pub fn int9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 10"]
    #[inline(always)]
    pub fn int10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 11"]
    #[inline(always)]
    pub fn int11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 12"]
    #[inline(always)]
    pub fn int12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 13"]
    #[inline(always)]
    pub fn int13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 14"]
    #[inline(always)]
    pub fn int14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 15"]
    #[inline(always)]
    pub fn int15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 16"]
    #[inline(always)]
    pub fn int16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 17"]
    #[inline(always)]
    pub fn int17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 18"]
    #[inline(always)]
    pub fn int18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 19"]
    #[inline(always)]
    pub fn int19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 20"]
    #[inline(always)]
    pub fn int20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 21"]
    #[inline(always)]
    pub fn int21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 22"]
    #[inline(always)]
    pub fn int22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 23"]
    #[inline(always)]
    pub fn int23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 24"]
    #[inline(always)]
    pub fn int24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 25"]
    #[inline(always)]
    pub fn int25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 26"]
    #[inline(always)]
    pub fn int26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 27"]
    #[inline(always)]
    pub fn int27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 28"]
    #[inline(always)]
    pub fn int28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 29"]
    #[inline(always)]
    pub fn int29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 30"]
    #[inline(always)]
    pub fn int30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 31"]
    #[inline(always)]
    pub fn int31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Disable1> for Disable1T {
    #[inline(always)]
    fn reset_value(&self) -> Disable1 {
        Disable1::new(0)
    }
}

#[doc = "Disable interrupts 32 - 63"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disable2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Disable2 {
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
pub struct Disable2T;
unsafe impl crate::common::AsPtr for Disable2T {}
impl crate::common::Reg<Disable2> for Disable2T {}

unsafe impl crate::common::Read<Disable2> for Disable2T {}
unsafe impl crate::common::Write<Disable2> for Disable2T {}
impl Disable2 {
    #[doc = "Interrupt 32"]
    #[inline(always)]
    pub fn int32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 33"]
    #[inline(always)]
    pub fn int33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 34"]
    #[inline(always)]
    pub fn int34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 35"]
    #[inline(always)]
    pub fn int35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 36"]
    #[inline(always)]
    pub fn int36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 37"]
    #[inline(always)]
    pub fn int37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 38"]
    #[inline(always)]
    pub fn int38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 39"]
    #[inline(always)]
    pub fn int39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 40"]
    #[inline(always)]
    pub fn int40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 41"]
    #[inline(always)]
    pub fn int41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 42"]
    #[inline(always)]
    pub fn int42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 43"]
    #[inline(always)]
    pub fn int43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 44"]
    #[inline(always)]
    pub fn int44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 45"]
    #[inline(always)]
    pub fn int45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 46"]
    #[inline(always)]
    pub fn int46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 47"]
    #[inline(always)]
    pub fn int47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 48"]
    #[inline(always)]
    pub fn int48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 49"]
    #[inline(always)]
    pub fn int49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 50"]
    #[inline(always)]
    pub fn int50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 51"]
    #[inline(always)]
    pub fn int51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 52"]
    #[inline(always)]
    pub fn int52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 53"]
    #[inline(always)]
    pub fn int53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 54"]
    #[inline(always)]
    pub fn int54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 55"]
    #[inline(always)]
    pub fn int55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 56"]
    #[inline(always)]
    pub fn int56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 57"]
    #[inline(always)]
    pub fn int57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 58"]
    #[inline(always)]
    pub fn int58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 59"]
    #[inline(always)]
    pub fn int59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 60"]
    #[inline(always)]
    pub fn int60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 61"]
    #[inline(always)]
    pub fn int61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 62"]
    #[inline(always)]
    pub fn int62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt 63"]
    #[inline(always)]
    pub fn int63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Disable2> for Disable2T {
    #[inline(always)]
    fn reset_value(&self) -> Disable2 {
        Disable2::new(0)
    }
}

#[doc = "Disable basic interrupts"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DisableBasic {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DisableBasic {
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
pub struct DisableBasicT;
unsafe impl crate::common::AsPtr for DisableBasicT {}
impl crate::common::Reg<DisableBasic> for DisableBasicT {}

unsafe impl crate::common::Read<DisableBasic> for DisableBasicT {}
unsafe impl crate::common::Write<DisableBasic> for DisableBasicT {}
impl DisableBasic {
    #[doc = "ARMC Timer"]
    #[inline(always)]
    pub fn timer(self) -> crate::common::RegisterFieldBool<0, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Mailbox"]
    #[inline(always)]
    pub fn mailbox(self) -> crate::common::RegisterFieldBool<1, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(self) -> crate::common::RegisterFieldBool<2, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(self) -> crate::common::RegisterFieldBool<3, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, DisableBasic, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, DisableBasic, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<DisableBasic> for DisableBasicT {
    #[inline(always)]
    fn reset_value(&self) -> DisableBasic {
        DisableBasic::new(0)
    }
}
