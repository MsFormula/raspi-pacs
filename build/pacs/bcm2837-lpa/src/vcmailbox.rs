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
#[doc = r"Mailboxes for talking to/from VideoCore"]
unsafe impl ::core::marker::Send for super::Vcmailbox {}
unsafe impl ::core::marker::Sync for super::Vcmailbox {}
impl super::Vcmailbox {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Read messages from the VideoCore"]
    #[inline(always)]
    pub const fn read(&self) -> &'static crate::common::Reg<self::Read_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Read_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn peek0(&self) -> &'static crate::common::Reg<self::Peek0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Peek0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sender0(
        &self,
    ) -> &'static crate::common::Reg<self::Sender0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sender0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn status0(
        &self,
    ) -> &'static crate::common::Reg<self::Status0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Status0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn config0(
        &self,
    ) -> &'static crate::common::Reg<self::Config0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Config0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Write messages to the VideoCore"]
    #[inline(always)]
    pub const fn write(&self) -> &'static crate::common::Reg<self::Write_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Write_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn peek1(&self) -> &'static crate::common::Reg<self::Peek1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Peek1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sender1(
        &self,
    ) -> &'static crate::common::Reg<self::Sender1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sender1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn status1(
        &self,
    ) -> &'static crate::common::Reg<self::Status1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Status1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn config1(
        &self,
    ) -> &'static crate::common::Reg<self::Config1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Config1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Read_SPEC;
impl crate::sealed::RegSpec for Read_SPEC {
    type DataType = u32;
}

#[doc = "Read messages from the VideoCore"]
pub type Read = crate::RegValueT<Read_SPEC>;

impl NoBitfieldReg<Read_SPEC> for Read {}
impl ::core::default::Default for Read {
    #[inline(always)]
    fn default() -> Read {
        <crate::RegValueT<Read_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek0_SPEC;
impl crate::sealed::RegSpec for Peek0_SPEC {
    type DataType = u32;
}

pub type Peek0 = crate::RegValueT<Peek0_SPEC>;

impl NoBitfieldReg<Peek0_SPEC> for Peek0 {}
impl ::core::default::Default for Peek0 {
    #[inline(always)]
    fn default() -> Peek0 {
        <crate::RegValueT<Peek0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sender0_SPEC;
impl crate::sealed::RegSpec for Sender0_SPEC {
    type DataType = u32;
}

pub type Sender0 = crate::RegValueT<Sender0_SPEC>;

impl NoBitfieldReg<Sender0_SPEC> for Sender0 {}
impl ::core::default::Default for Sender0 {
    #[inline(always)]
    fn default() -> Sender0 {
        <crate::RegValueT<Sender0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status0_SPEC;
impl crate::sealed::RegSpec for Status0_SPEC {
    type DataType = u32;
}

pub type Status0 = crate::RegValueT<Status0_SPEC>;

impl Status0 {
    #[inline(always)]
    pub fn full(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Status0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Status0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn empty(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Status0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Status0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Status0 {
    #[inline(always)]
    fn default() -> Status0 {
        <crate::RegValueT<Status0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config0_SPEC;
impl crate::sealed::RegSpec for Config0_SPEC {
    type DataType = u32;
}

pub type Config0 = crate::RegValueT<Config0_SPEC>;

impl Config0 {
    #[doc = "Enable the interrupt when data is available"]
    #[inline(always)]
    pub fn irqen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Config0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Config0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Config0 {
    #[inline(always)]
    fn default() -> Config0 {
        <crate::RegValueT<Config0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Write_SPEC;
impl crate::sealed::RegSpec for Write_SPEC {
    type DataType = u32;
}

#[doc = "Write messages to the VideoCore"]
pub type Write = crate::RegValueT<Write_SPEC>;

impl NoBitfieldReg<Write_SPEC> for Write {}
impl ::core::default::Default for Write {
    #[inline(always)]
    fn default() -> Write {
        <crate::RegValueT<Write_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek1_SPEC;
impl crate::sealed::RegSpec for Peek1_SPEC {
    type DataType = u32;
}

pub type Peek1 = crate::RegValueT<Peek1_SPEC>;

impl NoBitfieldReg<Peek1_SPEC> for Peek1 {}
impl ::core::default::Default for Peek1 {
    #[inline(always)]
    fn default() -> Peek1 {
        <crate::RegValueT<Peek1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sender1_SPEC;
impl crate::sealed::RegSpec for Sender1_SPEC {
    type DataType = u32;
}

pub type Sender1 = crate::RegValueT<Sender1_SPEC>;

impl NoBitfieldReg<Sender1_SPEC> for Sender1 {}
impl ::core::default::Default for Sender1 {
    #[inline(always)]
    fn default() -> Sender1 {
        <crate::RegValueT<Sender1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status1_SPEC;
impl crate::sealed::RegSpec for Status1_SPEC {
    type DataType = u32;
}

pub type Status1 = crate::RegValueT<Status1_SPEC>;

impl NoBitfieldReg<Status1_SPEC> for Status1 {}
impl ::core::default::Default for Status1 {
    #[inline(always)]
    fn default() -> Status1 {
        <crate::RegValueT<Status1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config1_SPEC;
impl crate::sealed::RegSpec for Config1_SPEC {
    type DataType = u32;
}

pub type Config1 = crate::RegValueT<Config1_SPEC>;

impl NoBitfieldReg<Config1_SPEC> for Config1 {}
impl ::core::default::Default for Config1 {
    #[inline(always)]
    fn default() -> Config1 {
        <crate::RegValueT<Config1_SPEC> as RegisterValue<_>>::new(0)
    }
}
