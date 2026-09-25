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
#[doc = r"Interrupt status of new peripherals"]
unsafe impl ::core::marker::Send for super::Pactl {}
unsafe impl ::core::marker::Sync for super::Pactl {}
impl super::Pactl {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Interrupt status"]
    #[inline(always)]
    pub fn cs(&self) -> &'static self::CsT {
        unsafe { self::CsT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }
}

#[doc = "Interrupt status"]
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
    #[doc = "SPI0 interrupt active"]
    #[inline(always)]
    pub fn spi_0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI1 interrupt active"]
    #[inline(always)]
    pub fn spi_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI2 interrupt active"]
    #[inline(always)]
    pub fn spi_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI3 interrupt active"]
    #[inline(always)]
    pub fn spi_3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI4 interrupt active"]
    #[inline(always)]
    pub fn spi_4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI5 interrupt active"]
    #[inline(always)]
    pub fn spi_5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI6 interrupt active"]
    #[inline(always)]
    pub fn spi_6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C0 interrupt active"]
    #[inline(always)]
    pub fn i2c_0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C1 interrupt active"]
    #[inline(always)]
    pub fn i2c_1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C2 interrupt active"]
    #[inline(always)]
    pub fn i2c_2(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C3 interrupt active"]
    #[inline(always)]
    pub fn i2c_3(self) -> crate::common::RegisterFieldBool<11, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C4 interrupt active"]
    #[inline(always)]
    pub fn i2c_4(self) -> crate::common::RegisterFieldBool<12, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C5 interrupt active"]
    #[inline(always)]
    pub fn i2c_5(self) -> crate::common::RegisterFieldBool<13, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C6 interrupt active"]
    #[inline(always)]
    pub fn i2c_6(self) -> crate::common::RegisterFieldBool<14, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "I2C7 interrupt active"]
    #[inline(always)]
    pub fn i2c_7(self) -> crate::common::RegisterFieldBool<15, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "UART5 interrupt active"]
    #[inline(always)]
    pub fn uart_5(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "UART4 interrupt active"]
    #[inline(always)]
    pub fn uart_4(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "UART3 interrupt active"]
    #[inline(always)]
    pub fn uart_3(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "UART2 interrupt active"]
    #[inline(always)]
    pub fn uart_2(self) -> crate::common::RegisterFieldBool<19, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "UART0 interrupt active"]
    #[inline(always)]
    pub fn uart_0(self) -> crate::common::RegisterFieldBool<20, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Cs, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs> for CsT {
    #[inline(always)]
    fn reset_value(&self) -> Cs {
        Cs::new(0)
    }
}
