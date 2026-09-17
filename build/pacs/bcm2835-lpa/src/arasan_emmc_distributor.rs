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
#[doc = r"Arasan SD3.0 Host AHB eMMC 4.4"]
unsafe impl ::core::marker::Send for super::ArasanEmmcDistributor {}
unsafe impl ::core::marker::Sync for super::ArasanEmmcDistributor {}
impl super::ArasanEmmcDistributor {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Argument for ACMD23 command"]
    #[inline(always)]
    pub const fn arg2(&self) -> &'static crate::common::Reg<self::Arg2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Arg2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Numer and size in bytes for data block to be transferred"]
    #[inline(always)]
    pub const fn blksizecnt(
        &self,
    ) -> &'static crate::common::Reg<self::Blksizecnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Blksizecnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Argument for everything but ACMD23"]
    #[inline(always)]
    pub const fn arg1(&self) -> &'static crate::common::Reg<self::Arg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Arg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Issue commands to the card"]
    #[inline(always)]
    pub const fn cmdtm(&self) -> &'static crate::common::Reg<self::Cmdtm_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmdtm_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Status bits of the response"]
    #[inline(always)]
    pub const fn resp0(&self) -> &'static crate::common::Reg<self::Resp0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Resp0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Bits 63:32 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp1(&self) -> &'static crate::common::Reg<self::Resp1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Resp1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Bits 95:64 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp2(&self) -> &'static crate::common::Reg<self::Resp2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Resp2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Bits 127:96 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub const fn resp3(&self) -> &'static crate::common::Reg<self::Resp3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Resp3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Data to/from the card"]
    #[inline(always)]
    pub const fn data(&self) -> &'static crate::common::Reg<self::Data_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Data_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Status info for debugging"]
    #[inline(always)]
    pub const fn status(
        &self,
    ) -> &'static crate::common::Reg<self::Status_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Status_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Control"]
    #[inline(always)]
    pub const fn control0(
        &self,
    ) -> &'static crate::common::Reg<self::Control0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Control0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Configure"]
    #[inline(always)]
    pub const fn control1(
        &self,
    ) -> &'static crate::common::Reg<self::Control1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Control1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Interrupt flags"]
    #[inline(always)]
    pub const fn interrupt(
        &self,
    ) -> &'static crate::common::Reg<self::Interrupt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Interrupt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Mask interrupts that change in INTERRUPT"]
    #[inline(always)]
    pub const fn irpt_mask(
        &self,
    ) -> &'static crate::common::Reg<self::IrptMask_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrptMask_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Enable interrupt to core"]
    #[inline(always)]
    pub const fn irpt_en(
        &self,
    ) -> &'static crate::common::Reg<self::IrptEn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IrptEn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Control 2"]
    #[inline(always)]
    pub const fn control2(
        &self,
    ) -> &'static crate::common::Reg<self::Control2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Control2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Force an interrupt"]
    #[inline(always)]
    pub const fn force_irpt(
        &self,
    ) -> &'static crate::common::Reg<self::ForceIrpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ForceIrpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Number of SD clock cycles to wait for boot"]
    #[inline(always)]
    pub const fn boot_timeout(
        &self,
    ) -> &'static crate::common::Reg<self::BootTimeout_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BootTimeout_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "What submodules are accessed by the debug bus"]
    #[inline(always)]
    pub const fn dbg_sel(
        &self,
    ) -> &'static crate::common::Reg<self::DbgSel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DbgSel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Fine tune DMA request generation"]
    #[inline(always)]
    pub const fn exrdfifo_cfg(
        &self,
    ) -> &'static crate::common::Reg<self::ExrdfifoCfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ExrdfifoCfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Enable the extension data register"]
    #[inline(always)]
    pub const fn exrdfifo_en(
        &self,
    ) -> &'static crate::common::Reg<self::ExrdfifoEn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ExrdfifoEn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Sample clock delay step duration"]
    #[inline(always)]
    pub const fn tune_step(
        &self,
    ) -> &'static crate::common::Reg<self::TuneStep_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TuneStep_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Sample clock delay step count for SDR"]
    #[inline(always)]
    pub const fn tune_steps_std(
        &self,
    ) -> &'static crate::common::Reg<self::TuneStepsStd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TuneStepsStd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Sample clock delay step count for DDR"]
    #[inline(always)]
    pub const fn tune_steps_ddr(
        &self,
    ) -> &'static crate::common::Reg<self::TuneStepsDdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TuneStepsDdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Interrupts in SPI mode depend on CS"]
    #[inline(always)]
    pub const fn spi_int_spt(
        &self,
    ) -> &'static crate::common::Reg<self::SpiIntSpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiIntSpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[doc = "Version information and slot interrupt status"]
    #[inline(always)]
    pub const fn slotisr_ver(
        &self,
    ) -> &'static crate::common::Reg<self::SlotisrVer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SlotisrVer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arg2_SPEC;
impl crate::sealed::RegSpec for Arg2_SPEC {
    type DataType = u32;
}

#[doc = "Argument for ACMD23 command"]
pub type Arg2 = crate::RegValueT<Arg2_SPEC>;

impl NoBitfieldReg<Arg2_SPEC> for Arg2 {}
impl ::core::default::Default for Arg2 {
    #[inline(always)]
    fn default() -> Arg2 {
        <crate::RegValueT<Arg2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blksizecnt_SPEC;
impl crate::sealed::RegSpec for Blksizecnt_SPEC {
    type DataType = u32;
}

#[doc = "Numer and size in bytes for data block to be transferred"]
pub type Blksizecnt = crate::RegValueT<Blksizecnt_SPEC>;

impl NoBitfieldReg<Blksizecnt_SPEC> for Blksizecnt {}
impl ::core::default::Default for Blksizecnt {
    #[inline(always)]
    fn default() -> Blksizecnt {
        <crate::RegValueT<Blksizecnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arg1_SPEC;
impl crate::sealed::RegSpec for Arg1_SPEC {
    type DataType = u32;
}

#[doc = "Argument for everything but ACMD23"]
pub type Arg1 = crate::RegValueT<Arg1_SPEC>;

impl NoBitfieldReg<Arg1_SPEC> for Arg1 {}
impl ::core::default::Default for Arg1 {
    #[inline(always)]
    fn default() -> Arg1 {
        <crate::RegValueT<Arg1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdtm_SPEC;
impl crate::sealed::RegSpec for Cmdtm_SPEC {
    type DataType = u32;
}

#[doc = "Issue commands to the card"]
pub type Cmdtm = crate::RegValueT<Cmdtm_SPEC>;

impl NoBitfieldReg<Cmdtm_SPEC> for Cmdtm {}
impl ::core::default::Default for Cmdtm {
    #[inline(always)]
    fn default() -> Cmdtm {
        <crate::RegValueT<Cmdtm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp0_SPEC;
impl crate::sealed::RegSpec for Resp0_SPEC {
    type DataType = u32;
}

#[doc = "Status bits of the response"]
pub type Resp0 = crate::RegValueT<Resp0_SPEC>;

impl NoBitfieldReg<Resp0_SPEC> for Resp0 {}
impl ::core::default::Default for Resp0 {
    #[inline(always)]
    fn default() -> Resp0 {
        <crate::RegValueT<Resp0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp1_SPEC;
impl crate::sealed::RegSpec for Resp1_SPEC {
    type DataType = u32;
}

#[doc = "Bits 63:32 of CMD2 and CMD10 responses"]
pub type Resp1 = crate::RegValueT<Resp1_SPEC>;

impl NoBitfieldReg<Resp1_SPEC> for Resp1 {}
impl ::core::default::Default for Resp1 {
    #[inline(always)]
    fn default() -> Resp1 {
        <crate::RegValueT<Resp1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp2_SPEC;
impl crate::sealed::RegSpec for Resp2_SPEC {
    type DataType = u32;
}

#[doc = "Bits 95:64 of CMD2 and CMD10 responses"]
pub type Resp2 = crate::RegValueT<Resp2_SPEC>;

impl NoBitfieldReg<Resp2_SPEC> for Resp2 {}
impl ::core::default::Default for Resp2 {
    #[inline(always)]
    fn default() -> Resp2 {
        <crate::RegValueT<Resp2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp3_SPEC;
impl crate::sealed::RegSpec for Resp3_SPEC {
    type DataType = u32;
}

#[doc = "Bits 127:96 of CMD2 and CMD10 responses"]
pub type Resp3 = crate::RegValueT<Resp3_SPEC>;

impl NoBitfieldReg<Resp3_SPEC> for Resp3 {}
impl ::core::default::Default for Resp3 {
    #[inline(always)]
    fn default() -> Resp3 {
        <crate::RegValueT<Resp3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data_SPEC;
impl crate::sealed::RegSpec for Data_SPEC {
    type DataType = u32;
}

#[doc = "Data to/from the card"]
pub type Data = crate::RegValueT<Data_SPEC>;

impl NoBitfieldReg<Data_SPEC> for Data {}
impl ::core::default::Default for Data {
    #[inline(always)]
    fn default() -> Data {
        <crate::RegValueT<Data_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status_SPEC;
impl crate::sealed::RegSpec for Status_SPEC {
    type DataType = u32;
}

#[doc = "Status info for debugging"]
pub type Status = crate::RegValueT<Status_SPEC>;

impl NoBitfieldReg<Status_SPEC> for Status {}
impl ::core::default::Default for Status {
    #[inline(always)]
    fn default() -> Status {
        <crate::RegValueT<Status_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control0_SPEC;
impl crate::sealed::RegSpec for Control0_SPEC {
    type DataType = u32;
}

#[doc = "Control"]
pub type Control0 = crate::RegValueT<Control0_SPEC>;

impl NoBitfieldReg<Control0_SPEC> for Control0 {}
impl ::core::default::Default for Control0 {
    #[inline(always)]
    fn default() -> Control0 {
        <crate::RegValueT<Control0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control1_SPEC;
impl crate::sealed::RegSpec for Control1_SPEC {
    type DataType = u32;
}

#[doc = "Configure"]
pub type Control1 = crate::RegValueT<Control1_SPEC>;

impl Control1 {
    #[doc = "SD Clock stable"]
    #[inline(always)]
    pub fn clk_stable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Control1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Control1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Control1 {
    #[inline(always)]
    fn default() -> Control1 {
        <crate::RegValueT<Control1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interrupt_SPEC;
impl crate::sealed::RegSpec for Interrupt_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt flags"]
pub type Interrupt = crate::RegValueT<Interrupt_SPEC>;

impl Interrupt {
    #[doc = "An error has occured"]
    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Interrupt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Interrupt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Interrupt {
    #[inline(always)]
    fn default() -> Interrupt {
        <crate::RegValueT<Interrupt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrptMask_SPEC;
impl crate::sealed::RegSpec for IrptMask_SPEC {
    type DataType = u32;
}

#[doc = "Mask interrupts that change in INTERRUPT"]
pub type IrptMask = crate::RegValueT<IrptMask_SPEC>;

impl NoBitfieldReg<IrptMask_SPEC> for IrptMask {}
impl ::core::default::Default for IrptMask {
    #[inline(always)]
    fn default() -> IrptMask {
        <crate::RegValueT<IrptMask_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrptEn_SPEC;
impl crate::sealed::RegSpec for IrptEn_SPEC {
    type DataType = u32;
}

#[doc = "Enable interrupt to core"]
pub type IrptEn = crate::RegValueT<IrptEn_SPEC>;

impl NoBitfieldReg<IrptEn_SPEC> for IrptEn {}
impl ::core::default::Default for IrptEn {
    #[inline(always)]
    fn default() -> IrptEn {
        <crate::RegValueT<IrptEn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control2_SPEC;
impl crate::sealed::RegSpec for Control2_SPEC {
    type DataType = u32;
}

#[doc = "Control 2"]
pub type Control2 = crate::RegValueT<Control2_SPEC>;

impl Control2 {
    #[doc = "Error during auto CMD12"]
    #[inline(always)]
    pub fn notc12_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Control2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Control2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Command index error during auto command"]
    #[inline(always)]
    pub fn acbad_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Control2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Control2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "End bit is not 1 during auto command"]
    #[inline(always)]
    pub fn acend_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Control2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Control2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Command CRC error during auto command"]
    #[inline(always)]
    pub fn accrc_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Control2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Control2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Auto command timeout"]
    #[inline(always)]
    pub fn acto_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Control2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Control2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Auto command not executed due to an error"]
    #[inline(always)]
    pub fn acnox_err(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Control2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Control2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Control2 {
    #[inline(always)]
    fn default() -> Control2 {
        <crate::RegValueT<Control2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceIrpt_SPEC;
impl crate::sealed::RegSpec for ForceIrpt_SPEC {
    type DataType = u32;
}

#[doc = "Force an interrupt"]
pub type ForceIrpt = crate::RegValueT<ForceIrpt_SPEC>;

impl NoBitfieldReg<ForceIrpt_SPEC> for ForceIrpt {}
impl ::core::default::Default for ForceIrpt {
    #[inline(always)]
    fn default() -> ForceIrpt {
        <crate::RegValueT<ForceIrpt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootTimeout_SPEC;
impl crate::sealed::RegSpec for BootTimeout_SPEC {
    type DataType = u32;
}

#[doc = "Number of SD clock cycles to wait for boot"]
pub type BootTimeout = crate::RegValueT<BootTimeout_SPEC>;

impl NoBitfieldReg<BootTimeout_SPEC> for BootTimeout {}
impl ::core::default::Default for BootTimeout {
    #[inline(always)]
    fn default() -> BootTimeout {
        <crate::RegValueT<BootTimeout_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgSel_SPEC;
impl crate::sealed::RegSpec for DbgSel_SPEC {
    type DataType = u32;
}

#[doc = "What submodules are accessed by the debug bus"]
pub type DbgSel = crate::RegValueT<DbgSel_SPEC>;

impl NoBitfieldReg<DbgSel_SPEC> for DbgSel {}
impl ::core::default::Default for DbgSel {
    #[inline(always)]
    fn default() -> DbgSel {
        <crate::RegValueT<DbgSel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExrdfifoCfg_SPEC;
impl crate::sealed::RegSpec for ExrdfifoCfg_SPEC {
    type DataType = u32;
}

#[doc = "Fine tune DMA request generation"]
pub type ExrdfifoCfg = crate::RegValueT<ExrdfifoCfg_SPEC>;

impl NoBitfieldReg<ExrdfifoCfg_SPEC> for ExrdfifoCfg {}
impl ::core::default::Default for ExrdfifoCfg {
    #[inline(always)]
    fn default() -> ExrdfifoCfg {
        <crate::RegValueT<ExrdfifoCfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExrdfifoEn_SPEC;
impl crate::sealed::RegSpec for ExrdfifoEn_SPEC {
    type DataType = u32;
}

#[doc = "Enable the extension data register"]
pub type ExrdfifoEn = crate::RegValueT<ExrdfifoEn_SPEC>;

impl NoBitfieldReg<ExrdfifoEn_SPEC> for ExrdfifoEn {}
impl ::core::default::Default for ExrdfifoEn {
    #[inline(always)]
    fn default() -> ExrdfifoEn {
        <crate::RegValueT<ExrdfifoEn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuneStep_SPEC;
impl crate::sealed::RegSpec for TuneStep_SPEC {
    type DataType = u32;
}

#[doc = "Sample clock delay step duration"]
pub type TuneStep = crate::RegValueT<TuneStep_SPEC>;

impl NoBitfieldReg<TuneStep_SPEC> for TuneStep {}
impl ::core::default::Default for TuneStep {
    #[inline(always)]
    fn default() -> TuneStep {
        <crate::RegValueT<TuneStep_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuneStepsStd_SPEC;
impl crate::sealed::RegSpec for TuneStepsStd_SPEC {
    type DataType = u32;
}

#[doc = "Sample clock delay step count for SDR"]
pub type TuneStepsStd = crate::RegValueT<TuneStepsStd_SPEC>;

impl NoBitfieldReg<TuneStepsStd_SPEC> for TuneStepsStd {}
impl ::core::default::Default for TuneStepsStd {
    #[inline(always)]
    fn default() -> TuneStepsStd {
        <crate::RegValueT<TuneStepsStd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuneStepsDdr_SPEC;
impl crate::sealed::RegSpec for TuneStepsDdr_SPEC {
    type DataType = u32;
}

#[doc = "Sample clock delay step count for DDR"]
pub type TuneStepsDdr = crate::RegValueT<TuneStepsDdr_SPEC>;

impl NoBitfieldReg<TuneStepsDdr_SPEC> for TuneStepsDdr {}
impl ::core::default::Default for TuneStepsDdr {
    #[inline(always)]
    fn default() -> TuneStepsDdr {
        <crate::RegValueT<TuneStepsDdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiIntSpt_SPEC;
impl crate::sealed::RegSpec for SpiIntSpt_SPEC {
    type DataType = u32;
}

#[doc = "Interrupts in SPI mode depend on CS"]
pub type SpiIntSpt = crate::RegValueT<SpiIntSpt_SPEC>;

impl NoBitfieldReg<SpiIntSpt_SPEC> for SpiIntSpt {}
impl ::core::default::Default for SpiIntSpt {
    #[inline(always)]
    fn default() -> SpiIntSpt {
        <crate::RegValueT<SpiIntSpt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlotisrVer_SPEC;
impl crate::sealed::RegSpec for SlotisrVer_SPEC {
    type DataType = u32;
}

#[doc = "Version information and slot interrupt status"]
pub type SlotisrVer = crate::RegValueT<SlotisrVer_SPEC>;

impl NoBitfieldReg<SlotisrVer_SPEC> for SlotisrVer {}
impl ::core::default::Default for SlotisrVer {
    #[inline(always)]
    fn default() -> SlotisrVer {
        <crate::RegValueT<SlotisrVer_SPEC> as RegisterValue<_>>::new(0)
    }
}
