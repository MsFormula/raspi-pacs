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
// Generated from SVD A, with svd2pac 0.7.0 on Thu, 17 Sep 2026 02:44:17 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
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
    pub const fn c(&self) -> &'static crate::common::Reg<self::C_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::C_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub const fn s(&self) -> &'static crate::common::Reg<self::S_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::S_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Data length"]
    #[inline(always)]
    pub const fn dlen(&self) -> &'static crate::common::Reg<self::Dlen_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dlen_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Slave address"]
    #[inline(always)]
    pub const fn a(&self) -> &'static crate::common::Reg<self::A_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::A_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Data FIFO"]
    #[inline(always)]
    pub const fn fifo(&self) -> &'static crate::common::Reg<self::Fifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Clock divider"]
    #[inline(always)]
    pub const fn div(&self) -> &'static crate::common::Reg<self::Div_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Div_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Data delay (Values must be under CDIV / 2)"]
    #[inline(always)]
    pub const fn del(&self) -> &'static crate::common::Reg<self::Del_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Del_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Clock stretch timeout (broken on 283x)"]
    #[inline(always)]
    pub const fn clkt(&self) -> &'static crate::common::Reg<self::Clkt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clkt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C_SPEC;
impl crate::sealed::RegSpec for C_SPEC {
    type DataType = u32;
}

#[doc = "Control"]
pub type C = crate::RegValueT<C_SPEC>;

impl NoBitfieldReg<C_SPEC> for C {}
impl ::core::default::Default for C {
    #[inline(always)]
    fn default() -> C {
        <crate::RegValueT<C_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S_SPEC;
impl crate::sealed::RegSpec for S_SPEC {
    type DataType = u32;
}

#[doc = "Status"]
pub type S = crate::RegValueT<S_SPEC>;

impl S {
    #[doc = "FIFO is full. Can\'t receive anything else"]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<7, 1, 0, S_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, S_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FIFO is empty. Nothing to transmit"]
    #[inline(always)]
    pub fn txe(self) -> crate::common::RegisterFieldBool<6, 1, 0, S_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, S_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FIFO contains at least one byte"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::RegisterFieldBool<5, 1, 0, S_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, S_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FIFO has space for at least one byte"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::RegisterFieldBool<4, 1, 0, S_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, S_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FIFO needs to be read"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<3, 1, 0, S_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, S_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FIFO needs to be written"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<2, 1, 0, S_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, S_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Transfer active"]
    #[inline(always)]
    pub fn ta(self) -> crate::common::RegisterFieldBool<0, 1, 0, S_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, S_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for S {
    #[inline(always)]
    fn default() -> S {
        <crate::RegValueT<S_SPEC> as RegisterValue<_>>::new(80)
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
pub struct A_SPEC;
impl crate::sealed::RegSpec for A_SPEC {
    type DataType = u32;
}

#[doc = "Slave address"]
pub type A = crate::RegValueT<A_SPEC>;

impl NoBitfieldReg<A_SPEC> for A {}
impl ::core::default::Default for A {
    #[inline(always)]
    fn default() -> A {
        <crate::RegValueT<A_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo_SPEC;
impl crate::sealed::RegSpec for Fifo_SPEC {
    type DataType = u32;
}

#[doc = "Data FIFO"]
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
pub struct Div_SPEC;
impl crate::sealed::RegSpec for Div_SPEC {
    type DataType = u32;
}

#[doc = "Clock divider"]
pub type Div = crate::RegValueT<Div_SPEC>;

impl NoBitfieldReg<Div_SPEC> for Div {}
impl ::core::default::Default for Div {
    #[inline(always)]
    fn default() -> Div {
        <crate::RegValueT<Div_SPEC> as RegisterValue<_>>::new(1500)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Del_SPEC;
impl crate::sealed::RegSpec for Del_SPEC {
    type DataType = u32;
}

#[doc = "Data delay (Values must be under CDIV / 2)"]
pub type Del = crate::RegValueT<Del_SPEC>;

impl NoBitfieldReg<Del_SPEC> for Del {}
impl ::core::default::Default for Del {
    #[inline(always)]
    fn default() -> Del {
        <crate::RegValueT<Del_SPEC> as RegisterValue<_>>::new(3145776)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkt_SPEC;
impl crate::sealed::RegSpec for Clkt_SPEC {
    type DataType = u32;
}

#[doc = "Clock stretch timeout (broken on 283x)"]
pub type Clkt = crate::RegValueT<Clkt_SPEC>;

impl NoBitfieldReg<Clkt_SPEC> for Clkt {}
impl ::core::default::Default for Clkt {
    #[inline(always)]
    fn default() -> Clkt {
        <crate::RegValueT<Clkt_SPEC> as RegisterValue<_>>::new(0)
    }
}
