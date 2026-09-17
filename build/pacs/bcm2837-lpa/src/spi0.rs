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
// Generated from SVD A, with svd2pac 0.7.0 on Thu, 17 Sep 2026 02:44:16 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
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
    pub const fn cs(&self) -> &'static crate::common::Reg<self::Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "FIFO access"]
    #[inline(always)]
    pub const fn fifo(&self) -> &'static crate::common::Reg<self::Fifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Clock divider"]
    #[inline(always)]
    pub const fn clk(&self) -> &'static crate::common::Reg<self::Clk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Data length"]
    #[inline(always)]
    pub const fn dlen(&self) -> &'static crate::common::Reg<self::Dlen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dlen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "LoSSI output hold delay"]
    #[inline(always)]
    pub const fn ltoh(&self) -> &'static crate::common::Reg<self::Ltoh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ltoh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dc(&self) -> &'static crate::common::Reg<self::Dc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs_SPEC;
impl crate::sealed::RegSpec for Cs_SPEC {
    type DataType = u32;
}

#[doc = "Control and Status"]
pub type Cs = crate::RegValueT<Cs_SPEC>;

impl Cs {
    #[doc = "RX FIFO full"]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<20, 1, 0, Cs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Cs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX FIFO has data to be read"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<19, 1, 0, Cs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Cs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TX FIFO can accept data"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX FIFO contains data"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Transfer is done"]
    #[inline(always)]
    pub fn done(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        <crate::RegValueT<Cs_SPEC> as RegisterValue<_>>::new(266240)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo_SPEC;
impl crate::sealed::RegSpec for Fifo_SPEC {
    type DataType = u32;
}

#[doc = "FIFO access"]
pub type Fifo = crate::RegValueT<Fifo_SPEC>;

impl NoBitfieldReg<Fifo_SPEC> for Fifo {}
impl ::core::default::Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        <crate::RegValueT<Fifo_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk_SPEC;
impl crate::sealed::RegSpec for Clk_SPEC {
    type DataType = u32;
}

#[doc = "Clock divider"]
pub type Clk = crate::RegValueT<Clk_SPEC>;

impl NoBitfieldReg<Clk_SPEC> for Clk {}
impl ::core::default::Default for Clk {
    #[inline(always)]
    fn default() -> Clk {
        <crate::RegValueT<Clk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlen_SPEC;
impl crate::sealed::RegSpec for Dlen_SPEC {
    type DataType = u32;
}

#[doc = "Data length"]
pub type Dlen = crate::RegValueT<Dlen_SPEC>;

impl NoBitfieldReg<Dlen_SPEC> for Dlen {}
impl ::core::default::Default for Dlen {
    #[inline(always)]
    fn default() -> Dlen {
        <crate::RegValueT<Dlen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltoh_SPEC;
impl crate::sealed::RegSpec for Ltoh_SPEC {
    type DataType = u32;
}

#[doc = "LoSSI output hold delay"]
pub type Ltoh = crate::RegValueT<Ltoh_SPEC>;

impl NoBitfieldReg<Ltoh_SPEC> for Ltoh {}
impl ::core::default::Default for Ltoh {
    #[inline(always)]
    fn default() -> Ltoh {
        <crate::RegValueT<Ltoh_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dc_SPEC;
impl crate::sealed::RegSpec for Dc_SPEC {
    type DataType = u32;
}

pub type Dc = crate::RegValueT<Dc_SPEC>;

impl NoBitfieldReg<Dc_SPEC> for Dc {}
impl ::core::default::Default for Dc {
    #[inline(always)]
    fn default() -> Dc {
        <crate::RegValueT<Dc_SPEC> as RegisterValue<_>>::new(807407696)
    }
}
