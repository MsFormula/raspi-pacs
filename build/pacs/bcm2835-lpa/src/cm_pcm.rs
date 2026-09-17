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
#[doc = r"Broadcom Clock Manager"]
unsafe impl ::core::marker::Send for super::CmPcm {}
unsafe impl ::core::marker::Sync for super::CmPcm {}
impl super::CmPcm {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control / Status"]
    #[inline(always)]
    pub const fn cs(&self) -> &'static crate::common::Reg<self::Cs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Clock divisor"]
    #[inline(always)]
    pub const fn div(&self) -> &'static crate::common::Reg<self::Div_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Div_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
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

#[doc = "Control / Status"]
pub type Cs = crate::RegValueT<Cs_SPEC>;

impl Cs {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        cs::Passwd,
        cs::Passwd,
        Cs_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            cs::Passwd,
            cs::Passwd,
            Cs_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Indicates the clock generator is running"]
    #[inline(always)]
    pub fn busy(self) -> crate::common::RegisterFieldBool<7, 1, 0, Cs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        <crate::RegValueT<Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Passwd_SPEC;
    pub type Passwd = crate::EnumBitfieldStruct<u8, Passwd_SPEC>;
    impl Passwd {
        pub const PASSWD: Self = Self::new(90);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div_SPEC;
impl crate::sealed::RegSpec for Div_SPEC {
    type DataType = u32;
}

#[doc = "Clock divisor"]
pub type Div = crate::RegValueT<Div_SPEC>;

impl Div {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        div::Passwd,
        div::Passwd,
        Div_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            div::Passwd,
            div::Passwd,
            Div_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Div {
    #[inline(always)]
    fn default() -> Div {
        <crate::RegValueT<Div_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod div {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Passwd_SPEC;
    pub type Passwd = crate::EnumBitfieldStruct<u8, Passwd_SPEC>;
    impl Passwd {
        pub const PASSWD: Self = Self::new(90);
    }
}
