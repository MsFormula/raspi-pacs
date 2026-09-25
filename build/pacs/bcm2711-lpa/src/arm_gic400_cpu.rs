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
#[doc = r"ARM GIC-400 Generic Interrupt Controller CPU Interface"]
unsafe impl ::core::marker::Send for super::ArmGic400Cpu {}
unsafe impl ::core::marker::Sync for super::ArmGic400Cpu {}
impl super::ArmGic400Cpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CPU Interface Control"]
    #[inline(always)]
    pub fn gicc_ctlr(&self) -> &'static self::GiccCtlrT {
        unsafe { self::GiccCtlrT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Interrupt Priority Mask"]
    #[inline(always)]
    pub fn gicc_pmr(&self) -> &'static self::GiccPmrT {
        unsafe { self::GiccPmrT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Binary Point"]
    #[inline(always)]
    pub fn gicc_bpr(&self) -> &'static self::GiccBprT {
        unsafe { self::GiccBprT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Interrupt Acknowledge"]
    #[inline(always)]
    pub fn gicc_iar(&self) -> &'static self::GiccIarT {
        unsafe { self::GiccIarT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "End of Interrupt"]
    #[inline(always)]
    pub fn gicc_eoir(&self) -> &'static self::GiccEoirT {
        unsafe { self::GiccEoirT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "Running Priority"]
    #[inline(always)]
    pub fn gicc_rpr(&self) -> &'static self::GiccRprT {
        unsafe { self::GiccRprT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "Highest Priority Pending Interrupt"]
    #[inline(always)]
    pub fn gicc_hppir(&self) -> &'static self::GiccHppirT {
        unsafe { self::GiccHppirT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "Aliased Binary Point"]
    #[inline(always)]
    pub fn gicc_abpr(&self) -> &'static self::GiccAbprT {
        unsafe { self::GiccAbprT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "Aliased Interrupt Acknowledge"]
    #[inline(always)]
    pub fn gicc_aiar(&self) -> &'static self::GiccAiarT {
        unsafe { self::GiccAiarT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "Aliased End of Interrupt"]
    #[inline(always)]
    pub fn gicc_aeoir(&self) -> &'static self::GiccAeoirT {
        unsafe { self::GiccAeoirT::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }

    #[doc = "Aliased Highest Priority Pending Interrupt"]
    #[inline(always)]
    pub fn gicc_ahppir(&self) -> &'static self::GiccAhppirT {
        unsafe { self::GiccAhppirT::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "Active Priority"]
    #[inline(always)]
    pub fn gicc_apr0(&self) -> &'static self::GiccApr0T {
        unsafe { self::GiccApr0T::from_ptr(self._svd2pac_as_ptr().add(208usize)) }
    }

    #[doc = "Non-Secure Active Priority"]
    #[inline(always)]
    pub fn gicc_nsapr0(&self) -> &'static self::GiccNsapr0T {
        unsafe { self::GiccNsapr0T::from_ptr(self._svd2pac_as_ptr().add(224usize)) }
    }

    #[doc = "CPU Interface Identification Register"]
    #[inline(always)]
    pub fn gicc_iidr(&self) -> &'static self::GiccIidrT {
        unsafe { self::GiccIidrT::from_ptr(self._svd2pac_as_ptr().add(252usize)) }
    }

    #[doc = "Deactivate Interrupt"]
    #[inline(always)]
    pub fn gicc_dir(&self) -> &'static self::GiccDirT {
        unsafe { self::GiccDirT::from_ptr(self._svd2pac_as_ptr().add(4096usize)) }
    }
}

#[doc = "CPU Interface Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccCtlr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccCtlr {
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
pub struct GiccCtlrT;
unsafe impl crate::common::AsPtr for GiccCtlrT {}
impl crate::common::Reg<GiccCtlr> for GiccCtlrT {}

