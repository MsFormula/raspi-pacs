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
#[doc = r""]
unsafe impl ::core::marker::Send for super::Pcm {}
unsafe impl ::core::marker::Sync for super::Pcm {}
impl super::Pcm {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "PCM Control and Status"]
    #[inline(always)]
    pub fn cs_a(&self) -> &'static self::CsAT {
        unsafe { self::CsAT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "This is the FIFO port of the PCM"]
    #[inline(always)]
    pub fn fifo_a(&self) -> &'static self::FifoAT {
        unsafe { self::FifoAT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "This register defines the basic PCM Operating Mode"]
    #[inline(always)]
    pub fn mode_a(&self) -> &'static self::ModeAT {
        unsafe { self::ModeAT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Sets the Channel configurations for Receiving"]
    #[inline(always)]
    pub fn rxc_a(&self) -> &'static self::RxcAT {
        unsafe { self::RxcAT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "Sets the Channel configurations for Transmitting"]
    #[inline(always)]
    pub fn txc_a(&self) -> &'static self::TxcAT {
        unsafe { self::TxcAT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "Set the DMA DREQ and Panic thresholds"]
    #[inline(always)]
    pub fn dreq_a(&self) -> &'static self::DreqAT {
        unsafe { self::DreqAT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "Set the reasons for generating an Interrupt"]
    #[inline(always)]
    pub fn inten_a(&self) -> &'static self::IntenAT {
        unsafe { self::IntenAT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "This register is used to read and clear the PCM interrupt status"]
    #[inline(always)]
    pub fn intstc_a(&self) -> &'static self::IntstcAT {
        unsafe { self::IntstcAT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "This register is used to control the gray mode generation"]
    #[inline(always)]
    pub fn gray(&self) -> &'static self::GrayT {
        unsafe { self::GrayT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }
}

#[doc = "PCM Control and Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for CsA {
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
pub struct CsAT;
unsafe impl crate::common::AsPtr for CsAT {}
impl crate::common::Reg<CsA> for CsAT {}

unsafe impl crate::common::Read<CsA> for CsAT {}
unsafe impl crate::common::Write<CsA> for CsAT {}
impl CsA {
    #[doc = "Enable the PCM Audio Interface"]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable reception"]
    #[inline(always)]
    pub fn rxon(self) -> crate::common::RegisterFieldBool<1, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable transmission"]
    #[inline(always)]
    pub fn txon(self) -> crate::common::RegisterFieldBool<2, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "Clear the TX FIFO"]
    #[inline(always)]
    pub fn txclr(self) -> crate::common::RegisterFieldBool<3, 1, 0, CsA, common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, CsA, common::W>::from_register(self, 0)
    }

    #[doc = "Clear the RX FIFO"]
    #[inline(always)]
    pub fn rxclr(self) -> crate::common::RegisterFieldBool<4, 1, 0, CsA, common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, CsA, common::W>::from_register(self, 0)
    }

    #[doc = "Sets the TX FIFO threshold at which point the TXW flag is set"]
    #[inline(always)]
    pub fn txthr(self) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CsA, common::RW> {
        crate::common::RegisterField::<5, 0x3, 1, 0, u8, u8, CsA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Sets the RX FIFO threshold at which point the RXR flag is set"]
    #[inline(always)]
    pub fn rxthr(self) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, CsA, common::RW> {
        crate::common::RegisterField::<7, 0x3, 1, 0, u8, u8, CsA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA DREQ Enable"]
    #[inline(always)]
    pub fn dmaen(self) -> crate::common::RegisterFieldBool<9, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "TX FIFO Sync"]
    #[inline(always)]
    pub fn txsync(self) -> crate::common::RegisterFieldBool<13, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "RX FIFO Sync"]
    #[inline(always)]
    pub fn rxsync(self) -> crate::common::RegisterFieldBool<14, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "TX FIFO Error"]
    #[inline(always)]
    pub fn txerr(self) -> crate::common::RegisterFieldBool<15, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "RX FIFO Error"]
    #[inline(always)]
    pub fn rxerr(self) -> crate::common::RegisterFieldBool<16, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "Indicates that the TX FIFO needs Writing"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<17, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "Indicates that the RX FIFO needs Reading"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<18, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "Indicates that the TX FIFO can accept data"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::RegisterFieldBool<19, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "Indicates that the RX FIFO contains data"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::RegisterFieldBool<20, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "TX FIFO is Empty"]
    #[inline(always)]
    pub fn txe(self) -> crate::common::RegisterFieldBool<21, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "RX FIFO is Full"]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<22, 1, 0, CsA, common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, CsA, common::R>::from_register(self, 0)
    }

    #[doc = "RX Sign Extend"]
    #[inline(always)]
    pub fn rxsex(self) -> crate::common::RegisterFieldBool<23, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "PCM Clock sync helper"]
    #[inline(always)]
    pub fn sync(self) -> crate::common::RegisterFieldBool<24, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, CsA, common::RW>::from_register(self, 0)
    }

    #[doc = "RAM Standby"]
    #[inline(always)]
    pub fn stby(self) -> crate::common::RegisterFieldBool<25, 1, 0, CsA, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, CsA, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<CsA> for CsAT {
    #[inline(always)]
    fn reset_value(&self) -> CsA {
        CsA::new(2752512)
    }
}

