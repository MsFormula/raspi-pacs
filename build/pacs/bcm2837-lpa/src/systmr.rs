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
#[doc = r"Broadcom System Timer"]
unsafe impl ::core::marker::Send for super::Systmr {}
unsafe impl ::core::marker::Sync for super::Systmr {}
impl super::Systmr {
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

    #[doc = "Lower 32 bits for the free running counter"]
    #[inline(always)]
    pub const fn clo(&self) -> &'static crate::common::Reg<self::Clo_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Clo_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Higher 32 bits for the free running counter"]
    #[inline(always)]
    pub const fn chi(&self) -> &'static crate::common::Reg<self::Chi_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Chi_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Compare channel 0"]
    #[inline(always)]
    pub const fn c0(&self) -> &'static crate::common::Reg<self::C0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::C0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Compare channel 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &'static crate::common::Reg<self::C1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::C1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Compare channel 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &'static crate::common::Reg<self::C2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::C2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Compare channel 3"]
    #[inline(always)]
    pub const fn c3(&self) -> &'static crate::common::Reg<self::C3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::C3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
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

impl NoBitfieldReg<Cs_SPEC> for Cs {}
impl ::core::default::Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        <crate::RegValueT<Cs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clo_SPEC;
impl crate::sealed::RegSpec for Clo_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits for the free running counter"]
pub type Clo = crate::RegValueT<Clo_SPEC>;

impl NoBitfieldReg<Clo_SPEC> for Clo {}
impl ::core::default::Default for Clo {
    #[inline(always)]
    fn default() -> Clo {
        <crate::RegValueT<Clo_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chi_SPEC;
impl crate::sealed::RegSpec for Chi_SPEC {
    type DataType = u32;
}

#[doc = "Higher 32 bits for the free running counter"]
pub type Chi = crate::RegValueT<Chi_SPEC>;

impl NoBitfieldReg<Chi_SPEC> for Chi {}
impl ::core::default::Default for Chi {
    #[inline(always)]
    fn default() -> Chi {
        <crate::RegValueT<Chi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C0_SPEC;
impl crate::sealed::RegSpec for C0_SPEC {
    type DataType = u32;
}

#[doc = "Compare channel 0"]
pub type C0 = crate::RegValueT<C0_SPEC>;

impl NoBitfieldReg<C0_SPEC> for C0 {}
impl ::core::default::Default for C0 {
    #[inline(always)]
    fn default() -> C0 {
        <crate::RegValueT<C0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C1_SPEC;
impl crate::sealed::RegSpec for C1_SPEC {
    type DataType = u32;
}

#[doc = "Compare channel 1"]
pub type C1 = crate::RegValueT<C1_SPEC>;

impl NoBitfieldReg<C1_SPEC> for C1 {}
impl ::core::default::Default for C1 {
    #[inline(always)]
    fn default() -> C1 {
        <crate::RegValueT<C1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C2_SPEC;
impl crate::sealed::RegSpec for C2_SPEC {
    type DataType = u32;
}

#[doc = "Compare channel 2"]
pub type C2 = crate::RegValueT<C2_SPEC>;

impl NoBitfieldReg<C2_SPEC> for C2 {}
impl ::core::default::Default for C2 {
    #[inline(always)]
    fn default() -> C2 {
        <crate::RegValueT<C2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C3_SPEC;
impl crate::sealed::RegSpec for C3_SPEC {
    type DataType = u32;
}

#[doc = "Compare channel 3"]
pub type C3 = crate::RegValueT<C3_SPEC>;

impl NoBitfieldReg<C3_SPEC> for C3 {}
impl ::core::default::Default for C3 {
    #[inline(always)]
    fn default() -> C3 {
        <crate::RegValueT<C3_SPEC> as RegisterValue<_>>::new(0)
    }
}
