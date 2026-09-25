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
#[doc = r"ARM Prime Cell PL011"]
unsafe impl ::core::marker::Send for super::ArmUartPl011 {}
unsafe impl ::core::marker::Sync for super::ArmUartPl011 {}
impl super::ArmUartPl011 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Data Register"]
    #[inline(always)]
    pub fn dr(&self) -> &'static self::DrT {
        unsafe { self::DrT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Receive Status Register"]
    #[inline(always)]
    pub fn rsr(&self) -> &'static self::RsrT {
        unsafe { self::RsrT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Error Clear Register"]
    #[inline(always)]
    pub fn ecr(&self) -> &'static self::EcrT {
        unsafe { self::EcrT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Flag Register"]
    #[inline(always)]
    pub fn fr(&self) -> &'static self::FrT {
        unsafe { self::FrT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "Integer Baud Rate Register"]
    #[inline(always)]
    pub fn ibrd(&self) -> &'static self::IbrdT {
        unsafe { self::IbrdT::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }

    #[doc = "Fractional Baud Rate Register"]
    #[inline(always)]
    pub fn fbrd(&self) -> &'static self::FbrdT {
        unsafe { self::FbrdT::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "Line Control Register"]
    #[inline(always)]
    pub fn lcr_h(&self) -> &'static self::LcrHT {
        unsafe { self::LcrHT::from_ptr(self._svd2pac_as_ptr().add(44usize)) }
    }

    #[doc = "Control Register"]
    #[inline(always)]
    pub fn cr(&self) -> &'static self::CrT {
        unsafe { self::CrT::from_ptr(self._svd2pac_as_ptr().add(48usize)) }
    }

    #[doc = "Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub fn ifls(&self) -> &'static self::IflsT {
        unsafe { self::IflsT::from_ptr(self._svd2pac_as_ptr().add(52usize)) }
    }

    #[doc = "Interrupt Mask set_Clear Register"]
    #[inline(always)]
    pub fn imsc(&self) -> &'static self::ImscT {
        unsafe { self::ImscT::from_ptr(self._svd2pac_as_ptr().add(56usize)) }
    }

    #[doc = "Raw Interrupt Status Register"]
    #[inline(always)]
    pub fn ris(&self) -> &'static self::RisT {
        unsafe { self::RisT::from_ptr(self._svd2pac_as_ptr().add(60usize)) }
    }

    #[doc = "Masked Interrupt Status Register"]
    #[inline(always)]
    pub fn mis(&self) -> &'static self::MisT {
        unsafe { self::MisT::from_ptr(self._svd2pac_as_ptr().add(64usize)) }
    }

    #[doc = "Interrupt Clear Register"]
    #[inline(always)]
    pub fn icr(&self) -> &'static self::IcrT {
        unsafe { self::IcrT::from_ptr(self._svd2pac_as_ptr().add(68usize)) }
    }

    #[doc = "DMA Control Register"]
    #[inline(always)]
    pub fn dmacr(&self) -> &'static self::DmacrT {
        unsafe { self::DmacrT::from_ptr(self._svd2pac_as_ptr().add(72usize)) }
    }
}

#[doc = "Data Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dr {
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
pub struct DrT;
unsafe impl crate::common::AsPtr for DrT {}
impl crate::common::Reg<Dr> for DrT {}

unsafe impl crate::common::Read<Dr> for DrT {}
unsafe impl crate::common::Write<Dr> for DrT {}
impl Dr {
    #[doc = "DATA"]
    #[inline(always)]
    pub fn data(self) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dr, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, Dr, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FE"]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dr, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dr, common::RW>::from_register(self, 0)
    }

    #[doc = "PE"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Dr, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Dr, common::RW>::from_register(self, 0)
    }

    #[doc = "BE"]
    #[inline(always)]
    pub fn be(self) -> crate::common::RegisterFieldBool<10, 1, 0, Dr, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Dr, common::RW>::from_register(self, 0)
    }

    #[doc = "OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<11, 1, 0, Dr, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Dr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Dr> for DrT {
    #[inline(always)]
    fn reset_value(&self) -> Dr {
        Dr::new(0)
    }
}