#[doc = "This is the FIFO port of the PCM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for FifoA {
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
pub struct FifoAT;
unsafe impl crate::common::AsPtr for FifoAT {}
impl crate::common::Reg<FifoA> for FifoAT {}

unsafe impl crate::common::Read<FifoA> for FifoAT {}
unsafe impl crate::common::Write<FifoA> for FifoAT {}
impl FifoA {
    #[doc = "FIFO"]
    #[inline(always)]
    pub fn fifo(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, FifoA, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,FifoA,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<FifoA> for FifoAT {
    #[inline(always)]
    fn reset_value(&self) -> FifoA {
        FifoA::new(0)
    }
}

#[doc = "This register defines the basic PCM Operating Mode"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModeA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ModeA {
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
pub struct ModeAT;
unsafe impl crate::common::AsPtr for ModeAT {}
impl crate::common::Reg<ModeA> for ModeAT {}

unsafe impl crate::common::Read<ModeA> for ModeAT {}
unsafe impl crate::common::Write<ModeA> for ModeAT {}
impl ModeA {
    #[doc = "Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, ModeA, common::RW> {
        crate::common::RegisterField::<0, 0x3ff, 1, 0, u16, u16, ModeA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Length"]
    #[inline(always)]
    pub fn flen(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, ModeA, common::RW> {
        crate::common::RegisterField::<10, 0x3ff, 1, 0, u16, u16, ModeA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Sync Invert This logically inverts the frame sync signal"]
    #[inline(always)]
    pub fn fsi(self) -> crate::common::RegisterFieldBool<20, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "Frame Sync Mode"]
    #[inline(always)]
    pub fn fsm(self) -> crate::common::RegisterFieldBool<21, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "Clock Invert this logically inverts the PCM_CLK signal"]
    #[inline(always)]
    pub fn clki(self) -> crate::common::RegisterFieldBool<22, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "PCM Clock Mode"]
    #[inline(always)]
    pub fn clkm(self) -> crate::common::RegisterFieldBool<23, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "Transmit Frame Packed Mode"]
    #[inline(always)]
    pub fn ftxp(self) -> crate::common::RegisterFieldBool<24, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive Frame Packed Mode"]
    #[inline(always)]
    pub fn frxp(self) -> crate::common::RegisterFieldBool<25, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "PDM Input Mode Enable"]
    #[inline(always)]
    pub fn pdme(self) -> crate::common::RegisterFieldBool<26, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "PDM Decimation Factor (N)"]
    #[inline(always)]
    pub fn pdmn(self) -> crate::common::RegisterFieldBool<27, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }

    #[doc = "PCM Clock Disable"]
    #[inline(always)]
    pub fn clk_dis(self) -> crate::common::RegisterFieldBool<28, 1, 0, ModeA, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, ModeA, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<ModeA> for ModeAT {
    #[inline(always)]
    fn reset_value(&self) -> ModeA {
        ModeA::new(0)
    }
}

#[doc = "Sets the Channel configurations for Receiving"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxcA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for RxcA {
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
pub struct RxcAT;
unsafe impl crate::common::AsPtr for RxcAT {}
impl crate::common::Reg<RxcA> for RxcAT {}

