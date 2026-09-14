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
#[doc = r"ARM GIC-400 Generic Interrupt Controller Distributor"]
unsafe impl ::core::marker::Send for super::ArmGic400Distributor {}
unsafe impl ::core::marker::Sync for super::ArmGic400Distributor {}
impl super::ArmGic400Distributor {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Distributor Control Register"]
    #[inline(always)]
    pub const fn gicd_ctlr(
        &self,
    ) -> &'static crate::common::Reg<self::GicdCtlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdCtlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Controller Type Register"]
    #[inline(always)]
    pub const fn gicd_typer(
        &self,
    ) -> &'static crate::common::Reg<self::GicdTyper_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdTyper_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Distributor Implementer Identification Register"]
    #[inline(always)]
    pub const fn gicd_iidr(
        &self,
    ) -> &'static crate::common::Reg<self::GicdIidr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdIidr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Private Peripheral Interrupt Status Register"]
    #[inline(always)]
    pub const fn gicd_ppisr(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPpisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdPpisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3328usize),
            )
        }
    }

    #[doc = "Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr0(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSpisr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdSpisr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3332usize),
            )
        }
    }

    #[doc = "Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr1(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSpisr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdSpisr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3336usize),
            )
        }
    }

    #[doc = "Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr2(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSpisr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdSpisr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3340usize),
            )
        }
    }

    #[doc = "Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr3(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSpisr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdSpisr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3344usize),
            )
        }
    }

    #[doc = "Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr4(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSpisr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdSpisr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3348usize),
            )
        }
    }

    #[doc = "Shared Peripheral Interrupt Status Registers"]
    #[inline(always)]
    pub const fn gicd_spisr5(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSpisr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdSpisr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3352usize),
            )
        }
    }

    #[doc = "Software Generated Interrupt Register"]
    #[inline(always)]
    pub const fn gicd_sgir(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSgir_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::GicdSgir_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(3840usize),
            )
        }
    }

    #[doc = "SGI Clear-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_cpendsgirn(
        &self,
    ) -> &'static crate::common::Reg<self::GicdCpendsgiRn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdCpendsgiRn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3856usize),
            )
        }
    }

    #[doc = "SGI Set-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_spendsgirn(
        &self,
    ) -> &'static crate::common::Reg<self::GicdSpendsgiRn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GicdSpendsgiRn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3872usize),
            )
        }
    }

    #[doc = "Peripheral ID 4"]
    #[inline(always)]
    pub const fn gicd_pidr4(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr4_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr4_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4048usize),
            )
        }
    }

    #[doc = "Peripheral ID 5"]
    #[inline(always)]
    pub const fn gicd_pidr5(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr5_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr5_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4052usize),
            )
        }
    }

    #[doc = "Peripheral ID 6"]
    #[inline(always)]
    pub const fn gicd_pidr6(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr6_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr6_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4056usize),
            )
        }
    }

    #[doc = "Peripheral ID 7"]
    #[inline(always)]
    pub const fn gicd_pidr7(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr7_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr7_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4060usize),
            )
        }
    }

    #[doc = "Peripheral ID 0"]
    #[inline(always)]
    pub const fn gicd_pidr0(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4064usize),
            )
        }
    }

    #[doc = "Peripheral ID 1"]
    #[inline(always)]
    pub const fn gicd_pidr1(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4068usize),
            )
        }
    }

    #[doc = "Peripheral ID 2"]
    #[inline(always)]
    pub const fn gicd_pidr2(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4072usize),
            )
        }
    }

    #[doc = "Peripheral ID 3"]
    #[inline(always)]
    pub const fn gicd_pidr3(
        &self,
    ) -> &'static crate::common::Reg<self::GicdPidr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdPidr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4076usize),
            )
        }
    }

    #[doc = "Component ID 0"]
    #[inline(always)]
    pub const fn gicd_cidr0(
        &self,
    ) -> &'static crate::common::Reg<self::GicdCidr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdCidr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4080usize),
            )
        }
    }

    #[doc = "Component ID 1"]
    #[inline(always)]
    pub const fn gicd_cidr1(
        &self,
    ) -> &'static crate::common::Reg<self::GicdCidr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdCidr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4084usize),
            )
        }
    }

    #[doc = "Component ID 2"]
    #[inline(always)]
    pub const fn gicd_cidr2(
        &self,
    ) -> &'static crate::common::Reg<self::GicdCidr2_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdCidr2_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4088usize),
            )
        }
    }

    #[doc = "Component ID 3"]
    #[inline(always)]
    pub const fn gicd_cidr3(
        &self,
    ) -> &'static crate::common::Reg<self::GicdCidr3_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GicdCidr3_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4092usize),
            )
        }
    }

    #[doc = "Interrupt Group Registers"]
    #[inline(always)]
    pub const fn gicd_igroupr(self) -> crate::arm_gic400_distributor::GicdIgroupr {
        unsafe {
            crate::arm_gic400_distributor::_GicdIgroupr::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Interrupt Set-Enable Registers"]
    #[inline(always)]
    pub const fn gicd_isenabler(self) -> crate::arm_gic400_distributor::GicdIsenabler {
        unsafe {
            crate::arm_gic400_distributor::_GicdIsenabler::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Enable Registers"]
    #[inline(always)]
    pub const fn gicd_icenabler(self) -> crate::arm_gic400_distributor::GicdIcenabler {
        unsafe {
            crate::arm_gic400_distributor::_GicdIcenabler::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(384usize),
            )
        }
    }

    #[doc = "Interrupt Set-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_ispendr(self) -> crate::arm_gic400_distributor::GicdIspendr {
        unsafe {
            crate::arm_gic400_distributor::_GicdIspendr::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Pending Registers"]
    #[inline(always)]
    pub const fn gicd_icpendr(self) -> crate::arm_gic400_distributor::GicdIcpendr {
        unsafe {
            crate::arm_gic400_distributor::_GicdIcpendr::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(640usize),
            )
        }
    }

    #[doc = "Interrupt Set-Active Registers"]
    #[inline(always)]
    pub const fn gicd_isactiver(self) -> crate::arm_gic400_distributor::GicdIsactiver {
        unsafe {
            crate::arm_gic400_distributor::_GicdIsactiver::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Active Registers"]
    #[inline(always)]
    pub const fn gicd_icactiver(self) -> crate::arm_gic400_distributor::GicdIcactiver {
        unsafe {
            crate::arm_gic400_distributor::_GicdIcactiver::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(896usize),
            )
        }
    }

    #[doc = "Interrupt Priority"]
    #[inline(always)]
    pub const fn gicd_ipriorityr(self) -> crate::arm_gic400_distributor::GicdIpriorityr {
        unsafe {
            crate::arm_gic400_distributor::_GicdIpriorityr::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "Interrupt Processor Targets"]
    #[inline(always)]
    pub const fn gicd_itargetsr(self) -> crate::arm_gic400_distributor::GicdItargetsr {
        unsafe {
            crate::arm_gic400_distributor::_GicdItargetsr::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(2048usize),
            )
        }
    }

    #[doc = "Interrupt Configuration"]
    #[inline(always)]
    pub const fn gicd_icfgr(self) -> crate::arm_gic400_distributor::GicdIcfgr {
        unsafe {
            crate::arm_gic400_distributor::_GicdIcfgr::_svd2pac_from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdCtlr_SPEC;
impl crate::sealed::RegSpec for GicdCtlr_SPEC {
    type DataType = u32;
}

#[doc = "Distributor Control Register"]
pub type GicdCtlr = crate::RegValueT<GicdCtlr_SPEC>;

impl NoBitfieldReg<GicdCtlr_SPEC> for GicdCtlr {}
impl ::core::default::Default for GicdCtlr {
    #[inline(always)]
    fn default() -> GicdCtlr {
        <crate::RegValueT<GicdCtlr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdTyper_SPEC;
impl crate::sealed::RegSpec for GicdTyper_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Controller Type Register"]
pub type GicdTyper = crate::RegValueT<GicdTyper_SPEC>;

impl NoBitfieldReg<GicdTyper_SPEC> for GicdTyper {}
impl ::core::default::Default for GicdTyper {
    #[inline(always)]
    fn default() -> GicdTyper {
        <crate::RegValueT<GicdTyper_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdIidr_SPEC;
impl crate::sealed::RegSpec for GicdIidr_SPEC {
    type DataType = u32;
}

#[doc = "Distributor Implementer Identification Register"]
pub type GicdIidr = crate::RegValueT<GicdIidr_SPEC>;

impl NoBitfieldReg<GicdIidr_SPEC> for GicdIidr {}
impl ::core::default::Default for GicdIidr {
    #[inline(always)]
    fn default() -> GicdIidr {
        <crate::RegValueT<GicdIidr_SPEC> as RegisterValue<_>>::new(33559611)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPpisr_SPEC;
impl crate::sealed::RegSpec for GicdPpisr_SPEC {
    type DataType = u32;
}

#[doc = "Private Peripheral Interrupt Status Register"]
pub type GicdPpisr = crate::RegValueT<GicdPpisr_SPEC>;

impl NoBitfieldReg<GicdPpisr_SPEC> for GicdPpisr {}
impl ::core::default::Default for GicdPpisr {
    #[inline(always)]
    fn default() -> GicdPpisr {
        <crate::RegValueT<GicdPpisr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSpisr0_SPEC;
impl crate::sealed::RegSpec for GicdSpisr0_SPEC {
    type DataType = u32;
}

#[doc = "Shared Peripheral Interrupt Status Registers"]
pub type GicdSpisr0 = crate::RegValueT<GicdSpisr0_SPEC>;

impl NoBitfieldReg<GicdSpisr0_SPEC> for GicdSpisr0 {}
impl ::core::default::Default for GicdSpisr0 {
    #[inline(always)]
    fn default() -> GicdSpisr0 {
        <crate::RegValueT<GicdSpisr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSpisr1_SPEC;
impl crate::sealed::RegSpec for GicdSpisr1_SPEC {
    type DataType = u32;
}

#[doc = "Shared Peripheral Interrupt Status Registers"]
pub type GicdSpisr1 = crate::RegValueT<GicdSpisr1_SPEC>;

impl NoBitfieldReg<GicdSpisr1_SPEC> for GicdSpisr1 {}
impl ::core::default::Default for GicdSpisr1 {
    #[inline(always)]
    fn default() -> GicdSpisr1 {
        <crate::RegValueT<GicdSpisr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSpisr2_SPEC;
impl crate::sealed::RegSpec for GicdSpisr2_SPEC {
    type DataType = u32;
}

#[doc = "Shared Peripheral Interrupt Status Registers"]
pub type GicdSpisr2 = crate::RegValueT<GicdSpisr2_SPEC>;

impl NoBitfieldReg<GicdSpisr2_SPEC> for GicdSpisr2 {}
impl ::core::default::Default for GicdSpisr2 {
    #[inline(always)]
    fn default() -> GicdSpisr2 {
        <crate::RegValueT<GicdSpisr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSpisr3_SPEC;
impl crate::sealed::RegSpec for GicdSpisr3_SPEC {
    type DataType = u32;
}

#[doc = "Shared Peripheral Interrupt Status Registers"]
pub type GicdSpisr3 = crate::RegValueT<GicdSpisr3_SPEC>;

impl NoBitfieldReg<GicdSpisr3_SPEC> for GicdSpisr3 {}
impl ::core::default::Default for GicdSpisr3 {
    #[inline(always)]
    fn default() -> GicdSpisr3 {
        <crate::RegValueT<GicdSpisr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSpisr4_SPEC;
impl crate::sealed::RegSpec for GicdSpisr4_SPEC {
    type DataType = u32;
}

#[doc = "Shared Peripheral Interrupt Status Registers"]
pub type GicdSpisr4 = crate::RegValueT<GicdSpisr4_SPEC>;

impl NoBitfieldReg<GicdSpisr4_SPEC> for GicdSpisr4 {}
impl ::core::default::Default for GicdSpisr4 {
    #[inline(always)]
    fn default() -> GicdSpisr4 {
        <crate::RegValueT<GicdSpisr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSpisr5_SPEC;
impl crate::sealed::RegSpec for GicdSpisr5_SPEC {
    type DataType = u32;
}

#[doc = "Shared Peripheral Interrupt Status Registers"]
pub type GicdSpisr5 = crate::RegValueT<GicdSpisr5_SPEC>;

impl NoBitfieldReg<GicdSpisr5_SPEC> for GicdSpisr5 {}
impl ::core::default::Default for GicdSpisr5 {
    #[inline(always)]
    fn default() -> GicdSpisr5 {
        <crate::RegValueT<GicdSpisr5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSgir_SPEC;
impl crate::sealed::RegSpec for GicdSgir_SPEC {
    type DataType = u32;
}

#[doc = "Software Generated Interrupt Register"]
pub type GicdSgir = crate::RegValueT<GicdSgir_SPEC>;

impl NoBitfieldReg<GicdSgir_SPEC> for GicdSgir {}
impl ::core::default::Default for GicdSgir {
    #[inline(always)]
    fn default() -> GicdSgir {
        <crate::RegValueT<GicdSgir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdCpendsgiRn_SPEC;
impl crate::sealed::RegSpec for GicdCpendsgiRn_SPEC {
    type DataType = u32;
}

#[doc = "SGI Clear-Pending Registers"]
pub type GicdCpendsgiRn = crate::RegValueT<GicdCpendsgiRn_SPEC>;

impl NoBitfieldReg<GicdCpendsgiRn_SPEC> for GicdCpendsgiRn {}
impl ::core::default::Default for GicdCpendsgiRn {
    #[inline(always)]
    fn default() -> GicdCpendsgiRn {
        <crate::RegValueT<GicdCpendsgiRn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdSpendsgiRn_SPEC;
impl crate::sealed::RegSpec for GicdSpendsgiRn_SPEC {
    type DataType = u32;
}

#[doc = "SGI Set-Pending Registers"]
pub type GicdSpendsgiRn = crate::RegValueT<GicdSpendsgiRn_SPEC>;

impl NoBitfieldReg<GicdSpendsgiRn_SPEC> for GicdSpendsgiRn {}
impl ::core::default::Default for GicdSpendsgiRn {
    #[inline(always)]
    fn default() -> GicdSpendsgiRn {
        <crate::RegValueT<GicdSpendsgiRn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr4_SPEC;
impl crate::sealed::RegSpec for GicdPidr4_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 4"]
pub type GicdPidr4 = crate::RegValueT<GicdPidr4_SPEC>;

impl NoBitfieldReg<GicdPidr4_SPEC> for GicdPidr4 {}
impl ::core::default::Default for GicdPidr4 {
    #[inline(always)]
    fn default() -> GicdPidr4 {
        <crate::RegValueT<GicdPidr4_SPEC> as RegisterValue<_>>::new(4)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr5_SPEC;
impl crate::sealed::RegSpec for GicdPidr5_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 5"]
pub type GicdPidr5 = crate::RegValueT<GicdPidr5_SPEC>;

impl NoBitfieldReg<GicdPidr5_SPEC> for GicdPidr5 {}
impl ::core::default::Default for GicdPidr5 {
    #[inline(always)]
    fn default() -> GicdPidr5 {
        <crate::RegValueT<GicdPidr5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr6_SPEC;
impl crate::sealed::RegSpec for GicdPidr6_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 6"]
pub type GicdPidr6 = crate::RegValueT<GicdPidr6_SPEC>;

impl NoBitfieldReg<GicdPidr6_SPEC> for GicdPidr6 {}
impl ::core::default::Default for GicdPidr6 {
    #[inline(always)]
    fn default() -> GicdPidr6 {
        <crate::RegValueT<GicdPidr6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr7_SPEC;
impl crate::sealed::RegSpec for GicdPidr7_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 7"]
pub type GicdPidr7 = crate::RegValueT<GicdPidr7_SPEC>;

impl NoBitfieldReg<GicdPidr7_SPEC> for GicdPidr7 {}
impl ::core::default::Default for GicdPidr7 {
    #[inline(always)]
    fn default() -> GicdPidr7 {
        <crate::RegValueT<GicdPidr7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr0_SPEC;
impl crate::sealed::RegSpec for GicdPidr0_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 0"]
pub type GicdPidr0 = crate::RegValueT<GicdPidr0_SPEC>;

impl NoBitfieldReg<GicdPidr0_SPEC> for GicdPidr0 {}
impl ::core::default::Default for GicdPidr0 {
    #[inline(always)]
    fn default() -> GicdPidr0 {
        <crate::RegValueT<GicdPidr0_SPEC> as RegisterValue<_>>::new(144)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr1_SPEC;
impl crate::sealed::RegSpec for GicdPidr1_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 1"]
pub type GicdPidr1 = crate::RegValueT<GicdPidr1_SPEC>;

impl NoBitfieldReg<GicdPidr1_SPEC> for GicdPidr1 {}
impl ::core::default::Default for GicdPidr1 {
    #[inline(always)]
    fn default() -> GicdPidr1 {
        <crate::RegValueT<GicdPidr1_SPEC> as RegisterValue<_>>::new(180)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr2_SPEC;
impl crate::sealed::RegSpec for GicdPidr2_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 2"]
pub type GicdPidr2 = crate::RegValueT<GicdPidr2_SPEC>;

impl NoBitfieldReg<GicdPidr2_SPEC> for GicdPidr2 {}
impl ::core::default::Default for GicdPidr2 {
    #[inline(always)]
    fn default() -> GicdPidr2 {
        <crate::RegValueT<GicdPidr2_SPEC> as RegisterValue<_>>::new(43)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdPidr3_SPEC;
impl crate::sealed::RegSpec for GicdPidr3_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral ID 3"]
pub type GicdPidr3 = crate::RegValueT<GicdPidr3_SPEC>;

impl NoBitfieldReg<GicdPidr3_SPEC> for GicdPidr3 {}
impl ::core::default::Default for GicdPidr3 {
    #[inline(always)]
    fn default() -> GicdPidr3 {
        <crate::RegValueT<GicdPidr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdCidr0_SPEC;
impl crate::sealed::RegSpec for GicdCidr0_SPEC {
    type DataType = u32;
}

#[doc = "Component ID 0"]
pub type GicdCidr0 = crate::RegValueT<GicdCidr0_SPEC>;

impl NoBitfieldReg<GicdCidr0_SPEC> for GicdCidr0 {}
impl ::core::default::Default for GicdCidr0 {
    #[inline(always)]
    fn default() -> GicdCidr0 {
        <crate::RegValueT<GicdCidr0_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdCidr1_SPEC;
impl crate::sealed::RegSpec for GicdCidr1_SPEC {
    type DataType = u32;
}

#[doc = "Component ID 1"]
pub type GicdCidr1 = crate::RegValueT<GicdCidr1_SPEC>;

impl NoBitfieldReg<GicdCidr1_SPEC> for GicdCidr1 {}
impl ::core::default::Default for GicdCidr1 {
    #[inline(always)]
    fn default() -> GicdCidr1 {
        <crate::RegValueT<GicdCidr1_SPEC> as RegisterValue<_>>::new(240)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdCidr2_SPEC;
impl crate::sealed::RegSpec for GicdCidr2_SPEC {
    type DataType = u32;
}

#[doc = "Component ID 2"]
pub type GicdCidr2 = crate::RegValueT<GicdCidr2_SPEC>;

impl NoBitfieldReg<GicdCidr2_SPEC> for GicdCidr2 {}
impl ::core::default::Default for GicdCidr2 {
    #[inline(always)]
    fn default() -> GicdCidr2 {
        <crate::RegValueT<GicdCidr2_SPEC> as RegisterValue<_>>::new(5)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GicdCidr3_SPEC;
impl crate::sealed::RegSpec for GicdCidr3_SPEC {
    type DataType = u32;
}

#[doc = "Component ID 3"]
pub type GicdCidr3 = crate::RegValueT<GicdCidr3_SPEC>;

impl NoBitfieldReg<GicdCidr3_SPEC> for GicdCidr3 {}
impl ::core::default::Default for GicdCidr3 {
    #[inline(always)]
    fn default() -> GicdCidr3 {
        <crate::RegValueT<GicdCidr3_SPEC> as RegisterValue<_>>::new(177)
    }
}

#[doc = "Interrupt Group Registers"]
#[non_exhaustive]
pub struct _GicdIgroupr;

#[doc = "Interrupt Group Registers"]
pub type GicdIgroupr = &'static _GicdIgroupr;

unsafe impl ::core::marker::Sync for _GicdIgroupr {}
impl _GicdIgroupr {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr0(
        &self,
    ) -> &'static crate::common::Reg<gicd_igroupr::GicdIgroupr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_igroupr::GicdIgroupr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr1(
        &self,
    ) -> &'static crate::common::Reg<gicd_igroupr::GicdIgroupr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_igroupr::GicdIgroupr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr2(
        &self,
    ) -> &'static crate::common::Reg<gicd_igroupr::GicdIgroupr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_igroupr::GicdIgroupr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr3(
        &self,
    ) -> &'static crate::common::Reg<gicd_igroupr::GicdIgroupr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_igroupr::GicdIgroupr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr4(
        &self,
    ) -> &'static crate::common::Reg<gicd_igroupr::GicdIgroupr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_igroupr::GicdIgroupr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr5(
        &self,
    ) -> &'static crate::common::Reg<gicd_igroupr::GicdIgroupr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_igroupr::GicdIgroupr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Group"]
    #[inline(always)]
    pub const fn gicd_igroupr6(
        &self,
    ) -> &'static crate::common::Reg<gicd_igroupr::GicdIgroupr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_igroupr::GicdIgroupr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIgroupr {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_igroupr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIgroupr0_SPEC;
    impl crate::sealed::RegSpec for GicdIgroupr0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Group"]
    pub type GicdIgroupr0 = crate::RegValueT<GicdIgroupr0_SPEC>;

    impl NoBitfieldReg<GicdIgroupr0_SPEC> for GicdIgroupr0 {}
    impl ::core::default::Default for GicdIgroupr0 {
        #[inline(always)]
        fn default() -> GicdIgroupr0 {
            <crate::RegValueT<GicdIgroupr0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIgroupr1_SPEC;
    impl crate::sealed::RegSpec for GicdIgroupr1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Group"]
    pub type GicdIgroupr1 = crate::RegValueT<GicdIgroupr1_SPEC>;

    impl NoBitfieldReg<GicdIgroupr1_SPEC> for GicdIgroupr1 {}
    impl ::core::default::Default for GicdIgroupr1 {
        #[inline(always)]
        fn default() -> GicdIgroupr1 {
            <crate::RegValueT<GicdIgroupr1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIgroupr2_SPEC;
    impl crate::sealed::RegSpec for GicdIgroupr2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Group"]
    pub type GicdIgroupr2 = crate::RegValueT<GicdIgroupr2_SPEC>;

    impl NoBitfieldReg<GicdIgroupr2_SPEC> for GicdIgroupr2 {}
    impl ::core::default::Default for GicdIgroupr2 {
        #[inline(always)]
        fn default() -> GicdIgroupr2 {
            <crate::RegValueT<GicdIgroupr2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIgroupr3_SPEC;
    impl crate::sealed::RegSpec for GicdIgroupr3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Group"]
    pub type GicdIgroupr3 = crate::RegValueT<GicdIgroupr3_SPEC>;

    impl NoBitfieldReg<GicdIgroupr3_SPEC> for GicdIgroupr3 {}
    impl ::core::default::Default for GicdIgroupr3 {
        #[inline(always)]
        fn default() -> GicdIgroupr3 {
            <crate::RegValueT<GicdIgroupr3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIgroupr4_SPEC;
    impl crate::sealed::RegSpec for GicdIgroupr4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Group"]
    pub type GicdIgroupr4 = crate::RegValueT<GicdIgroupr4_SPEC>;

    impl NoBitfieldReg<GicdIgroupr4_SPEC> for GicdIgroupr4 {}
    impl ::core::default::Default for GicdIgroupr4 {
        #[inline(always)]
        fn default() -> GicdIgroupr4 {
            <crate::RegValueT<GicdIgroupr4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIgroupr5_SPEC;
    impl crate::sealed::RegSpec for GicdIgroupr5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Group"]
    pub type GicdIgroupr5 = crate::RegValueT<GicdIgroupr5_SPEC>;

    impl NoBitfieldReg<GicdIgroupr5_SPEC> for GicdIgroupr5 {}
    impl ::core::default::Default for GicdIgroupr5 {
        #[inline(always)]
        fn default() -> GicdIgroupr5 {
            <crate::RegValueT<GicdIgroupr5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIgroupr6_SPEC;
    impl crate::sealed::RegSpec for GicdIgroupr6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Group"]
    pub type GicdIgroupr6 = crate::RegValueT<GicdIgroupr6_SPEC>;

    impl NoBitfieldReg<GicdIgroupr6_SPEC> for GicdIgroupr6 {}
    impl ::core::default::Default for GicdIgroupr6 {
        #[inline(always)]
        fn default() -> GicdIgroupr6 {
            <crate::RegValueT<GicdIgroupr6_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Set-Enable Registers"]
#[non_exhaustive]
pub struct _GicdIsenabler;

#[doc = "Interrupt Set-Enable Registers"]
pub type GicdIsenabler = &'static _GicdIsenabler;

unsafe impl ::core::marker::Sync for _GicdIsenabler {}
impl _GicdIsenabler {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler0(
        &self,
    ) -> &'static crate::common::Reg<gicd_isenabler::GicdIsenabler0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isenabler::GicdIsenabler0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler1(
        &self,
    ) -> &'static crate::common::Reg<gicd_isenabler::GicdIsenabler1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isenabler::GicdIsenabler1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler2(
        &self,
    ) -> &'static crate::common::Reg<gicd_isenabler::GicdIsenabler2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isenabler::GicdIsenabler2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler3(
        &self,
    ) -> &'static crate::common::Reg<gicd_isenabler::GicdIsenabler3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isenabler::GicdIsenabler3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler4(
        &self,
    ) -> &'static crate::common::Reg<gicd_isenabler::GicdIsenabler4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isenabler::GicdIsenabler4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler5(
        &self,
    ) -> &'static crate::common::Reg<gicd_isenabler::GicdIsenabler5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isenabler::GicdIsenabler5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Set-Enable"]
    #[inline(always)]
    pub const fn gicd_isenabler6(
        &self,
    ) -> &'static crate::common::Reg<gicd_isenabler::GicdIsenabler6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isenabler::GicdIsenabler6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIsenabler {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_isenabler {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsenabler0_SPEC;
    impl crate::sealed::RegSpec for GicdIsenabler0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Enable"]
    pub type GicdIsenabler0 = crate::RegValueT<GicdIsenabler0_SPEC>;

    impl NoBitfieldReg<GicdIsenabler0_SPEC> for GicdIsenabler0 {}
    impl ::core::default::Default for GicdIsenabler0 {
        #[inline(always)]
        fn default() -> GicdIsenabler0 {
            <crate::RegValueT<GicdIsenabler0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsenabler1_SPEC;
    impl crate::sealed::RegSpec for GicdIsenabler1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Enable"]
    pub type GicdIsenabler1 = crate::RegValueT<GicdIsenabler1_SPEC>;

    impl NoBitfieldReg<GicdIsenabler1_SPEC> for GicdIsenabler1 {}
    impl ::core::default::Default for GicdIsenabler1 {
        #[inline(always)]
        fn default() -> GicdIsenabler1 {
            <crate::RegValueT<GicdIsenabler1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsenabler2_SPEC;
    impl crate::sealed::RegSpec for GicdIsenabler2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Enable"]
    pub type GicdIsenabler2 = crate::RegValueT<GicdIsenabler2_SPEC>;

    impl NoBitfieldReg<GicdIsenabler2_SPEC> for GicdIsenabler2 {}
    impl ::core::default::Default for GicdIsenabler2 {
        #[inline(always)]
        fn default() -> GicdIsenabler2 {
            <crate::RegValueT<GicdIsenabler2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsenabler3_SPEC;
    impl crate::sealed::RegSpec for GicdIsenabler3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Enable"]
    pub type GicdIsenabler3 = crate::RegValueT<GicdIsenabler3_SPEC>;

    impl NoBitfieldReg<GicdIsenabler3_SPEC> for GicdIsenabler3 {}
    impl ::core::default::Default for GicdIsenabler3 {
        #[inline(always)]
        fn default() -> GicdIsenabler3 {
            <crate::RegValueT<GicdIsenabler3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsenabler4_SPEC;
    impl crate::sealed::RegSpec for GicdIsenabler4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Enable"]
    pub type GicdIsenabler4 = crate::RegValueT<GicdIsenabler4_SPEC>;

    impl NoBitfieldReg<GicdIsenabler4_SPEC> for GicdIsenabler4 {}
    impl ::core::default::Default for GicdIsenabler4 {
        #[inline(always)]
        fn default() -> GicdIsenabler4 {
            <crate::RegValueT<GicdIsenabler4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsenabler5_SPEC;
    impl crate::sealed::RegSpec for GicdIsenabler5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Enable"]
    pub type GicdIsenabler5 = crate::RegValueT<GicdIsenabler5_SPEC>;

    impl NoBitfieldReg<GicdIsenabler5_SPEC> for GicdIsenabler5 {}
    impl ::core::default::Default for GicdIsenabler5 {
        #[inline(always)]
        fn default() -> GicdIsenabler5 {
            <crate::RegValueT<GicdIsenabler5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsenabler6_SPEC;
    impl crate::sealed::RegSpec for GicdIsenabler6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Enable"]
    pub type GicdIsenabler6 = crate::RegValueT<GicdIsenabler6_SPEC>;

    impl NoBitfieldReg<GicdIsenabler6_SPEC> for GicdIsenabler6 {}
    impl ::core::default::Default for GicdIsenabler6 {
        #[inline(always)]
        fn default() -> GicdIsenabler6 {
            <crate::RegValueT<GicdIsenabler6_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Clear-Enable Registers"]
#[non_exhaustive]
pub struct _GicdIcenabler;

#[doc = "Interrupt Clear-Enable Registers"]
pub type GicdIcenabler = &'static _GicdIcenabler;

unsafe impl ::core::marker::Sync for _GicdIcenabler {}
impl _GicdIcenabler {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler0(
        &self,
    ) -> &'static crate::common::Reg<gicd_icenabler::GicdIcenabler0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icenabler::GicdIcenabler0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler1(
        &self,
    ) -> &'static crate::common::Reg<gicd_icenabler::GicdIcenabler1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icenabler::GicdIcenabler1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler2(
        &self,
    ) -> &'static crate::common::Reg<gicd_icenabler::GicdIcenabler2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icenabler::GicdIcenabler2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler3(
        &self,
    ) -> &'static crate::common::Reg<gicd_icenabler::GicdIcenabler3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icenabler::GicdIcenabler3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler4(
        &self,
    ) -> &'static crate::common::Reg<gicd_icenabler::GicdIcenabler4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icenabler::GicdIcenabler4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler5(
        &self,
    ) -> &'static crate::common::Reg<gicd_icenabler::GicdIcenabler5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icenabler::GicdIcenabler5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Enable"]
    #[inline(always)]
    pub const fn gicd_icenabler6(
        &self,
    ) -> &'static crate::common::Reg<gicd_icenabler::GicdIcenabler6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icenabler::GicdIcenabler6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIcenabler {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_icenabler {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcenabler0_SPEC;
    impl crate::sealed::RegSpec for GicdIcenabler0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Enable"]
    pub type GicdIcenabler0 = crate::RegValueT<GicdIcenabler0_SPEC>;

    impl NoBitfieldReg<GicdIcenabler0_SPEC> for GicdIcenabler0 {}
    impl ::core::default::Default for GicdIcenabler0 {
        #[inline(always)]
        fn default() -> GicdIcenabler0 {
            <crate::RegValueT<GicdIcenabler0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcenabler1_SPEC;
    impl crate::sealed::RegSpec for GicdIcenabler1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Enable"]
    pub type GicdIcenabler1 = crate::RegValueT<GicdIcenabler1_SPEC>;

    impl NoBitfieldReg<GicdIcenabler1_SPEC> for GicdIcenabler1 {}
    impl ::core::default::Default for GicdIcenabler1 {
        #[inline(always)]
        fn default() -> GicdIcenabler1 {
            <crate::RegValueT<GicdIcenabler1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcenabler2_SPEC;
    impl crate::sealed::RegSpec for GicdIcenabler2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Enable"]
    pub type GicdIcenabler2 = crate::RegValueT<GicdIcenabler2_SPEC>;

    impl NoBitfieldReg<GicdIcenabler2_SPEC> for GicdIcenabler2 {}
    impl ::core::default::Default for GicdIcenabler2 {
        #[inline(always)]
        fn default() -> GicdIcenabler2 {
            <crate::RegValueT<GicdIcenabler2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcenabler3_SPEC;
    impl crate::sealed::RegSpec for GicdIcenabler3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Enable"]
    pub type GicdIcenabler3 = crate::RegValueT<GicdIcenabler3_SPEC>;

    impl NoBitfieldReg<GicdIcenabler3_SPEC> for GicdIcenabler3 {}
    impl ::core::default::Default for GicdIcenabler3 {
        #[inline(always)]
        fn default() -> GicdIcenabler3 {
            <crate::RegValueT<GicdIcenabler3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcenabler4_SPEC;
    impl crate::sealed::RegSpec for GicdIcenabler4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Enable"]
    pub type GicdIcenabler4 = crate::RegValueT<GicdIcenabler4_SPEC>;

    impl NoBitfieldReg<GicdIcenabler4_SPEC> for GicdIcenabler4 {}
    impl ::core::default::Default for GicdIcenabler4 {
        #[inline(always)]
        fn default() -> GicdIcenabler4 {
            <crate::RegValueT<GicdIcenabler4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcenabler5_SPEC;
    impl crate::sealed::RegSpec for GicdIcenabler5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Enable"]
    pub type GicdIcenabler5 = crate::RegValueT<GicdIcenabler5_SPEC>;

    impl NoBitfieldReg<GicdIcenabler5_SPEC> for GicdIcenabler5 {}
    impl ::core::default::Default for GicdIcenabler5 {
        #[inline(always)]
        fn default() -> GicdIcenabler5 {
            <crate::RegValueT<GicdIcenabler5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcenabler6_SPEC;
    impl crate::sealed::RegSpec for GicdIcenabler6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Enable"]
    pub type GicdIcenabler6 = crate::RegValueT<GicdIcenabler6_SPEC>;

    impl NoBitfieldReg<GicdIcenabler6_SPEC> for GicdIcenabler6 {}
    impl ::core::default::Default for GicdIcenabler6 {
        #[inline(always)]
        fn default() -> GicdIcenabler6 {
            <crate::RegValueT<GicdIcenabler6_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Set-Pending Registers"]
#[non_exhaustive]
pub struct _GicdIspendr;

#[doc = "Interrupt Set-Pending Registers"]
pub type GicdIspendr = &'static _GicdIspendr;

unsafe impl ::core::marker::Sync for _GicdIspendr {}
impl _GicdIspendr {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr0(
        &self,
    ) -> &'static crate::common::Reg<gicd_ispendr::GicdIspendr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ispendr::GicdIspendr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr1(
        &self,
    ) -> &'static crate::common::Reg<gicd_ispendr::GicdIspendr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ispendr::GicdIspendr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr2(
        &self,
    ) -> &'static crate::common::Reg<gicd_ispendr::GicdIspendr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ispendr::GicdIspendr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr3(
        &self,
    ) -> &'static crate::common::Reg<gicd_ispendr::GicdIspendr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ispendr::GicdIspendr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr4(
        &self,
    ) -> &'static crate::common::Reg<gicd_ispendr::GicdIspendr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ispendr::GicdIspendr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr5(
        &self,
    ) -> &'static crate::common::Reg<gicd_ispendr::GicdIspendr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ispendr::GicdIspendr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Set-Pending"]
    #[inline(always)]
    pub const fn gicd_ispendr6(
        &self,
    ) -> &'static crate::common::Reg<gicd_ispendr::GicdIspendr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ispendr::GicdIspendr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIspendr {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_ispendr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIspendr0_SPEC;
    impl crate::sealed::RegSpec for GicdIspendr0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Pending"]
    pub type GicdIspendr0 = crate::RegValueT<GicdIspendr0_SPEC>;

    impl NoBitfieldReg<GicdIspendr0_SPEC> for GicdIspendr0 {}
    impl ::core::default::Default for GicdIspendr0 {
        #[inline(always)]
        fn default() -> GicdIspendr0 {
            <crate::RegValueT<GicdIspendr0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIspendr1_SPEC;
    impl crate::sealed::RegSpec for GicdIspendr1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Pending"]
    pub type GicdIspendr1 = crate::RegValueT<GicdIspendr1_SPEC>;

    impl NoBitfieldReg<GicdIspendr1_SPEC> for GicdIspendr1 {}
    impl ::core::default::Default for GicdIspendr1 {
        #[inline(always)]
        fn default() -> GicdIspendr1 {
            <crate::RegValueT<GicdIspendr1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIspendr2_SPEC;
    impl crate::sealed::RegSpec for GicdIspendr2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Pending"]
    pub type GicdIspendr2 = crate::RegValueT<GicdIspendr2_SPEC>;

    impl NoBitfieldReg<GicdIspendr2_SPEC> for GicdIspendr2 {}
    impl ::core::default::Default for GicdIspendr2 {
        #[inline(always)]
        fn default() -> GicdIspendr2 {
            <crate::RegValueT<GicdIspendr2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIspendr3_SPEC;
    impl crate::sealed::RegSpec for GicdIspendr3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Pending"]
    pub type GicdIspendr3 = crate::RegValueT<GicdIspendr3_SPEC>;

    impl NoBitfieldReg<GicdIspendr3_SPEC> for GicdIspendr3 {}
    impl ::core::default::Default for GicdIspendr3 {
        #[inline(always)]
        fn default() -> GicdIspendr3 {
            <crate::RegValueT<GicdIspendr3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIspendr4_SPEC;
    impl crate::sealed::RegSpec for GicdIspendr4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Pending"]
    pub type GicdIspendr4 = crate::RegValueT<GicdIspendr4_SPEC>;

    impl NoBitfieldReg<GicdIspendr4_SPEC> for GicdIspendr4 {}
    impl ::core::default::Default for GicdIspendr4 {
        #[inline(always)]
        fn default() -> GicdIspendr4 {
            <crate::RegValueT<GicdIspendr4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIspendr5_SPEC;
    impl crate::sealed::RegSpec for GicdIspendr5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Pending"]
    pub type GicdIspendr5 = crate::RegValueT<GicdIspendr5_SPEC>;

    impl NoBitfieldReg<GicdIspendr5_SPEC> for GicdIspendr5 {}
    impl ::core::default::Default for GicdIspendr5 {
        #[inline(always)]
        fn default() -> GicdIspendr5 {
            <crate::RegValueT<GicdIspendr5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIspendr6_SPEC;
    impl crate::sealed::RegSpec for GicdIspendr6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Pending"]
    pub type GicdIspendr6 = crate::RegValueT<GicdIspendr6_SPEC>;

    impl NoBitfieldReg<GicdIspendr6_SPEC> for GicdIspendr6 {}
    impl ::core::default::Default for GicdIspendr6 {
        #[inline(always)]
        fn default() -> GicdIspendr6 {
            <crate::RegValueT<GicdIspendr6_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Clear-Pending Registers"]
#[non_exhaustive]
pub struct _GicdIcpendr;

#[doc = "Interrupt Clear-Pending Registers"]
pub type GicdIcpendr = &'static _GicdIcpendr;

unsafe impl ::core::marker::Sync for _GicdIcpendr {}
impl _GicdIcpendr {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr0(
        &self,
    ) -> &'static crate::common::Reg<gicd_icpendr::GicdIcpendr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icpendr::GicdIcpendr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr1(
        &self,
    ) -> &'static crate::common::Reg<gicd_icpendr::GicdIcpendr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icpendr::GicdIcpendr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr2(
        &self,
    ) -> &'static crate::common::Reg<gicd_icpendr::GicdIcpendr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icpendr::GicdIcpendr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr3(
        &self,
    ) -> &'static crate::common::Reg<gicd_icpendr::GicdIcpendr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icpendr::GicdIcpendr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr4(
        &self,
    ) -> &'static crate::common::Reg<gicd_icpendr::GicdIcpendr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icpendr::GicdIcpendr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr5(
        &self,
    ) -> &'static crate::common::Reg<gicd_icpendr::GicdIcpendr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icpendr::GicdIcpendr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Pending"]
    #[inline(always)]
    pub const fn gicd_icpendr6(
        &self,
    ) -> &'static crate::common::Reg<gicd_icpendr::GicdIcpendr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icpendr::GicdIcpendr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIcpendr {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_icpendr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcpendr0_SPEC;
    impl crate::sealed::RegSpec for GicdIcpendr0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Pending"]
    pub type GicdIcpendr0 = crate::RegValueT<GicdIcpendr0_SPEC>;

    impl NoBitfieldReg<GicdIcpendr0_SPEC> for GicdIcpendr0 {}
    impl ::core::default::Default for GicdIcpendr0 {
        #[inline(always)]
        fn default() -> GicdIcpendr0 {
            <crate::RegValueT<GicdIcpendr0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcpendr1_SPEC;
    impl crate::sealed::RegSpec for GicdIcpendr1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Pending"]
    pub type GicdIcpendr1 = crate::RegValueT<GicdIcpendr1_SPEC>;

    impl NoBitfieldReg<GicdIcpendr1_SPEC> for GicdIcpendr1 {}
    impl ::core::default::Default for GicdIcpendr1 {
        #[inline(always)]
        fn default() -> GicdIcpendr1 {
            <crate::RegValueT<GicdIcpendr1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcpendr2_SPEC;
    impl crate::sealed::RegSpec for GicdIcpendr2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Pending"]
    pub type GicdIcpendr2 = crate::RegValueT<GicdIcpendr2_SPEC>;

    impl NoBitfieldReg<GicdIcpendr2_SPEC> for GicdIcpendr2 {}
    impl ::core::default::Default for GicdIcpendr2 {
        #[inline(always)]
        fn default() -> GicdIcpendr2 {
            <crate::RegValueT<GicdIcpendr2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcpendr3_SPEC;
    impl crate::sealed::RegSpec for GicdIcpendr3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Pending"]
    pub type GicdIcpendr3 = crate::RegValueT<GicdIcpendr3_SPEC>;

    impl NoBitfieldReg<GicdIcpendr3_SPEC> for GicdIcpendr3 {}
    impl ::core::default::Default for GicdIcpendr3 {
        #[inline(always)]
        fn default() -> GicdIcpendr3 {
            <crate::RegValueT<GicdIcpendr3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcpendr4_SPEC;
    impl crate::sealed::RegSpec for GicdIcpendr4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Pending"]
    pub type GicdIcpendr4 = crate::RegValueT<GicdIcpendr4_SPEC>;

    impl NoBitfieldReg<GicdIcpendr4_SPEC> for GicdIcpendr4 {}
    impl ::core::default::Default for GicdIcpendr4 {
        #[inline(always)]
        fn default() -> GicdIcpendr4 {
            <crate::RegValueT<GicdIcpendr4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcpendr5_SPEC;
    impl crate::sealed::RegSpec for GicdIcpendr5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Pending"]
    pub type GicdIcpendr5 = crate::RegValueT<GicdIcpendr5_SPEC>;

    impl NoBitfieldReg<GicdIcpendr5_SPEC> for GicdIcpendr5 {}
    impl ::core::default::Default for GicdIcpendr5 {
        #[inline(always)]
        fn default() -> GicdIcpendr5 {
            <crate::RegValueT<GicdIcpendr5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcpendr6_SPEC;
    impl crate::sealed::RegSpec for GicdIcpendr6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Pending"]
    pub type GicdIcpendr6 = crate::RegValueT<GicdIcpendr6_SPEC>;

    impl NoBitfieldReg<GicdIcpendr6_SPEC> for GicdIcpendr6 {}
    impl ::core::default::Default for GicdIcpendr6 {
        #[inline(always)]
        fn default() -> GicdIcpendr6 {
            <crate::RegValueT<GicdIcpendr6_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Set-Active Registers"]
#[non_exhaustive]
pub struct _GicdIsactiver;

#[doc = "Interrupt Set-Active Registers"]
pub type GicdIsactiver = &'static _GicdIsactiver;

unsafe impl ::core::marker::Sync for _GicdIsactiver {}
impl _GicdIsactiver {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver0(
        &self,
    ) -> &'static crate::common::Reg<gicd_isactiver::GicdIsactiver0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isactiver::GicdIsactiver0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver1(
        &self,
    ) -> &'static crate::common::Reg<gicd_isactiver::GicdIsactiver1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isactiver::GicdIsactiver1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver2(
        &self,
    ) -> &'static crate::common::Reg<gicd_isactiver::GicdIsactiver2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isactiver::GicdIsactiver2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver3(
        &self,
    ) -> &'static crate::common::Reg<gicd_isactiver::GicdIsactiver3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isactiver::GicdIsactiver3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver4(
        &self,
    ) -> &'static crate::common::Reg<gicd_isactiver::GicdIsactiver4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isactiver::GicdIsactiver4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver5(
        &self,
    ) -> &'static crate::common::Reg<gicd_isactiver::GicdIsactiver5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isactiver::GicdIsactiver5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Set-Active"]
    #[inline(always)]
    pub const fn gicd_isactiver6(
        &self,
    ) -> &'static crate::common::Reg<gicd_isactiver::GicdIsactiver6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_isactiver::GicdIsactiver6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIsactiver {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_isactiver {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsactiver0_SPEC;
    impl crate::sealed::RegSpec for GicdIsactiver0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Active"]
    pub type GicdIsactiver0 = crate::RegValueT<GicdIsactiver0_SPEC>;

    impl NoBitfieldReg<GicdIsactiver0_SPEC> for GicdIsactiver0 {}
    impl ::core::default::Default for GicdIsactiver0 {
        #[inline(always)]
        fn default() -> GicdIsactiver0 {
            <crate::RegValueT<GicdIsactiver0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsactiver1_SPEC;
    impl crate::sealed::RegSpec for GicdIsactiver1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Active"]
    pub type GicdIsactiver1 = crate::RegValueT<GicdIsactiver1_SPEC>;

    impl NoBitfieldReg<GicdIsactiver1_SPEC> for GicdIsactiver1 {}
    impl ::core::default::Default for GicdIsactiver1 {
        #[inline(always)]
        fn default() -> GicdIsactiver1 {
            <crate::RegValueT<GicdIsactiver1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsactiver2_SPEC;
    impl crate::sealed::RegSpec for GicdIsactiver2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Active"]
    pub type GicdIsactiver2 = crate::RegValueT<GicdIsactiver2_SPEC>;

    impl NoBitfieldReg<GicdIsactiver2_SPEC> for GicdIsactiver2 {}
    impl ::core::default::Default for GicdIsactiver2 {
        #[inline(always)]
        fn default() -> GicdIsactiver2 {
            <crate::RegValueT<GicdIsactiver2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsactiver3_SPEC;
    impl crate::sealed::RegSpec for GicdIsactiver3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Active"]
    pub type GicdIsactiver3 = crate::RegValueT<GicdIsactiver3_SPEC>;

    impl NoBitfieldReg<GicdIsactiver3_SPEC> for GicdIsactiver3 {}
    impl ::core::default::Default for GicdIsactiver3 {
        #[inline(always)]
        fn default() -> GicdIsactiver3 {
            <crate::RegValueT<GicdIsactiver3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsactiver4_SPEC;
    impl crate::sealed::RegSpec for GicdIsactiver4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Active"]
    pub type GicdIsactiver4 = crate::RegValueT<GicdIsactiver4_SPEC>;

    impl NoBitfieldReg<GicdIsactiver4_SPEC> for GicdIsactiver4 {}
    impl ::core::default::Default for GicdIsactiver4 {
        #[inline(always)]
        fn default() -> GicdIsactiver4 {
            <crate::RegValueT<GicdIsactiver4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsactiver5_SPEC;
    impl crate::sealed::RegSpec for GicdIsactiver5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Active"]
    pub type GicdIsactiver5 = crate::RegValueT<GicdIsactiver5_SPEC>;

    impl NoBitfieldReg<GicdIsactiver5_SPEC> for GicdIsactiver5 {}
    impl ::core::default::Default for GicdIsactiver5 {
        #[inline(always)]
        fn default() -> GicdIsactiver5 {
            <crate::RegValueT<GicdIsactiver5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIsactiver6_SPEC;
    impl crate::sealed::RegSpec for GicdIsactiver6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Set-Active"]
    pub type GicdIsactiver6 = crate::RegValueT<GicdIsactiver6_SPEC>;

    impl NoBitfieldReg<GicdIsactiver6_SPEC> for GicdIsactiver6 {}
    impl ::core::default::Default for GicdIsactiver6 {
        #[inline(always)]
        fn default() -> GicdIsactiver6 {
            <crate::RegValueT<GicdIsactiver6_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Clear-Active Registers"]
#[non_exhaustive]
pub struct _GicdIcactiver;

#[doc = "Interrupt Clear-Active Registers"]
pub type GicdIcactiver = &'static _GicdIcactiver;

unsafe impl ::core::marker::Sync for _GicdIcactiver {}
impl _GicdIcactiver {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver0(
        &self,
    ) -> &'static crate::common::Reg<gicd_icactiver::GicdIcactiver0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icactiver::GicdIcactiver0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver1(
        &self,
    ) -> &'static crate::common::Reg<gicd_icactiver::GicdIcactiver1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icactiver::GicdIcactiver1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver2(
        &self,
    ) -> &'static crate::common::Reg<gicd_icactiver::GicdIcactiver2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icactiver::GicdIcactiver2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver3(
        &self,
    ) -> &'static crate::common::Reg<gicd_icactiver::GicdIcactiver3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icactiver::GicdIcactiver3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver4(
        &self,
    ) -> &'static crate::common::Reg<gicd_icactiver::GicdIcactiver4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icactiver::GicdIcactiver4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver5(
        &self,
    ) -> &'static crate::common::Reg<gicd_icactiver::GicdIcactiver5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icactiver::GicdIcactiver5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Clear-Active"]
    #[inline(always)]
    pub const fn gicd_icactiver6(
        &self,
    ) -> &'static crate::common::Reg<gicd_icactiver::GicdIcactiver6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icactiver::GicdIcactiver6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIcactiver {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_icactiver {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcactiver0_SPEC;
    impl crate::sealed::RegSpec for GicdIcactiver0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Active"]
    pub type GicdIcactiver0 = crate::RegValueT<GicdIcactiver0_SPEC>;

    impl NoBitfieldReg<GicdIcactiver0_SPEC> for GicdIcactiver0 {}
    impl ::core::default::Default for GicdIcactiver0 {
        #[inline(always)]
        fn default() -> GicdIcactiver0 {
            <crate::RegValueT<GicdIcactiver0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcactiver1_SPEC;
    impl crate::sealed::RegSpec for GicdIcactiver1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Active"]
    pub type GicdIcactiver1 = crate::RegValueT<GicdIcactiver1_SPEC>;

    impl NoBitfieldReg<GicdIcactiver1_SPEC> for GicdIcactiver1 {}
    impl ::core::default::Default for GicdIcactiver1 {
        #[inline(always)]
        fn default() -> GicdIcactiver1 {
            <crate::RegValueT<GicdIcactiver1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcactiver2_SPEC;
    impl crate::sealed::RegSpec for GicdIcactiver2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Active"]
    pub type GicdIcactiver2 = crate::RegValueT<GicdIcactiver2_SPEC>;

    impl NoBitfieldReg<GicdIcactiver2_SPEC> for GicdIcactiver2 {}
    impl ::core::default::Default for GicdIcactiver2 {
        #[inline(always)]
        fn default() -> GicdIcactiver2 {
            <crate::RegValueT<GicdIcactiver2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcactiver3_SPEC;
    impl crate::sealed::RegSpec for GicdIcactiver3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Active"]
    pub type GicdIcactiver3 = crate::RegValueT<GicdIcactiver3_SPEC>;

    impl NoBitfieldReg<GicdIcactiver3_SPEC> for GicdIcactiver3 {}
    impl ::core::default::Default for GicdIcactiver3 {
        #[inline(always)]
        fn default() -> GicdIcactiver3 {
            <crate::RegValueT<GicdIcactiver3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcactiver4_SPEC;
    impl crate::sealed::RegSpec for GicdIcactiver4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Active"]
    pub type GicdIcactiver4 = crate::RegValueT<GicdIcactiver4_SPEC>;

    impl NoBitfieldReg<GicdIcactiver4_SPEC> for GicdIcactiver4 {}
    impl ::core::default::Default for GicdIcactiver4 {
        #[inline(always)]
        fn default() -> GicdIcactiver4 {
            <crate::RegValueT<GicdIcactiver4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcactiver5_SPEC;
    impl crate::sealed::RegSpec for GicdIcactiver5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Active"]
    pub type GicdIcactiver5 = crate::RegValueT<GicdIcactiver5_SPEC>;

    impl NoBitfieldReg<GicdIcactiver5_SPEC> for GicdIcactiver5 {}
    impl ::core::default::Default for GicdIcactiver5 {
        #[inline(always)]
        fn default() -> GicdIcactiver5 {
            <crate::RegValueT<GicdIcactiver5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcactiver6_SPEC;
    impl crate::sealed::RegSpec for GicdIcactiver6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Clear-Active"]
    pub type GicdIcactiver6 = crate::RegValueT<GicdIcactiver6_SPEC>;

    impl NoBitfieldReg<GicdIcactiver6_SPEC> for GicdIcactiver6 {}
    impl ::core::default::Default for GicdIcactiver6 {
        #[inline(always)]
        fn default() -> GicdIcactiver6 {
            <crate::RegValueT<GicdIcactiver6_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Priority"]
#[non_exhaustive]
pub struct _GicdIpriorityr;

#[doc = "Interrupt Priority"]
pub type GicdIpriorityr = &'static _GicdIpriorityr;

unsafe impl ::core::marker::Sync for _GicdIpriorityr {}
impl _GicdIpriorityr {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Priority 0 - 3 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr0(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Priority 4 - 7 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr1(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Priority 8 - 11 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr2(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Priority 12 - 15 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr3(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Priority 16 - 19 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr4(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Priority 20 - 23 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr5(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Priority 24 - 27 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr6(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Interrupt Priority 28 - 31 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr7(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Interrupt Priority 32 - 35 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr8(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Interrupt Priority 36 - 39 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr9(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Interrupt Priority 40 - 43 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr10(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr10_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr10_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(40usize))
        }
    }

    #[doc = "Interrupt Priority 44 - 47 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr11(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr11_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr11_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(44usize))
        }
    }

    #[doc = "Interrupt Priority 48 - 51 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr12(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr12_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr12_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(48usize))
        }
    }

    #[doc = "Interrupt Priority 52 - 55 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr13(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr13_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr13_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(52usize))
        }
    }

    #[doc = "Interrupt Priority 56 - 59 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr14(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr14_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr14_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(56usize))
        }
    }

    #[doc = "Interrupt Priority 60 - 63 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr15(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr15_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr15_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(60usize))
        }
    }

    #[doc = "Interrupt Priority 64 - 67 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr16(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr16_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr16_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(64usize))
        }
    }

    #[doc = "Interrupt Priority 68 - 71 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr17(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr17_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr17_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(68usize))
        }
    }

    #[doc = "Interrupt Priority 72 - 75 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr18(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr18_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr18_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(72usize))
        }
    }

    #[doc = "Interrupt Priority 76 - 79 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr19(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr19_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr19_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(76usize))
        }
    }

    #[doc = "Interrupt Priority 80 - 83 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr20(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr20_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr20_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(80usize))
        }
    }

    #[doc = "Interrupt Priority 84 - 87 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr21(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr21_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr21_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(84usize))
        }
    }

    #[doc = "Interrupt Priority 88 - 91 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr22(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr22_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr22_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(88usize))
        }
    }

    #[doc = "Interrupt Priority 92 - 95 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr23(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr23_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr23_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(92usize))
        }
    }

    #[doc = "Interrupt Priority 96 - 99 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr24(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr24_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr24_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(96usize))
        }
    }

    #[doc = "Interrupt Priority 100 - 103 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr25(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr25_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr25_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(100usize))
        }
    }

    #[doc = "Interrupt Priority 104 - 107 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr26(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr26_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr26_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(104usize))
        }
    }

    #[doc = "Interrupt Priority 108 - 111 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr27(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr27_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr27_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(108usize))
        }
    }

    #[doc = "Interrupt Priority 112 - 115 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr28(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr28_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr28_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(112usize))
        }
    }

    #[doc = "Interrupt Priority 116 - 119 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr29(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr29_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr29_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(116usize))
        }
    }

    #[doc = "Interrupt Priority 120 - 123 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr30(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr30_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr30_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(120usize))
        }
    }

    #[doc = "Interrupt Priority 124 - 127 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr31(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr31_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr31_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(124usize))
        }
    }

    #[doc = "Interrupt Priority 128 - 131 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr32(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr32_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr32_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(128usize))
        }
    }

    #[doc = "Interrupt Priority 132 - 135 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr33(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr33_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr33_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(132usize))
        }
    }

    #[doc = "Interrupt Priority 136 - 139 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr34(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr34_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr34_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(136usize))
        }
    }

    #[doc = "Interrupt Priority 140 - 143 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr35(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr35_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr35_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(140usize))
        }
    }

    #[doc = "Interrupt Priority 144 - 147 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr36(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr36_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr36_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(144usize))
        }
    }

    #[doc = "Interrupt Priority 148 - 151 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr37(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr37_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr37_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(148usize))
        }
    }

    #[doc = "Interrupt Priority 152 - 155 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr38(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr38_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr38_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(152usize))
        }
    }

    #[doc = "Interrupt Priority 156 - 159 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr39(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr39_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr39_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(156usize))
        }
    }

    #[doc = "Interrupt Priority 160 - 163 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr40(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr40_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr40_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(160usize))
        }
    }

    #[doc = "Interrupt Priority 164 - 167 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr41(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr41_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr41_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(164usize))
        }
    }

    #[doc = "Interrupt Priority 168 - 171 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr42(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr42_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr42_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(168usize))
        }
    }

    #[doc = "Interrupt Priority 172 - 175 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr43(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr43_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr43_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(172usize))
        }
    }

    #[doc = "Interrupt Priority 176 - 179 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr44(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr44_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr44_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(176usize))
        }
    }

    #[doc = "Interrupt Priority 180 - 183 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr45(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr45_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr45_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(180usize))
        }
    }

    #[doc = "Interrupt Priority 184 - 187 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr46(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr46_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr46_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(184usize))
        }
    }

    #[doc = "Interrupt Priority 188 - 191 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr47(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr47_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr47_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(188usize))
        }
    }

    #[doc = "Interrupt Priority 192 - 195 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr48(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr48_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr48_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(192usize))
        }
    }

    #[doc = "Interrupt Priority 196 - 199 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr49(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr49_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr49_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(196usize))
        }
    }

    #[doc = "Interrupt Priority 200 - 203 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr50(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr50_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr50_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(200usize))
        }
    }

    #[doc = "Interrupt Priority 204 - 207 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr51(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr51_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr51_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(204usize))
        }
    }

    #[doc = "Interrupt Priority 208 - 211 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr52(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr52_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr52_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(208usize))
        }
    }

    #[doc = "Interrupt Priority 212 - 215 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr53(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr53_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr53_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(212usize))
        }
    }

    #[doc = "Interrupt Priority 216 - 219 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr54(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr54_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr54_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(216usize))
        }
    }

    #[doc = "Interrupt Priority 220 - 223 (Lower is first)"]
    #[inline(always)]
    pub const fn gicd_ipriorityr55(
        &self,
    ) -> &'static crate::common::Reg<gicd_ipriorityr::GicdIpriorityr55_SPEC, crate::common::RW>
    {
        unsafe {
            crate::common::Reg::<gicd_ipriorityr::GicdIpriorityr55_SPEC, crate::common::RW>::from_ptr(self._svd2pac_as_ptr().add(220usize))
        }
    }
}

unsafe impl AsPtr for _GicdIpriorityr {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_ipriorityr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr0_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 0 - 3 (Lower is first)"]
    pub type GicdIpriorityr0 = crate::RegValueT<GicdIpriorityr0_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr0_SPEC> for GicdIpriorityr0 {}
    impl ::core::default::Default for GicdIpriorityr0 {
        #[inline(always)]
        fn default() -> GicdIpriorityr0 {
            <crate::RegValueT<GicdIpriorityr0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr1_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 4 - 7 (Lower is first)"]
    pub type GicdIpriorityr1 = crate::RegValueT<GicdIpriorityr1_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr1_SPEC> for GicdIpriorityr1 {}
    impl ::core::default::Default for GicdIpriorityr1 {
        #[inline(always)]
        fn default() -> GicdIpriorityr1 {
            <crate::RegValueT<GicdIpriorityr1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr2_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 8 - 11 (Lower is first)"]
    pub type GicdIpriorityr2 = crate::RegValueT<GicdIpriorityr2_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr2_SPEC> for GicdIpriorityr2 {}
    impl ::core::default::Default for GicdIpriorityr2 {
        #[inline(always)]
        fn default() -> GicdIpriorityr2 {
            <crate::RegValueT<GicdIpriorityr2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr3_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 12 - 15 (Lower is first)"]
    pub type GicdIpriorityr3 = crate::RegValueT<GicdIpriorityr3_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr3_SPEC> for GicdIpriorityr3 {}
    impl ::core::default::Default for GicdIpriorityr3 {
        #[inline(always)]
        fn default() -> GicdIpriorityr3 {
            <crate::RegValueT<GicdIpriorityr3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr4_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 16 - 19 (Lower is first)"]
    pub type GicdIpriorityr4 = crate::RegValueT<GicdIpriorityr4_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr4_SPEC> for GicdIpriorityr4 {}
    impl ::core::default::Default for GicdIpriorityr4 {
        #[inline(always)]
        fn default() -> GicdIpriorityr4 {
            <crate::RegValueT<GicdIpriorityr4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr5_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 20 - 23 (Lower is first)"]
    pub type GicdIpriorityr5 = crate::RegValueT<GicdIpriorityr5_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr5_SPEC> for GicdIpriorityr5 {}
    impl ::core::default::Default for GicdIpriorityr5 {
        #[inline(always)]
        fn default() -> GicdIpriorityr5 {
            <crate::RegValueT<GicdIpriorityr5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr6_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 24 - 27 (Lower is first)"]
    pub type GicdIpriorityr6 = crate::RegValueT<GicdIpriorityr6_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr6_SPEC> for GicdIpriorityr6 {}
    impl ::core::default::Default for GicdIpriorityr6 {
        #[inline(always)]
        fn default() -> GicdIpriorityr6 {
            <crate::RegValueT<GicdIpriorityr6_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr7_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr7_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 28 - 31 (Lower is first)"]
    pub type GicdIpriorityr7 = crate::RegValueT<GicdIpriorityr7_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr7_SPEC> for GicdIpriorityr7 {}
    impl ::core::default::Default for GicdIpriorityr7 {
        #[inline(always)]
        fn default() -> GicdIpriorityr7 {
            <crate::RegValueT<GicdIpriorityr7_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr8_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr8_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 32 - 35 (Lower is first)"]
    pub type GicdIpriorityr8 = crate::RegValueT<GicdIpriorityr8_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr8_SPEC> for GicdIpriorityr8 {}
    impl ::core::default::Default for GicdIpriorityr8 {
        #[inline(always)]
        fn default() -> GicdIpriorityr8 {
            <crate::RegValueT<GicdIpriorityr8_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr9_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr9_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 36 - 39 (Lower is first)"]
    pub type GicdIpriorityr9 = crate::RegValueT<GicdIpriorityr9_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr9_SPEC> for GicdIpriorityr9 {}
    impl ::core::default::Default for GicdIpriorityr9 {
        #[inline(always)]
        fn default() -> GicdIpriorityr9 {
            <crate::RegValueT<GicdIpriorityr9_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr10_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr10_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 40 - 43 (Lower is first)"]
    pub type GicdIpriorityr10 = crate::RegValueT<GicdIpriorityr10_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr10_SPEC> for GicdIpriorityr10 {}
    impl ::core::default::Default for GicdIpriorityr10 {
        #[inline(always)]
        fn default() -> GicdIpriorityr10 {
            <crate::RegValueT<GicdIpriorityr10_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr11_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr11_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 44 - 47 (Lower is first)"]
    pub type GicdIpriorityr11 = crate::RegValueT<GicdIpriorityr11_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr11_SPEC> for GicdIpriorityr11 {}
    impl ::core::default::Default for GicdIpriorityr11 {
        #[inline(always)]
        fn default() -> GicdIpriorityr11 {
            <crate::RegValueT<GicdIpriorityr11_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr12_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr12_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 48 - 51 (Lower is first)"]
    pub type GicdIpriorityr12 = crate::RegValueT<GicdIpriorityr12_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr12_SPEC> for GicdIpriorityr12 {}
    impl ::core::default::Default for GicdIpriorityr12 {
        #[inline(always)]
        fn default() -> GicdIpriorityr12 {
            <crate::RegValueT<GicdIpriorityr12_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr13_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr13_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 52 - 55 (Lower is first)"]
    pub type GicdIpriorityr13 = crate::RegValueT<GicdIpriorityr13_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr13_SPEC> for GicdIpriorityr13 {}
    impl ::core::default::Default for GicdIpriorityr13 {
        #[inline(always)]
        fn default() -> GicdIpriorityr13 {
            <crate::RegValueT<GicdIpriorityr13_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr14_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr14_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 56 - 59 (Lower is first)"]
    pub type GicdIpriorityr14 = crate::RegValueT<GicdIpriorityr14_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr14_SPEC> for GicdIpriorityr14 {}
    impl ::core::default::Default for GicdIpriorityr14 {
        #[inline(always)]
        fn default() -> GicdIpriorityr14 {
            <crate::RegValueT<GicdIpriorityr14_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr15_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr15_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 60 - 63 (Lower is first)"]
    pub type GicdIpriorityr15 = crate::RegValueT<GicdIpriorityr15_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr15_SPEC> for GicdIpriorityr15 {}
    impl ::core::default::Default for GicdIpriorityr15 {
        #[inline(always)]
        fn default() -> GicdIpriorityr15 {
            <crate::RegValueT<GicdIpriorityr15_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr16_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr16_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 64 - 67 (Lower is first)"]
    pub type GicdIpriorityr16 = crate::RegValueT<GicdIpriorityr16_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr16_SPEC> for GicdIpriorityr16 {}
    impl ::core::default::Default for GicdIpriorityr16 {
        #[inline(always)]
        fn default() -> GicdIpriorityr16 {
            <crate::RegValueT<GicdIpriorityr16_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr17_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr17_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 68 - 71 (Lower is first)"]
    pub type GicdIpriorityr17 = crate::RegValueT<GicdIpriorityr17_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr17_SPEC> for GicdIpriorityr17 {}
    impl ::core::default::Default for GicdIpriorityr17 {
        #[inline(always)]
        fn default() -> GicdIpriorityr17 {
            <crate::RegValueT<GicdIpriorityr17_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr18_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr18_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 72 - 75 (Lower is first)"]
    pub type GicdIpriorityr18 = crate::RegValueT<GicdIpriorityr18_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr18_SPEC> for GicdIpriorityr18 {}
    impl ::core::default::Default for GicdIpriorityr18 {
        #[inline(always)]
        fn default() -> GicdIpriorityr18 {
            <crate::RegValueT<GicdIpriorityr18_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr19_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr19_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 76 - 79 (Lower is first)"]
    pub type GicdIpriorityr19 = crate::RegValueT<GicdIpriorityr19_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr19_SPEC> for GicdIpriorityr19 {}
    impl ::core::default::Default for GicdIpriorityr19 {
        #[inline(always)]
        fn default() -> GicdIpriorityr19 {
            <crate::RegValueT<GicdIpriorityr19_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr20_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr20_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 80 - 83 (Lower is first)"]
    pub type GicdIpriorityr20 = crate::RegValueT<GicdIpriorityr20_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr20_SPEC> for GicdIpriorityr20 {}
    impl ::core::default::Default for GicdIpriorityr20 {
        #[inline(always)]
        fn default() -> GicdIpriorityr20 {
            <crate::RegValueT<GicdIpriorityr20_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr21_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr21_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 84 - 87 (Lower is first)"]
    pub type GicdIpriorityr21 = crate::RegValueT<GicdIpriorityr21_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr21_SPEC> for GicdIpriorityr21 {}
    impl ::core::default::Default for GicdIpriorityr21 {
        #[inline(always)]
        fn default() -> GicdIpriorityr21 {
            <crate::RegValueT<GicdIpriorityr21_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr22_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr22_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 88 - 91 (Lower is first)"]
    pub type GicdIpriorityr22 = crate::RegValueT<GicdIpriorityr22_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr22_SPEC> for GicdIpriorityr22 {}
    impl ::core::default::Default for GicdIpriorityr22 {
        #[inline(always)]
        fn default() -> GicdIpriorityr22 {
            <crate::RegValueT<GicdIpriorityr22_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr23_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr23_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 92 - 95 (Lower is first)"]
    pub type GicdIpriorityr23 = crate::RegValueT<GicdIpriorityr23_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr23_SPEC> for GicdIpriorityr23 {}
    impl ::core::default::Default for GicdIpriorityr23 {
        #[inline(always)]
        fn default() -> GicdIpriorityr23 {
            <crate::RegValueT<GicdIpriorityr23_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr24_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr24_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 96 - 99 (Lower is first)"]
    pub type GicdIpriorityr24 = crate::RegValueT<GicdIpriorityr24_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr24_SPEC> for GicdIpriorityr24 {}
    impl ::core::default::Default for GicdIpriorityr24 {
        #[inline(always)]
        fn default() -> GicdIpriorityr24 {
            <crate::RegValueT<GicdIpriorityr24_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr25_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr25_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 100 - 103 (Lower is first)"]
    pub type GicdIpriorityr25 = crate::RegValueT<GicdIpriorityr25_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr25_SPEC> for GicdIpriorityr25 {}
    impl ::core::default::Default for GicdIpriorityr25 {
        #[inline(always)]
        fn default() -> GicdIpriorityr25 {
            <crate::RegValueT<GicdIpriorityr25_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr26_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr26_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 104 - 107 (Lower is first)"]
    pub type GicdIpriorityr26 = crate::RegValueT<GicdIpriorityr26_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr26_SPEC> for GicdIpriorityr26 {}
    impl ::core::default::Default for GicdIpriorityr26 {
        #[inline(always)]
        fn default() -> GicdIpriorityr26 {
            <crate::RegValueT<GicdIpriorityr26_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr27_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr27_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 108 - 111 (Lower is first)"]
    pub type GicdIpriorityr27 = crate::RegValueT<GicdIpriorityr27_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr27_SPEC> for GicdIpriorityr27 {}
    impl ::core::default::Default for GicdIpriorityr27 {
        #[inline(always)]
        fn default() -> GicdIpriorityr27 {
            <crate::RegValueT<GicdIpriorityr27_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr28_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr28_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 112 - 115 (Lower is first)"]
    pub type GicdIpriorityr28 = crate::RegValueT<GicdIpriorityr28_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr28_SPEC> for GicdIpriorityr28 {}
    impl ::core::default::Default for GicdIpriorityr28 {
        #[inline(always)]
        fn default() -> GicdIpriorityr28 {
            <crate::RegValueT<GicdIpriorityr28_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr29_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr29_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 116 - 119 (Lower is first)"]
    pub type GicdIpriorityr29 = crate::RegValueT<GicdIpriorityr29_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr29_SPEC> for GicdIpriorityr29 {}
    impl ::core::default::Default for GicdIpriorityr29 {
        #[inline(always)]
        fn default() -> GicdIpriorityr29 {
            <crate::RegValueT<GicdIpriorityr29_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr30_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr30_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 120 - 123 (Lower is first)"]
    pub type GicdIpriorityr30 = crate::RegValueT<GicdIpriorityr30_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr30_SPEC> for GicdIpriorityr30 {}
    impl ::core::default::Default for GicdIpriorityr30 {
        #[inline(always)]
        fn default() -> GicdIpriorityr30 {
            <crate::RegValueT<GicdIpriorityr30_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr31_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr31_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 124 - 127 (Lower is first)"]
    pub type GicdIpriorityr31 = crate::RegValueT<GicdIpriorityr31_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr31_SPEC> for GicdIpriorityr31 {}
    impl ::core::default::Default for GicdIpriorityr31 {
        #[inline(always)]
        fn default() -> GicdIpriorityr31 {
            <crate::RegValueT<GicdIpriorityr31_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr32_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr32_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 128 - 131 (Lower is first)"]
    pub type GicdIpriorityr32 = crate::RegValueT<GicdIpriorityr32_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr32_SPEC> for GicdIpriorityr32 {}
    impl ::core::default::Default for GicdIpriorityr32 {
        #[inline(always)]
        fn default() -> GicdIpriorityr32 {
            <crate::RegValueT<GicdIpriorityr32_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr33_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr33_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 132 - 135 (Lower is first)"]
    pub type GicdIpriorityr33 = crate::RegValueT<GicdIpriorityr33_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr33_SPEC> for GicdIpriorityr33 {}
    impl ::core::default::Default for GicdIpriorityr33 {
        #[inline(always)]
        fn default() -> GicdIpriorityr33 {
            <crate::RegValueT<GicdIpriorityr33_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr34_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr34_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 136 - 139 (Lower is first)"]
    pub type GicdIpriorityr34 = crate::RegValueT<GicdIpriorityr34_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr34_SPEC> for GicdIpriorityr34 {}
    impl ::core::default::Default for GicdIpriorityr34 {
        #[inline(always)]
        fn default() -> GicdIpriorityr34 {
            <crate::RegValueT<GicdIpriorityr34_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr35_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr35_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 140 - 143 (Lower is first)"]
    pub type GicdIpriorityr35 = crate::RegValueT<GicdIpriorityr35_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr35_SPEC> for GicdIpriorityr35 {}
    impl ::core::default::Default for GicdIpriorityr35 {
        #[inline(always)]
        fn default() -> GicdIpriorityr35 {
            <crate::RegValueT<GicdIpriorityr35_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr36_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr36_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 144 - 147 (Lower is first)"]
    pub type GicdIpriorityr36 = crate::RegValueT<GicdIpriorityr36_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr36_SPEC> for GicdIpriorityr36 {}
    impl ::core::default::Default for GicdIpriorityr36 {
        #[inline(always)]
        fn default() -> GicdIpriorityr36 {
            <crate::RegValueT<GicdIpriorityr36_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr37_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr37_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 148 - 151 (Lower is first)"]
    pub type GicdIpriorityr37 = crate::RegValueT<GicdIpriorityr37_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr37_SPEC> for GicdIpriorityr37 {}
    impl ::core::default::Default for GicdIpriorityr37 {
        #[inline(always)]
        fn default() -> GicdIpriorityr37 {
            <crate::RegValueT<GicdIpriorityr37_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr38_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr38_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 152 - 155 (Lower is first)"]
    pub type GicdIpriorityr38 = crate::RegValueT<GicdIpriorityr38_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr38_SPEC> for GicdIpriorityr38 {}
    impl ::core::default::Default for GicdIpriorityr38 {
        #[inline(always)]
        fn default() -> GicdIpriorityr38 {
            <crate::RegValueT<GicdIpriorityr38_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr39_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr39_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 156 - 159 (Lower is first)"]
    pub type GicdIpriorityr39 = crate::RegValueT<GicdIpriorityr39_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr39_SPEC> for GicdIpriorityr39 {}
    impl ::core::default::Default for GicdIpriorityr39 {
        #[inline(always)]
        fn default() -> GicdIpriorityr39 {
            <crate::RegValueT<GicdIpriorityr39_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr40_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr40_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 160 - 163 (Lower is first)"]
    pub type GicdIpriorityr40 = crate::RegValueT<GicdIpriorityr40_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr40_SPEC> for GicdIpriorityr40 {}
    impl ::core::default::Default for GicdIpriorityr40 {
        #[inline(always)]
        fn default() -> GicdIpriorityr40 {
            <crate::RegValueT<GicdIpriorityr40_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr41_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr41_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 164 - 167 (Lower is first)"]
    pub type GicdIpriorityr41 = crate::RegValueT<GicdIpriorityr41_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr41_SPEC> for GicdIpriorityr41 {}
    impl ::core::default::Default for GicdIpriorityr41 {
        #[inline(always)]
        fn default() -> GicdIpriorityr41 {
            <crate::RegValueT<GicdIpriorityr41_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr42_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr42_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 168 - 171 (Lower is first)"]
    pub type GicdIpriorityr42 = crate::RegValueT<GicdIpriorityr42_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr42_SPEC> for GicdIpriorityr42 {}
    impl ::core::default::Default for GicdIpriorityr42 {
        #[inline(always)]
        fn default() -> GicdIpriorityr42 {
            <crate::RegValueT<GicdIpriorityr42_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr43_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr43_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 172 - 175 (Lower is first)"]
    pub type GicdIpriorityr43 = crate::RegValueT<GicdIpriorityr43_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr43_SPEC> for GicdIpriorityr43 {}
    impl ::core::default::Default for GicdIpriorityr43 {
        #[inline(always)]
        fn default() -> GicdIpriorityr43 {
            <crate::RegValueT<GicdIpriorityr43_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr44_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr44_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 176 - 179 (Lower is first)"]
    pub type GicdIpriorityr44 = crate::RegValueT<GicdIpriorityr44_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr44_SPEC> for GicdIpriorityr44 {}
    impl ::core::default::Default for GicdIpriorityr44 {
        #[inline(always)]
        fn default() -> GicdIpriorityr44 {
            <crate::RegValueT<GicdIpriorityr44_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr45_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr45_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 180 - 183 (Lower is first)"]
    pub type GicdIpriorityr45 = crate::RegValueT<GicdIpriorityr45_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr45_SPEC> for GicdIpriorityr45 {}
    impl ::core::default::Default for GicdIpriorityr45 {
        #[inline(always)]
        fn default() -> GicdIpriorityr45 {
            <crate::RegValueT<GicdIpriorityr45_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr46_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr46_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 184 - 187 (Lower is first)"]
    pub type GicdIpriorityr46 = crate::RegValueT<GicdIpriorityr46_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr46_SPEC> for GicdIpriorityr46 {}
    impl ::core::default::Default for GicdIpriorityr46 {
        #[inline(always)]
        fn default() -> GicdIpriorityr46 {
            <crate::RegValueT<GicdIpriorityr46_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr47_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr47_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 188 - 191 (Lower is first)"]
    pub type GicdIpriorityr47 = crate::RegValueT<GicdIpriorityr47_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr47_SPEC> for GicdIpriorityr47 {}
    impl ::core::default::Default for GicdIpriorityr47 {
        #[inline(always)]
        fn default() -> GicdIpriorityr47 {
            <crate::RegValueT<GicdIpriorityr47_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr48_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr48_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 192 - 195 (Lower is first)"]
    pub type GicdIpriorityr48 = crate::RegValueT<GicdIpriorityr48_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr48_SPEC> for GicdIpriorityr48 {}
    impl ::core::default::Default for GicdIpriorityr48 {
        #[inline(always)]
        fn default() -> GicdIpriorityr48 {
            <crate::RegValueT<GicdIpriorityr48_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr49_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr49_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 196 - 199 (Lower is first)"]
    pub type GicdIpriorityr49 = crate::RegValueT<GicdIpriorityr49_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr49_SPEC> for GicdIpriorityr49 {}
    impl ::core::default::Default for GicdIpriorityr49 {
        #[inline(always)]
        fn default() -> GicdIpriorityr49 {
            <crate::RegValueT<GicdIpriorityr49_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr50_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr50_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 200 - 203 (Lower is first)"]
    pub type GicdIpriorityr50 = crate::RegValueT<GicdIpriorityr50_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr50_SPEC> for GicdIpriorityr50 {}
    impl ::core::default::Default for GicdIpriorityr50 {
        #[inline(always)]
        fn default() -> GicdIpriorityr50 {
            <crate::RegValueT<GicdIpriorityr50_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr51_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr51_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 204 - 207 (Lower is first)"]
    pub type GicdIpriorityr51 = crate::RegValueT<GicdIpriorityr51_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr51_SPEC> for GicdIpriorityr51 {}
    impl ::core::default::Default for GicdIpriorityr51 {
        #[inline(always)]
        fn default() -> GicdIpriorityr51 {
            <crate::RegValueT<GicdIpriorityr51_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr52_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr52_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 208 - 211 (Lower is first)"]
    pub type GicdIpriorityr52 = crate::RegValueT<GicdIpriorityr52_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr52_SPEC> for GicdIpriorityr52 {}
    impl ::core::default::Default for GicdIpriorityr52 {
        #[inline(always)]
        fn default() -> GicdIpriorityr52 {
            <crate::RegValueT<GicdIpriorityr52_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr53_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr53_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 212 - 215 (Lower is first)"]
    pub type GicdIpriorityr53 = crate::RegValueT<GicdIpriorityr53_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr53_SPEC> for GicdIpriorityr53 {}
    impl ::core::default::Default for GicdIpriorityr53 {
        #[inline(always)]
        fn default() -> GicdIpriorityr53 {
            <crate::RegValueT<GicdIpriorityr53_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr54_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr54_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 216 - 219 (Lower is first)"]
    pub type GicdIpriorityr54 = crate::RegValueT<GicdIpriorityr54_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr54_SPEC> for GicdIpriorityr54 {}
    impl ::core::default::Default for GicdIpriorityr54 {
        #[inline(always)]
        fn default() -> GicdIpriorityr54 {
            <crate::RegValueT<GicdIpriorityr54_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIpriorityr55_SPEC;
    impl crate::sealed::RegSpec for GicdIpriorityr55_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Priority 220 - 223 (Lower is first)"]
    pub type GicdIpriorityr55 = crate::RegValueT<GicdIpriorityr55_SPEC>;

    impl NoBitfieldReg<GicdIpriorityr55_SPEC> for GicdIpriorityr55 {}
    impl ::core::default::Default for GicdIpriorityr55 {
        #[inline(always)]
        fn default() -> GicdIpriorityr55 {
            <crate::RegValueT<GicdIpriorityr55_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Processor Targets"]
#[non_exhaustive]
pub struct _GicdItargetsr;

#[doc = "Interrupt Processor Targets"]
pub type GicdItargetsr = &'static _GicdItargetsr;

unsafe impl ::core::marker::Sync for _GicdItargetsr {}
impl _GicdItargetsr {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Processor Target 0 - 3"]
    #[inline(always)]
    pub const fn gicd_itargetsr0(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 4 - 7"]
    #[inline(always)]
    pub const fn gicd_itargetsr1(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 8 - 11"]
    #[inline(always)]
    pub const fn gicd_itargetsr2(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 12 - 15"]
    #[inline(always)]
    pub const fn gicd_itargetsr3(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 16 - 19"]
    #[inline(always)]
    pub const fn gicd_itargetsr4(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 20 - 23"]
    #[inline(always)]
    pub const fn gicd_itargetsr5(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 24 - 27"]
    #[inline(always)]
    pub const fn gicd_itargetsr6(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 28 - 31"]
    #[inline(always)]
    pub const fn gicd_itargetsr7(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 32 - 35"]
    #[inline(always)]
    pub const fn gicd_itargetsr8(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 36 - 39"]
    #[inline(always)]
    pub const fn gicd_itargetsr9(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 40 - 43"]
    #[inline(always)]
    pub const fn gicd_itargetsr10(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 44 - 47"]
    #[inline(always)]
    pub const fn gicd_itargetsr11(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 48 - 51"]
    #[inline(always)]
    pub const fn gicd_itargetsr12(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 52 - 55"]
    #[inline(always)]
    pub const fn gicd_itargetsr13(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 56 - 59"]
    #[inline(always)]
    pub const fn gicd_itargetsr14(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 60 - 63"]
    #[inline(always)]
    pub const fn gicd_itargetsr15(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr15_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr15_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 64 - 67"]
    #[inline(always)]
    pub const fn gicd_itargetsr16(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr16_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr16_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 68 - 71"]
    #[inline(always)]
    pub const fn gicd_itargetsr17(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr17_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr17_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 72 - 75"]
    #[inline(always)]
    pub const fn gicd_itargetsr18(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr18_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr18_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 76 - 79"]
    #[inline(always)]
    pub const fn gicd_itargetsr19(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr19_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr19_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 80 - 83"]
    #[inline(always)]
    pub const fn gicd_itargetsr20(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 84 - 87"]
    #[inline(always)]
    pub const fn gicd_itargetsr21(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr21_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr21_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 88 - 91"]
    #[inline(always)]
    pub const fn gicd_itargetsr22(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr22_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr22_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 92 - 95"]
    #[inline(always)]
    pub const fn gicd_itargetsr23(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr23_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr23_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 96 - 99"]
    #[inline(always)]
    pub const fn gicd_itargetsr24(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr24_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr24_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 100 - 103"]
    #[inline(always)]
    pub const fn gicd_itargetsr25(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr25_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr25_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 104 - 107"]
    #[inline(always)]
    pub const fn gicd_itargetsr26(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr26_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr26_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 108 - 111"]
    #[inline(always)]
    pub const fn gicd_itargetsr27(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr27_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr27_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 112 - 115"]
    #[inline(always)]
    pub const fn gicd_itargetsr28(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr28_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr28_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 116 - 119"]
    #[inline(always)]
    pub const fn gicd_itargetsr29(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr29_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr29_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 120 - 123"]
    #[inline(always)]
    pub const fn gicd_itargetsr30(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr30_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr30_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 124 - 127"]
    #[inline(always)]
    pub const fn gicd_itargetsr31(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr31_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr31_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 128 - 131"]
    #[inline(always)]
    pub const fn gicd_itargetsr32(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr32_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr32_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 132 - 135"]
    #[inline(always)]
    pub const fn gicd_itargetsr33(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr33_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr33_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 136 - 139"]
    #[inline(always)]
    pub const fn gicd_itargetsr34(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr34_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr34_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 140 - 143"]
    #[inline(always)]
    pub const fn gicd_itargetsr35(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr35_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr35_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 144 - 147"]
    #[inline(always)]
    pub const fn gicd_itargetsr36(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr36_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr36_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 148 - 151"]
    #[inline(always)]
    pub const fn gicd_itargetsr37(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr37_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr37_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 152 - 155"]
    #[inline(always)]
    pub const fn gicd_itargetsr38(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr38_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr38_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 156 - 159"]
    #[inline(always)]
    pub const fn gicd_itargetsr39(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr39_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr39_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 160 - 163"]
    #[inline(always)]
    pub const fn gicd_itargetsr40(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr40_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr40_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 164 - 167"]
    #[inline(always)]
    pub const fn gicd_itargetsr41(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr41_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr41_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 168 - 171"]
    #[inline(always)]
    pub const fn gicd_itargetsr42(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr42_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr42_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 172 - 175"]
    #[inline(always)]
    pub const fn gicd_itargetsr43(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr43_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr43_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 176 - 179"]
    #[inline(always)]
    pub const fn gicd_itargetsr44(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr44_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr44_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 180 - 183"]
    #[inline(always)]
    pub const fn gicd_itargetsr45(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr45_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr45_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 184 - 187"]
    #[inline(always)]
    pub const fn gicd_itargetsr46(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr46_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr46_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 188 - 191"]
    #[inline(always)]
    pub const fn gicd_itargetsr47(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr47_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr47_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 192 - 195"]
    #[inline(always)]
    pub const fn gicd_itargetsr48(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr48_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr48_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 196 - 199"]
    #[inline(always)]
    pub const fn gicd_itargetsr49(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr49_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr49_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 200 - 203"]
    #[inline(always)]
    pub const fn gicd_itargetsr50(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr50_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr50_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 204 - 207"]
    #[inline(always)]
    pub const fn gicd_itargetsr51(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr51_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr51_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 208 - 211"]
    #[inline(always)]
    pub const fn gicd_itargetsr52(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr52_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr52_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 212 - 215"]
    #[inline(always)]
    pub const fn gicd_itargetsr53(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr53_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr53_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 216 - 219"]
    #[inline(always)]
    pub const fn gicd_itargetsr54(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr54_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr54_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[doc = "Interrupt Processor Target 220 - 223"]
    #[inline(always)]
    pub const fn gicd_itargetsr55(
        &self,
    ) -> &'static crate::common::Reg<gicd_itargetsr::GicdItargetsr55_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_itargetsr::GicdItargetsr55_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdItargetsr {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_itargetsr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr0_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 0 - 3"]
    pub type GicdItargetsr0 = crate::RegValueT<GicdItargetsr0_SPEC>;

    impl NoBitfieldReg<GicdItargetsr0_SPEC> for GicdItargetsr0 {}
    impl ::core::default::Default for GicdItargetsr0 {
        #[inline(always)]
        fn default() -> GicdItargetsr0 {
            <crate::RegValueT<GicdItargetsr0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr1_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr1_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 4 - 7"]
    pub type GicdItargetsr1 = crate::RegValueT<GicdItargetsr1_SPEC>;

    impl NoBitfieldReg<GicdItargetsr1_SPEC> for GicdItargetsr1 {}
    impl ::core::default::Default for GicdItargetsr1 {
        #[inline(always)]
        fn default() -> GicdItargetsr1 {
            <crate::RegValueT<GicdItargetsr1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr2_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr2_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 8 - 11"]
    pub type GicdItargetsr2 = crate::RegValueT<GicdItargetsr2_SPEC>;

    impl NoBitfieldReg<GicdItargetsr2_SPEC> for GicdItargetsr2 {}
    impl ::core::default::Default for GicdItargetsr2 {
        #[inline(always)]
        fn default() -> GicdItargetsr2 {
            <crate::RegValueT<GicdItargetsr2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr3_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr3_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 12 - 15"]
    pub type GicdItargetsr3 = crate::RegValueT<GicdItargetsr3_SPEC>;

    impl NoBitfieldReg<GicdItargetsr3_SPEC> for GicdItargetsr3 {}
    impl ::core::default::Default for GicdItargetsr3 {
        #[inline(always)]
        fn default() -> GicdItargetsr3 {
            <crate::RegValueT<GicdItargetsr3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr4_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 16 - 19"]
    pub type GicdItargetsr4 = crate::RegValueT<GicdItargetsr4_SPEC>;

    impl NoBitfieldReg<GicdItargetsr4_SPEC> for GicdItargetsr4 {}
    impl ::core::default::Default for GicdItargetsr4 {
        #[inline(always)]
        fn default() -> GicdItargetsr4 {
            <crate::RegValueT<GicdItargetsr4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr5_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr5_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 20 - 23"]
    pub type GicdItargetsr5 = crate::RegValueT<GicdItargetsr5_SPEC>;

    impl NoBitfieldReg<GicdItargetsr5_SPEC> for GicdItargetsr5 {}
    impl ::core::default::Default for GicdItargetsr5 {
        #[inline(always)]
        fn default() -> GicdItargetsr5 {
            <crate::RegValueT<GicdItargetsr5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr6_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr6_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 24 - 27"]
    pub type GicdItargetsr6 = crate::RegValueT<GicdItargetsr6_SPEC>;

    impl NoBitfieldReg<GicdItargetsr6_SPEC> for GicdItargetsr6 {}
    impl ::core::default::Default for GicdItargetsr6 {
        #[inline(always)]
        fn default() -> GicdItargetsr6 {
            <crate::RegValueT<GicdItargetsr6_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr7_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr7_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 28 - 31"]
    pub type GicdItargetsr7 = crate::RegValueT<GicdItargetsr7_SPEC>;

    impl NoBitfieldReg<GicdItargetsr7_SPEC> for GicdItargetsr7 {}
    impl ::core::default::Default for GicdItargetsr7 {
        #[inline(always)]
        fn default() -> GicdItargetsr7 {
            <crate::RegValueT<GicdItargetsr7_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr8_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr8_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 32 - 35"]
    pub type GicdItargetsr8 = crate::RegValueT<GicdItargetsr8_SPEC>;

    impl NoBitfieldReg<GicdItargetsr8_SPEC> for GicdItargetsr8 {}
    impl ::core::default::Default for GicdItargetsr8 {
        #[inline(always)]
        fn default() -> GicdItargetsr8 {
            <crate::RegValueT<GicdItargetsr8_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr9_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr9_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 36 - 39"]
    pub type GicdItargetsr9 = crate::RegValueT<GicdItargetsr9_SPEC>;

    impl NoBitfieldReg<GicdItargetsr9_SPEC> for GicdItargetsr9 {}
    impl ::core::default::Default for GicdItargetsr9 {
        #[inline(always)]
        fn default() -> GicdItargetsr9 {
            <crate::RegValueT<GicdItargetsr9_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr10_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr10_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 40 - 43"]
    pub type GicdItargetsr10 = crate::RegValueT<GicdItargetsr10_SPEC>;

    impl NoBitfieldReg<GicdItargetsr10_SPEC> for GicdItargetsr10 {}
    impl ::core::default::Default for GicdItargetsr10 {
        #[inline(always)]
        fn default() -> GicdItargetsr10 {
            <crate::RegValueT<GicdItargetsr10_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr11_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr11_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 44 - 47"]
    pub type GicdItargetsr11 = crate::RegValueT<GicdItargetsr11_SPEC>;

    impl NoBitfieldReg<GicdItargetsr11_SPEC> for GicdItargetsr11 {}
    impl ::core::default::Default for GicdItargetsr11 {
        #[inline(always)]
        fn default() -> GicdItargetsr11 {
            <crate::RegValueT<GicdItargetsr11_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr12_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr12_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 48 - 51"]
    pub type GicdItargetsr12 = crate::RegValueT<GicdItargetsr12_SPEC>;

    impl NoBitfieldReg<GicdItargetsr12_SPEC> for GicdItargetsr12 {}
    impl ::core::default::Default for GicdItargetsr12 {
        #[inline(always)]
        fn default() -> GicdItargetsr12 {
            <crate::RegValueT<GicdItargetsr12_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr13_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr13_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 52 - 55"]
    pub type GicdItargetsr13 = crate::RegValueT<GicdItargetsr13_SPEC>;

    impl NoBitfieldReg<GicdItargetsr13_SPEC> for GicdItargetsr13 {}
    impl ::core::default::Default for GicdItargetsr13 {
        #[inline(always)]
        fn default() -> GicdItargetsr13 {
            <crate::RegValueT<GicdItargetsr13_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr14_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr14_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 56 - 59"]
    pub type GicdItargetsr14 = crate::RegValueT<GicdItargetsr14_SPEC>;

    impl NoBitfieldReg<GicdItargetsr14_SPEC> for GicdItargetsr14 {}
    impl ::core::default::Default for GicdItargetsr14 {
        #[inline(always)]
        fn default() -> GicdItargetsr14 {
            <crate::RegValueT<GicdItargetsr14_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr15_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr15_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 60 - 63"]
    pub type GicdItargetsr15 = crate::RegValueT<GicdItargetsr15_SPEC>;

    impl NoBitfieldReg<GicdItargetsr15_SPEC> for GicdItargetsr15 {}
    impl ::core::default::Default for GicdItargetsr15 {
        #[inline(always)]
        fn default() -> GicdItargetsr15 {
            <crate::RegValueT<GicdItargetsr15_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr16_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr16_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 64 - 67"]
    pub type GicdItargetsr16 = crate::RegValueT<GicdItargetsr16_SPEC>;

    impl NoBitfieldReg<GicdItargetsr16_SPEC> for GicdItargetsr16 {}
    impl ::core::default::Default for GicdItargetsr16 {
        #[inline(always)]
        fn default() -> GicdItargetsr16 {
            <crate::RegValueT<GicdItargetsr16_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr17_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr17_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 68 - 71"]
    pub type GicdItargetsr17 = crate::RegValueT<GicdItargetsr17_SPEC>;

    impl NoBitfieldReg<GicdItargetsr17_SPEC> for GicdItargetsr17 {}
    impl ::core::default::Default for GicdItargetsr17 {
        #[inline(always)]
        fn default() -> GicdItargetsr17 {
            <crate::RegValueT<GicdItargetsr17_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr18_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr18_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 72 - 75"]
    pub type GicdItargetsr18 = crate::RegValueT<GicdItargetsr18_SPEC>;

    impl NoBitfieldReg<GicdItargetsr18_SPEC> for GicdItargetsr18 {}
    impl ::core::default::Default for GicdItargetsr18 {
        #[inline(always)]
        fn default() -> GicdItargetsr18 {
            <crate::RegValueT<GicdItargetsr18_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr19_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr19_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 76 - 79"]
    pub type GicdItargetsr19 = crate::RegValueT<GicdItargetsr19_SPEC>;

    impl NoBitfieldReg<GicdItargetsr19_SPEC> for GicdItargetsr19 {}
    impl ::core::default::Default for GicdItargetsr19 {
        #[inline(always)]
        fn default() -> GicdItargetsr19 {
            <crate::RegValueT<GicdItargetsr19_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr20_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr20_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 80 - 83"]
    pub type GicdItargetsr20 = crate::RegValueT<GicdItargetsr20_SPEC>;

    impl NoBitfieldReg<GicdItargetsr20_SPEC> for GicdItargetsr20 {}
    impl ::core::default::Default for GicdItargetsr20 {
        #[inline(always)]
        fn default() -> GicdItargetsr20 {
            <crate::RegValueT<GicdItargetsr20_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr21_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr21_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 84 - 87"]
    pub type GicdItargetsr21 = crate::RegValueT<GicdItargetsr21_SPEC>;

    impl NoBitfieldReg<GicdItargetsr21_SPEC> for GicdItargetsr21 {}
    impl ::core::default::Default for GicdItargetsr21 {
        #[inline(always)]
        fn default() -> GicdItargetsr21 {
            <crate::RegValueT<GicdItargetsr21_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr22_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr22_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 88 - 91"]
    pub type GicdItargetsr22 = crate::RegValueT<GicdItargetsr22_SPEC>;

    impl NoBitfieldReg<GicdItargetsr22_SPEC> for GicdItargetsr22 {}
    impl ::core::default::Default for GicdItargetsr22 {
        #[inline(always)]
        fn default() -> GicdItargetsr22 {
            <crate::RegValueT<GicdItargetsr22_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr23_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr23_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 92 - 95"]
    pub type GicdItargetsr23 = crate::RegValueT<GicdItargetsr23_SPEC>;

    impl NoBitfieldReg<GicdItargetsr23_SPEC> for GicdItargetsr23 {}
    impl ::core::default::Default for GicdItargetsr23 {
        #[inline(always)]
        fn default() -> GicdItargetsr23 {
            <crate::RegValueT<GicdItargetsr23_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr24_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr24_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 96 - 99"]
    pub type GicdItargetsr24 = crate::RegValueT<GicdItargetsr24_SPEC>;

    impl NoBitfieldReg<GicdItargetsr24_SPEC> for GicdItargetsr24 {}
    impl ::core::default::Default for GicdItargetsr24 {
        #[inline(always)]
        fn default() -> GicdItargetsr24 {
            <crate::RegValueT<GicdItargetsr24_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr25_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr25_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 100 - 103"]
    pub type GicdItargetsr25 = crate::RegValueT<GicdItargetsr25_SPEC>;

    impl NoBitfieldReg<GicdItargetsr25_SPEC> for GicdItargetsr25 {}
    impl ::core::default::Default for GicdItargetsr25 {
        #[inline(always)]
        fn default() -> GicdItargetsr25 {
            <crate::RegValueT<GicdItargetsr25_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr26_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr26_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 104 - 107"]
    pub type GicdItargetsr26 = crate::RegValueT<GicdItargetsr26_SPEC>;

    impl NoBitfieldReg<GicdItargetsr26_SPEC> for GicdItargetsr26 {}
    impl ::core::default::Default for GicdItargetsr26 {
        #[inline(always)]
        fn default() -> GicdItargetsr26 {
            <crate::RegValueT<GicdItargetsr26_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr27_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr27_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 108 - 111"]
    pub type GicdItargetsr27 = crate::RegValueT<GicdItargetsr27_SPEC>;

    impl NoBitfieldReg<GicdItargetsr27_SPEC> for GicdItargetsr27 {}
    impl ::core::default::Default for GicdItargetsr27 {
        #[inline(always)]
        fn default() -> GicdItargetsr27 {
            <crate::RegValueT<GicdItargetsr27_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr28_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr28_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 112 - 115"]
    pub type GicdItargetsr28 = crate::RegValueT<GicdItargetsr28_SPEC>;

    impl NoBitfieldReg<GicdItargetsr28_SPEC> for GicdItargetsr28 {}
    impl ::core::default::Default for GicdItargetsr28 {
        #[inline(always)]
        fn default() -> GicdItargetsr28 {
            <crate::RegValueT<GicdItargetsr28_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr29_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr29_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 116 - 119"]
    pub type GicdItargetsr29 = crate::RegValueT<GicdItargetsr29_SPEC>;

    impl NoBitfieldReg<GicdItargetsr29_SPEC> for GicdItargetsr29 {}
    impl ::core::default::Default for GicdItargetsr29 {
        #[inline(always)]
        fn default() -> GicdItargetsr29 {
            <crate::RegValueT<GicdItargetsr29_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr30_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr30_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 120 - 123"]
    pub type GicdItargetsr30 = crate::RegValueT<GicdItargetsr30_SPEC>;

    impl NoBitfieldReg<GicdItargetsr30_SPEC> for GicdItargetsr30 {}
    impl ::core::default::Default for GicdItargetsr30 {
        #[inline(always)]
        fn default() -> GicdItargetsr30 {
            <crate::RegValueT<GicdItargetsr30_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr31_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr31_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 124 - 127"]
    pub type GicdItargetsr31 = crate::RegValueT<GicdItargetsr31_SPEC>;

    impl NoBitfieldReg<GicdItargetsr31_SPEC> for GicdItargetsr31 {}
    impl ::core::default::Default for GicdItargetsr31 {
        #[inline(always)]
        fn default() -> GicdItargetsr31 {
            <crate::RegValueT<GicdItargetsr31_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr32_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr32_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 128 - 131"]
    pub type GicdItargetsr32 = crate::RegValueT<GicdItargetsr32_SPEC>;

    impl NoBitfieldReg<GicdItargetsr32_SPEC> for GicdItargetsr32 {}
    impl ::core::default::Default for GicdItargetsr32 {
        #[inline(always)]
        fn default() -> GicdItargetsr32 {
            <crate::RegValueT<GicdItargetsr32_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr33_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr33_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 132 - 135"]
    pub type GicdItargetsr33 = crate::RegValueT<GicdItargetsr33_SPEC>;

    impl NoBitfieldReg<GicdItargetsr33_SPEC> for GicdItargetsr33 {}
    impl ::core::default::Default for GicdItargetsr33 {
        #[inline(always)]
        fn default() -> GicdItargetsr33 {
            <crate::RegValueT<GicdItargetsr33_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr34_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr34_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 136 - 139"]
    pub type GicdItargetsr34 = crate::RegValueT<GicdItargetsr34_SPEC>;

    impl NoBitfieldReg<GicdItargetsr34_SPEC> for GicdItargetsr34 {}
    impl ::core::default::Default for GicdItargetsr34 {
        #[inline(always)]
        fn default() -> GicdItargetsr34 {
            <crate::RegValueT<GicdItargetsr34_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr35_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr35_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 140 - 143"]
    pub type GicdItargetsr35 = crate::RegValueT<GicdItargetsr35_SPEC>;

    impl NoBitfieldReg<GicdItargetsr35_SPEC> for GicdItargetsr35 {}
    impl ::core::default::Default for GicdItargetsr35 {
        #[inline(always)]
        fn default() -> GicdItargetsr35 {
            <crate::RegValueT<GicdItargetsr35_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr36_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr36_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 144 - 147"]
    pub type GicdItargetsr36 = crate::RegValueT<GicdItargetsr36_SPEC>;

    impl NoBitfieldReg<GicdItargetsr36_SPEC> for GicdItargetsr36 {}
    impl ::core::default::Default for GicdItargetsr36 {
        #[inline(always)]
        fn default() -> GicdItargetsr36 {
            <crate::RegValueT<GicdItargetsr36_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr37_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr37_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 148 - 151"]
    pub type GicdItargetsr37 = crate::RegValueT<GicdItargetsr37_SPEC>;

    impl NoBitfieldReg<GicdItargetsr37_SPEC> for GicdItargetsr37 {}
    impl ::core::default::Default for GicdItargetsr37 {
        #[inline(always)]
        fn default() -> GicdItargetsr37 {
            <crate::RegValueT<GicdItargetsr37_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr38_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr38_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 152 - 155"]
    pub type GicdItargetsr38 = crate::RegValueT<GicdItargetsr38_SPEC>;

    impl NoBitfieldReg<GicdItargetsr38_SPEC> for GicdItargetsr38 {}
    impl ::core::default::Default for GicdItargetsr38 {
        #[inline(always)]
        fn default() -> GicdItargetsr38 {
            <crate::RegValueT<GicdItargetsr38_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr39_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr39_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 156 - 159"]
    pub type GicdItargetsr39 = crate::RegValueT<GicdItargetsr39_SPEC>;

    impl NoBitfieldReg<GicdItargetsr39_SPEC> for GicdItargetsr39 {}
    impl ::core::default::Default for GicdItargetsr39 {
        #[inline(always)]
        fn default() -> GicdItargetsr39 {
            <crate::RegValueT<GicdItargetsr39_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr40_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr40_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 160 - 163"]
    pub type GicdItargetsr40 = crate::RegValueT<GicdItargetsr40_SPEC>;

    impl NoBitfieldReg<GicdItargetsr40_SPEC> for GicdItargetsr40 {}
    impl ::core::default::Default for GicdItargetsr40 {
        #[inline(always)]
        fn default() -> GicdItargetsr40 {
            <crate::RegValueT<GicdItargetsr40_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr41_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr41_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 164 - 167"]
    pub type GicdItargetsr41 = crate::RegValueT<GicdItargetsr41_SPEC>;

    impl NoBitfieldReg<GicdItargetsr41_SPEC> for GicdItargetsr41 {}
    impl ::core::default::Default for GicdItargetsr41 {
        #[inline(always)]
        fn default() -> GicdItargetsr41 {
            <crate::RegValueT<GicdItargetsr41_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr42_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr42_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 168 - 171"]
    pub type GicdItargetsr42 = crate::RegValueT<GicdItargetsr42_SPEC>;

    impl NoBitfieldReg<GicdItargetsr42_SPEC> for GicdItargetsr42 {}
    impl ::core::default::Default for GicdItargetsr42 {
        #[inline(always)]
        fn default() -> GicdItargetsr42 {
            <crate::RegValueT<GicdItargetsr42_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr43_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr43_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 172 - 175"]
    pub type GicdItargetsr43 = crate::RegValueT<GicdItargetsr43_SPEC>;

    impl NoBitfieldReg<GicdItargetsr43_SPEC> for GicdItargetsr43 {}
    impl ::core::default::Default for GicdItargetsr43 {
        #[inline(always)]
        fn default() -> GicdItargetsr43 {
            <crate::RegValueT<GicdItargetsr43_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr44_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr44_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 176 - 179"]
    pub type GicdItargetsr44 = crate::RegValueT<GicdItargetsr44_SPEC>;

    impl NoBitfieldReg<GicdItargetsr44_SPEC> for GicdItargetsr44 {}
    impl ::core::default::Default for GicdItargetsr44 {
        #[inline(always)]
        fn default() -> GicdItargetsr44 {
            <crate::RegValueT<GicdItargetsr44_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr45_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr45_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 180 - 183"]
    pub type GicdItargetsr45 = crate::RegValueT<GicdItargetsr45_SPEC>;

    impl NoBitfieldReg<GicdItargetsr45_SPEC> for GicdItargetsr45 {}
    impl ::core::default::Default for GicdItargetsr45 {
        #[inline(always)]
        fn default() -> GicdItargetsr45 {
            <crate::RegValueT<GicdItargetsr45_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr46_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr46_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 184 - 187"]
    pub type GicdItargetsr46 = crate::RegValueT<GicdItargetsr46_SPEC>;

    impl NoBitfieldReg<GicdItargetsr46_SPEC> for GicdItargetsr46 {}
    impl ::core::default::Default for GicdItargetsr46 {
        #[inline(always)]
        fn default() -> GicdItargetsr46 {
            <crate::RegValueT<GicdItargetsr46_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr47_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr47_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 188 - 191"]
    pub type GicdItargetsr47 = crate::RegValueT<GicdItargetsr47_SPEC>;

    impl NoBitfieldReg<GicdItargetsr47_SPEC> for GicdItargetsr47 {}
    impl ::core::default::Default for GicdItargetsr47 {
        #[inline(always)]
        fn default() -> GicdItargetsr47 {
            <crate::RegValueT<GicdItargetsr47_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr48_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr48_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 192 - 195"]
    pub type GicdItargetsr48 = crate::RegValueT<GicdItargetsr48_SPEC>;

    impl NoBitfieldReg<GicdItargetsr48_SPEC> for GicdItargetsr48 {}
    impl ::core::default::Default for GicdItargetsr48 {
        #[inline(always)]
        fn default() -> GicdItargetsr48 {
            <crate::RegValueT<GicdItargetsr48_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr49_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr49_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 196 - 199"]
    pub type GicdItargetsr49 = crate::RegValueT<GicdItargetsr49_SPEC>;

    impl NoBitfieldReg<GicdItargetsr49_SPEC> for GicdItargetsr49 {}
    impl ::core::default::Default for GicdItargetsr49 {
        #[inline(always)]
        fn default() -> GicdItargetsr49 {
            <crate::RegValueT<GicdItargetsr49_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr50_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr50_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 200 - 203"]
    pub type GicdItargetsr50 = crate::RegValueT<GicdItargetsr50_SPEC>;

    impl NoBitfieldReg<GicdItargetsr50_SPEC> for GicdItargetsr50 {}
    impl ::core::default::Default for GicdItargetsr50 {
        #[inline(always)]
        fn default() -> GicdItargetsr50 {
            <crate::RegValueT<GicdItargetsr50_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr51_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr51_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 204 - 207"]
    pub type GicdItargetsr51 = crate::RegValueT<GicdItargetsr51_SPEC>;

    impl NoBitfieldReg<GicdItargetsr51_SPEC> for GicdItargetsr51 {}
    impl ::core::default::Default for GicdItargetsr51 {
        #[inline(always)]
        fn default() -> GicdItargetsr51 {
            <crate::RegValueT<GicdItargetsr51_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr52_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr52_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 208 - 211"]
    pub type GicdItargetsr52 = crate::RegValueT<GicdItargetsr52_SPEC>;

    impl NoBitfieldReg<GicdItargetsr52_SPEC> for GicdItargetsr52 {}
    impl ::core::default::Default for GicdItargetsr52 {
        #[inline(always)]
        fn default() -> GicdItargetsr52 {
            <crate::RegValueT<GicdItargetsr52_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr53_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr53_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 212 - 215"]
    pub type GicdItargetsr53 = crate::RegValueT<GicdItargetsr53_SPEC>;

    impl NoBitfieldReg<GicdItargetsr53_SPEC> for GicdItargetsr53 {}
    impl ::core::default::Default for GicdItargetsr53 {
        #[inline(always)]
        fn default() -> GicdItargetsr53 {
            <crate::RegValueT<GicdItargetsr53_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr54_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr54_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 216 - 219"]
    pub type GicdItargetsr54 = crate::RegValueT<GicdItargetsr54_SPEC>;

    impl NoBitfieldReg<GicdItargetsr54_SPEC> for GicdItargetsr54 {}
    impl ::core::default::Default for GicdItargetsr54 {
        #[inline(always)]
        fn default() -> GicdItargetsr54 {
            <crate::RegValueT<GicdItargetsr54_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdItargetsr55_SPEC;
    impl crate::sealed::RegSpec for GicdItargetsr55_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Processor Target 220 - 223"]
    pub type GicdItargetsr55 = crate::RegValueT<GicdItargetsr55_SPEC>;

    impl NoBitfieldReg<GicdItargetsr55_SPEC> for GicdItargetsr55 {}
    impl ::core::default::Default for GicdItargetsr55 {
        #[inline(always)]
        fn default() -> GicdItargetsr55 {
            <crate::RegValueT<GicdItargetsr55_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "Interrupt Configuration"]
#[non_exhaustive]
pub struct _GicdIcfgr;

#[doc = "Interrupt Configuration"]
pub type GicdIcfgr = &'static _GicdIcfgr;

unsafe impl ::core::marker::Sync for _GicdIcfgr {}
impl _GicdIcfgr {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Interrupt Configuration 0 - 15"]
    #[inline(always)]
    pub const fn gicd_icfgr0(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 16 - 31"]
    #[inline(always)]
    pub const fn gicd_icfgr4(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 32 - 47"]
    #[inline(always)]
    pub const fn gicd_icfgr8(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 48 - 63"]
    #[inline(always)]
    pub const fn gicd_icfgr12(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 64 - 79"]
    #[inline(always)]
    pub const fn gicd_icfgr16(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr16_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr16_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 80 - 95"]
    #[inline(always)]
    pub const fn gicd_icfgr20(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr20_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr20_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 96 - 111"]
    #[inline(always)]
    pub const fn gicd_icfgr24(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr24_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr24_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 112 - 127"]
    #[inline(always)]
    pub const fn gicd_icfgr28(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr28_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr28_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 128 - 143"]
    #[inline(always)]
    pub const fn gicd_icfgr32(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr32_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr32_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 144 - 159"]
    #[inline(always)]
    pub const fn gicd_icfgr36(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr36_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr36_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 160 - 175"]
    #[inline(always)]
    pub const fn gicd_icfgr40(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr40_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr40_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 176 - 191"]
    #[inline(always)]
    pub const fn gicd_icfgr44(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr44_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr44_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 192 - 207"]
    #[inline(always)]
    pub const fn gicd_icfgr48(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr48_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr48_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Interrupt Configuration 208 - 223"]
    #[inline(always)]
    pub const fn gicd_icfgr52(
        &self,
    ) -> &'static crate::common::Reg<gicd_icfgr::GicdIcfgr52_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<gicd_icfgr::GicdIcfgr52_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }
}

unsafe impl AsPtr for _GicdIcfgr {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod gicd_icfgr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr0_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr0_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 0 - 15"]
    pub type GicdIcfgr0 = crate::RegValueT<GicdIcfgr0_SPEC>;

    impl NoBitfieldReg<GicdIcfgr0_SPEC> for GicdIcfgr0 {}
    impl ::core::default::Default for GicdIcfgr0 {
        #[inline(always)]
        fn default() -> GicdIcfgr0 {
            <crate::RegValueT<GicdIcfgr0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr4_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr4_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 16 - 31"]
    pub type GicdIcfgr4 = crate::RegValueT<GicdIcfgr4_SPEC>;

    impl NoBitfieldReg<GicdIcfgr4_SPEC> for GicdIcfgr4 {}
    impl ::core::default::Default for GicdIcfgr4 {
        #[inline(always)]
        fn default() -> GicdIcfgr4 {
            <crate::RegValueT<GicdIcfgr4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr8_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr8_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 32 - 47"]
    pub type GicdIcfgr8 = crate::RegValueT<GicdIcfgr8_SPEC>;

    impl NoBitfieldReg<GicdIcfgr8_SPEC> for GicdIcfgr8 {}
    impl ::core::default::Default for GicdIcfgr8 {
        #[inline(always)]
        fn default() -> GicdIcfgr8 {
            <crate::RegValueT<GicdIcfgr8_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr12_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr12_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 48 - 63"]
    pub type GicdIcfgr12 = crate::RegValueT<GicdIcfgr12_SPEC>;

    impl NoBitfieldReg<GicdIcfgr12_SPEC> for GicdIcfgr12 {}
    impl ::core::default::Default for GicdIcfgr12 {
        #[inline(always)]
        fn default() -> GicdIcfgr12 {
            <crate::RegValueT<GicdIcfgr12_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr16_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr16_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 64 - 79"]
    pub type GicdIcfgr16 = crate::RegValueT<GicdIcfgr16_SPEC>;

    impl NoBitfieldReg<GicdIcfgr16_SPEC> for GicdIcfgr16 {}
    impl ::core::default::Default for GicdIcfgr16 {
        #[inline(always)]
        fn default() -> GicdIcfgr16 {
            <crate::RegValueT<GicdIcfgr16_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr20_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr20_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 80 - 95"]
    pub type GicdIcfgr20 = crate::RegValueT<GicdIcfgr20_SPEC>;

    impl NoBitfieldReg<GicdIcfgr20_SPEC> for GicdIcfgr20 {}
    impl ::core::default::Default for GicdIcfgr20 {
        #[inline(always)]
        fn default() -> GicdIcfgr20 {
            <crate::RegValueT<GicdIcfgr20_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr24_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr24_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 96 - 111"]
    pub type GicdIcfgr24 = crate::RegValueT<GicdIcfgr24_SPEC>;

    impl NoBitfieldReg<GicdIcfgr24_SPEC> for GicdIcfgr24 {}
    impl ::core::default::Default for GicdIcfgr24 {
        #[inline(always)]
        fn default() -> GicdIcfgr24 {
            <crate::RegValueT<GicdIcfgr24_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr28_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr28_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 112 - 127"]
    pub type GicdIcfgr28 = crate::RegValueT<GicdIcfgr28_SPEC>;

    impl NoBitfieldReg<GicdIcfgr28_SPEC> for GicdIcfgr28 {}
    impl ::core::default::Default for GicdIcfgr28 {
        #[inline(always)]
        fn default() -> GicdIcfgr28 {
            <crate::RegValueT<GicdIcfgr28_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr32_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr32_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 128 - 143"]
    pub type GicdIcfgr32 = crate::RegValueT<GicdIcfgr32_SPEC>;

    impl NoBitfieldReg<GicdIcfgr32_SPEC> for GicdIcfgr32 {}
    impl ::core::default::Default for GicdIcfgr32 {
        #[inline(always)]
        fn default() -> GicdIcfgr32 {
            <crate::RegValueT<GicdIcfgr32_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr36_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr36_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 144 - 159"]
    pub type GicdIcfgr36 = crate::RegValueT<GicdIcfgr36_SPEC>;

    impl NoBitfieldReg<GicdIcfgr36_SPEC> for GicdIcfgr36 {}
    impl ::core::default::Default for GicdIcfgr36 {
        #[inline(always)]
        fn default() -> GicdIcfgr36 {
            <crate::RegValueT<GicdIcfgr36_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr40_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr40_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 160 - 175"]
    pub type GicdIcfgr40 = crate::RegValueT<GicdIcfgr40_SPEC>;

    impl NoBitfieldReg<GicdIcfgr40_SPEC> for GicdIcfgr40 {}
    impl ::core::default::Default for GicdIcfgr40 {
        #[inline(always)]
        fn default() -> GicdIcfgr40 {
            <crate::RegValueT<GicdIcfgr40_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr44_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr44_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 176 - 191"]
    pub type GicdIcfgr44 = crate::RegValueT<GicdIcfgr44_SPEC>;

    impl NoBitfieldReg<GicdIcfgr44_SPEC> for GicdIcfgr44 {}
    impl ::core::default::Default for GicdIcfgr44 {
        #[inline(always)]
        fn default() -> GicdIcfgr44 {
            <crate::RegValueT<GicdIcfgr44_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr48_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr48_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 192 - 207"]
    pub type GicdIcfgr48 = crate::RegValueT<GicdIcfgr48_SPEC>;

    impl NoBitfieldReg<GicdIcfgr48_SPEC> for GicdIcfgr48 {}
    impl ::core::default::Default for GicdIcfgr48 {
        #[inline(always)]
        fn default() -> GicdIcfgr48 {
            <crate::RegValueT<GicdIcfgr48_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GicdIcfgr52_SPEC;
    impl crate::sealed::RegSpec for GicdIcfgr52_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt Configuration 208 - 223"]
    pub type GicdIcfgr52 = crate::RegValueT<GicdIcfgr52_SPEC>;

    impl NoBitfieldReg<GicdIcfgr52_SPEC> for GicdIcfgr52 {}
    impl ::core::default::Default for GicdIcfgr52 {
        #[inline(always)]
        fn default() -> GicdIcfgr52 {
            <crate::RegValueT<GicdIcfgr52_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
