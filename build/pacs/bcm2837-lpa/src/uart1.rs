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
#[doc = r"Mini UART"]
unsafe impl ::core::marker::Send for super::Uart1 {}
unsafe impl ::core::marker::Sync for super::Uart1 {}
impl super::Uart1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "I/O Data"]
    #[inline(always)]
    pub const fn io(&self) -> &'static crate::common::Reg<self::Io_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Io_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Lower bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudl(&self) -> &'static crate::common::Reg<self::Baudl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Baudl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(&self) -> &'static crate::common::Reg<self::Ier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "High bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudh(&self) -> &'static crate::common::Reg<self::Baudh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Baudh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Identify"]
    #[inline(always)]
    pub const fn iir(&self) -> &'static crate::common::Reg<self::Iir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Iir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Line control"]
    #[inline(always)]
    pub const fn lcr(&self) -> &'static crate::common::Reg<self::Lcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Modem Control"]
    #[inline(always)]
    pub const fn mcr(&self) -> &'static crate::common::Reg<self::Mcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Line Status"]
    #[inline(always)]
    pub const fn lsr(&self) -> &'static crate::common::Reg<self::Lsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Modem Status"]
    #[inline(always)]
    pub const fn msr(&self) -> &'static crate::common::Reg<self::Msr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Scratch"]
    #[inline(always)]
    pub const fn scratch(
        &self,
    ) -> &'static crate::common::Reg<self::Scratch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scratch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Control"]
    #[inline(always)]
    pub const fn cntl(&self) -> &'static crate::common::Reg<self::Cntl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cntl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &'static crate::common::Reg<self::Stat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Baudrate"]
    #[inline(always)]
    pub const fn baud(&self) -> &'static crate::common::Reg<self::Baud_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Baud_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io_SPEC;
impl crate::sealed::RegSpec for Io_SPEC {
    type DataType = u32;
}

#[doc = "I/O Data"]
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
pub struct Baudl_SPEC;
impl crate::sealed::RegSpec for Baudl_SPEC {
    type DataType = u8;
}

#[doc = "Lower bits of baudrate when DLAB is set"]
pub type Baudl = crate::RegValueT<Baudl_SPEC>;

impl NoBitfieldReg<Baudl_SPEC> for Baudl {}
impl ::core::default::Default for Baudl {
    #[inline(always)]
    fn default() -> Baudl {
        <crate::RegValueT<Baudl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier_SPEC;
impl crate::sealed::RegSpec for Ier_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Enable"]
pub type Ier = crate::RegValueT<Ier_SPEC>;

impl NoBitfieldReg<Ier_SPEC> for Ier {}
impl ::core::default::Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        <crate::RegValueT<Ier_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baudh_SPEC;
impl crate::sealed::RegSpec for Baudh_SPEC {
    type DataType = u8;
}

#[doc = "High bits of baudrate when DLAB is set"]
pub type Baudh = crate::RegValueT<Baudh_SPEC>;

impl NoBitfieldReg<Baudh_SPEC> for Baudh {}
impl ::core::default::Default for Baudh {
    #[inline(always)]
    fn default() -> Baudh {
        <crate::RegValueT<Baudh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iir_SPEC;
impl crate::sealed::RegSpec for Iir_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Identify"]
pub type Iir = crate::RegValueT<Iir_SPEC>;

impl NoBitfieldReg<Iir_SPEC> for Iir {}
impl ::core::default::Default for Iir {
    #[inline(always)]
    fn default() -> Iir {
        <crate::RegValueT<Iir_SPEC> as RegisterValue<_>>::new(45057)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcr_SPEC;
impl crate::sealed::RegSpec for Lcr_SPEC {
    type DataType = u32;
}

#[doc = "Line control"]
pub type Lcr = crate::RegValueT<Lcr_SPEC>;

impl NoBitfieldReg<Lcr_SPEC> for Lcr {}
impl ::core::default::Default for Lcr {
    #[inline(always)]
    fn default() -> Lcr {
        <crate::RegValueT<Lcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr_SPEC;
impl crate::sealed::RegSpec for Mcr_SPEC {
    type DataType = u32;
}

#[doc = "Modem Control"]
pub type Mcr = crate::RegValueT<Mcr_SPEC>;

impl NoBitfieldReg<Mcr_SPEC> for Mcr {}
impl ::core::default::Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        <crate::RegValueT<Mcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lsr_SPEC;
impl crate::sealed::RegSpec for Lsr_SPEC {
    type DataType = u32;
}

#[doc = "Line Status"]
pub type Lsr = crate::RegValueT<Lsr_SPEC>;

impl NoBitfieldReg<Lsr_SPEC> for Lsr {}
impl ::core::default::Default for Lsr {
    #[inline(always)]
    fn default() -> Lsr {
        <crate::RegValueT<Lsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr_SPEC;
impl crate::sealed::RegSpec for Msr_SPEC {
    type DataType = u32;
}

#[doc = "Modem Status"]
pub type Msr = crate::RegValueT<Msr_SPEC>;

impl NoBitfieldReg<Msr_SPEC> for Msr {}
impl ::core::default::Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        <crate::RegValueT<Msr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scratch_SPEC;
impl crate::sealed::RegSpec for Scratch_SPEC {
    type DataType = u8;
}

#[doc = "Scratch"]
pub type Scratch = crate::RegValueT<Scratch_SPEC>;

impl NoBitfieldReg<Scratch_SPEC> for Scratch {}
impl ::core::default::Default for Scratch {
    #[inline(always)]
    fn default() -> Scratch {
        <crate::RegValueT<Scratch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntl_SPEC;
impl crate::sealed::RegSpec for Cntl_SPEC {
    type DataType = u32;
}

#[doc = "Control"]
pub type Cntl = crate::RegValueT<Cntl_SPEC>;

impl NoBitfieldReg<Cntl_SPEC> for Cntl {}
impl ::core::default::Default for Cntl {
    #[inline(always)]
    fn default() -> Cntl {
        <crate::RegValueT<Cntl_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Baud_SPEC;
impl crate::sealed::RegSpec for Baud_SPEC {
    type DataType = u16;
}

#[doc = "Baudrate"]
pub type Baud = crate::RegValueT<Baud_SPEC>;

impl NoBitfieldReg<Baud_SPEC> for Baud {}
impl ::core::default::Default for Baud {
    #[inline(always)]
    fn default() -> Baud {
        <crate::RegValueT<Baud_SPEC> as RegisterValue<_>>::new(0)
    }
}