unsafe impl crate::common::Read<RxcA> for RxcAT {}
unsafe impl crate::common::Write<RxcA> for RxcAT {}
impl RxcA {
    #[doc = "Channel 2 Width"]
    #[inline(always)]
    pub fn ch2wid(self) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RxcA, common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, u8, RxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 2 Position"]
    #[inline(always)]
    pub fn ch2pos(
        self,
    ) -> crate::common::RegisterField<4, 0x3ff, 1, 0, u16, u16, RxcA, common::RW> {
        crate::common::RegisterField::<4, 0x3ff, 1, 0, u16, u16, RxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 2 Enable"]
    #[inline(always)]
    pub fn ch2en(self) -> crate::common::RegisterFieldBool<14, 1, 0, RxcA, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, RxcA, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 2 Width Extension Bit"]
    #[inline(always)]
    pub fn ch2wex(self) -> crate::common::RegisterFieldBool<15, 1, 0, RxcA, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, RxcA, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 Width"]
    #[inline(always)]
    pub fn ch1wid(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, RxcA, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, RxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Position"]
    #[inline(always)]
    pub fn ch1pos(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, RxcA, common::RW> {
        crate::common::RegisterField::<20, 0x3ff, 1, 0, u16, u16, RxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(self) -> crate::common::RegisterFieldBool<30, 1, 0, RxcA, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, RxcA, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 Width Extension Bit"]
    #[inline(always)]
    pub fn ch1wex(self) -> crate::common::RegisterFieldBool<31, 1, 0, RxcA, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, RxcA, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<RxcA> for RxcAT {
    #[inline(always)]
    fn reset_value(&self) -> RxcA {
        RxcA::new(0)
    }
}

#[doc = "Sets the Channel configurations for Transmitting"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxcA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxcA {
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
pub struct TxcAT;
unsafe impl crate::common::AsPtr for TxcAT {}
impl crate::common::Reg<TxcA> for TxcAT {}

unsafe impl crate::common::Read<TxcA> for TxcAT {}
unsafe impl crate::common::Write<TxcA> for TxcAT {}
impl TxcA {
    #[doc = "Channel 2 Width"]
    #[inline(always)]
    pub fn ch2wid(self) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, TxcA, common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, u8, TxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 2 Position"]
    #[inline(always)]
    pub fn ch2pos(
        self,
    ) -> crate::common::RegisterField<4, 0x3ff, 1, 0, u16, u16, TxcA, common::RW> {
        crate::common::RegisterField::<4, 0x3ff, 1, 0, u16, u16, TxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 2 Enable"]
    #[inline(always)]
    pub fn ch2en(self) -> crate::common::RegisterFieldBool<14, 1, 0, TxcA, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, TxcA, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 2 Width Extension Bit"]
    #[inline(always)]
    pub fn ch2wex(self) -> crate::common::RegisterFieldBool<15, 1, 0, TxcA, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, TxcA, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 Width"]
    #[inline(always)]
    pub fn ch1wid(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, TxcA, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, TxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Position"]
    #[inline(always)]
    pub fn ch1pos(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, TxcA, common::RW> {
        crate::common::RegisterField::<20, 0x3ff, 1, 0, u16, u16, TxcA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(self) -> crate::common::RegisterFieldBool<30, 1, 0, TxcA, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, TxcA, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 Width Extension Bit"]
    #[inline(always)]
    pub fn ch1wex(self) -> crate::common::RegisterFieldBool<31, 1, 0, TxcA, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, TxcA, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<TxcA> for TxcAT {
    #[inline(always)]
    fn reset_value(&self) -> TxcA {
        TxcA::new(0)
    }
}

#[doc = "Set the DMA DREQ and Panic thresholds"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DreqA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DreqA {
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
pub struct DreqAT;
unsafe impl crate::common::AsPtr for DreqAT {}
impl crate::common::Reg<DreqA> for DreqAT {}

unsafe impl crate::common::Read<DreqA> for DreqAT {}
unsafe impl crate::common::Write<DreqA> for DreqAT {}
impl DreqA {
    #[doc = "RX Request Level"]
    #[inline(always)]
    pub fn rx(self) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, DreqA, common::RW> {
        crate::common::RegisterField::<0, 0x7f, 1, 0, u8, u8, DreqA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Request Level"]
    #[inline(always)]
    pub fn tx(self) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, DreqA, common::RW> {
        crate::common::RegisterField::<8, 0x7f, 1, 0, u8, u8, DreqA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Panic Level"]
    #[inline(always)]
    pub fn rx_panic(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, DreqA, common::RW> {
        crate::common::RegisterField::<16, 0x7f, 1, 0, u8, u8, DreqA, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Panic Level"]
    #[inline(always)]
    pub fn tx_panic(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, u8, DreqA, common::RW> {
        crate::common::RegisterField::<24, 0x7f, 1, 0, u8, u8, DreqA, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<DreqA> for DreqAT {
    #[inline(always)]
    fn reset_value(&self) -> DreqA {
        DreqA::new(271593504)
    }
}

#[doc = "Set the reasons for generating an Interrupt"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntenA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for IntenA {
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
pub struct IntenAT;
unsafe impl crate::common::AsPtr for IntenAT {}
impl crate::common::Reg<IntenA> for IntenAT {}

unsafe impl crate::common::Read<IntenA> for IntenAT {}
unsafe impl crate::common::Write<IntenA> for IntenAT {}
impl IntenA {
    #[doc = "TX Write Interrupt Enable"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntenA, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntenA, common::RW>::from_register(self, 0)
    }

    #[doc = "RX Read Interrupt Enable"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntenA, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntenA, common::RW>::from_register(self, 0)
    }

    #[doc = "TX Error Interrupt"]
    #[inline(always)]
    pub fn txerr(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntenA, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntenA, common::RW>::from_register(self, 0)
    }

    #[doc = "RX Error Interrupt"]
    #[inline(always)]
    pub fn rxerr(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntenA, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntenA, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<IntenA> for IntenAT {
    #[inline(always)]
    fn reset_value(&self) -> IntenA {
        IntenA::new(0)
    }
}

#[doc = "This register is used to read and clear the PCM interrupt status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntstcA {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for IntstcA {
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
pub struct IntstcAT;
unsafe impl crate::common::AsPtr for IntstcAT {}
impl crate::common::Reg<IntstcA> for IntstcAT {}

unsafe impl crate::common::Read<IntstcA> for IntstcAT {}
unsafe impl crate::common::Write<IntstcA> for IntstcAT {}
impl IntstcA {
    #[doc = "TX Write Interrupt Status / Clear"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntstcA, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntstcA, common::RW>::from_register(self, 0)
    }

    #[doc = "RX Read Interrupt Status / Clear"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntstcA, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntstcA, common::RW>::from_register(self, 0)
    }

    #[doc = "TX Error Interrupt Status / Clear"]
    #[inline(always)]
    pub fn txerr(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntstcA, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntstcA, common::RW>::from_register(self, 0)
    }

    #[doc = "RX Error Interrupt Status / Clear"]
    #[inline(always)]
    pub fn rxerr(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntstcA, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntstcA, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<IntstcA> for IntstcAT {
    #[inline(always)]
    fn reset_value(&self) -> IntstcA {
        IntstcA::new(0)
    }
}

#[doc = "This register is used to control the gray mode generation"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gray {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gray {
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
pub struct GrayT;
unsafe impl crate::common::AsPtr for GrayT {}
impl crate::common::Reg<Gray> for GrayT {}

unsafe impl crate::common::Read<Gray> for GrayT {}
unsafe impl crate::common::Write<Gray> for GrayT {}
impl Gray {
    #[doc = "Enable GRAY Mode"]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gray, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gray, common::RW>::from_register(self, 0)
    }

    #[doc = "Clear the GRAY Mode Logic"]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gray, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gray, common::RW>::from_register(self, 0)
    }

    #[doc = "Flush the RX Buffer into the RX FIFO"]
    #[inline(always)]
    pub fn flush(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gray, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gray, common::RW>::from_register(self, 0)
    }

    #[doc = "The Current fill level of the RX Buffer"]
    #[inline(always)]
    pub fn rxlevel(self) -> crate::common::RegisterField<4, 0x3f, 1, 0, u8, u8, Gray, common::R> {
        crate::common::RegisterField::<4, 0x3f, 1, 0, u8, u8, Gray, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The Number of bits that were flushed into the RXFIFO"]
    #[inline(always)]
    pub fn flushed(self) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Gray, common::R> {
        crate::common::RegisterField::<10, 0x3f, 1, 0, u8, u8, Gray, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The Current level of the RXFIFO"]
    #[inline(always)]
    pub fn rxfifolevel(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Gray, common::R> {
        crate::common::RegisterField::<16, 0x3f, 1, 0, u8, u8, Gray, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Gray> for GrayT {
    #[inline(always)]
    fn reset_value(&self) -> Gray {
        Gray::new(0)
    }
}
