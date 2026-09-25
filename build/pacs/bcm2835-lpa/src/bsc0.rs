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
#[doc = r"Broadcom Serial Controller (I2C compatible)"]
unsafe impl ::core::marker::Send for super::Bsc0 {}
unsafe impl ::core::marker::Sync for super::Bsc0 {}
impl super::Bsc0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control"]
    #[inline(always)]
    pub fn c(&self) -> &'static self::CT {
        unsafe { self::CT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub fn s(&self) -> &'static self::ST {
        unsafe { self::ST::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Data length"]
    #[inline(always)]
    pub fn dlen(&self) -> &'static self::DlenT {
        unsafe { self::DlenT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Slave address"]
    #[inline(always)]
    pub fn a(&self) -> &'static self::AT {
        unsafe { self::AT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "Data FIFO"]
    #[inline(always)]
    pub fn fifo(&self) -> &'static self::FifoT {
        unsafe { self::FifoT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "Clock divider"]
    #[inline(always)]
    pub fn div(&self) -> &'static self::DivT {
        unsafe { self::DivT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "Data delay (Values must be under CDIV / 2)"]
    #[inline(always)]
    pub fn del(&self) -> &'static self::DelT {
        unsafe { self::DelT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "Clock stretch timeout (broken on 283x)"]
    #[inline(always)]
    pub fn clkt(&self) -> &'static self::ClktT {
        unsafe { self::ClktT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }
}

#[doc = "Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for C {
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
pub struct CT;
unsafe impl crate::common::AsPtr for CT {}
impl crate::common::Reg<C> for CT {}

unsafe impl crate::common::Read<C> for CT {}
unsafe impl crate::common::Write<C> for CT {}
impl C {
    #[doc = "I2C Enable"]
    #[inline(always)]
    pub fn i2cen(self) -> crate::common::RegisterFieldBool<15, 1, 0, C, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, C, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt on RX"]
    #[inline(always)]
    pub fn intr(self) -> crate::common::RegisterFieldBool<10, 1, 0, C, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, C, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt on TX"]
    #[inline(always)]
    pub fn intt(self) -> crate::common::RegisterFieldBool<9, 1, 0, C, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, C, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt on done"]
    #[inline(always)]
    pub fn intd(self) -> crate::common::RegisterFieldBool<8, 1, 0, C, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, C, common::RW>::from_register(self, 0)
    }

    #[doc = "Start transfer"]
    #[inline(always)]
    pub fn st(self) -> crate::common::RegisterFieldBool<7, 1, 0, C, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, C, common::RW>::from_register(self, 0)
    }

    #[doc = "Clear the FIFO"]
    #[inline(always)]
    pub fn clear(self) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, C, common::RW> {
        crate::common::RegisterField::<4, 0x3, 1, 0, u8, u8, C, common::RW>::from_register(self, 0)
    }

    #[doc = "Transfer is read"]
    #[inline(always)]
    pub fn read(self) -> crate::common::RegisterFieldBool<0, 1, 0, C, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, C, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<C> for CT {
    #[inline(always)]
    fn reset_value(&self) -> C {
        C::new(0)
    }
}

#[doc = "Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for S {
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
pub struct ST;
unsafe impl crate::common::AsPtr for ST {}
impl crate::common::Reg<S> for ST {}

unsafe impl crate::common::Read<S> for ST {}
unsafe impl crate::common::Write<S> for ST {}
impl S {
    #[doc = "Clock stretch timeout"]
    #[inline(always)]
    pub fn clkt(self) -> crate::common::RegisterFieldBool<9, 1, 0, S, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, S, common::RW>::from_register(self, 0)
    }

    #[doc = "Error: No ack"]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<8, 1, 0, S, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, S, common::RW>::from_register(self, 0)
    }

    #[doc = "FIFO is full. Can\'t receive anything else"]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<7, 1, 0, S, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, S, common::R>::from_register(self, 0)
    }

    #[doc = "FIFO is empty. Nothing to transmit"]
    #[inline(always)]
    pub fn txe(self) -> crate::common::RegisterFieldBool<6, 1, 0, S, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, S, common::R>::from_register(self, 0)
    }

    #[doc = "FIFO contains at least one byte"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::RegisterFieldBool<5, 1, 0, S, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, S, common::R>::from_register(self, 0)
    }

    #[doc = "FIFO has space for at least one byte"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::RegisterFieldBool<4, 1, 0, S, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, S, common::R>::from_register(self, 0)
    }

    #[doc = "FIFO needs to be read"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<3, 1, 0, S, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, S, common::R>::from_register(self, 0)
    }

    #[doc = "FIFO needs to be written"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<2, 1, 0, S, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, S, common::R>::from_register(self, 0)
    }

    #[doc = "Transfer done"]
    #[inline(always)]
    pub fn done(self) -> crate::common::RegisterFieldBool<1, 1, 0, S, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, S, common::RW>::from_register(self, 0)
    }

    #[doc = "Transfer active"]
    #[inline(always)]
    pub fn ta(self) -> crate::common::RegisterFieldBool<0, 1, 0, S, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, S, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<S> for ST {
    #[inline(always)]
    fn reset_value(&self) -> S {
        S::new(80)
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

#[doc = "Slave address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for A {
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
pub struct AT;
unsafe impl crate::common::AsPtr for AT {}
impl crate::common::Reg<A> for AT {}

unsafe impl crate::common::Read<A> for AT {}
unsafe impl crate::common::Write<A> for AT {}
impl A {
    #[doc = "Slave address"]
    #[inline(always)]
    pub fn addr(self) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, A, common::RW> {
        crate::common::RegisterField::<0, 0x7f, 1, 0, u8, u8, A, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<A> for AT {
    #[inline(always)]
    fn reset_value(&self) -> A {
        A::new(0)
    }
}

#[doc = "Data FIFO"]
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
    #[doc = "Access the FIFO"]
    #[inline(always)]
    pub fn data(self) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Fifo, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, Fifo, common::RW>::from_register(
            self, 0,
        )
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
pub struct Div {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Div {
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
pub struct DivT;
unsafe impl crate::common::AsPtr for DivT {}
impl crate::common::Reg<Div> for DivT {}

unsafe impl crate::common::Read<Div> for DivT {}
unsafe impl crate::common::Write<Div> for DivT {}
impl Div {
    #[doc = "Divide the source clock"]
    #[inline(always)]
    pub fn cdiv(self) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Div, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Div, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Div> for DivT {
    #[inline(always)]
    fn reset_value(&self) -> Div {
        Div::new(1500)
    }
}

#[doc = "Data delay (Values must be under CDIV / 2)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Del {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Del {
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
pub struct DelT;
unsafe impl crate::common::AsPtr for DelT {}
impl crate::common::Reg<Del> for DelT {}

unsafe impl crate::common::Read<Del> for DelT {}
unsafe impl crate::common::Write<Del> for DelT {}
impl Del {
    #[doc = "Delay before reading after a falling edge"]
    #[inline(always)]
    pub fn fedl(self) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Del, common::RW> {
        crate::common::RegisterField::<16, 0xffff, 1, 0, u16, u16, Del, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Delay before reading after a rising edge"]
    #[inline(always)]
    pub fn redl(self) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Del, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Del, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Del> for DelT {
    #[inline(always)]
    fn reset_value(&self) -> Del {
        Del::new(3145776)
    }
}

#[doc = "Clock stretch timeout (broken on 283x)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkt {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Clkt {
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
pub struct ClktT;
unsafe impl crate::common::AsPtr for ClktT {}
impl crate::common::Reg<Clkt> for ClktT {}

unsafe impl crate::common::Read<Clkt> for ClktT {}
unsafe impl crate::common::Write<Clkt> for ClktT {}
impl Clkt {
    #[doc = "Number of SCL clock cycles to wait"]
    #[inline(always)]
    pub fn tout(self) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Clkt, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Clkt, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Clkt> for ClktT {
    #[inline(always)]
    fn reset_value(&self) -> Clkt {
        Clkt::new(0)
    }
}
