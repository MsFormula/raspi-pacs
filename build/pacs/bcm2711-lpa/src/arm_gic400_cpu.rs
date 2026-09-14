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
// Generated from SVD A, with svd2pac 0.7.0 on Mon, 14 Sep 2026 00:08:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ARM GIC-400 Generic Interrupt Controller CPU Interface"]
unsafe impl ::core::marker::Send for super::ArmGic400Cpu {}
unsafe impl ::core::marker::Sync for super::ArmGic400Cpu {}
impl super::ArmGic400Cpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "CPU Interface Control"]
    #[inline(always)]
    pub const fn gicc_ctlr(
        &self,
    ) -> &'static crate::common::Reg<self::GiccCtlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccCtlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Priority Mask"]
    #[inline(always)]
    pub const fn gicc_pmr(
        &self,
    ) -> &'static crate::common::Reg<self::GiccPmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccPmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Binary Point"]
    #[inline(always)]
    pub const fn gicc_bpr(
        &self,
    ) -> &'static crate::common::Reg<self::GiccBpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccBpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Acknowledge"]
    #[inline(always)]
    pub const fn gicc_iar(
        &self,
    ) -> &'static crate::common::Reg<self::GiccIar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GiccIar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "End of Interrupt"]
    #[inline(always)]
    pub const fn gicc_eoir(
        &self,
    ) -> &'static crate::common::Reg<self::GiccEoir_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::GiccEoir_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Running Priority"]
    #[inline(always)]
    pub const fn gicc_rpr(
        &self,
    ) -> &'static crate::common::Reg<self::GiccRpr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GiccRpr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Highest Priority Pending Interrupt"]
    #[inline(always)]
    pub const fn gicc_hppir(
        &self,
    ) -> &'static crate::common::Reg<self::GiccHppir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccHppir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Aliased Binary Point"]
    #[inline(always)]
    pub const fn gicc_abpr(
        &self,
    ) -> &'static crate::common::Reg<self::GiccAbpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccAbpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Aliased Interrupt Acknowledge"]
    #[inline(always)]
    pub const fn gicc_aiar(
        &self,
    ) -> &'static crate::common::Reg<self::GiccAiar_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GiccAiar_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Aliased End of Interrupt"]
    #[inline(always)]
    pub const fn gicc_aeoir(
        &self,
    ) -> &'static crate::common::Reg<self::GiccAeoir_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::GiccAeoir_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Aliased Highest Priority Pending Interrupt"]
    #[inline(always)]
    pub const fn gicc_ahppir(
        &self,
    ) -> &'static crate::common::Reg<self::GiccAhppir_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GiccAhppir_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Active Priority"]
    #[inline(always)]
    pub const fn gicc_apr0(
        &self,
    ) -> &'static crate::common::Reg<self::GiccApr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccApr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Non-Secure Active Priority"]
    #[inline(always)]
    pub const fn gicc_nsapr0(
        &self,
    ) -> &'static crate::common::Reg<self::GiccNsapr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccNsapr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[doc = "CPU Interface Identification Register"]
    #[inline(always)]
    pub const fn gicc_iidr(
        &self,
    ) -> &'static crate::common::Reg<self::GiccIidr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GiccIidr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[doc = "Deactivate Interrupt"]
    #[inline(always)]
    pub const fn gicc_dir(
        &self,
    ) -> &'static crate::common::Reg<self::GiccDir_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::GiccDir_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccCtlr_SPEC;
impl crate::sealed::RegSpec for GiccCtlr_SPEC {
    type DataType = u32;
}

#[doc = "CPU Interface Control"]
pub type GiccCtlr = crate::RegValueT<GiccCtlr_SPEC>;

