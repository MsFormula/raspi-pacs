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
#[doc = r"USB on the go high speed"]
unsafe impl ::core::marker::Send for super::UsbOtgGlobal {}
unsafe impl ::core::marker::Sync for super::UsbOtgGlobal {}
impl super::UsbOtgGlobal {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "OTG_HS control and status\n          register"]
    #[inline(always)]
    pub const fn gotgctl(
        &self,
    ) -> &'static crate::common::Reg<self::Gotgctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gotgctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "OTG_HS interrupt register"]
    #[inline(always)]
    pub const fn gotgint(
        &self,
    ) -> &'static crate::common::Reg<self::Gotgint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gotgint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "OTG_HS AHB configuration\n          register"]
    #[inline(always)]
    pub const fn gahbcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Gahbcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gahbcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "OTG_HS USB configuration\n          register"]
    #[inline(always)]
    pub const fn gusbcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Gusbcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gusbcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "OTG_HS reset register"]
    #[inline(always)]
    pub const fn grstctl(
        &self,
    ) -> &'static crate::common::Reg<self::Grstctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Grstctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "OTG_HS core interrupt register"]
    #[inline(always)]
    pub const fn gintsts(
        &self,
    ) -> &'static crate::common::Reg<self::Gintsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gintsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "OTG_HS interrupt mask register"]
    #[inline(always)]
    pub const fn gintmsk(
        &self,
    ) -> &'static crate::common::Reg<self::Gintmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gintmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "OTG_HS Receive status debug read register\n          (host mode)"]
    #[inline(always)]
    pub const fn grxstsr_host(
        &self,
    ) -> &'static crate::common::Reg<self::GrxstsrHost_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrxstsrHost_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "OTG_HS status read and pop register (host\n          mode)"]
    #[inline(always)]
    pub const fn grxstsp_host(
        &self,
    ) -> &'static crate::common::Reg<self::GrxstspHost_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrxstspHost_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "OTG_HS Receive FIFO size\n          register"]
    #[inline(always)]
    pub const fn grxfsiz(
        &self,
    ) -> &'static crate::common::Reg<self::Grxfsiz_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Grxfsiz_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "OTG_HS nonperiodic transmit FIFO size\n          register (host mode)"]
    #[inline(always)]
    pub const fn gnptxfsiz_host(
        &self,
    ) -> &'static crate::common::Reg<self::GnptxfsizHost_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GnptxfsizHost_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Endpoint 0 transmit FIFO size (peripheral\n          mode)"]
    #[inline(always)]
    pub const fn tx0fsiz_peripheral(
        &self,
    ) -> &'static crate::common::Reg<self::Tx0FsizPeripheral_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tx0FsizPeripheral_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "OTG_HS nonperiodic transmit FIFO/queue\n          status register"]
    #[inline(always)]
    pub const fn gnptxsts(
        &self,
    ) -> &'static crate::common::Reg<self::Gnptxsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gnptxsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "OTG_HS general core configuration\n          register"]
    #[inline(always)]
    pub const fn gccfg(&self) -> &'static crate::common::Reg<self::Gccfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gccfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "OTG_HS core ID register"]
    #[inline(always)]
    pub const fn cid(&self) -> &'static crate::common::Reg<self::Cid_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cid_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "OTG_HS vendor ID register"]
    #[inline(always)]
    pub const fn vid(&self) -> &'static crate::common::Reg<self::Vid_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vid_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Direction"]
    #[inline(always)]
    pub const fn hw_direction(
        &self,
    ) -> &'static crate::common::Reg<self::HwDirection_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::HwDirection_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Hardware Config 0"]
    #[inline(always)]
    pub const fn hw_config0(
        &self,
    ) -> &'static crate::common::Reg<self::HwConfig0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::HwConfig0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "OTG_HS Host periodic transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn hptxfsiz(
        &self,
    ) -> &'static crate::common::Reg<self::Hptxfsiz_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hptxfsiz_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn dieptxf1(
        &self,
    ) -> &'static crate::common::Reg<self::Dieptxf1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dieptxf1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn dieptxf2(
        &self,
    ) -> &'static crate::common::Reg<self::Dieptxf2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dieptxf2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn dieptxf3(
        &self,
    ) -> &'static crate::common::Reg<self::Dieptxf3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dieptxf3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn dieptxf4(
        &self,
    ) -> &'static crate::common::Reg<self::Dieptxf4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dieptxf4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn dieptxf5(
        &self,
    ) -> &'static crate::common::Reg<self::Dieptxf5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dieptxf5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn dieptxf6(
        &self,
    ) -> &'static crate::common::Reg<self::Dieptxf6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dieptxf6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub const fn dieptxf7(
        &self,
    ) -> &'static crate::common::Reg<self::Dieptxf7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dieptxf7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[doc = "OTG_HS Receive status debug read register\n          (peripheral mode mode)"]
    #[inline(always)]
    pub const fn grxstsr_peripheral(
        &self,
    ) -> &'static crate::common::Reg<self::GrxstsrPeripheral_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrxstsrPeripheral_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "OTG_HS status read and pop register\n          (peripheral mode)"]
    #[inline(always)]
    pub const fn grxstsp_peripheral(
        &self,
    ) -> &'static crate::common::Reg<self::GrxstspPeripheral_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::GrxstspPeripheral_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gotgctl_SPEC;