#[doc = "Receive Status Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Rsr {
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
pub struct RsrT;
unsafe impl crate::common::AsPtr for RsrT {}
impl crate::common::Reg<Rsr> for RsrT {}

unsafe impl crate::common::Read<Rsr> for RsrT {}
impl Rsr {
    #[doc = "FE"]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rsr, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rsr, common::R>::from_register(self, 0)
    }

    #[doc = "PE"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rsr, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rsr, common::R>::from_register(self, 0)
    }

    #[doc = "BE"]
    #[inline(always)]
    pub fn be(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rsr, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rsr, common::R>::from_register(self, 0)
    }

    #[doc = "OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rsr, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rsr, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Rsr> for RsrT {
    #[inline(always)]
    fn reset_value(&self) -> Rsr {
        Rsr::new(0)
    }
}

#[doc = "Error Clear Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ecr {
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
pub struct EcrT;
unsafe impl crate::common::AsPtr for EcrT {}
impl crate::common::Reg<Ecr> for EcrT {}

unsafe impl crate::common::Write<Ecr> for EcrT {}
impl Ecr {
    #[doc = "FE"]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ecr, common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ecr, common::W>::from_register(self, 0)
    }

    #[doc = "PE"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ecr, common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ecr, common::W>::from_register(self, 0)
    }

    #[doc = "BE"]
    #[inline(always)]
    pub fn be(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ecr, common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ecr, common::W>::from_register(self, 0)
    }

    #[doc = "OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ecr, common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ecr, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ecr> for EcrT {
    #[inline(always)]
    fn reset_value(&self) -> Ecr {
        Ecr::new(0)
    }
}

#[doc = "Flag Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Fr {
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
pub struct FrT;
unsafe impl crate::common::AsPtr for FrT {}
impl crate::common::Reg<Fr> for FrT {}

unsafe impl crate::common::Read<Fr> for FrT {}
unsafe impl crate::common::Write<Fr> for FrT {}
impl Fr {
    #[doc = "CTS"]
    #[inline(always)]
    pub fn cts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "DSR"]
    #[inline(always)]
    pub fn dsr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "DCD"]
    #[inline(always)]
    pub fn dcd(self) -> crate::common::RegisterFieldBool<2, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "BUSY"]
    #[inline(always)]
    pub fn busy(self) -> crate::common::RegisterFieldBool<3, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "RXFE"]
    #[inline(always)]
    pub fn rxfe(self) -> crate::common::RegisterFieldBool<4, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "TXFF"]
    #[inline(always)]
    pub fn txff(self) -> crate::common::RegisterFieldBool<5, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "RXFF"]
    #[inline(always)]
    pub fn rxff(self) -> crate::common::RegisterFieldBool<6, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "TXFE"]
    #[inline(always)]
    pub fn txfe(self) -> crate::common::RegisterFieldBool<7, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fr, common::RW>::from_register(self, 0)
    }

    #[doc = "RI"]
    #[inline(always)]
    pub fn ri(self) -> crate::common::RegisterFieldBool<8, 1, 0, Fr, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Fr> for FrT {
    #[inline(always)]
    fn reset_value(&self) -> Fr {
        Fr::new(0)
    }
}

#[doc = "Integer Baud Rate Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibrd {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ibrd {
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
pub struct IbrdT;
unsafe impl crate::common::AsPtr for IbrdT {}
impl crate::common::Reg<Ibrd> for IbrdT {}

