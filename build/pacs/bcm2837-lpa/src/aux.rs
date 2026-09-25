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
    pub fn irq(&self) -> &'static self::IrqT {
        unsafe { self::IrqT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Enable sub-peripherals"]
    #[inline(always)]
    pub fn enables(&self) -> &'static self::EnablesT {
        unsafe { self::EnablesT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }
}

#[doc = "Interrupt status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Irq {
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
pub struct IrqT;
unsafe impl crate::common::AsPtr for IrqT {}
impl crate::common::Reg<Irq> for IrqT {}

unsafe impl crate::common::Read<Irq> for IrqT {}
unsafe impl crate::common::Write<Irq> for IrqT {}
impl Irq {
    #[doc = "SPI2 interrupt active"]
    #[inline(always)]
    pub fn spi_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Irq, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Irq, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI1 interrupt active"]
    #[inline(always)]
    pub fn spi_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Irq, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Irq, common::RW>::from_register(self, 0)
    }

    #[doc = "UART1 interrupt active"]
    #[inline(always)]
    pub fn uart_1(self) -> crate::common::RegisterFieldBool<0, 1, 0, Irq, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Irq, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Irq> for IrqT {
    #[inline(always)]
    fn reset_value(&self) -> Irq {
        Irq::new(0)
    }
}

#[doc = "Enable sub-peripherals"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enables {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Enables {
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
pub struct EnablesT;
unsafe impl crate::common::AsPtr for EnablesT {}
impl crate::common::Reg<Enables> for EnablesT {}

unsafe impl crate::common::Read<Enables> for EnablesT {}
unsafe impl crate::common::Write<Enables> for EnablesT {}
impl Enables {
    #[doc = "SPI2 enabled"]
    #[inline(always)]
    pub fn spi_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Enables, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Enables, common::RW>::from_register(self, 0)
    }

    #[doc = "SPI1 enabled"]
    #[inline(always)]
    pub fn spi_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Enables, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Enables, common::RW>::from_register(self, 0)
    }

    #[doc = "UART1 enabled"]
    #[inline(always)]
    pub fn uart_1(self) -> crate::common::RegisterFieldBool<0, 1, 0, Enables, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Enables, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Enables> for EnablesT {
    #[inline(always)]
    fn reset_value(&self) -> Enables {
        Enables::new(0)
    }
}
