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
// Generated from SVD A, with svd2pac 0.7.0 on Mon, 14 Sep 2026 00:08:42 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Broadcom Legacy Interrupt Controller"]
unsafe impl ::core::marker::Send for super::BcmLic {}
unsafe impl ::core::marker::Sync for super::BcmLic {}
impl super::BcmLic {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Basic pending info"]
    #[inline(always)]
    pub const fn basic_pending(
        &self,
    ) -> &'static crate::common::Reg<self::BasicPending_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::BasicPending_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Pending state for interrupts 1 - 31"]
    #[inline(always)]
    pub const fn pending_1(
        &self,
    ) -> &'static crate::common::Reg<self::Pending1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pending1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "Pending state for interrupts 32 - 63"]
    #[inline(always)]
    pub const fn pending_2(
        &self,
    ) -> &'static crate::common::Reg<self::Pending2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pending2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[doc = "FIQ control"]
    #[inline(always)]
    pub const fn fiq_control(
        &self,
    ) -> &'static crate::common::Reg<self::FiqControl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FiqControl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(524usize),
            )
        }
    }

    #[doc = "Enable interrupts 1 - 31"]
    #[inline(always)]
    pub const fn enable_1(
        &self,
    ) -> &'static crate::common::Reg<self::Enable1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Enable1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "Enable interrupts 32 - 63"]
    #[inline(always)]
    pub const fn enable_2(
        &self,
    ) -> &'static crate::common::Reg<self::Enable2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Enable2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "Enable basic interrupts"]
    #[inline(always)]
    pub const fn enable_basic(
        &self,
    ) -> &'static crate::common::Reg<self::EnableBasic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EnableBasic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "Disable interrupts 1 - 31"]
    #[inline(always)]
    pub const fn disable_1(
        &self,
    ) -> &'static crate::common::Reg<self::Disable1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Disable1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(540usize),
            )
        }
    }

    #[doc = "Disable interrupts 32 - 63"]
    #[inline(always)]
    pub const fn disable_2(
        &self,
    ) -> &'static crate::common::Reg<self::Disable2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Disable2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[doc = "Disable basic interrupts"]
    #[inline(always)]
    pub const fn disable_basic(
        &self,
    ) -> &'static crate::common::Reg<self::DisableBasic_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DisableBasic_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(548usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BasicPending_SPEC;
impl crate::sealed::RegSpec for BasicPending_SPEC {
    type DataType = u32;
}

#[doc = "Basic pending info"]
pub type BasicPending = crate::RegValueT<BasicPending_SPEC>;

impl NoBitfieldReg<BasicPending_SPEC> for BasicPending {}
impl ::core::default::Default for BasicPending {
    #[inline(always)]
    fn default() -> BasicPending {
        <crate::RegValueT<BasicPending_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending1_SPEC;
impl crate::sealed::RegSpec for Pending1_SPEC {
    type DataType = u32;
}

#[doc = "Pending state for interrupts 1 - 31"]
pub type Pending1 = crate::RegValueT<Pending1_SPEC>;

impl NoBitfieldReg<Pending1_SPEC> for Pending1 {}
impl ::core::default::Default for Pending1 {
    #[inline(always)]
    fn default() -> Pending1 {
        <crate::RegValueT<Pending1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pending2_SPEC;
impl crate::sealed::RegSpec for Pending2_SPEC {
    type DataType = u32;
}

#[doc = "Pending state for interrupts 32 - 63"]
pub type Pending2 = crate::RegValueT<Pending2_SPEC>;

impl NoBitfieldReg<Pending2_SPEC> for Pending2 {}
impl ::core::default::Default for Pending2 {
    #[inline(always)]
    fn default() -> Pending2 {
        <crate::RegValueT<Pending2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FiqControl_SPEC;
impl crate::sealed::RegSpec for FiqControl_SPEC {
    type DataType = u32;
}

#[doc = "FIQ control"]
pub type FiqControl = crate::RegValueT<FiqControl_SPEC>;

impl NoBitfieldReg<FiqControl_SPEC> for FiqControl {}
impl ::core::default::Default for FiqControl {
    #[inline(always)]
    fn default() -> FiqControl {
        <crate::RegValueT<FiqControl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable1_SPEC;
impl crate::sealed::RegSpec for Enable1_SPEC {
    type DataType = u32;
}

#[doc = "Enable interrupts 1 - 31"]
pub type Enable1 = crate::RegValueT<Enable1_SPEC>;

impl NoBitfieldReg<Enable1_SPEC> for Enable1 {}
impl ::core::default::Default for Enable1 {
    #[inline(always)]
    fn default() -> Enable1 {
        <crate::RegValueT<Enable1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable2_SPEC;
impl crate::sealed::RegSpec for Enable2_SPEC {
    type DataType = u32;
}

#[doc = "Enable interrupts 32 - 63"]
pub type Enable2 = crate::RegValueT<Enable2_SPEC>;

impl NoBitfieldReg<Enable2_SPEC> for Enable2 {}
impl ::core::default::Default for Enable2 {
    #[inline(always)]
    fn default() -> Enable2 {
        <crate::RegValueT<Enable2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnableBasic_SPEC;
impl crate::sealed::RegSpec for EnableBasic_SPEC {
    type DataType = u32;
}

#[doc = "Enable basic interrupts"]
pub type EnableBasic = crate::RegValueT<EnableBasic_SPEC>;

impl NoBitfieldReg<EnableBasic_SPEC> for EnableBasic {}
impl ::core::default::Default for EnableBasic {
    #[inline(always)]
    fn default() -> EnableBasic {
        <crate::RegValueT<EnableBasic_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disable1_SPEC;
impl crate::sealed::RegSpec for Disable1_SPEC {
    type DataType = u32;
}

#[doc = "Disable interrupts 1 - 31"]
pub type Disable1 = crate::RegValueT<Disable1_SPEC>;

impl NoBitfieldReg<Disable1_SPEC> for Disable1 {}
impl ::core::default::Default for Disable1 {
    #[inline(always)]
    fn default() -> Disable1 {
        <crate::RegValueT<Disable1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disable2_SPEC;
impl crate::sealed::RegSpec for Disable2_SPEC {
    type DataType = u32;
}

#[doc = "Disable interrupts 32 - 63"]
pub type Disable2 = crate::RegValueT<Disable2_SPEC>;

impl NoBitfieldReg<Disable2_SPEC> for Disable2 {}
impl ::core::default::Default for Disable2 {
    #[inline(always)]
    fn default() -> Disable2 {
        <crate::RegValueT<Disable2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DisableBasic_SPEC;
impl crate::sealed::RegSpec for DisableBasic_SPEC {
    type DataType = u32;
}

#[doc = "Disable basic interrupts"]
pub type DisableBasic = crate::RegValueT<DisableBasic_SPEC>;

impl NoBitfieldReg<DisableBasic_SPEC> for DisableBasic {}
impl ::core::default::Default for DisableBasic {
    #[inline(always)]
    fn default() -> DisableBasic {
        <crate::RegValueT<DisableBasic_SPEC> as RegisterValue<_>>::new(0)
    }
}