unsafe impl crate::common::Read<Ibrd> for IbrdT {}
unsafe impl crate::common::Write<Ibrd> for IbrdT {}
impl Ibrd {
    #[doc = "BAUDDIVINT"]
    #[inline(always)]
    pub fn bauddivint(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ibrd, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Ibrd, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ibrd> for IbrdT {
    #[inline(always)]
    fn reset_value(&self) -> Ibrd {
        Ibrd::new(0)
    }
}

#[doc = "Fractional Baud Rate Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbrd {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Fbrd {
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
pub struct FbrdT;
unsafe impl crate::common::AsPtr for FbrdT {}
impl crate::common::Reg<Fbrd> for FbrdT {}

unsafe impl crate::common::Read<Fbrd> for FbrdT {}
unsafe impl crate::common::Write<Fbrd> for FbrdT {}
impl Fbrd {
    #[doc = "BAUDDIVFRAC"]
    #[inline(always)]
    pub fn bauddivfrac(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fbrd, common::RW> {
        crate::common::RegisterField::<0, 0x3f, 1, 0, u8, u8, Fbrd, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Fbrd> for FbrdT {
    #[inline(always)]
    fn reset_value(&self) -> Fbrd {
        Fbrd::new(0)
    }
}

#[doc = "Line Control Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcrH {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for LcrH {
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
pub struct LcrHT;
unsafe impl crate::common::AsPtr for LcrHT {}
impl crate::common::Reg<LcrH> for LcrHT {}

unsafe impl crate::common::Read<LcrH> for LcrHT {}
unsafe impl crate::common::Write<LcrH> for LcrHT {}
impl LcrH {
    #[doc = "BRK"]
    #[inline(always)]
    pub fn brk(self) -> crate::common::RegisterFieldBool<0, 1, 0, LcrH, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, LcrH, common::RW>::from_register(self, 0)
    }

    #[doc = "PEN"]
    #[inline(always)]
    pub fn pen(self) -> crate::common::RegisterFieldBool<1, 1, 0, LcrH, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, LcrH, common::RW>::from_register(self, 0)
    }

    #[doc = "EPS"]
    #[inline(always)]
    pub fn eps(self) -> crate::common::RegisterFieldBool<2, 1, 0, LcrH, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, LcrH, common::RW>::from_register(self, 0)
    }

    #[doc = "STP2"]
    #[inline(always)]
    pub fn stp2(self) -> crate::common::RegisterFieldBool<3, 1, 0, LcrH, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, LcrH, common::RW>::from_register(self, 0)
    }

    #[doc = "FEN"]
    #[inline(always)]
    pub fn fen(self) -> crate::common::RegisterFieldBool<4, 1, 0, LcrH, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, LcrH, common::RW>::from_register(self, 0)
    }

    #[doc = "WLEN"]
    #[inline(always)]
    pub fn wlen(self) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, LcrH, common::RW> {
        crate::common::RegisterField::<5, 0x3, 1, 0, u8, u8, LcrH, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SPS"]
    #[inline(always)]
    pub fn sps(self) -> crate::common::RegisterFieldBool<7, 1, 0, LcrH, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, LcrH, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<LcrH> for LcrHT {
    #[inline(always)]
    fn reset_value(&self) -> LcrH {
        LcrH::new(0)
    }
}

#[doc = "Control Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cr {
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
pub struct CrT;
unsafe impl crate::common::AsPtr for CrT {}
impl crate::common::Reg<Cr> for CrT {}

unsafe impl crate::common::Read<Cr> for CrT {}
unsafe impl crate::common::Write<Cr> for CrT {}
impl Cr {
    #[doc = "UARTEN"]
    #[inline(always)]
    pub fn uarten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "SIREN"]
    #[inline(always)]
    pub fn siren(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "SIRLP"]
    #[inline(always)]
    pub fn sirlp(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "TXE"]
    #[inline(always)]
    pub fn txe(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "RXE"]
    #[inline(always)]
    pub fn rxe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "DTR"]
    #[inline(always)]
    pub fn dtr(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "RTS"]
    #[inline(always)]
    pub fn rts(self) -> crate::common::RegisterFieldBool<11, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "RTSEN"]
    #[inline(always)]
    pub fn rtsen(self) -> crate::common::RegisterFieldBool<14, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cr, common::RW>::from_register(self, 0)
    }

    #[doc = "CTSEN"]
    #[inline(always)]
    pub fn ctsen(self) -> crate::common::RegisterFieldBool<15, 1, 0, Cr, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cr> for CrT {
    #[inline(always)]
    fn reset_value(&self) -> Cr {
        Cr::new(0)
    }
}

#[doc = "Interrupt FIFO Level Select Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifls {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ifls {
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
pub struct IflsT;
unsafe impl crate::common::AsPtr for IflsT {}
impl crate::common::Reg<Ifls> for IflsT {}

