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
#[doc = r"Broadcom SPI Controller"]
unsafe impl ::core::marker::Send for super::Spi0 {}
unsafe impl ::core::marker::Sync for super::Spi0 {}
impl super::Spi0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control and Status"]
    #[inline(always)]
    pub fn cs(&self) -> &'static self::CsT {
        unsafe { self::CsT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "FIFO access"]
    #[inline(always)]
    pub fn fifo(&self) -> &'static self::FifoT {
        unsafe { self::FifoT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Clock divider"]
    #[inline(always)]
    pub fn clk(&self) -> &'static self::ClkT {
        unsafe { self::ClkT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Data length"]
    #[inline(always)]
    pub fn dlen(&self) -> &'static self::DlenT {
        unsafe { self::DlenT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "LoSSI output hold delay"]
    #[inline(always)]
    pub fn ltoh(&self) -> &'static self::LtohT {
        unsafe { self::LtohT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[inline(always)]
    pub fn dc(&self) -> &'static self::DcT {
        unsafe { self::DcT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }
}

#[doc = "Control and Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs {
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
pub struct CsT;
unsafe impl crate::common::AsPtr for CsT {}
impl crate::common::Reg<Cs> for CsT {}

unsafe impl crate::common::Read<Cs> for CsT {}
unsafe impl crate::common::Write<Cs> for CsT {}
impl Cs {
    #[doc = "Enable long data word in LoSSI mode"]
    #[inline(always)]
    pub fn len_long(self) -> crate::common::RegisterFieldBool<25, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable DMA in LoSSI mode"]
    #[inline(always)]
    pub fn dma_len(self) -> crate::common::RegisterFieldBool<24, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Chip select 2 polarity"]
    #[inline(always)]
    pub fn cspol2(self) -> crate::common::RegisterFieldBool<23, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Chip select 1 polarity"]
    #[inline(always)]
    pub fn cspol1(self) -> crate::common::RegisterFieldBool<22, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Chip select 0 polarity"]
    #[inline(always)]
    pub fn cspol0(self) -> crate::common::RegisterFieldBool<21, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "RX FIFO full"]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<20, 1, 0, Cs, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Cs, common::R>::from_register(self, 0)
    }

    #[doc = "RX FIFO has data to be read"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<19, 1, 0, Cs, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Cs, common::R>::from_register(self, 0)
    }

    #[doc = "TX FIFO can accept data"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cs, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cs, common::R>::from_register(self, 0)
    }

    #[doc = "RX FIFO contains data"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cs, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cs, common::R>::from_register(self, 0)
    }

    #[doc = "Transfer is done"]
    #[inline(always)]
    pub fn done(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cs, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cs, common::R>::from_register(self, 0)
    }

    #[inline(always)]
    pub fn te_en(self) -> crate::common::RegisterFieldBool<15, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lmono(self) -> crate::common::RegisterFieldBool<14, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "LoSSI enable"]
    #[inline(always)]
    pub fn len(self) -> crate::common::RegisterFieldBool<13, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Read enable"]
    #[inline(always)]
    pub fn ren(self) -> crate::common::RegisterFieldBool<12, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Automatically deassert chip select"]
    #[inline(always)]
    pub fn adcs(self) -> crate::common::RegisterFieldBool<11, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt on RX"]
    #[inline(always)]
    pub fn intr(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt on done"]
    #[inline(always)]
    pub fn intd(self) -> crate::common::RegisterFieldBool<9, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable DMA"]
    #[inline(always)]
    pub fn dmaen(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Transfer active"]
    #[inline(always)]
    pub fn ta(self) -> crate::common::RegisterFieldBool<7, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Chip select polarity"]
    #[inline(always)]
    pub fn cspol(self) -> crate::common::RegisterFieldBool<6, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Clear the FIFO(s)"]
    #[inline(always)]
    pub fn clear(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, cs::Clear, cs::Clear, Cs, common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,cs::Clear,cs::Clear,Cs,common::RW>::from_register(self,0)
    }

    #[doc = "Clock polarity"]
    #[inline(always)]
    pub fn cpol(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Clock phase"]
    #[inline(always)]
    pub fn cpha(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Chip select"]
    #[inline(always)]
    pub fn cs(self) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Cs, common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, u8, Cs, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs> for CsT {
    #[inline(always)]
    fn reset_value(&self) -> Cs {
        Cs::new(266240)
    }
}
pub mod cs {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Clear(u8);

    impl Clear {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Clear {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Clear {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Clear> for u64 {
        #[inline(always)]
        fn from(value: Clear) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Clear {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Clear {
        pub const TX: Self = Self(1);

        pub const RX: Self = Self(2);

        pub const BOTH: Self = Self(3);
    }
}

#[doc = "FIFO access"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Fifo {
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
pub struct FifoT;
unsafe impl crate::common::AsPtr for FifoT {}
impl crate::common::Reg<Fifo> for FifoT {}

unsafe impl crate::common::Read<Fifo> for FifoT {}
unsafe impl crate::common::Write<Fifo> for FifoT {}
impl Fifo {
    #[doc = "Data"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Fifo, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Fifo,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Fifo> for FifoT {
    #[inline(always)]
    fn reset_value(&self) -> Fifo {
        Fifo::new(0)
    }
}

#[doc = "Clock divider"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Clk {
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
pub struct ClkT;
unsafe impl crate::common::AsPtr for ClkT {}
impl crate::common::Reg<Clk> for ClkT {}

unsafe impl crate::common::Read<Clk> for ClkT {}
unsafe impl crate::common::Write<Clk> for ClkT {}
impl Clk {
    #[doc = "Clock divider"]
    #[inline(always)]
    pub fn cdiv(self) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Clk, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Clk, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Clk> for ClkT {
    #[inline(always)]
    fn reset_value(&self) -> Clk {
        Clk::new(0)
    }
}

#[doc = "Data length"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlen {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dlen {
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
pub struct DlenT;
unsafe impl crate::common::AsPtr for DlenT {}
impl crate::common::Reg<Dlen> for DlenT {}

unsafe impl crate::common::Read<Dlen> for DlenT {}
unsafe impl crate::common::Write<Dlen> for DlenT {}
impl Dlen {
    #[doc = "Data length"]
    #[inline(always)]
    pub fn dlen(self) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dlen, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Dlen, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Dlen> for DlenT {
    #[inline(always)]
    fn reset_value(&self) -> Dlen {
        Dlen::new(0)
    }
}

#[doc = "LoSSI output hold delay"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltoh {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ltoh {
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
pub struct LtohT;
unsafe impl crate::common::AsPtr for LtohT {}
impl crate::common::Reg<Ltoh> for LtohT {}

unsafe impl crate::common::Read<Ltoh> for LtohT {}
unsafe impl crate::common::Write<Ltoh> for LtohT {}
impl Ltoh {
    #[doc = "Output hold delay"]
    #[inline(always)]
    pub fn toh(self) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Ltoh, common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, u8, Ltoh, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ltoh> for LtohT {
    #[inline(always)]
    fn reset_value(&self) -> Ltoh {
        Ltoh::new(1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dc {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dc {
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
pub struct DcT;
unsafe impl crate::common::AsPtr for DcT {}
impl crate::common::Reg<Dc> for DcT {}

unsafe impl crate::common::Read<Dc> for DcT {}
unsafe impl crate::common::Write<Dc> for DcT {}
impl Dc {
    #[doc = "DMA read panic threshold"]
    #[inline(always)]
    pub fn rpanic(self) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Dc, common::RW> {
        crate::common::RegisterField::<24, 0xff, 1, 0, u8, u8, Dc, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA read request threshold"]
    #[inline(always)]
    pub fn rdreq(self) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Dc, common::RW> {
        crate::common::RegisterField::<16, 0xff, 1, 0, u8, u8, Dc, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA write panic threshold"]
    #[inline(always)]
    pub fn tpanic(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dc, common::RW> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Dc, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Write request threshold"]
    #[inline(always)]
    pub fn tdreq(self) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dc, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, Dc, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Dc> for DcT {
    #[inline(always)]
    fn reset_value(&self) -> Dc {
        Dc::new(807407696)
    }
}