unsafe impl crate::common::Read<GiccCtlr> for GiccCtlrT {}
unsafe impl crate::common::Write<GiccCtlr> for GiccCtlrT {}
impl GiccCtlr {
    #[doc = "Enable signaling of group 0"]
    #[inline(always)]
    pub fn enable_group_0(self) -> crate::common::RegisterFieldBool<0, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable signaling of group 1"]
    #[inline(always)]
    pub fn enable_group_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Whether a read of IAR acknowledges the interrupt"]
    #[inline(always)]
    pub fn ackctl(self) -> crate::common::RegisterFieldBool<2, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Group 0 triggers FIQ"]
    #[inline(always)]
    pub fn fiqen(self) -> crate::common::RegisterFieldBool<3, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Common control of interrupts through GICC_BPR"]
    #[inline(always)]
    pub fn cbpr(self) -> crate::common::RegisterFieldBool<4, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Bypass FIQ is not signaled to processor"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(self) -> crate::common::RegisterFieldBool<5, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Bypass IRQ is not signaled to processor"]
    #[inline(always)]
    pub fn irqbypdisgrp0(self) -> crate::common::RegisterFieldBool<6, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Alias of group 1 FIQ bypass disable"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(self) -> crate::common::RegisterFieldBool<7, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Alias of group 1 IRQ bypass disable"]
    #[inline(always)]
    pub fn irqbypdisgrp1(self) -> crate::common::RegisterFieldBool<8, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    pub fn eoimodes(self) -> crate::common::RegisterFieldBool<9, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }

    #[doc = "Non-Secure EOIR does priority drop. DIR does deactivate."]
    #[inline(always)]
    pub fn eoimodens(self) -> crate::common::RegisterFieldBool<10, 1, 0, GiccCtlr, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, GiccCtlr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<GiccCtlr> for GiccCtlrT {
    #[inline(always)]
    fn reset_value(&self) -> GiccCtlr {
        GiccCtlr::new(0)
    }
}

#[doc = "Interrupt Priority Mask"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccPmr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccPmr {
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
pub struct GiccPmrT;
unsafe impl crate::common::AsPtr for GiccPmrT {}
impl crate::common::Reg<GiccPmr> for GiccPmrT {}

unsafe impl crate::common::Read<GiccPmr> for GiccPmrT {}
unsafe impl crate::common::Write<GiccPmr> for GiccPmrT {}
impl GiccPmr {
    #[doc = "Interrupts with a higher number are not signaled"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GiccPmr, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, GiccPmr, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GiccPmr> for GiccPmrT {
    #[inline(always)]
    fn reset_value(&self) -> GiccPmr {
        GiccPmr::new(0)
    }
}

#[doc = "Binary Point"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccBpr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccBpr {
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
pub struct GiccBprT;
unsafe impl crate::common::AsPtr for GiccBprT {}
impl crate::common::Reg<GiccBpr> for GiccBprT {}