unsafe impl crate::common::Read<Ifls> for IflsT {}
unsafe impl crate::common::Write<Ifls> for IflsT {}
impl Ifls {
    #[doc = "TXIFLSEL"]
    #[inline(always)]
    pub fn txiflsel(self) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Ifls, common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, Ifls, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RXIFLSEL"]
    #[inline(always)]
    pub fn rxiflsel(self) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, Ifls, common::RW> {
        crate::common::RegisterField::<3, 0x7, 1, 0, u8, u8, Ifls, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ifls> for IflsT {
    #[inline(always)]
    fn reset_value(&self) -> Ifls {
        Ifls::new(0)
    }
}

#[doc = "Interrupt Mask set_Clear Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imsc {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Imsc {
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
pub struct ImscT;
unsafe impl crate::common::AsPtr for ImscT {}
impl crate::common::Reg<Imsc> for ImscT {}

unsafe impl crate::common::Read<Imsc> for ImscT {}
unsafe impl crate::common::Write<Imsc> for ImscT {}
impl Imsc {
    #[doc = "RIMIM"]
    #[inline(always)]
    pub fn rimim(self) -> crate::common::RegisterFieldBool<0, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "CTSMIM"]
    #[inline(always)]
    pub fn ctsmim(self) -> crate::common::RegisterFieldBool<1, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "DCDMIM"]
    #[inline(always)]
    pub fn dcdmim(self) -> crate::common::RegisterFieldBool<2, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "DSRMIM"]
    #[inline(always)]
    pub fn dsrmim(self) -> crate::common::RegisterFieldBool<3, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "RXIM"]
    #[inline(always)]
    pub fn rxim(self) -> crate::common::RegisterFieldBool<4, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "TXIM"]
    #[inline(always)]
    pub fn txim(self) -> crate::common::RegisterFieldBool<5, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "RTIM"]
    #[inline(always)]
    pub fn rtim(self) -> crate::common::RegisterFieldBool<6, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "FEIM"]
    #[inline(always)]
    pub fn feim(self) -> crate::common::RegisterFieldBool<7, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "PEIM"]
    #[inline(always)]
    pub fn peim(self) -> crate::common::RegisterFieldBool<8, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "BEIM"]
    #[inline(always)]
    pub fn beim(self) -> crate::common::RegisterFieldBool<9, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }

    #[doc = "OEIM"]
    #[inline(always)]
    pub fn oeim(self) -> crate::common::RegisterFieldBool<10, 1, 0, Imsc, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Imsc, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Imsc> for ImscT {
    #[inline(always)]
    fn reset_value(&self) -> Imsc {
        Imsc::new(0)
    }
}

#[doc = "Raw Interrupt Status Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ris {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ris {
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
pub struct RisT;
unsafe impl crate::common::AsPtr for RisT {}
impl crate::common::Reg<Ris> for RisT {}

