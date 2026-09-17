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
#[doc = r"Aux SPI"]
unsafe impl ::core::marker::Send for super::Spi1 {}
unsafe impl ::core::marker::Sync for super::Spi1 {}
impl super::Spi1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control 0"]
    #[inline(always)]
    pub const fn cntl0(&self) -> &'static crate::common::Reg<self::Cntl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cntl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Control 1"]
    #[inline(always)]
    pub const fn cntl1(&self) -> &'static crate::common::Reg<self::Cntl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cntl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &'static crate::common::Reg<self::Stat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Read the RXFIFO without removing an entry"]
    #[inline(always)]
    pub const fn peek(&self) -> &'static crate::common::Reg<self::Peek_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Peek_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Writing to the FIFO will deassert CS at the end of the access"]
    #[inline(always)]
    pub const fn io(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Io_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }

    #[doc = "Writing to the FIFO will maintain CS at the end of the access"]
    #[inline(always)]
    pub const fn txhold(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Txhold_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntl0_SPEC;
impl crate::sealed::RegSpec for Cntl0_SPEC {
    type DataType = u32;
}

#[doc = "Control 0"]
pub type Cntl0 = crate::RegValueT<Cntl0_SPEC>;

impl NoBitfieldReg<Cntl0_SPEC> for Cntl0 {}
impl ::core::default::Default for Cntl0 {
    #[inline(always)]
    fn default() -> Cntl0 {
        <crate::RegValueT<Cntl0_SPEC> as RegisterValue<_>>::new(917504)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntl1_SPEC;
impl crate::sealed::RegSpec for Cntl1_SPEC {
    type DataType = u32;
}

#[doc = "Control 1"]
pub type Cntl1 = crate::RegValueT<Cntl1_SPEC>;

impl NoBitfieldReg<Cntl1_SPEC> for Cntl1 {}
impl ::core::default::Default for Cntl1 {
    #[inline(always)]
    fn default() -> Cntl1 {
        <crate::RegValueT<Cntl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat_SPEC;
impl crate::sealed::RegSpec for Stat_SPEC {
    type DataType = u32;
}

#[doc = "Status"]
pub type Stat = crate::RegValueT<Stat_SPEC>;

impl NoBitfieldReg<Stat_SPEC> for Stat {}
impl ::core::default::Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        <crate::RegValueT<Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek_SPEC;
impl crate::sealed::RegSpec for Peek_SPEC {
    type DataType = u32;
}

#[doc = "Read the RXFIFO without removing an entry"]
pub type Peek = crate::RegValueT<Peek_SPEC>;

impl NoBitfieldReg<Peek_SPEC> for Peek {}
impl ::core::default::Default for Peek {
    #[inline(always)]
    fn default() -> Peek {
        <crate::RegValueT<Peek_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io_SPEC;
impl crate::sealed::RegSpec for Io_SPEC {
    type DataType = u32;
}

#[doc = "Writing to the FIFO will deassert CS at the end of the access"]
pub type Io = crate::RegValueT<Io_SPEC>;

impl NoBitfieldReg<Io_SPEC> for Io {}
impl ::core::default::Default for Io {
    #[inline(always)]
    fn default() -> Io {
        <crate::RegValueT<Io_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txhold_SPEC;
impl crate::sealed::RegSpec for Txhold_SPEC {
    type DataType = u32;
}

#[doc = "Writing to the FIFO will maintain CS at the end of the access"]
pub type Txhold = crate::RegValueT<Txhold_SPEC>;

impl NoBitfieldReg<Txhold_SPEC> for Txhold {}
impl ::core::default::Default for Txhold {
    #[inline(always)]
    fn default() -> Txhold {
        <crate::RegValueT<Txhold_SPEC> as RegisterValue<_>>::new(0)
    }
}