impl crate::sealed::RegSpec for Gotgctl_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS control and status\n          register"]
pub type Gotgctl = crate::RegValueT<Gotgctl_SPEC>;

impl Gotgctl {
    #[doc = "Session request success"]
    #[inline(always)]
    pub fn srqscs(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Gotgctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gotgctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Session request"]
    #[inline(always)]
    pub fn srq(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gotgctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gotgctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Gotgctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gotgctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "HNP request"]
    #[inline(always)]
    pub fn hnprq(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Gotgctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gotgctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gotgctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gotgctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Gotgctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gotgctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Connector ID status"]
    #[inline(always)]
    pub fn cidsts(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Gotgctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gotgctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gotgctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gotgctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "A-session valid"]
    #[inline(always)]
    pub fn asvld(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Gotgctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gotgctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "B-session valid"]
    #[inline(always)]
    pub fn bsvld(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Gotgctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gotgctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gotgctl {
    #[inline(always)]
    fn default() -> Gotgctl {
        <crate::RegValueT<Gotgctl_SPEC> as RegisterValue<_>>::new(2048)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gotgint_SPEC;
impl crate::sealed::RegSpec for Gotgint_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS interrupt register"]
pub type Gotgint = crate::RegValueT<Gotgint_SPEC>;

impl NoBitfieldReg<Gotgint_SPEC> for Gotgint {}
impl ::core::default::Default for Gotgint {
    #[inline(always)]
    fn default() -> Gotgint {
        <crate::RegValueT<Gotgint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gahbcfg_SPEC;
impl crate::sealed::RegSpec for Gahbcfg_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS AHB configuration\n          register"]
pub type Gahbcfg = crate::RegValueT<Gahbcfg_SPEC>;

impl NoBitfieldReg<Gahbcfg_SPEC> for Gahbcfg {}
impl ::core::default::Default for Gahbcfg {
    #[inline(always)]
    fn default() -> Gahbcfg {
        <crate::RegValueT<Gahbcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gusbcfg_SPEC;
impl crate::sealed::RegSpec for Gusbcfg_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS USB configuration\n          register"]
pub type Gusbcfg = crate::RegValueT<Gusbcfg_SPEC>;

impl Gusbcfg {
    #[doc = "FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Gusbcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Interface width"]
    #[inline(always)]
    pub fn phyif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gusbcfg::Phyif,
        gusbcfg::Phyif,
        Gusbcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gusbcfg::Phyif,
            gusbcfg::Phyif,
            Gusbcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PHY Type"]
    #[inline(always)]
    pub fn phytype(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gusbcfg::Phytype,
        gusbcfg::Phytype,
        Gusbcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gusbcfg::Phytype,
            gusbcfg::Phytype,
            Gusbcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Full speed interface"]
    #[inline(always)]
    pub fn fsif(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gusbcfg::Fsif,
        gusbcfg::Fsif,
        Gusbcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gusbcfg::Fsif,
            gusbcfg::Fsif,
            Gusbcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transceiver select"]
    #[inline(always)]
    pub fn physel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gusbcfg::Physel,
        gusbcfg::Physel,
        Gusbcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gusbcfg::Physel,
            gusbcfg::Physel,
            Gusbcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ULPI data rate"]
    #[inline(always)]
    pub fn ddrsel(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gusbcfg::Ddrsel,
        gusbcfg::Ddrsel,
        Gusbcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gusbcfg::Ddrsel,
            gusbcfg::Ddrsel,
            Gusbcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRP-capable"]
    #[inline(always)]
    pub fn srpcap(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB turnaround time"]
    #[inline(always)]
    pub fn trdt(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,Gusbcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Low-power clock select"]
    #[inline(always)]
    pub fn phylpcs(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPI FS/LS select"]
    #[inline(always)]
    pub fn ulpifsls(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPI Auto-resume"]
    #[inline(always)]
    pub fn ulpiar(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPI Clock SuspendM"]
    #[inline(always)]
    pub fn ulpicsm(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPI External VBUS Drive"]
    #[inline(always)]
    pub fn ulpievbusd(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPI external VBUS\n              indicator"]
    #[inline(always)]
    pub fn ulpievbusi(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TermSel DLine pulsing\n              selection"]
    #[inline(always)]
    pub fn tsdps(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicator complement"]
    #[inline(always)]
    pub fn pcci(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicator pass through"]
    #[inline(always)]
    pub fn ptci(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ULPI interface protect\n              disable"]
    #[inline(always)]
    pub fn ulpiipd(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Forced host mode"]
    #[inline(always)]
    pub fn fhmod(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Forced peripheral mode"]
    #[inline(always)]
    pub fn fdmod(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Gusbcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gusbcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gusbcfg {
    #[inline(always)]
    fn default() -> Gusbcfg {
        <crate::RegValueT<Gusbcfg_SPEC> as RegisterValue<_>>::new(2560)
    }
}
pub mod gusbcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phyif_SPEC;
    pub type Phyif = crate::EnumBitfieldStruct<u8, Phyif_SPEC>;
    impl Phyif {
        pub const _8_BIT: Self = Self::new(0);

        pub const _16_BIT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phytype_SPEC;
    pub type Phytype = crate::EnumBitfieldStruct<u8, Phytype_SPEC>;
    impl Phytype {
        pub const UTMI: Self = Self::new(0);

        pub const ULPI: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsif_SPEC;
    pub type Fsif = crate::EnumBitfieldStruct<u8, Fsif_SPEC>;
    impl Fsif {
        pub const _6_PIN: Self = Self::new(0);

        pub const _3_PIN: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Physel_SPEC;
    pub type Physel = crate::EnumBitfieldStruct<u8, Physel_SPEC>;
    impl Physel {
        pub const USB_20: Self = Self::new(0);

        pub const USB_11: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ddrsel_SPEC;
    pub type Ddrsel = crate::EnumBitfieldStruct<u8, Ddrsel_SPEC>;
    impl Ddrsel {
        pub const SINGLE: Self = Self::new(0);

        pub const DOUBLE: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grstctl_SPEC;
impl crate::sealed::RegSpec for Grstctl_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS reset register"]
pub type Grstctl = crate::RegValueT<Grstctl_SPEC>;

impl Grstctl {
    #[doc = "Core soft reset"]
    #[inline(always)]
    pub fn csrst(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Grstctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Grstctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "HCLK soft reset"]
    #[inline(always)]
    pub fn hsrst(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Grstctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Grstctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Host frame counter reset"]
    #[inline(always)]
    pub fn fcrst(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Grstctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Grstctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Grstctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Grstctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Grstctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Grstctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(
        self,
    ) -> crate::common::RegisterField<6, 0x1f, 1, 0, u8, u8, Grstctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1f,1,0,u8,u8,Grstctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA request signal"]
    #[inline(always)]
    pub fn dmareq(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Grstctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Grstctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AHB master idle"]
    #[inline(always)]
    pub fn ahbidl(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Grstctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Grstctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Grstctl {
    #[inline(always)]
    fn default() -> Grstctl {
        <crate::RegValueT<Grstctl_SPEC> as RegisterValue<_>>::new(536870912)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gintsts_SPEC;
impl crate::sealed::RegSpec for Gintsts_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS core interrupt register"]
pub type Gintsts = crate::RegValueT<Gintsts_SPEC>;

impl Gintsts {
    #[doc = "Current mode of operation"]
    #[inline(always)]
    pub fn cmod(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OTG interrupt"]
    #[inline(always)]
    pub fn otgint(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Start of frame"]
    #[inline(always)]
    pub fn sof(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RxFIFO nonempty"]
    #[inline(always)]
    pub fn rxflvl(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Nonperiodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfe(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Global IN nonperiodic NAK\n              effective"]
    #[inline(always)]
    pub fn ginakeff(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Global OUT NAK effective"]
    #[inline(always)]
    pub fn boutnakeff(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Early suspend"]
    #[inline(always)]
    pub fn esusp(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB suspend"]
    #[inline(always)]
    pub fn usbsusp(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB reset"]
    #[inline(always)]
    pub fn usbrst(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enumeration done"]
    #[inline(always)]
    pub fn enumdne(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Isochronous OUT packet dropped\n              interrupt"]
    #[inline(always)]
    pub fn isoodrp(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End of periodic frame\n              interrupt"]
    #[inline(always)]
    pub fn eopf(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN endpoint interrupt"]
    #[inline(always)]
    pub fn iepint(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oepint(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Incomplete isochronous IN\n              transfer"]
    #[inline(always)]
    pub fn iisoixfr(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Incomplete periodic\n              transfer"]
    #[inline(always)]
    pub fn pxfr_incompisoout(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Data fetch suspended"]
    #[inline(always)]
    pub fn datafsusp(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Host port interrupt"]
    #[inline(always)]
    pub fn hprtint(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Host channels interrupt"]
    #[inline(always)]
    pub fn hcint(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfe(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Gintsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gintsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disconnect detected\n              interrupt"]
    #[inline(always)]
    pub fn discint(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Session request/new session detected\n              interrupt"]
    #[inline(always)]
    pub fn srqint(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Resume/remote wakeup detected\n              interrupt"]
    #[inline(always)]
    pub fn wkuint(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Gintsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gintsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gintsts {
    #[inline(always)]
    fn default() -> Gintsts {
        <crate::RegValueT<Gintsts_SPEC> as RegisterValue<_>>::new(67108896)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gintmsk_SPEC;
impl crate::sealed::RegSpec for Gintmsk_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS interrupt mask register"]
pub type Gintmsk = crate::RegValueT<Gintmsk_SPEC>;

impl Gintmsk {
    #[doc = "Mode mismatch interrupt\n              mask"]
    #[inline(always)]
    pub fn mmism(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Start of frame mask"]
    #[inline(always)]
    pub fn sofm(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Receive FIFO nonempty mask"]
    #[inline(always)]
    pub fn rxflvlm(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Nonperiodic TxFIFO empty\n              mask"]
    #[inline(always)]
    pub fn nptxfem(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Global nonperiodic IN NAK effective\n              mask"]
    #[inline(always)]
    pub fn ginakeffm(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Global OUT NAK effective\n              mask"]
    #[inline(always)]
    pub fn gonakeffm(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB reset mask"]
    #[inline(always)]
    pub fn usbrst(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Isochronous OUT packet dropped interrupt\n              mask"]
    #[inline(always)]
    pub fn isoodrpm(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End of periodic frame interrupt\n              mask"]
    #[inline(always)]
    pub fn eopfm(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Endpoint mismatch interrupt\n              mask"]
    #[inline(always)]
    pub fn epmism(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN endpoints interrupt\n              mask"]
    #[inline(always)]
    pub fn iepint(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OUT endpoints interrupt\n              mask"]
    #[inline(always)]
    pub fn oepint(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Incomplete isochronous IN transfer\n              mask"]
    #[inline(always)]
    pub fn iisoixfrm(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Incomplete periodic transfer\n              mask"]
    #[inline(always)]
    pub fn pxfrm_iisooxfrm(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Data fetch suspended mask"]
    #[inline(always)]
    pub fn fsuspm(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Host port interrupt mask"]
    #[inline(always)]
    pub fn prtim(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Gintmsk_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gintmsk_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Host channels interrupt\n              mask"]
    #[inline(always)]
    pub fn hcim(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Connector ID status change\n              mask"]
    #[inline(always)]
    pub fn cidschgm(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disconnect detected interrupt\n              mask"]
    #[inline(always)]
    pub fn discint(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Session request/new session detected\n              interrupt mask"]
    #[inline(always)]
    pub fn srqim(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Resume/remote wakeup detected interrupt\n              mask"]
    #[inline(always)]
    pub fn wuim(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Gintmsk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gintmsk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gintmsk {
    #[inline(always)]
    fn default() -> Gintmsk {
        <crate::RegValueT<Gintmsk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstsrHost_SPEC;
impl crate::sealed::RegSpec for GrxstsrHost_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS Receive status debug read register\n          (host mode)"]
pub type GrxstsrHost = crate::RegValueT<GrxstsrHost_SPEC>;

impl NoBitfieldReg<GrxstsrHost_SPEC> for GrxstsrHost {}
impl ::core::default::Default for GrxstsrHost {
    #[inline(always)]
    fn default() -> GrxstsrHost {
        <crate::RegValueT<GrxstsrHost_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstspHost_SPEC;
impl crate::sealed::RegSpec for GrxstspHost_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS status read and pop register (host\n          mode)"]
pub type GrxstspHost = crate::RegValueT<GrxstspHost_SPEC>;

impl NoBitfieldReg<GrxstspHost_SPEC> for GrxstspHost {}
impl ::core::default::Default for GrxstspHost {
    #[inline(always)]
    fn default() -> GrxstspHost {
        <crate::RegValueT<GrxstspHost_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grxfsiz_SPEC;
impl crate::sealed::RegSpec for Grxfsiz_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS Receive FIFO size\n          register"]
pub type Grxfsiz = crate::RegValueT<Grxfsiz_SPEC>;

impl NoBitfieldReg<Grxfsiz_SPEC> for Grxfsiz {}
impl ::core::default::Default for Grxfsiz {
    #[inline(always)]
    fn default() -> Grxfsiz {
        <crate::RegValueT<Grxfsiz_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GnptxfsizHost_SPEC;
impl crate::sealed::RegSpec for GnptxfsizHost_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS nonperiodic transmit FIFO size\n          register (host mode)"]
pub type GnptxfsizHost = crate::RegValueT<GnptxfsizHost_SPEC>;

impl NoBitfieldReg<GnptxfsizHost_SPEC> for GnptxfsizHost {}
impl ::core::default::Default for GnptxfsizHost {
    #[inline(always)]
    fn default() -> GnptxfsizHost {
        <crate::RegValueT<GnptxfsizHost_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx0FsizPeripheral_SPEC;
impl crate::sealed::RegSpec for Tx0FsizPeripheral_SPEC {
    type DataType = u32;
}

#[doc = "Endpoint 0 transmit FIFO size (peripheral\n          mode)"]
pub type Tx0FsizPeripheral = crate::RegValueT<Tx0FsizPeripheral_SPEC>;

impl NoBitfieldReg<Tx0FsizPeripheral_SPEC> for Tx0FsizPeripheral {}
impl ::core::default::Default for Tx0FsizPeripheral {
    #[inline(always)]
    fn default() -> Tx0FsizPeripheral {
        <crate::RegValueT<Tx0FsizPeripheral_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gnptxsts_SPEC;
impl crate::sealed::RegSpec for Gnptxsts_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS nonperiodic transmit FIFO/queue\n          status register"]
pub type Gnptxsts = crate::RegValueT<Gnptxsts_SPEC>;

impl NoBitfieldReg<Gnptxsts_SPEC> for Gnptxsts {}
impl ::core::default::Default for Gnptxsts {
    #[inline(always)]
    fn default() -> Gnptxsts {
        <crate::RegValueT<Gnptxsts_SPEC> as RegisterValue<_>>::new(524800)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gccfg_SPEC;
impl crate::sealed::RegSpec for Gccfg_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS general core configuration\n          register"]
pub type Gccfg = crate::RegValueT<Gccfg_SPEC>;

impl NoBitfieldReg<Gccfg_SPEC> for Gccfg {}
impl ::core::default::Default for Gccfg {
    #[inline(always)]
    fn default() -> Gccfg {
        <crate::RegValueT<Gccfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cid_SPEC;
impl crate::sealed::RegSpec for Cid_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS core ID register"]
pub type Cid = crate::RegValueT<Cid_SPEC>;

impl NoBitfieldReg<Cid_SPEC> for Cid {}
impl ::core::default::Default for Cid {
    #[inline(always)]
    fn default() -> Cid {
        <crate::RegValueT<Cid_SPEC> as RegisterValue<_>>::new(4608)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid_SPEC;
impl crate::sealed::RegSpec for Vid_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS vendor ID register"]
pub type Vid = crate::RegValueT<Vid_SPEC>;

impl NoBitfieldReg<Vid_SPEC> for Vid {}
impl ::core::default::Default for Vid {
    #[inline(always)]
    fn default() -> Vid {
        <crate::RegValueT<Vid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwDirection_SPEC;
impl crate::sealed::RegSpec for HwDirection_SPEC {
    type DataType = u32;
}

#[doc = "Direction"]
pub type HwDirection = crate::RegValueT<HwDirection_SPEC>;

impl NoBitfieldReg<HwDirection_SPEC> for HwDirection {}
impl ::core::default::Default for HwDirection {
    #[inline(always)]
    fn default() -> HwDirection {
        <crate::RegValueT<HwDirection_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwConfig0_SPEC;
impl crate::sealed::RegSpec for HwConfig0_SPEC {
    type DataType = u32;
}

#[doc = "Hardware Config 0"]
pub type HwConfig0 = crate::RegValueT<HwConfig0_SPEC>;

impl NoBitfieldReg<HwConfig0_SPEC> for HwConfig0 {}
impl ::core::default::Default for HwConfig0 {
    #[inline(always)]
    fn default() -> HwConfig0 {
        <crate::RegValueT<HwConfig0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptxfsiz_SPEC;
impl crate::sealed::RegSpec for Hptxfsiz_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS Host periodic transmit FIFO size\n          register"]
pub type Hptxfsiz = crate::RegValueT<Hptxfsiz_SPEC>;

impl NoBitfieldReg<Hptxfsiz_SPEC> for Hptxfsiz {}
impl ::core::default::Default for Hptxfsiz {
    #[inline(always)]
    fn default() -> Hptxfsiz {
        <crate::RegValueT<Hptxfsiz_SPEC> as RegisterValue<_>>::new(33555968)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf1_SPEC;
impl crate::sealed::RegSpec for Dieptxf1_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
pub type Dieptxf1 = crate::RegValueT<Dieptxf1_SPEC>;

impl NoBitfieldReg<Dieptxf1_SPEC> for Dieptxf1 {}
impl ::core::default::Default for Dieptxf1 {
    #[inline(always)]
    fn default() -> Dieptxf1 {
        <crate::RegValueT<Dieptxf1_SPEC> as RegisterValue<_>>::new(33555456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf2_SPEC;
impl crate::sealed::RegSpec for Dieptxf2_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
pub type Dieptxf2 = crate::RegValueT<Dieptxf2_SPEC>;

impl NoBitfieldReg<Dieptxf2_SPEC> for Dieptxf2 {}
impl ::core::default::Default for Dieptxf2 {
    #[inline(always)]
    fn default() -> Dieptxf2 {
        <crate::RegValueT<Dieptxf2_SPEC> as RegisterValue<_>>::new(33555456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf3_SPEC;
impl crate::sealed::RegSpec for Dieptxf3_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
pub type Dieptxf3 = crate::RegValueT<Dieptxf3_SPEC>;

impl NoBitfieldReg<Dieptxf3_SPEC> for Dieptxf3 {}
impl ::core::default::Default for Dieptxf3 {
    #[inline(always)]
    fn default() -> Dieptxf3 {
        <crate::RegValueT<Dieptxf3_SPEC> as RegisterValue<_>>::new(33555456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf4_SPEC;
impl crate::sealed::RegSpec for Dieptxf4_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
pub type Dieptxf4 = crate::RegValueT<Dieptxf4_SPEC>;

impl NoBitfieldReg<Dieptxf4_SPEC> for Dieptxf4 {}
impl ::core::default::Default for Dieptxf4 {
    #[inline(always)]
    fn default() -> Dieptxf4 {
        <crate::RegValueT<Dieptxf4_SPEC> as RegisterValue<_>>::new(33555456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf5_SPEC;
impl crate::sealed::RegSpec for Dieptxf5_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
pub type Dieptxf5 = crate::RegValueT<Dieptxf5_SPEC>;

impl NoBitfieldReg<Dieptxf5_SPEC> for Dieptxf5 {}
impl ::core::default::Default for Dieptxf5 {
    #[inline(always)]
    fn default() -> Dieptxf5 {
        <crate::RegValueT<Dieptxf5_SPEC> as RegisterValue<_>>::new(33555456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf6_SPEC;
impl crate::sealed::RegSpec for Dieptxf6_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
pub type Dieptxf6 = crate::RegValueT<Dieptxf6_SPEC>;

impl NoBitfieldReg<Dieptxf6_SPEC> for Dieptxf6 {}
impl ::core::default::Default for Dieptxf6 {
    #[inline(always)]
    fn default() -> Dieptxf6 {
        <crate::RegValueT<Dieptxf6_SPEC> as RegisterValue<_>>::new(33555456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf7_SPEC;
impl crate::sealed::RegSpec for Dieptxf7_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
pub type Dieptxf7 = crate::RegValueT<Dieptxf7_SPEC>;

impl NoBitfieldReg<Dieptxf7_SPEC> for Dieptxf7 {}
impl ::core::default::Default for Dieptxf7 {
    #[inline(always)]
    fn default() -> Dieptxf7 {
        <crate::RegValueT<Dieptxf7_SPEC> as RegisterValue<_>>::new(33555456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstsrPeripheral_SPEC;
impl crate::sealed::RegSpec for GrxstsrPeripheral_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS Receive status debug read register\n          (peripheral mode mode)"]
pub type GrxstsrPeripheral = crate::RegValueT<GrxstsrPeripheral_SPEC>;

impl NoBitfieldReg<GrxstsrPeripheral_SPEC> for GrxstsrPeripheral {}
impl ::core::default::Default for GrxstsrPeripheral {
    #[inline(always)]
    fn default() -> GrxstsrPeripheral {
        <crate::RegValueT<GrxstsrPeripheral_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstspPeripheral_SPEC;
impl crate::sealed::RegSpec for GrxstspPeripheral_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS status read and pop register\n          (peripheral mode)"]
pub type GrxstspPeripheral = crate::RegValueT<GrxstspPeripheral_SPEC>;

impl NoBitfieldReg<GrxstspPeripheral_SPEC> for GrxstspPeripheral {}
impl ::core::default::Default for GrxstspPeripheral {
    #[inline(always)]
    fn default() -> GrxstspPeripheral {
        <crate::RegValueT<GrxstspPeripheral_SPEC> as RegisterValue<_>>::new(0)
    }
}