unsafe impl crate::common::Read<Ris> for RisT {}
impl Ris {
    #[doc = "RIRMIS"]
    #[inline(always)]
    pub fn rirmis(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "CTSRMIS"]
    #[inline(always)]
    pub fn ctsrmis(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "DCDRMIS"]
    #[inline(always)]
    pub fn dcdrmis(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "DSRRMIS"]
    #[inline(always)]
    pub fn dsrrmis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "RXRIS"]
    #[inline(always)]
    pub fn rxris(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "TXRIS"]
    #[inline(always)]
    pub fn txris(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "RTRIS"]
    #[inline(always)]
    pub fn rtris(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "FERIS"]
    #[inline(always)]
    pub fn feris(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "PERIS"]
    #[inline(always)]
    pub fn peris(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "BERIS"]
    #[inline(always)]
    pub fn beris(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ris, common::R>::from_register(self, 0)
    }

    #[doc = "OERIS"]
    #[inline(always)]
    pub fn oeris(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ris, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ris, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ris> for RisT {
    #[inline(always)]
    fn reset_value(&self) -> Ris {
        Ris::new(0)
    }
}

#[doc = "Masked Interrupt Status Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mis {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Mis {
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
pub struct MisT;
unsafe impl crate::common::AsPtr for MisT {}
impl crate::common::Reg<Mis> for MisT {}

unsafe impl crate::common::Read<Mis> for MisT {}
impl Mis {
    #[doc = "RIMMIS"]
    #[inline(always)]
    pub fn rimmis(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "CTSMMIS"]
    #[inline(always)]
    pub fn ctsmmis(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "DCDMMIS"]
    #[inline(always)]
    pub fn dcdmmis(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "DSRMMIS"]
    #[inline(always)]
    pub fn dsrmmis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "RXMIS"]
    #[inline(always)]
    pub fn rxmis(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "TXMIS"]
    #[inline(always)]
    pub fn txmis(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "RTMIS"]
    #[inline(always)]
    pub fn rtmis(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "FEMIS"]
    #[inline(always)]
    pub fn femis(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "PEMIS"]
    #[inline(always)]
    pub fn pemis(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "BEMIS"]
    #[inline(always)]
    pub fn bemis(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mis, common::R>::from_register(self, 0)
    }

    #[doc = "OEMIS"]
    #[inline(always)]
    pub fn oemis(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mis, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mis, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Mis> for MisT {
    #[inline(always)]
    fn reset_value(&self) -> Mis {
        Mis::new(0)
    }
}

#[doc = "Interrupt Clear Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Icr {
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
pub struct IcrT;
unsafe impl crate::common::AsPtr for IcrT {}
impl crate::common::Reg<Icr> for IcrT {}

unsafe impl crate::common::Write<Icr> for IcrT {}
impl Icr {
    #[doc = "RIMIC"]
    #[inline(always)]
    pub fn rimic(self) -> crate::common::RegisterFieldBool<0, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "CTSMIC"]
    #[inline(always)]
    pub fn ctsmic(self) -> crate::common::RegisterFieldBool<1, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "DCDMIC"]
    #[inline(always)]
    pub fn dcdmic(self) -> crate::common::RegisterFieldBool<2, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "DSRMIC"]
    #[inline(always)]
    pub fn dsrmic(self) -> crate::common::RegisterFieldBool<3, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "RXIC"]
    #[inline(always)]
    pub fn rxic(self) -> crate::common::RegisterFieldBool<4, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "TXIC"]
    #[inline(always)]
    pub fn txic(self) -> crate::common::RegisterFieldBool<5, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "RTIC"]
    #[inline(always)]
    pub fn rtic(self) -> crate::common::RegisterFieldBool<6, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "FEIC"]
    #[inline(always)]
    pub fn feic(self) -> crate::common::RegisterFieldBool<7, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "PEIC"]
    #[inline(always)]
    pub fn peic(self) -> crate::common::RegisterFieldBool<8, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "BEIC"]
    #[inline(always)]
    pub fn beic(self) -> crate::common::RegisterFieldBool<9, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icr, common::W>::from_register(self, 0)
    }

    #[doc = "OEIC"]
    #[inline(always)]
    pub fn oeic(self) -> crate::common::RegisterFieldBool<10, 1, 0, Icr, common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icr, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Icr> for IcrT {
    #[inline(always)]
    fn reset_value(&self) -> Icr {
        Icr::new(0)
    }
}

#[doc = "DMA Control Register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dmacr {
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
pub struct DmacrT;
unsafe impl crate::common::AsPtr for DmacrT {}
impl crate::common::Reg<Dmacr> for DmacrT {}

unsafe impl crate::common::Read<Dmacr> for DmacrT {}
unsafe impl crate::common::Write<Dmacr> for DmacrT {}
impl Dmacr {
    #[doc = "RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dmacr, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dmacr, common::RW>::from_register(self, 0)
    }

    #[doc = "TXDMAE"]
    #[inline(always)]
    pub fn txdmae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dmacr, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dmacr, common::RW>::from_register(self, 0)
    }

    #[doc = "DMAONERR"]
    #[inline(always)]
    pub fn dmaonerr(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dmacr, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dmacr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Dmacr> for DmacrT {
    #[inline(always)]
    fn reset_value(&self) -> Dmacr {
        Dmacr::new(0)
    }
}
