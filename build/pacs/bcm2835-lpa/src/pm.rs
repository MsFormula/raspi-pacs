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
#[doc = r"Broadcom Power Manager"]
unsafe impl ::core::marker::Send for super::Pm {}
unsafe impl ::core::marker::Sync for super::Pm {}
impl super::Pm {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Reset Control"]
    #[inline(always)]
    pub const fn rstc(&self) -> &'static crate::common::Reg<self::Rstc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Watchdog control"]
    #[inline(always)]
    pub const fn wdog(&self) -> &'static crate::common::Reg<self::Wdog_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Wdog_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstc_SPEC;
impl crate::sealed::RegSpec for Rstc_SPEC {
    type DataType = u32;
}

#[doc = "Reset Control"]
pub type Rstc = crate::RegValueT<Rstc_SPEC>;

impl Rstc {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        rstc::Passwd,
        rstc::Passwd,
        Rstc_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            rstc::Passwd,
            rstc::Passwd,
            Rstc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rstc {
    #[inline(always)]
    fn default() -> Rstc {
        <crate::RegValueT<Rstc_SPEC> as RegisterValue<_>>::new(258)
    }
}
pub mod rstc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Passwd_SPEC;
    pub type Passwd = crate::EnumBitfieldStruct<u8, Passwd_SPEC>;
    impl Passwd {
        pub const PASSWD: Self = Self::new(90);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdog_SPEC;
impl crate::sealed::RegSpec for Wdog_SPEC {
    type DataType = u32;
}

#[doc = "Watchdog control"]
pub type Wdog = crate::RegValueT<Wdog_SPEC>;

impl Wdog {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        wdog::Passwd,
        wdog::Passwd,
        Wdog_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            wdog::Passwd,
            wdog::Passwd,
            Wdog_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Wdog {
    #[inline(always)]
    fn default() -> Wdog {
        <crate::RegValueT<Wdog_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wdog {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Passwd_SPEC;
    pub type Passwd = crate::EnumBitfieldStruct<u8, Passwd_SPEC>;
    impl Passwd {
        pub const PASSWD: Self = Self::new(90);
    }
}
