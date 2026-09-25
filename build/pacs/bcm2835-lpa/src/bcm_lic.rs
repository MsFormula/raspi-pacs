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
// Generated from SVD A, with svd2pac 0.8.0 on Fri, 25 Sep 2026 21:30:43 +0000

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

    #[doc = "JPEG"]
    #[inline(always)]
    pub fn jpeg(self) -> crate::common::RegisterFieldBool<10, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> crate::common::RegisterFieldBool<11, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "V3D"]
    #[inline(always)]
    pub fn v3d(self) -> crate::common::RegisterFieldBool<12, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn dma_2(self) -> crate::common::RegisterFieldBool<13, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn dma_3(self) -> crate::common::RegisterFieldBool<14, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn i2c(self) -> crate::common::RegisterFieldBool<15, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn spi(self) -> crate::common::RegisterFieldBool<16, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(self) -> crate::common::RegisterFieldBool<17, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn sdhost(self) -> crate::common::RegisterFieldBool<18, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(self) -> crate::common::RegisterFieldBool<19, 1, 0, BasicPending, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, BasicPending, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(self) -> crate::common::RegisterFieldBool<20, 1, 0, BasicPending, common::R> {
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
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer_0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer_3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "H264 0"]
    #[inline(always)]
    pub fn h264_0(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "H264 1"]
    #[inline(always)]
    pub fn h264_1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "H264 2"]
    #[inline(always)]
    pub fn h264_2(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "JPEG"]
    #[inline(always)]
    pub fn jpeg(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "ISP"]
    #[inline(always)]
    pub fn isp(self) -> crate::common::RegisterFieldBool<8, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> crate::common::RegisterFieldBool<9, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "V3D"]
    #[inline(always)]
    pub fn v3d(self) -> crate::common::RegisterFieldBool<10, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Transposer"]
    #[inline(always)]
    pub fn transposer(self) -> crate::common::RegisterFieldBool<11, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 0"]
    #[inline(always)]
    pub fn dma_0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 1"]
    #[inline(always)]
    pub fn dma_1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn dma_2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn dma_3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 4"]
    #[inline(always)]
    pub fn dma_4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 5"]
    #[inline(always)]
    pub fn dma_5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 6"]
    #[inline(always)]
    pub fn dma_6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(self) -> crate::common::RegisterFieldBool<23, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(self) -> crate::common::RegisterFieldBool<24, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 11"]
    #[inline(always)]
    pub fn dma_11(self) -> crate::common::RegisterFieldBool<25, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 12"]
    #[inline(always)]
    pub fn dma_12(self) -> crate::common::RegisterFieldBool<26, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 13"]
    #[inline(always)]
    pub fn dma_13(self) -> crate::common::RegisterFieldBool<27, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 14"]
    #[inline(always)]
    pub fn dma_14(self) -> crate::common::RegisterFieldBool<28, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(self) -> crate::common::RegisterFieldBool<29, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "ARM"]
    #[inline(always)]
    pub fn arm(self) -> crate::common::RegisterFieldBool<30, 1, 0, Pending1, common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pending1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA 15"]
    #[inline(always)]
    pub fn dma_15(self) -> crate::common::RegisterFieldBool<31, 1, 0, Pending1, common::R> {
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
    #[doc = "HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "HVS"]
    #[inline(always)]
    pub fn hvs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "RPIVID"]
    #[inline(always)]
    pub fn rpivid(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "SDC"]
    #[inline(always)]
    pub fn sdc(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "DSI 0"]
    #[inline(always)]
    pub fn dsi_0(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Camera 0"]
    #[inline(always)]
    pub fn camera_0(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Camera 1"]
    #[inline(always)]
    pub fn camera_1(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(self) -> crate::common::RegisterFieldBool<10, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(self) -> crate::common::RegisterFieldBool<11, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "DSI 1"]
    #[inline(always)]
    pub fn dsi_1(self) -> crate::common::RegisterFieldBool<12, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(self) -> crate::common::RegisterFieldBool<13, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "CPR"]
    #[inline(always)]
    pub fn cpr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "SMI"]
    #[inline(always)]
    pub fn smi(self) -> crate::common::RegisterFieldBool<16, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(self) -> crate::common::RegisterFieldBool<17, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(self) -> crate::common::RegisterFieldBool<18, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(self) -> crate::common::RegisterFieldBool<19, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(self) -> crate::common::RegisterFieldBool<20, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn i2c(self) -> crate::common::RegisterFieldBool<21, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn spi(self) -> crate::common::RegisterFieldBool<22, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(self) -> crate::common::RegisterFieldBool<23, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn sdhost(self) -> crate::common::RegisterFieldBool<24, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(self) -> crate::common::RegisterFieldBool<25, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(self) -> crate::common::RegisterFieldBool<26, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "VEC"]
    #[inline(always)]
    pub fn vec(self) -> crate::common::RegisterFieldBool<27, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "CPG"]
    #[inline(always)]
    pub fn cpg(self) -> crate::common::RegisterFieldBool<28, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "RNG"]
    #[inline(always)]
    pub fn rng(self) -> crate::common::RegisterFieldBool<29, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(self) -> crate::common::RegisterFieldBool<30, 1, 0, Pending2, common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pending2, common::R>::from_register(self, 0)
    }

    #[doc = "ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pending2, common::R> {
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
        #[doc = "Timer 0"]
        pub const TIMER_0: Self = Self(0);

        #[doc = "Timer 1"]
        pub const TIMER_1: Self = Self(1);

        #[doc = "Timer 2"]
        pub const TIMER_2: Self = Self(2);

        #[doc = "Timer 3"]
        pub const TIMER_3: Self = Self(3);

        #[doc = "H264 0"]
        pub const H_264_0: Self = Self(4);

        #[doc = "H264 1"]
        pub const H_264_1: Self = Self(5);

        #[doc = "H264 2"]
        pub const H_264_2: Self = Self(6);

        #[doc = "JPEG"]
        pub const JPEG: Self = Self(7);

        #[doc = "ISP"]
        pub const ISP: Self = Self(8);

        #[doc = "USB"]
        pub const USB: Self = Self(9);

        #[doc = "V3D"]
        pub const V_3_D: Self = Self(10);

        #[doc = "Transposer"]
        pub const TRANSPOSER: Self = Self(11);

        #[doc = "Multicore Sync 0"]
        pub const MULTICORE_SYNC_0: Self = Self(12);

        #[doc = "Multicore Sync 1"]
        pub const MULTICORE_SYNC_1: Self = Self(13);

        #[doc = "Multicore Sync 2"]
        pub const MULTICORE_SYNC_2: Self = Self(14);

        #[doc = "Multicore Sync 3"]
        pub const MULTICORE_SYNC_3: Self = Self(15);

        #[doc = "DMA 0"]
        pub const DMA_0: Self = Self(16);

        #[doc = "DMA 1"]
        pub const DMA_1: Self = Self(17);

        #[doc = "DMA 2"]
        pub const DMA_2: Self = Self(18);

        #[doc = "DMA 3"]
        pub const DMA_3: Self = Self(19);

        #[doc = "DMA 4"]
        pub const DMA_4: Self = Self(20);

        #[doc = "DMA 5"]
        pub const DMA_5: Self = Self(21);

        #[doc = "DMA 6"]
        pub const DMA_6: Self = Self(22);

        #[doc = "OR of DMA 7 and 8"]
        pub const DMA_7_8: Self = Self(23);

        #[doc = "OR of DMA 9 and 10"]
        pub const DMA_9_10: Self = Self(24);

        #[doc = "DMA 11"]
        pub const DMA_11: Self = Self(25);

        #[doc = "DMA 12"]
        pub const DMA_12: Self = Self(26);

        #[doc = "DMA 13"]
        pub const DMA_13: Self = Self(27);

        #[doc = "DMA 14"]
        pub const DMA_14: Self = Self(28);

        #[doc = "OR of UART1, SPI1 and SPI2"]
        pub const AUX: Self = Self(29);

        #[doc = "ARM"]
        pub const ARM: Self = Self(30);

        #[doc = "DMA 15"]
        pub const DMA_15: Self = Self(31);

        #[doc = "HDMI CEC"]
        pub const HDMI_CEC: Self = Self(32);

        #[doc = "HVS"]
        pub const HVS: Self = Self(33);

        #[doc = "RPIVID"]
        pub const RPIVID: Self = Self(34);

        #[doc = "SDC"]
        pub const SDC: Self = Self(35);

        #[doc = "DSI 0"]
        pub const DSI_0: Self = Self(36);

        #[doc = "Pixel Valve 2"]
        pub const PIXEL_VALVE_2: Self = Self(37);

        #[doc = "Camera 0"]
        pub const CAMERA_0: Self = Self(38);

        #[doc = "Camera 1"]
        pub const CAMERA_1: Self = Self(39);

        #[doc = "HDMI 0"]
        pub const HDMI_0: Self = Self(40);

        #[doc = "HDMI 1"]
        pub const HDMI_1: Self = Self(41);

        #[doc = "Pixel Valve 3"]
        pub const PIXEL_VALVE_3: Self = Self(42);

        #[doc = "SPI/BSC Slave"]
        pub const SPI_BSC_SLAVE: Self = Self(43);

        #[doc = "DSI 1"]
        pub const DSI_1: Self = Self(44);

        #[doc = "Pixel Valve 0"]
        pub const PIXEL_VALVE_0: Self = Self(45);

        #[doc = "OR of Pixel Valve 1 and 2"]
        pub const PIXEL_VALVE_1_2: Self = Self(46);

        #[doc = "CPR"]
        pub const CPR: Self = Self(47);

        #[doc = "SMI"]
        pub const SMI: Self = Self(48);

        #[doc = "GPIO 0"]
        pub const GPIO_0: Self = Self(49);

        #[doc = "GPIO 1"]
        pub const GPIO_1: Self = Self(50);

        #[doc = "GPIO 2"]
        pub const GPIO_2: Self = Self(51);

        #[doc = "GPIO 3"]
        pub const GPIO_3: Self = Self(52);

        #[doc = "OR of all I2C"]
        pub const I_2_C: Self = Self(53);

        #[doc = "OR of all SPI"]
        pub const SPI: Self = Self(54);

        #[doc = "PCM/I2S"]
        pub const PCM_I_2_S: Self = Self(55);

        #[doc = "SDHOST"]
        pub const SDHOST: Self = Self(56);

        #[doc = "OR of all PL011 UARTs"]
        pub const UART: Self = Self(57);

        #[doc = "OR of all ETH_PCIe L2"]
        pub const ETH_PCIE: Self = Self(58);

        #[doc = "VEC"]
        pub const VEC: Self = Self(59);

        #[doc = "CPG"]
        pub const CPG: Self = Self(60);

        #[doc = "RNG"]
        pub const RNG: Self = Self(61);

        #[doc = "OR of EMMC and EMMC2"]
        pub const EMMC: Self = Self(62);

        #[doc = "ETH_PCIe secure"]
        pub const ETH_PCIE_SECURE: Self = Self(63);

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
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer_0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer_3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "H264 0"]
    #[inline(always)]
    pub fn h264_0(self) -> crate::common::RegisterFieldBool<4, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "H264 1"]
    #[inline(always)]
    pub fn h264_1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "H264 2"]
    #[inline(always)]
    pub fn h264_2(self) -> crate::common::RegisterFieldBool<6, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "JPEG"]
    #[inline(always)]
    pub fn jpeg(self) -> crate::common::RegisterFieldBool<7, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "ISP"]
    #[inline(always)]
    pub fn isp(self) -> crate::common::RegisterFieldBool<8, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> crate::common::RegisterFieldBool<9, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "V3D"]
    #[inline(always)]
    pub fn v3d(self) -> crate::common::RegisterFieldBool<10, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Transposer"]
    #[inline(always)]
    pub fn transposer(self) -> crate::common::RegisterFieldBool<11, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 0"]
    #[inline(always)]
    pub fn dma_0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 1"]
    #[inline(always)]
    pub fn dma_1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn dma_2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn dma_3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 4"]
    #[inline(always)]
    pub fn dma_4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 5"]
    #[inline(always)]
    pub fn dma_5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 6"]
    #[inline(always)]
    pub fn dma_6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(self) -> crate::common::RegisterFieldBool<23, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(self) -> crate::common::RegisterFieldBool<24, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 11"]
    #[inline(always)]
    pub fn dma_11(self) -> crate::common::RegisterFieldBool<25, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 12"]
    #[inline(always)]
    pub fn dma_12(self) -> crate::common::RegisterFieldBool<26, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 13"]
    #[inline(always)]
    pub fn dma_13(self) -> crate::common::RegisterFieldBool<27, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 14"]
    #[inline(always)]
    pub fn dma_14(self) -> crate::common::RegisterFieldBool<28, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(self) -> crate::common::RegisterFieldBool<29, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "ARM"]
    #[inline(always)]
    pub fn arm(self) -> crate::common::RegisterFieldBool<30, 1, 0, Enable1, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Enable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 15"]
    #[inline(always)]
    pub fn dma_15(self) -> crate::common::RegisterFieldBool<31, 1, 0, Enable1, common::RW> {
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
    #[doc = "HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(self) -> crate::common::RegisterFieldBool<0, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "HVS"]
    #[inline(always)]
    pub fn hvs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "RPIVID"]
    #[inline(always)]
    pub fn rpivid(self) -> crate::common::RegisterFieldBool<2, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SDC"]
    #[inline(always)]
    pub fn sdc(self) -> crate::common::RegisterFieldBool<3, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "DSI 0"]
    #[inline(always)]
    pub fn dsi_0(self) -> crate::common::RegisterFieldBool<4, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(self) -> crate::common::RegisterFieldBool<5, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Camera 0"]
    #[inline(always)]
    pub fn camera_0(self) -> crate::common::RegisterFieldBool<6, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Camera 1"]
    #[inline(always)]
    pub fn camera_1(self) -> crate::common::RegisterFieldBool<7, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(self) -> crate::common::RegisterFieldBool<10, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(self) -> crate::common::RegisterFieldBool<11, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "DSI 1"]
    #[inline(always)]
    pub fn dsi_1(self) -> crate::common::RegisterFieldBool<12, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(self) -> crate::common::RegisterFieldBool<13, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "CPR"]
    #[inline(always)]
    pub fn cpr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SMI"]
    #[inline(always)]
    pub fn smi(self) -> crate::common::RegisterFieldBool<16, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(self) -> crate::common::RegisterFieldBool<17, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(self) -> crate::common::RegisterFieldBool<18, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(self) -> crate::common::RegisterFieldBool<19, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(self) -> crate::common::RegisterFieldBool<20, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn i2c(self) -> crate::common::RegisterFieldBool<21, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn spi(self) -> crate::common::RegisterFieldBool<22, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(self) -> crate::common::RegisterFieldBool<23, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn sdhost(self) -> crate::common::RegisterFieldBool<24, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(self) -> crate::common::RegisterFieldBool<25, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(self) -> crate::common::RegisterFieldBool<26, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "VEC"]
    #[inline(always)]
    pub fn vec(self) -> crate::common::RegisterFieldBool<27, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "CPG"]
    #[inline(always)]
    pub fn cpg(self) -> crate::common::RegisterFieldBool<28, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "RNG"]
    #[inline(always)]
    pub fn rng(self) -> crate::common::RegisterFieldBool<29, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(self) -> crate::common::RegisterFieldBool<30, 1, 0, Enable2, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Enable2, common::RW>::from_register(self, 0)
    }

    #[doc = "ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Enable2, common::RW> {
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
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer_0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer_3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "H264 0"]
    #[inline(always)]
    pub fn h264_0(self) -> crate::common::RegisterFieldBool<4, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "H264 1"]
    #[inline(always)]
    pub fn h264_1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "H264 2"]
    #[inline(always)]
    pub fn h264_2(self) -> crate::common::RegisterFieldBool<6, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "JPEG"]
    #[inline(always)]
    pub fn jpeg(self) -> crate::common::RegisterFieldBool<7, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "ISP"]
    #[inline(always)]
    pub fn isp(self) -> crate::common::RegisterFieldBool<8, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "USB"]
    #[inline(always)]
    pub fn usb(self) -> crate::common::RegisterFieldBool<9, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "V3D"]
    #[inline(always)]
    pub fn v3d(self) -> crate::common::RegisterFieldBool<10, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Transposer"]
    #[inline(always)]
    pub fn transposer(self) -> crate::common::RegisterFieldBool<11, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 0"]
    #[inline(always)]
    pub fn dma_0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 1"]
    #[inline(always)]
    pub fn dma_1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 2"]
    #[inline(always)]
    pub fn dma_2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 3"]
    #[inline(always)]
    pub fn dma_3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 4"]
    #[inline(always)]
    pub fn dma_4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 5"]
    #[inline(always)]
    pub fn dma_5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 6"]
    #[inline(always)]
    pub fn dma_6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of DMA 7 and 8"]
    #[inline(always)]
    pub fn dma_7_8(self) -> crate::common::RegisterFieldBool<23, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of DMA 9 and 10"]
    #[inline(always)]
    pub fn dma_9_10(self) -> crate::common::RegisterFieldBool<24, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 11"]
    #[inline(always)]
    pub fn dma_11(self) -> crate::common::RegisterFieldBool<25, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 12"]
    #[inline(always)]
    pub fn dma_12(self) -> crate::common::RegisterFieldBool<26, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 13"]
    #[inline(always)]
    pub fn dma_13(self) -> crate::common::RegisterFieldBool<27, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 14"]
    #[inline(always)]
    pub fn dma_14(self) -> crate::common::RegisterFieldBool<28, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of UART1, SPI1 and SPI2"]
    #[inline(always)]
    pub fn aux(self) -> crate::common::RegisterFieldBool<29, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "ARM"]
    #[inline(always)]
    pub fn arm(self) -> crate::common::RegisterFieldBool<30, 1, 0, Disable1, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Disable1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA 15"]
    #[inline(always)]
    pub fn dma_15(self) -> crate::common::RegisterFieldBool<31, 1, 0, Disable1, common::RW> {
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
    #[doc = "HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(self) -> crate::common::RegisterFieldBool<0, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "HVS"]
    #[inline(always)]
    pub fn hvs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "RPIVID"]
    #[inline(always)]
    pub fn rpivid(self) -> crate::common::RegisterFieldBool<2, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SDC"]
    #[inline(always)]
    pub fn sdc(self) -> crate::common::RegisterFieldBool<3, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "DSI 0"]
    #[inline(always)]
    pub fn dsi_0(self) -> crate::common::RegisterFieldBool<4, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 2"]
    #[inline(always)]
    pub fn pixel_valve_2(self) -> crate::common::RegisterFieldBool<5, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Camera 0"]
    #[inline(always)]
    pub fn camera_0(self) -> crate::common::RegisterFieldBool<6, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Camera 1"]
    #[inline(always)]
    pub fn camera_1(self) -> crate::common::RegisterFieldBool<7, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "HDMI 0"]
    #[inline(always)]
    pub fn hdmi_0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "HDMI 1"]
    #[inline(always)]
    pub fn hdmi_1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 3"]
    #[inline(always)]
    pub fn pixel_valve_3(self) -> crate::common::RegisterFieldBool<10, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI/BSC Slave"]
    #[inline(always)]
    pub fn spi_bsc_slave(self) -> crate::common::RegisterFieldBool<11, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "DSI 1"]
    #[inline(always)]
    pub fn dsi_1(self) -> crate::common::RegisterFieldBool<12, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "Pixel Valve 0"]
    #[inline(always)]
    pub fn pixel_valve_0(self) -> crate::common::RegisterFieldBool<13, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of Pixel Valve 1 and 2"]
    #[inline(always)]
    pub fn pixel_valve_1_2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "CPR"]
    #[inline(always)]
    pub fn cpr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SMI"]
    #[inline(always)]
    pub fn smi(self) -> crate::common::RegisterFieldBool<16, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 0"]
    #[inline(always)]
    pub fn gpio_0(self) -> crate::common::RegisterFieldBool<17, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 1"]
    #[inline(always)]
    pub fn gpio_1(self) -> crate::common::RegisterFieldBool<18, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 2"]
    #[inline(always)]
    pub fn gpio_2(self) -> crate::common::RegisterFieldBool<19, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "GPIO 3"]
    #[inline(always)]
    pub fn gpio_3(self) -> crate::common::RegisterFieldBool<20, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all I2C"]
    #[inline(always)]
    pub fn i2c(self) -> crate::common::RegisterFieldBool<21, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all SPI"]
    #[inline(always)]
    pub fn spi(self) -> crate::common::RegisterFieldBool<22, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "PCM/I2S"]
    #[inline(always)]
    pub fn pcm_i2s(self) -> crate::common::RegisterFieldBool<23, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "SDHOST"]
    #[inline(always)]
    pub fn sdhost(self) -> crate::common::RegisterFieldBool<24, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(self) -> crate::common::RegisterFieldBool<25, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(self) -> crate::common::RegisterFieldBool<26, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "VEC"]
    #[inline(always)]
    pub fn vec(self) -> crate::common::RegisterFieldBool<27, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "CPG"]
    #[inline(always)]
    pub fn cpg(self) -> crate::common::RegisterFieldBool<28, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "RNG"]
    #[inline(always)]
    pub fn rng(self) -> crate::common::RegisterFieldBool<29, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(self) -> crate::common::RegisterFieldBool<30, 1, 0, Disable2, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Disable2, common::RW>::from_register(self, 0)
    }

    #[doc = "ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Disable2, common::RW> {
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
