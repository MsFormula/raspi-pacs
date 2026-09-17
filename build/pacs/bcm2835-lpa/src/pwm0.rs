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
#[doc = r"Broadcom PWM"]
unsafe impl ::core::marker::Send for super::Pwm0 {}
unsafe impl ::core::marker::Sync for super::Pwm0 {}
impl super::Pwm0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &'static crate::common::Reg<self::Ctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub const fn sta(&self) -> &'static crate::common::Reg<self::Sta_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sta_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DMA control"]
    #[inline(always)]
    pub const fn dmac(&self) -> &'static crate::common::Reg<self::Dmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Range for channel 1"]
    #[inline(always)]
    pub const fn rng1(&self) -> &'static crate::common::Reg<self::Rng1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rng1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Channel 1 data"]
    #[inline(always)]
    pub const fn dat1(&self) -> &'static crate::common::Reg<self::Dat1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dat1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "FIFO input"]
    #[inline(always)]
    pub const fn fif1(&self) -> &'static crate::common::Reg<self::Fif1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Fif1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Range for channel 2"]
    #[inline(always)]
    pub const fn rng2(&self) -> &'static crate::common::Reg<self::Rng2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rng2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Channel 2 data"]
    #[inline(always)]
    pub const fn dat2(&self) -> &'static crate::common::Reg<self::Dat2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dat2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl_SPEC;
impl crate::sealed::RegSpec for Ctl_SPEC {
    type DataType = u32;
}

#[doc = "Control"]
pub type Ctl = crate::RegValueT<Ctl_SPEC>;

impl NoBitfieldReg<Ctl_SPEC> for Ctl {}
impl ::core::default::Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        <crate::RegValueT<Ctl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sta_SPEC;
impl crate::sealed::RegSpec for Sta_SPEC {
    type DataType = u32;
}

#[doc = "Status"]
pub type Sta = crate::RegValueT<Sta_SPEC>;

impl NoBitfieldReg<Sta_SPEC> for Sta {}
impl ::core::default::Default for Sta {
    #[inline(always)]
    fn default() -> Sta {
        <crate::RegValueT<Sta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac_SPEC;
impl crate::sealed::RegSpec for Dmac_SPEC {
    type DataType = u32;
}

#[doc = "DMA control"]
pub type Dmac = crate::RegValueT<Dmac_SPEC>;

impl NoBitfieldReg<Dmac_SPEC> for Dmac {}
impl ::core::default::Default for Dmac {
    #[inline(always)]
    fn default() -> Dmac {
        <crate::RegValueT<Dmac_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng1_SPEC;
impl crate::sealed::RegSpec for Rng1_SPEC {
    type DataType = u32;
}

#[doc = "Range for channel 1"]
pub type Rng1 = crate::RegValueT<Rng1_SPEC>;

impl NoBitfieldReg<Rng1_SPEC> for Rng1 {}
impl ::core::default::Default for Rng1 {
    #[inline(always)]
    fn default() -> Rng1 {
        <crate::RegValueT<Rng1_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dat1_SPEC;
impl crate::sealed::RegSpec for Dat1_SPEC {
    type DataType = u32;
}

#[doc = "Channel 1 data"]
pub type Dat1 = crate::RegValueT<Dat1_SPEC>;

impl NoBitfieldReg<Dat1_SPEC> for Dat1 {}
impl ::core::default::Default for Dat1 {
    #[inline(always)]
    fn default() -> Dat1 {
        <crate::RegValueT<Dat1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fif1_SPEC;
impl crate::sealed::RegSpec for Fif1_SPEC {
    type DataType = u32;
}

#[doc = "FIFO input"]
pub type Fif1 = crate::RegValueT<Fif1_SPEC>;

impl NoBitfieldReg<Fif1_SPEC> for Fif1 {}
impl ::core::default::Default for Fif1 {
    #[inline(always)]
    fn default() -> Fif1 {
        <crate::RegValueT<Fif1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng2_SPEC;
impl crate::sealed::RegSpec for Rng2_SPEC {
    type DataType = u32;
}

#[doc = "Range for channel 2"]
pub type Rng2 = crate::RegValueT<Rng2_SPEC>;

impl NoBitfieldReg<Rng2_SPEC> for Rng2 {}
impl ::core::default::Default for Rng2 {
    #[inline(always)]
    fn default() -> Rng2 {
        <crate::RegValueT<Rng2_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dat2_SPEC;
impl crate::sealed::RegSpec for Dat2_SPEC {
    type DataType = u32;
}

#[doc = "Channel 2 data"]
pub type Dat2 = crate::RegValueT<Dat2_SPEC>;

impl NoBitfieldReg<Dat2_SPEC> for Dat2 {}
impl ::core::default::Default for Dat2 {
    #[inline(always)]
    fn default() -> Dat2 {
        <crate::RegValueT<Dat2_SPEC> as RegisterValue<_>>::new(0)
    }
}