unsafe impl crate::common::Read<GiccBpr> for GiccBprT {}
unsafe impl crate::common::Write<GiccBpr> for GiccBprT {}
impl GiccBpr {
    #[doc = "Split point between group priority and subpriority"]
    #[inline(always)]
    pub fn binary_point(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, GiccBpr, common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, GiccBpr, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GiccBpr> for GiccBprT {
    #[inline(always)]
    fn reset_value(&self) -> GiccBpr {
        GiccBpr::new(0)
    }
}

#[doc = "Interrupt Acknowledge"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccIar {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccIar {
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
pub struct GiccIarT;
unsafe impl crate::common::AsPtr for GiccIarT {}
impl crate::common::Reg<GiccIar> for GiccIarT {}

unsafe impl crate::common::Read<GiccIar> for GiccIarT {}
impl GiccIar {
    #[doc = "CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(self) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, GiccIar, common::R> {
        crate::common::RegisterField::<10, 0x7, 1, 0, u8, u8, GiccIar, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GiccIar, common::R> {
        crate::common::RegisterField::<0, 0x3ff, 1, 0, u16, u16, GiccIar, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GiccIar> for GiccIarT {
    #[inline(always)]
    fn reset_value(&self) -> GiccIar {
        GiccIar::new(0)
    }
}

#[doc = "End of Interrupt"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccEoir {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccEoir {
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
pub struct GiccEoirT;
unsafe impl crate::common::AsPtr for GiccEoirT {}
impl crate::common::Reg<GiccEoir> for GiccEoirT {}

unsafe impl crate::common::Write<GiccEoir> for GiccEoirT {}
impl GiccEoir {
    #[doc = "CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(self) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, GiccEoir, common::W> {
        crate::common::RegisterField::<10, 0x7, 1, 0, u8, u8, GiccEoir, common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GiccEoir, common::W> {
        crate::common::RegisterField::<0, 0x3ff, 1, 0, u16, u16, GiccEoir, common::W>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GiccEoir> for GiccEoirT {
    #[inline(always)]
    fn reset_value(&self) -> GiccEoir {
        GiccEoir::new(0)
    }
}

#[doc = "Running Priority"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccRpr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccRpr {
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
pub struct GiccRprT;
unsafe impl crate::common::AsPtr for GiccRprT {}
impl crate::common::Reg<GiccRpr> for GiccRprT {}

unsafe impl crate::common::Read<GiccRpr> for GiccRprT {}
impl GiccRpr {
    #[doc = "Current running priority"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, GiccRpr, common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, GiccRpr, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GiccRpr> for GiccRprT {
    #[inline(always)]
    fn reset_value(&self) -> GiccRpr {
        GiccRpr::new(0)
    }
}

#[doc = "Highest Priority Pending Interrupt"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccHppir {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccHppir {
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
pub struct GiccHppirT;
unsafe impl crate::common::AsPtr for GiccHppirT {}
impl crate::common::Reg<GiccHppir> for GiccHppirT {}

unsafe impl crate::common::Read<GiccHppir> for GiccHppirT {}
unsafe impl crate::common::Write<GiccHppir> for GiccHppirT {}
impl GiccHppir {
    #[doc = "CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, GiccHppir, common::RW> {
        crate::common::RegisterField::<10, 0x7, 1, 0, u8, u8, GiccHppir, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Pending Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GiccHppir, common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GiccHppir,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<GiccHppir> for GiccHppirT {
    #[inline(always)]
    fn reset_value(&self) -> GiccHppir {
        GiccHppir::new(0)
    }
}

#[doc = "Aliased Binary Point"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAbpr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccAbpr {
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
pub struct GiccAbprT;
unsafe impl crate::common::AsPtr for GiccAbprT {}
impl crate::common::Reg<GiccAbpr> for GiccAbprT {}

unsafe impl crate::common::Read<GiccAbpr> for GiccAbprT {}
unsafe impl crate::common::Write<GiccAbpr> for GiccAbprT {}
impl GiccAbpr {
    #[doc = "Split point between group priority and subpriority"]
    #[inline(always)]
    pub fn binary_point(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, GiccAbpr, common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, GiccAbpr, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GiccAbpr> for GiccAbprT {
    #[inline(always)]
    fn reset_value(&self) -> GiccAbpr {
        GiccAbpr::new(0)
    }
}

#[doc = "Aliased Interrupt Acknowledge"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAiar {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccAiar {
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
pub struct GiccAiarT;
unsafe impl crate::common::AsPtr for GiccAiarT {}
impl crate::common::Reg<GiccAiar> for GiccAiarT {}

unsafe impl crate::common::Read<GiccAiar> for GiccAiarT {}
impl GiccAiar {
    #[doc = "CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(self) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, GiccAiar, common::R> {
        crate::common::RegisterField::<10, 0x7, 1, 0, u8, u8, GiccAiar, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GiccAiar, common::R> {
        crate::common::RegisterField::<0, 0x3ff, 1, 0, u16, u16, GiccAiar, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GiccAiar> for GiccAiarT {
    #[inline(always)]
    fn reset_value(&self) -> GiccAiar {
        GiccAiar::new(0)
    }
}

#[doc = "Aliased End of Interrupt"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAeoir {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccAeoir {
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
pub struct GiccAeoirT;
unsafe impl crate::common::AsPtr for GiccAeoirT {}
impl crate::common::Reg<GiccAeoir> for GiccAeoirT {}

unsafe impl crate::common::Write<GiccAeoir> for GiccAeoirT {}
impl GiccAeoir {
    #[doc = "CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, GiccAeoir, common::W> {
        crate::common::RegisterField::<10, 0x7, 1, 0, u8, u8, GiccAeoir, common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GiccAeoir, common::W> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GiccAeoir,common::W>::from_register(self,0)
    }
}
impl crate::common::ResetValue<GiccAeoir> for GiccAeoirT {
    #[inline(always)]
    fn reset_value(&self) -> GiccAeoir {
        GiccAeoir::new(0)
    }
}

#[doc = "Aliased Highest Priority Pending Interrupt"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAhppir {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccAhppir {
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
pub struct GiccAhppirT;
unsafe impl crate::common::AsPtr for GiccAhppirT {}
impl crate::common::Reg<GiccAhppir> for GiccAhppirT {}

unsafe impl crate::common::Read<GiccAhppir> for GiccAhppirT {}
impl GiccAhppir {
    #[doc = "CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, GiccAhppir, common::R> {
        crate::common::RegisterField::<10, 0x7, 1, 0, u8, u8, GiccAhppir, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Pending Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, GiccAhppir, common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,GiccAhppir,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<GiccAhppir> for GiccAhppirT {
    #[inline(always)]
    fn reset_value(&self) -> GiccAhppir {
        GiccAhppir::new(0)
    }
}

#[doc = "Active Priority"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccApr0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccApr0 {
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
pub struct GiccApr0T;
unsafe impl crate::common::AsPtr for GiccApr0T {}
impl crate::common::Reg<GiccApr0> for GiccApr0T {}

unsafe impl crate::common::Read<GiccApr0> for GiccApr0T {}
unsafe impl crate::common::Write<GiccApr0> for GiccApr0T {}

impl crate::common::NoBitfieldReg for GiccApr0 {}
impl crate::common::ResetValue<GiccApr0> for GiccApr0T {
    #[inline(always)]
    fn reset_value(&self) -> GiccApr0 {
        GiccApr0::new(0)
    }
}

#[doc = "Non-Secure Active Priority"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccNsapr0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccNsapr0 {
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
pub struct GiccNsapr0T;
unsafe impl crate::common::AsPtr for GiccNsapr0T {}
impl crate::common::Reg<GiccNsapr0> for GiccNsapr0T {}

unsafe impl crate::common::Read<GiccNsapr0> for GiccNsapr0T {}
unsafe impl crate::common::Write<GiccNsapr0> for GiccNsapr0T {}

impl crate::common::NoBitfieldReg for GiccNsapr0 {}
impl crate::common::ResetValue<GiccNsapr0> for GiccNsapr0T {
    #[inline(always)]
    fn reset_value(&self) -> GiccNsapr0 {
        GiccNsapr0::new(0)
    }
}

#[doc = "CPU Interface Identification Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccIidr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccIidr {
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
pub struct GiccIidrT;
unsafe impl crate::common::AsPtr for GiccIidrT {}
impl crate::common::Reg<GiccIidr> for GiccIidrT {}

unsafe impl crate::common::Read<GiccIidr> for GiccIidrT {}
unsafe impl crate::common::Write<GiccIidr> for GiccIidrT {}
impl GiccIidr {
    #[doc = "ID"]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        gicc_iidr::Id,
        gicc_iidr::Id,
        GiccIidr,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            gicc_iidr::Id,
            gicc_iidr::Id,
            GiccIidr,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<GiccIidr> for GiccIidrT {
    #[inline(always)]
    fn reset_value(&self) -> GiccIidr {
        GiccIidr::new(33690683)
    }
}
pub mod gicc_iidr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Id(u32);

    impl Id {
        pub fn new(value: u32) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Id {
        type RegNumberT = u32;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u32> for Id {
        #[inline(always)]
        fn from(value: u32) -> Self {
            Self(value)
        }
    }

    impl From<Id> for u64 {
        #[inline(always)]
        fn from(value: Id) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Id {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u32::cast_from(val))
        }
    }

    impl Id {
        #[doc = "ID is valid"]
        pub const VALID: Self = Self(33690683);
    }
}

#[doc = "Deactivate Interrupt"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccDir {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GiccDir {
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
pub struct GiccDirT;
unsafe impl crate::common::AsPtr for GiccDirT {}
impl crate::common::Reg<GiccDir> for GiccDirT {}

unsafe impl crate::common::Write<GiccDir> for GiccDirT {}

impl crate::common::NoBitfieldReg for GiccDir {}
impl crate::common::ResetValue<GiccDir> for GiccDirT {
    #[inline(always)]
    fn reset_value(&self) -> GiccDir {
        GiccDir::new(0)
    }
}