impl NoBitfieldReg<GiccCtlr_SPEC> for GiccCtlr {}
impl ::core::default::Default for GiccCtlr {
    #[inline(always)]
    fn default() -> GiccCtlr {
        <crate::RegValueT<GiccCtlr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccPmr_SPEC;
impl crate::sealed::RegSpec for GiccPmr_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Priority Mask"]
pub type GiccPmr = crate::RegValueT<GiccPmr_SPEC>;

impl NoBitfieldReg<GiccPmr_SPEC> for GiccPmr {}
impl ::core::default::Default for GiccPmr {
    #[inline(always)]
    fn default() -> GiccPmr {
        <crate::RegValueT<GiccPmr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccBpr_SPEC;
impl crate::sealed::RegSpec for GiccBpr_SPEC {
    type DataType = u32;
}

#[doc = "Binary Point"]
pub type GiccBpr = crate::RegValueT<GiccBpr_SPEC>;

impl NoBitfieldReg<GiccBpr_SPEC> for GiccBpr {}
impl ::core::default::Default for GiccBpr {
    #[inline(always)]
    fn default() -> GiccBpr {
        <crate::RegValueT<GiccBpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccIar_SPEC;
impl crate::sealed::RegSpec for GiccIar_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Acknowledge"]
pub type GiccIar = crate::RegValueT<GiccIar_SPEC>;

impl NoBitfieldReg<GiccIar_SPEC> for GiccIar {}
impl ::core::default::Default for GiccIar {
    #[inline(always)]
    fn default() -> GiccIar {
        <crate::RegValueT<GiccIar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccEoir_SPEC;
impl crate::sealed::RegSpec for GiccEoir_SPEC {
    type DataType = u32;
}

#[doc = "End of Interrupt"]
pub type GiccEoir = crate::RegValueT<GiccEoir_SPEC>;

impl NoBitfieldReg<GiccEoir_SPEC> for GiccEoir {}
impl ::core::default::Default for GiccEoir {
    #[inline(always)]
    fn default() -> GiccEoir {
        <crate::RegValueT<GiccEoir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccRpr_SPEC;
impl crate::sealed::RegSpec for GiccRpr_SPEC {
    type DataType = u32;
}

#[doc = "Running Priority"]
pub type GiccRpr = crate::RegValueT<GiccRpr_SPEC>;

impl NoBitfieldReg<GiccRpr_SPEC> for GiccRpr {}
impl ::core::default::Default for GiccRpr {
    #[inline(always)]
    fn default() -> GiccRpr {
        <crate::RegValueT<GiccRpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccHppir_SPEC;
impl crate::sealed::RegSpec for GiccHppir_SPEC {
    type DataType = u32;
}

#[doc = "Highest Priority Pending Interrupt"]
pub type GiccHppir = crate::RegValueT<GiccHppir_SPEC>;

impl NoBitfieldReg<GiccHppir_SPEC> for GiccHppir {}
impl ::core::default::Default for GiccHppir {
    #[inline(always)]
    fn default() -> GiccHppir {
        <crate::RegValueT<GiccHppir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAbpr_SPEC;
impl crate::sealed::RegSpec for GiccAbpr_SPEC {
    type DataType = u32;
}

#[doc = "Aliased Binary Point"]
pub type GiccAbpr = crate::RegValueT<GiccAbpr_SPEC>;

impl NoBitfieldReg<GiccAbpr_SPEC> for GiccAbpr {}
impl ::core::default::Default for GiccAbpr {
    #[inline(always)]
    fn default() -> GiccAbpr {
        <crate::RegValueT<GiccAbpr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAiar_SPEC;
impl crate::sealed::RegSpec for GiccAiar_SPEC {
    type DataType = u32;
}

#[doc = "Aliased Interrupt Acknowledge"]
pub type GiccAiar = crate::RegValueT<GiccAiar_SPEC>;

impl NoBitfieldReg<GiccAiar_SPEC> for GiccAiar {}
impl ::core::default::Default for GiccAiar {
    #[inline(always)]
    fn default() -> GiccAiar {
        <crate::RegValueT<GiccAiar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAeoir_SPEC;
impl crate::sealed::RegSpec for GiccAeoir_SPEC {
    type DataType = u32;
}

#[doc = "Aliased End of Interrupt"]
pub type GiccAeoir = crate::RegValueT<GiccAeoir_SPEC>;

impl NoBitfieldReg<GiccAeoir_SPEC> for GiccAeoir {}
impl ::core::default::Default for GiccAeoir {
    #[inline(always)]
    fn default() -> GiccAeoir {
        <crate::RegValueT<GiccAeoir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccAhppir_SPEC;
impl crate::sealed::RegSpec for GiccAhppir_SPEC {
    type DataType = u32;
}

#[doc = "Aliased Highest Priority Pending Interrupt"]
pub type GiccAhppir = crate::RegValueT<GiccAhppir_SPEC>;

impl NoBitfieldReg<GiccAhppir_SPEC> for GiccAhppir {}
impl ::core::default::Default for GiccAhppir {
    #[inline(always)]
    fn default() -> GiccAhppir {
        <crate::RegValueT<GiccAhppir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccApr0_SPEC;
impl crate::sealed::RegSpec for GiccApr0_SPEC {
    type DataType = u32;
}

#[doc = "Active Priority"]
pub type GiccApr0 = crate::RegValueT<GiccApr0_SPEC>;

impl NoBitfieldReg<GiccApr0_SPEC> for GiccApr0 {}
impl ::core::default::Default for GiccApr0 {
    #[inline(always)]
    fn default() -> GiccApr0 {
        <crate::RegValueT<GiccApr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccNsapr0_SPEC;
impl crate::sealed::RegSpec for GiccNsapr0_SPEC {
    type DataType = u32;
}

#[doc = "Non-Secure Active Priority"]
pub type GiccNsapr0 = crate::RegValueT<GiccNsapr0_SPEC>;

impl NoBitfieldReg<GiccNsapr0_SPEC> for GiccNsapr0 {}
impl ::core::default::Default for GiccNsapr0 {
    #[inline(always)]
    fn default() -> GiccNsapr0 {
        <crate::RegValueT<GiccNsapr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccIidr_SPEC;
impl crate::sealed::RegSpec for GiccIidr_SPEC {
    type DataType = u32;
}

#[doc = "CPU Interface Identification Register"]
pub type GiccIidr = crate::RegValueT<GiccIidr_SPEC>;

impl NoBitfieldReg<GiccIidr_SPEC> for GiccIidr {}
impl ::core::default::Default for GiccIidr {
    #[inline(always)]
    fn default() -> GiccIidr {
        <crate::RegValueT<GiccIidr_SPEC> as RegisterValue<_>>::new(33690683)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GiccDir_SPEC;
impl crate::sealed::RegSpec for GiccDir_SPEC {
    type DataType = u32;
}

#[doc = "Deactivate Interrupt"]
pub type GiccDir = crate::RegValueT<GiccDir_SPEC>;

impl NoBitfieldReg<GiccDir_SPEC> for GiccDir {}
impl ::core::default::Default for GiccDir {
    #[inline(always)]
    fn default() -> GiccDir {
        <crate::RegValueT<GiccDir_SPEC> as RegisterValue<_>>::new(0)
    }
}
