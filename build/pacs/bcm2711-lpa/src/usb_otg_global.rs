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
// Generated from SVD A, with svd2pac 0.8.0 on Fri, 25 Sep 2026 21:30:45 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common;
#[allow(unused_imports)]
use crate::common::{
    AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
    Write as _,
};
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
    pub fn gotgctl(&self) -> &'static self::GotgctlT {
        unsafe { self::GotgctlT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "OTG_HS interrupt register"]
    #[inline(always)]
    pub fn gotgint(&self) -> &'static self::GotgintT {
        unsafe { self::GotgintT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "OTG_HS AHB configuration\n          register"]
    #[inline(always)]
    pub fn gahbcfg(&self) -> &'static self::GahbcfgT {
        unsafe { self::GahbcfgT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "OTG_HS USB configuration\n          register"]
    #[inline(always)]
    pub fn gusbcfg(&self) -> &'static self::GusbcfgT {
        unsafe { self::GusbcfgT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "OTG_HS reset register"]
    #[inline(always)]
    pub fn grstctl(&self) -> &'static self::GrstctlT {
        unsafe { self::GrstctlT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "OTG_HS core interrupt register"]
    #[inline(always)]
    pub fn gintsts(&self) -> &'static self::GintstsT {
        unsafe { self::GintstsT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "OTG_HS interrupt mask register"]
    #[inline(always)]
    pub fn gintmsk(&self) -> &'static self::GintmskT {
        unsafe { self::GintmskT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "OTG_HS Receive status debug read register\n          (host mode)"]
    #[inline(always)]
    pub fn grxstsr_host(&self) -> &'static self::GrxstsrHostT {
        unsafe { self::GrxstsrHostT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "OTG_HS status read and pop register (host\n          mode)"]
    #[inline(always)]
    pub fn grxstsp_host(&self) -> &'static self::GrxstspHostT {
        unsafe { self::GrxstspHostT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "OTG_HS Receive FIFO size\n          register"]
    #[inline(always)]
    pub fn grxfsiz(&self) -> &'static self::GrxfsizT {
        unsafe { self::GrxfsizT::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }

    #[doc = "OTG_HS nonperiodic transmit FIFO size\n          register (host mode)"]
    #[inline(always)]
    pub fn gnptxfsiz_host(&self) -> &'static self::GnptxfsizHostT {
        unsafe { self::GnptxfsizHostT::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "Endpoint 0 transmit FIFO size (peripheral\n          mode)"]
    #[inline(always)]
    pub fn tx0fsiz_peripheral(&self) -> &'static self::Tx0FsizPeripheralT {
        unsafe { self::Tx0FsizPeripheralT::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "OTG_HS nonperiodic transmit FIFO/queue\n          status register"]
    #[inline(always)]
    pub fn gnptxsts(&self) -> &'static self::GnptxstsT {
        unsafe { self::GnptxstsT::from_ptr(self._svd2pac_as_ptr().add(44usize)) }
    }

    #[doc = "OTG_HS general core configuration\n          register"]
    #[inline(always)]
    pub fn gccfg(&self) -> &'static self::GccfgT {
        unsafe { self::GccfgT::from_ptr(self._svd2pac_as_ptr().add(56usize)) }
    }

    #[doc = "OTG_HS core ID register"]
    #[inline(always)]
    pub fn cid(&self) -> &'static self::CidT {
        unsafe { self::CidT::from_ptr(self._svd2pac_as_ptr().add(60usize)) }
    }

    #[doc = "OTG_HS vendor ID register"]
    #[inline(always)]
    pub fn vid(&self) -> &'static self::VidT {
        unsafe { self::VidT::from_ptr(self._svd2pac_as_ptr().add(64usize)) }
    }

    #[doc = "Direction"]
    #[inline(always)]
    pub fn hw_direction(&self) -> &'static self::HwDirectionT {
        unsafe { self::HwDirectionT::from_ptr(self._svd2pac_as_ptr().add(68usize)) }
    }

    #[doc = "Hardware Config 0"]
    #[inline(always)]
    pub fn hw_config0(&self) -> &'static self::HwConfig0T {
        unsafe { self::HwConfig0T::from_ptr(self._svd2pac_as_ptr().add(72usize)) }
    }

    #[doc = "OTG_HS Host periodic transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn hptxfsiz(&self) -> &'static self::HptxfsizT {
        unsafe { self::HptxfsizT::from_ptr(self._svd2pac_as_ptr().add(256usize)) }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn dieptxf1(&self) -> &'static self::Dieptxf1T {
        unsafe { self::Dieptxf1T::from_ptr(self._svd2pac_as_ptr().add(260usize)) }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn dieptxf2(&self) -> &'static self::Dieptxf2T {
        unsafe { self::Dieptxf2T::from_ptr(self._svd2pac_as_ptr().add(264usize)) }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn dieptxf3(&self) -> &'static self::Dieptxf3T {
        unsafe { self::Dieptxf3T::from_ptr(self._svd2pac_as_ptr().add(284usize)) }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn dieptxf4(&self) -> &'static self::Dieptxf4T {
        unsafe { self::Dieptxf4T::from_ptr(self._svd2pac_as_ptr().add(288usize)) }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn dieptxf5(&self) -> &'static self::Dieptxf5T {
        unsafe { self::Dieptxf5T::from_ptr(self._svd2pac_as_ptr().add(292usize)) }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn dieptxf6(&self) -> &'static self::Dieptxf6T {
        unsafe { self::Dieptxf6T::from_ptr(self._svd2pac_as_ptr().add(296usize)) }
    }

    #[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
    #[inline(always)]
    pub fn dieptxf7(&self) -> &'static self::Dieptxf7T {
        unsafe { self::Dieptxf7T::from_ptr(self._svd2pac_as_ptr().add(300usize)) }
    }

    #[doc = "OTG_HS Receive status debug read register\n          (peripheral mode mode)"]
    #[inline(always)]
    pub fn grxstsr_peripheral(&self) -> &'static self::GrxstsrPeripheralT {
        unsafe { self::GrxstsrPeripheralT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "OTG_HS status read and pop register\n          (peripheral mode)"]
    #[inline(always)]
    pub fn grxstsp_peripheral(&self) -> &'static self::GrxstspPeripheralT {
        unsafe { self::GrxstspPeripheralT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }
}

#[doc = "OTG_HS control and status\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gotgctl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gotgctl {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GotgctlT;
unsafe impl crate::common::AsPtr for GotgctlT {}
impl crate::common::Reg<Gotgctl> for GotgctlT {}

unsafe impl crate::common::Read<Gotgctl> for GotgctlT {}
unsafe impl crate::common::Write<Gotgctl> for GotgctlT {}
impl Gotgctl {
    #[doc = "Session request success"]
    #[inline(always)]
    pub fn srqscs(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gotgctl, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gotgctl, common::R>::from_register(self, 0)
    }

    #[doc = "Session request"]
    #[inline(always)]
    pub fn srq(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gotgctl, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gotgctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gotgctl, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gotgctl, common::R>::from_register(self, 0)
    }

    #[doc = "HNP request"]
    #[inline(always)]
    pub fn hnprq(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gotgctl, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gotgctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gotgctl, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gotgctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gotgctl, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gotgctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Connector ID status"]
    #[inline(always)]
    pub fn cidsts(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gotgctl, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gotgctl, common::R>::from_register(self, 0)
    }

    #[doc = "Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gotgctl, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gotgctl, common::R>::from_register(self, 0)
    }

    #[doc = "A-session valid"]
    #[inline(always)]
    pub fn asvld(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gotgctl, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gotgctl, common::R>::from_register(self, 0)
    }

    #[doc = "B-session valid"]
    #[inline(always)]
    pub fn bsvld(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gotgctl, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gotgctl, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gotgctl> for GotgctlT {
    #[inline(always)]
    fn reset_value(&self) -> Gotgctl {
        Gotgctl::new(2048)
    }
}

#[doc = "OTG_HS interrupt register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gotgint {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gotgint {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GotgintT;
unsafe impl crate::common::AsPtr for GotgintT {}
impl crate::common::Reg<Gotgint> for GotgintT {}

unsafe impl crate::common::Read<Gotgint> for GotgintT {}
unsafe impl crate::common::Write<Gotgint> for GotgintT {}
impl Gotgint {
    #[doc = "Session end detected"]
    #[inline(always)]
    pub fn sedet(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gotgint, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gotgint, common::RW>::from_register(self, 0)
    }

    #[doc = "Session request success status\n              change"]
    #[inline(always)]
    pub fn srsschg(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gotgint, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gotgint, common::RW>::from_register(self, 0)
    }

    #[doc = "Host negotiation success status\n              change"]
    #[inline(always)]
    pub fn hnsschg(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gotgint, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gotgint, common::RW>::from_register(self, 0)
    }

    #[doc = "Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gotgint, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gotgint, common::RW>::from_register(self, 0)
    }

    #[doc = "A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gotgint, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gotgint, common::RW>::from_register(self, 0)
    }

    #[doc = "Debounce done"]
    #[inline(always)]
    pub fn dbcdne(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gotgint, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gotgint, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gotgint> for GotgintT {
    #[inline(always)]
    fn reset_value(&self) -> Gotgint {
        Gotgint::new(0)
    }
}

#[doc = "OTG_HS AHB configuration\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gahbcfg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gahbcfg {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GahbcfgT;
unsafe impl crate::common::AsPtr for GahbcfgT {}
impl crate::common::Reg<Gahbcfg> for GahbcfgT {}

unsafe impl crate::common::Read<Gahbcfg> for GahbcfgT {}
unsafe impl crate::common::Write<Gahbcfg> for GahbcfgT {}
impl Gahbcfg {
    #[doc = "Global interrupt mask"]
    #[inline(always)]
    pub fn gint(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gahbcfg, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gahbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for all AXI writes before signaling DMA"]
    #[inline(always)]
    pub fn axi_wait(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gahbcfg, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gahbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Maximum AXI burst length"]
    #[inline(always)]
    pub fn axi_burst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        gahbcfg::AxiBurst,
        gahbcfg::AxiBurst,
        Gahbcfg,
        common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            gahbcfg::AxiBurst,
            gahbcfg::AxiBurst,
            Gahbcfg,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMA enable"]
    #[inline(always)]
    pub fn dmaen(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gahbcfg, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gahbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gahbcfg, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gahbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Periodic TxFIFO empty\n              level"]
    #[inline(always)]
    pub fn ptxfelvl(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gahbcfg, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gahbcfg, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gahbcfg> for GahbcfgT {
    #[inline(always)]
    fn reset_value(&self) -> Gahbcfg {
        Gahbcfg::new(0)
    }
}
pub mod gahbcfg {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct AxiBurst(u8);

    impl AxiBurst {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for AxiBurst {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for AxiBurst {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<AxiBurst> for u64 {
        #[inline(always)]
        fn from(value: AxiBurst) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for AxiBurst {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl AxiBurst {
        pub const BURST_4: Self = Self(0);

        pub const BURST_3: Self = Self(1);

        pub const BURST_2: Self = Self(2);

        pub const BURST_1: Self = Self(3);
    }
}

#[doc = "OTG_HS USB configuration\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gusbcfg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gusbcfg {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GusbcfgT;
unsafe impl crate::common::AsPtr for GusbcfgT {}
impl crate::common::Reg<Gusbcfg> for GusbcfgT {}

unsafe impl crate::common::Read<Gusbcfg> for GusbcfgT {}
unsafe impl crate::common::Write<Gusbcfg> for GusbcfgT {}
impl Gusbcfg {
    #[doc = "FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(self) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Gusbcfg, common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, Gusbcfg, common::RW>::from_register(
            self, 0,
        )
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
        Gusbcfg,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gusbcfg::Phyif,
            gusbcfg::Phyif,
            Gusbcfg,
            common::RW,
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
        Gusbcfg,
        common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gusbcfg::Phytype,
            gusbcfg::Phytype,
            Gusbcfg,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Full speed interface"]
    #[inline(always)]
    pub fn fsif(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gusbcfg::Fsif, gusbcfg::Fsif, Gusbcfg, common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gusbcfg::Fsif,
            gusbcfg::Fsif,
            Gusbcfg,
            common::RW,
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
        Gusbcfg,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gusbcfg::Physel,
            gusbcfg::Physel,
            Gusbcfg,
            common::RW,
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
        Gusbcfg,
        common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gusbcfg::Ddrsel,
            gusbcfg::Ddrsel,
            Gusbcfg,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SRP-capable"]
    #[inline(always)]
    pub fn srpcap(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "USB turnaround time"]
    #[inline(always)]
    pub fn trdt(self) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, Gusbcfg, common::RW> {
        crate::common::RegisterField::<10, 0xf, 1, 0, u8, u8, Gusbcfg, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PHY Low-power clock select"]
    #[inline(always)]
    pub fn phylpcs(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "ULPI FS/LS select"]
    #[inline(always)]
    pub fn ulpifsls(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "ULPI Auto-resume"]
    #[inline(always)]
    pub fn ulpiar(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "ULPI Clock SuspendM"]
    #[inline(always)]
    pub fn ulpicsm(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "ULPI External VBUS Drive"]
    #[inline(always)]
    pub fn ulpievbusd(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "ULPI external VBUS\n              indicator"]
    #[inline(always)]
    pub fn ulpievbusi(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "TermSel DLine pulsing\n              selection"]
    #[inline(always)]
    pub fn tsdps(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Indicator complement"]
    #[inline(always)]
    pub fn pcci(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Indicator pass through"]
    #[inline(always)]
    pub fn ptci(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "ULPI interface protect\n              disable"]
    #[inline(always)]
    pub fn ulpiipd(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Forced host mode"]
    #[inline(always)]
    pub fn fhmod(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Forced peripheral mode"]
    #[inline(always)]
    pub fn fdmod(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gusbcfg, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gusbcfg, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gusbcfg> for GusbcfgT {
    #[inline(always)]
    fn reset_value(&self) -> Gusbcfg {
        Gusbcfg::new(2560)
    }
}
pub mod gusbcfg {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Phyif(u8);

    impl Phyif {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Phyif {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Phyif {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Phyif> for u64 {
        #[inline(always)]
        fn from(value: Phyif) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Phyif {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Phyif {
        pub const _8_BIT: Self = Self(0);

        pub const _16_BIT: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Phytype(u8);

    impl Phytype {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Phytype {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Phytype {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Phytype> for u64 {
        #[inline(always)]
        fn from(value: Phytype) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Phytype {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Phytype {
        pub const UTMI: Self = Self(0);

        pub const ULPI: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsif(u8);

    impl Fsif {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsif {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsif {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsif> for u64 {
        #[inline(always)]
        fn from(value: Fsif) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsif {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsif {
        pub const _6_PIN: Self = Self(0);

        pub const _3_PIN: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Physel(u8);

    impl Physel {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Physel {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Physel {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Physel> for u64 {
        #[inline(always)]
        fn from(value: Physel) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Physel {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Physel {
        pub const USB_20: Self = Self(0);

        pub const USB_11: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Ddrsel(u8);

    impl Ddrsel {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Ddrsel {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Ddrsel {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Ddrsel> for u64 {
        #[inline(always)]
        fn from(value: Ddrsel) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Ddrsel {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Ddrsel {
        pub const SINGLE: Self = Self(0);

        pub const DOUBLE: Self = Self(1);
    }
}

#[doc = "OTG_HS reset register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grstctl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Grstctl {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GrstctlT;
unsafe impl crate::common::AsPtr for GrstctlT {}
impl crate::common::Reg<Grstctl> for GrstctlT {}

unsafe impl crate::common::Read<Grstctl> for GrstctlT {}
unsafe impl crate::common::Write<Grstctl> for GrstctlT {}
impl Grstctl {
    #[doc = "Core soft reset"]
    #[inline(always)]
    pub fn csrst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Grstctl, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Grstctl, common::RW>::from_register(self, 0)
    }

    #[doc = "HCLK soft reset"]
    #[inline(always)]
    pub fn hsrst(self) -> crate::common::RegisterFieldBool<1, 1, 0, Grstctl, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Grstctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Host frame counter reset"]
    #[inline(always)]
    pub fn fcrst(self) -> crate::common::RegisterFieldBool<2, 1, 0, Grstctl, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Grstctl, common::RW>::from_register(self, 0)
    }

    #[doc = "RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(self) -> crate::common::RegisterFieldBool<4, 1, 0, Grstctl, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Grstctl, common::RW>::from_register(self, 0)
    }

    #[doc = "TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(self) -> crate::common::RegisterFieldBool<5, 1, 0, Grstctl, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Grstctl, common::RW>::from_register(self, 0)
    }

    #[doc = "TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(
        self,
    ) -> crate::common::RegisterField<6, 0x1f, 1, 0, u8, u8, Grstctl, common::RW> {
        crate::common::RegisterField::<6, 0x1f, 1, 0, u8, u8, Grstctl, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA request signal"]
    #[inline(always)]
    pub fn dmareq(self) -> crate::common::RegisterFieldBool<30, 1, 0, Grstctl, common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Grstctl, common::R>::from_register(self, 0)
    }

    #[doc = "AHB master idle"]
    #[inline(always)]
    pub fn ahbidl(self) -> crate::common::RegisterFieldBool<31, 1, 0, Grstctl, common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Grstctl, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Grstctl> for GrstctlT {
    #[inline(always)]
    fn reset_value(&self) -> Grstctl {
        Grstctl::new(536870912)
    }
}

#[doc = "OTG_HS core interrupt register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gintsts {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gintsts {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GintstsT;
unsafe impl crate::common::AsPtr for GintstsT {}
impl crate::common::Reg<Gintsts> for GintstsT {}

unsafe impl crate::common::Read<Gintsts> for GintstsT {}
unsafe impl crate::common::Write<Gintsts> for GintstsT {}
impl Gintsts {
    #[doc = "Current mode of operation"]
    #[inline(always)]
    pub fn cmod(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "OTG interrupt"]
    #[inline(always)]
    pub fn otgint(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Start of frame"]
    #[inline(always)]
    pub fn sof(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "RxFIFO nonempty"]
    #[inline(always)]
    pub fn rxflvl(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Nonperiodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfe(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Global IN nonperiodic NAK\n              effective"]
    #[inline(always)]
    pub fn ginakeff(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Global OUT NAK effective"]
    #[inline(always)]
    pub fn boutnakeff(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Early suspend"]
    #[inline(always)]
    pub fn esusp(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "USB suspend"]
    #[inline(always)]
    pub fn usbsusp(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "USB reset"]
    #[inline(always)]
    pub fn usbrst(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Enumeration done"]
    #[inline(always)]
    pub fn enumdne(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Isochronous OUT packet dropped\n              interrupt"]
    #[inline(always)]
    pub fn isoodrp(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "End of periodic frame\n              interrupt"]
    #[inline(always)]
    pub fn eopf(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "IN endpoint interrupt"]
    #[inline(always)]
    pub fn iepint(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oepint(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Incomplete isochronous IN\n              transfer"]
    #[inline(always)]
    pub fn iisoixfr(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Incomplete periodic\n              transfer"]
    #[inline(always)]
    pub fn pxfr_incompisoout(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Data fetch suspended"]
    #[inline(always)]
    pub fn datafsusp(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Host port interrupt"]
    #[inline(always)]
    pub fn hprtint(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Host channels interrupt"]
    #[inline(always)]
    pub fn hcint(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfe(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gintsts, common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gintsts, common::R>::from_register(self, 0)
    }

    #[doc = "Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Disconnect detected\n              interrupt"]
    #[inline(always)]
    pub fn discint(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Session request/new session detected\n              interrupt"]
    #[inline(always)]
    pub fn srqint(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }

    #[doc = "Resume/remote wakeup detected\n              interrupt"]
    #[inline(always)]
    pub fn wkuint(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gintsts, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gintsts, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gintsts> for GintstsT {
    #[inline(always)]
    fn reset_value(&self) -> Gintsts {
        Gintsts::new(67108896)
    }
}

#[doc = "OTG_HS interrupt mask register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gintmsk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gintmsk {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GintmskT;
unsafe impl crate::common::AsPtr for GintmskT {}
impl crate::common::Reg<Gintmsk> for GintmskT {}

unsafe impl crate::common::Read<Gintmsk> for GintmskT {}
unsafe impl crate::common::Write<Gintmsk> for GintmskT {}
impl Gintmsk {
    #[doc = "Mode mismatch interrupt\n              mask"]
    #[inline(always)]
    pub fn mmism(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Start of frame mask"]
    #[inline(always)]
    pub fn sofm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive FIFO nonempty mask"]
    #[inline(always)]
    pub fn rxflvlm(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Nonperiodic TxFIFO empty\n              mask"]
    #[inline(always)]
    pub fn nptxfem(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Global nonperiodic IN NAK effective\n              mask"]
    #[inline(always)]
    pub fn ginakeffm(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Global OUT NAK effective\n              mask"]
    #[inline(always)]
    pub fn gonakeffm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "USB reset mask"]
    #[inline(always)]
    pub fn usbrst(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Isochronous OUT packet dropped interrupt\n              mask"]
    #[inline(always)]
    pub fn isoodrpm(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "End of periodic frame interrupt\n              mask"]
    #[inline(always)]
    pub fn eopfm(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Endpoint mismatch interrupt\n              mask"]
    #[inline(always)]
    pub fn epmism(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "IN endpoints interrupt\n              mask"]
    #[inline(always)]
    pub fn iepint(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "OUT endpoints interrupt\n              mask"]
    #[inline(always)]
    pub fn oepint(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Incomplete isochronous IN transfer\n              mask"]
    #[inline(always)]
    pub fn iisoixfrm(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Incomplete periodic transfer\n              mask"]
    #[inline(always)]
    pub fn pxfrm_iisooxfrm(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Data fetch suspended mask"]
    #[inline(always)]
    pub fn fsuspm(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Host port interrupt mask"]
    #[inline(always)]
    pub fn prtim(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gintmsk, common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gintmsk, common::R>::from_register(self, 0)
    }

    #[doc = "Host channels interrupt\n              mask"]
    #[inline(always)]
    pub fn hcim(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Connector ID status change\n              mask"]
    #[inline(always)]
    pub fn cidschgm(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Disconnect detected interrupt\n              mask"]
    #[inline(always)]
    pub fn discint(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Session request/new session detected\n              interrupt mask"]
    #[inline(always)]
    pub fn srqim(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Resume/remote wakeup detected interrupt\n              mask"]
    #[inline(always)]
    pub fn wuim(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gintmsk, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gintmsk, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gintmsk> for GintmskT {
    #[inline(always)]
    fn reset_value(&self) -> Gintmsk {
        Gintmsk::new(0)
    }
}

#[doc = "OTG_HS Receive status debug read register\n          (host mode)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstsrHost {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GrxstsrHost {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GrxstsrHostT;
unsafe impl crate::common::AsPtr for GrxstsrHostT {}
impl crate::common::Reg<GrxstsrHost> for GrxstsrHostT {}

unsafe impl crate::common::Read<GrxstsrHost> for GrxstsrHostT {}
impl GrxstsrHost {
    #[doc = "Channel number"]
    #[inline(always)]
    pub fn chnum(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, GrxstsrHost, common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, u8, GrxstsrHost, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Byte count"]
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<4, 0x7ff, 1, 0, u16, u16, GrxstsrHost, common::R> {
        crate::common::RegisterField::<4,0x7ff,1,0,u16,u16,GrxstsrHost,common::R>::from_register(self,0)
    }

    #[doc = "Data PID"]
    #[inline(always)]
    pub fn dpid(
        self,
    ) -> crate::common::RegisterField<15, 0x3, 1, 0, u8, u8, GrxstsrHost, common::R> {
        crate::common::RegisterField::<15, 0x3, 1, 0, u8, u8, GrxstsrHost, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Packet status"]
    #[inline(always)]
    pub fn pktsts(
        self,
    ) -> crate::common::RegisterField<17, 0xf, 1, 0, u8, u8, GrxstsrHost, common::R> {
        crate::common::RegisterField::<17, 0xf, 1, 0, u8, u8, GrxstsrHost, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GrxstsrHost> for GrxstsrHostT {
    #[inline(always)]
    fn reset_value(&self) -> GrxstsrHost {
        GrxstsrHost::new(0)
    }
}

#[doc = "OTG_HS status read and pop register (host\n          mode)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstspHost {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GrxstspHost {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GrxstspHostT;
unsafe impl crate::common::AsPtr for GrxstspHostT {}
impl crate::common::Reg<GrxstspHost> for GrxstspHostT {}

unsafe impl crate::common::Read<GrxstspHost> for GrxstspHostT {}
impl GrxstspHost {
    #[doc = "Channel number"]
    #[inline(always)]
    pub fn chnum(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, GrxstspHost, common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, u8, GrxstspHost, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Byte count"]
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<4, 0x7ff, 1, 0, u16, u16, GrxstspHost, common::R> {
        crate::common::RegisterField::<4,0x7ff,1,0,u16,u16,GrxstspHost,common::R>::from_register(self,0)
    }

    #[doc = "Data PID"]
    #[inline(always)]
    pub fn dpid(
        self,
    ) -> crate::common::RegisterField<15, 0x3, 1, 0, u8, u8, GrxstspHost, common::R> {
        crate::common::RegisterField::<15, 0x3, 1, 0, u8, u8, GrxstspHost, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Packet status"]
    #[inline(always)]
    pub fn pktsts(
        self,
    ) -> crate::common::RegisterField<17, 0xf, 1, 0, u8, u8, GrxstspHost, common::R> {
        crate::common::RegisterField::<17, 0xf, 1, 0, u8, u8, GrxstspHost, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<GrxstspHost> for GrxstspHostT {
    #[inline(always)]
    fn reset_value(&self) -> GrxstspHost {
        GrxstspHost::new(0)
    }
}

#[doc = "OTG_HS Receive FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Grxfsiz {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Grxfsiz {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GrxfsizT;
unsafe impl crate::common::AsPtr for GrxfsizT {}
impl crate::common::Reg<Grxfsiz> for GrxfsizT {}

unsafe impl crate::common::Read<Grxfsiz> for GrxfsizT {}
unsafe impl crate::common::Write<Grxfsiz> for GrxfsizT {}
impl Grxfsiz {
    #[doc = "RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Grxfsiz, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Grxfsiz,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Grxfsiz> for GrxfsizT {
    #[inline(always)]
    fn reset_value(&self) -> Grxfsiz {
        Grxfsiz::new(512)
    }
}

#[doc = "OTG_HS nonperiodic transmit FIFO size\n          register (host mode)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GnptxfsizHost {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GnptxfsizHost {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GnptxfsizHostT;
unsafe impl crate::common::AsPtr for GnptxfsizHostT {}
impl crate::common::Reg<GnptxfsizHost> for GnptxfsizHostT {}

unsafe impl crate::common::Read<GnptxfsizHost> for GnptxfsizHostT {}
unsafe impl crate::common::Write<GnptxfsizHost> for GnptxfsizHostT {}
impl GnptxfsizHost {
    #[doc = "Nonperiodic transmit RAM start\n              address"]
    #[inline(always)]
    pub fn nptxfsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, GnptxfsizHost, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,GnptxfsizHost,common::RW>::from_register(self,0)
    }

    #[doc = "Nonperiodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, GnptxfsizHost, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,GnptxfsizHost,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<GnptxfsizHost> for GnptxfsizHostT {
    #[inline(always)]
    fn reset_value(&self) -> GnptxfsizHost {
        GnptxfsizHost::new(512)
    }
}

#[doc = "Endpoint 0 transmit FIFO size (peripheral\n          mode)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx0FsizPeripheral {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Tx0FsizPeripheral {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Tx0FsizPeripheralT;
unsafe impl crate::common::AsPtr for Tx0FsizPeripheralT {}
impl crate::common::Reg<Tx0FsizPeripheral> for Tx0FsizPeripheralT {}

unsafe impl crate::common::Read<Tx0FsizPeripheral> for Tx0FsizPeripheralT {}
unsafe impl crate::common::Write<Tx0FsizPeripheral> for Tx0FsizPeripheralT {}
impl Tx0FsizPeripheral {
    #[doc = "Endpoint 0 transmit RAM start\n              address"]
    #[inline(always)]
    pub fn tx0fsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tx0FsizPeripheral, common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tx0FsizPeripheral,common::RW>::from_register(self,0)
    }

    #[doc = "Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Tx0FsizPeripheral, common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Tx0FsizPeripheral,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Tx0FsizPeripheral> for Tx0FsizPeripheralT {
    #[inline(always)]
    fn reset_value(&self) -> Tx0FsizPeripheral {
        Tx0FsizPeripheral::new(512)
    }
}

#[doc = "OTG_HS nonperiodic transmit FIFO/queue\n          status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gnptxsts {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gnptxsts {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GnptxstsT;
unsafe impl crate::common::AsPtr for GnptxstsT {}
impl crate::common::Reg<Gnptxsts> for GnptxstsT {}

unsafe impl crate::common::Read<Gnptxsts> for GnptxstsT {}
impl Gnptxsts {
    #[doc = "Nonperiodic TxFIFO space\n              available"]
    #[inline(always)]
    pub fn nptxfsav(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gnptxsts, common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gnptxsts,common::R>::from_register(self,0)
    }

    #[doc = "Nonperiodic transmit request queue space\n              available"]
    #[inline(always)]
    pub fn nptqxsav(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Gnptxsts, common::R> {
        crate::common::RegisterField::<16, 0xff, 1, 0, u8, u8, Gnptxsts, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Top of the nonperiodic transmit request\n              queue"]
    #[inline(always)]
    pub fn nptxqtop(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, u8, Gnptxsts, common::R> {
        crate::common::RegisterField::<24, 0x7f, 1, 0, u8, u8, Gnptxsts, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Gnptxsts> for GnptxstsT {
    #[inline(always)]
    fn reset_value(&self) -> Gnptxsts {
        Gnptxsts::new(524800)
    }
}

#[doc = "OTG_HS general core configuration\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gccfg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gccfg {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GccfgT;
unsafe impl crate::common::AsPtr for GccfgT {}
impl crate::common::Reg<Gccfg> for GccfgT {}

unsafe impl crate::common::Read<Gccfg> for GccfgT {}
unsafe impl crate::common::Write<Gccfg> for GccfgT {}
impl Gccfg {
    #[doc = "Power down"]
    #[inline(always)]
    pub fn pwrdwn(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gccfg, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gccfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable I2C bus connection for the\n              external I2C PHY interface"]
    #[inline(always)]
    pub fn i2cpaden(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gccfg, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gccfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable the VBUS sensing\n              device"]
    #[inline(always)]
    pub fn vbusasen(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gccfg, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gccfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable the VBUS sensing\n              device"]
    #[inline(always)]
    pub fn vbusbsen(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gccfg, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gccfg, common::RW>::from_register(self, 0)
    }

    #[doc = "SOF output enable"]
    #[inline(always)]
    pub fn sofouten(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gccfg, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gccfg, common::RW>::from_register(self, 0)
    }

    #[doc = "VBUS sensing disable\n              option"]
    #[inline(always)]
    pub fn novbussens(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gccfg, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gccfg, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gccfg> for GccfgT {
    #[inline(always)]
    fn reset_value(&self) -> Gccfg {
        Gccfg::new(0)
    }
}

#[doc = "OTG_HS core ID register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cid {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cid {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct CidT;
unsafe impl crate::common::AsPtr for CidT {}
impl crate::common::Reg<Cid> for CidT {}

unsafe impl crate::common::Read<Cid> for CidT {}
unsafe impl crate::common::Write<Cid> for CidT {}
impl Cid {
    #[doc = "Product ID field"]
    #[inline(always)]
    pub fn product_id(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cid, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cid,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Cid> for CidT {
    #[inline(always)]
    fn reset_value(&self) -> Cid {
        Cid::new(4608)
    }
}

#[doc = "OTG_HS vendor ID register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Vid {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct VidT;
unsafe impl crate::common::AsPtr for VidT {}
impl crate::common::Reg<Vid> for VidT {}

unsafe impl crate::common::Read<Vid> for VidT {}

impl crate::common::NoBitfieldReg for Vid {}
impl crate::common::ResetValue<Vid> for VidT {
    #[inline(always)]
    fn reset_value(&self) -> Vid {
        Vid::new(0)
    }
}

#[doc = "Direction"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwDirection {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for HwDirection {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct HwDirectionT;
unsafe impl crate::common::AsPtr for HwDirectionT {}
impl crate::common::Reg<HwDirection> for HwDirectionT {}

unsafe impl crate::common::Read<HwDirection> for HwDirectionT {}
impl HwDirection {
    #[doc = "Direction %s"]
    #[inline(always)]
    pub fn direction(
        self,
        index: u8,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        16,
        2,
        hw_direction::Direction,
        hw_direction::Direction,
        HwDirection,
        common::R,
    > {
        assert!(index < 16);
        crate::common::RegisterField::<
            0,
            0x3,
            16,
            2,
            hw_direction::Direction,
            hw_direction::Direction,
            HwDirection,
            common::R,
        >::from_register(self, index)
    }
}
impl crate::common::ResetValue<HwDirection> for HwDirectionT {
    #[inline(always)]
    fn reset_value(&self) -> HwDirection {
        HwDirection::new(0)
    }
}
pub mod hw_direction {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Direction(u8);

    impl Direction {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Direction {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Direction {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Direction> for u64 {
        #[inline(always)]
        fn from(value: Direction) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Direction {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Direction {
        pub const BIDIR: Self = Self(0);

        pub const IN: Self = Self(1);

        pub const OUT: Self = Self(2);
    }
}

#[doc = "Hardware Config 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwConfig0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for HwConfig0 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct HwConfig0T;
unsafe impl crate::common::AsPtr for HwConfig0T {}
impl crate::common::Reg<HwConfig0> for HwConfig0T {}

unsafe impl crate::common::Read<HwConfig0> for HwConfig0T {}
impl HwConfig0 {
    #[doc = "Operating Mode"]
    #[inline(always)]
    pub fn operating_mode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        hw_config0::OperatingMode,
        hw_config0::OperatingMode,
        HwConfig0,
        common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            hw_config0::OperatingMode,
            hw_config0::OperatingMode,
            HwConfig0,
            common::R,
        >::from_register(self, 0)
    }

    #[doc = "Architecture"]
    #[inline(always)]
    pub fn architecture(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x3,
        1,
        0,
        hw_config0::Architecture,
        hw_config0::Architecture,
        HwConfig0,
        common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x3,
            1,
            0,
            hw_config0::Architecture,
            hw_config0::Architecture,
            HwConfig0,
            common::R,
        >::from_register(self, 0)
    }

    #[doc = "Point to Point"]
    #[inline(always)]
    pub fn point_to_point(self) -> crate::common::RegisterFieldBool<5, 1, 0, HwConfig0, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, HwConfig0, common::R>::from_register(self, 0)
    }

    #[doc = "High Speed Physical"]
    #[inline(always)]
    pub fn high_speed_phy(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        hw_config0::HighSpeedPhy,
        hw_config0::HighSpeedPhy,
        HwConfig0,
        common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            hw_config0::HighSpeedPhy,
            hw_config0::HighSpeedPhy,
            HwConfig0,
            common::R,
        >::from_register(self, 0)
    }

    #[doc = "Full Speed Physical"]
    #[inline(always)]
    pub fn full_speed_phy(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        hw_config0::FullSpeedPhy,
        hw_config0::FullSpeedPhy,
        HwConfig0,
        common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            hw_config0::FullSpeedPhy,
            hw_config0::FullSpeedPhy,
            HwConfig0,
            common::R,
        >::from_register(self, 0)
    }

    #[doc = "Device end point count"]
    #[inline(always)]
    pub fn device_end_point_count(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, HwConfig0, common::R> {
        crate::common::RegisterField::<10, 0xf, 1, 0, u8, u8, HwConfig0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Host channel count"]
    #[inline(always)]
    pub fn host_channel_count(
        self,
    ) -> crate::common::RegisterField<14, 0xf, 1, 0, u8, u8, HwConfig0, common::R> {
        crate::common::RegisterField::<14, 0xf, 1, 0, u8, u8, HwConfig0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Supports periodic endpoints"]
    #[inline(always)]
    pub fn supports_periodic_endpoints(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, HwConfig0, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, HwConfig0, common::R>::from_register(self, 0)
    }

    #[doc = "Dynamic FIFO"]
    #[inline(always)]
    pub fn dynamic_fifo(self) -> crate::common::RegisterFieldBool<19, 1, 0, HwConfig0, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, HwConfig0, common::R>::from_register(self, 0)
    }

    #[doc = "Multi proc int"]
    #[inline(always)]
    pub fn multi_proc_int(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, HwConfig0, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, HwConfig0, common::R>::from_register(self, 0)
    }

    #[doc = "Non periodic queue depth"]
    #[inline(always)]
    pub fn non_periodic_queue_depth(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, HwConfig0, common::R> {
        crate::common::RegisterField::<22, 0x3, 1, 0, u8, u8, HwConfig0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Host periodic queue depth"]
    #[inline(always)]
    pub fn host_periodic_queue_depth(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, HwConfig0, common::R> {
        crate::common::RegisterField::<24, 0x3, 1, 0, u8, u8, HwConfig0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Device token queue depth"]
    #[inline(always)]
    pub fn device_token_queue_depth(
        self,
    ) -> crate::common::RegisterField<26, 0x1f, 1, 0, u8, u8, HwConfig0, common::R> {
        crate::common::RegisterField::<26, 0x1f, 1, 0, u8, u8, HwConfig0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable IC USB"]
    #[inline(always)]
    pub fn enable_ic_usb(self) -> crate::common::RegisterFieldBool<31, 1, 0, HwConfig0, common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, HwConfig0, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<HwConfig0> for HwConfig0T {
    #[inline(always)]
    fn reset_value(&self) -> HwConfig0 {
        HwConfig0::new(0)
    }
}
pub mod hw_config0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct OperatingMode(u8);

    impl OperatingMode {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for OperatingMode {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for OperatingMode {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<OperatingMode> for u64 {
        #[inline(always)]
        fn from(value: OperatingMode) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for OperatingMode {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl OperatingMode {
        pub const HNP_SRP_CAPABLE: Self = Self(0);

        pub const SRP_ONLY_CAPABLE: Self = Self(1);

        pub const NO_HNP_SRP_CAPABLE: Self = Self(2);

        pub const SRP_CAPABLE_DEVICE: Self = Self(3);

        pub const NO_SRP_CAPABLE_DEVICE: Self = Self(4);

        pub const SRP_CAPABLE_HOST: Self = Self(5);

        pub const NO_SRP_CAPABLE_HOST: Self = Self(6);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Architecture(u8);

    impl Architecture {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Architecture {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Architecture {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Architecture> for u64 {
        #[inline(always)]
        fn from(value: Architecture) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Architecture {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Architecture {
        pub const SLAVE_ONLY: Self = Self(0);

        pub const EXTERNAL_DMA: Self = Self(1);

        pub const INTERNAL_DMA: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct HighSpeedPhy(u8);

    impl HighSpeedPhy {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for HighSpeedPhy {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for HighSpeedPhy {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<HighSpeedPhy> for u64 {
        #[inline(always)]
        fn from(value: HighSpeedPhy) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for HighSpeedPhy {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl HighSpeedPhy {
        pub const NOT_SUPPORTED: Self = Self(0);

        pub const UTMI: Self = Self(1);

        pub const ULPI: Self = Self(2);

        pub const UTMI_ULPI: Self = Self(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct FullSpeedPhy(u8);

    impl FullSpeedPhy {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for FullSpeedPhy {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for FullSpeedPhy {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<FullSpeedPhy> for u64 {
        #[inline(always)]
        fn from(value: FullSpeedPhy) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for FullSpeedPhy {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl FullSpeedPhy {
        pub const PHY_0: Self = Self(0);

        pub const DEDICATED: Self = Self(1);

        pub const PHY_2: Self = Self(2);

        pub const PHY_3: Self = Self(3);
    }
}

#[doc = "OTG_HS Host periodic transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptxfsiz {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Hptxfsiz {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct HptxfsizT;
unsafe impl crate::common::AsPtr for HptxfsizT {}
impl crate::common::Reg<Hptxfsiz> for HptxfsizT {}

unsafe impl crate::common::Read<Hptxfsiz> for HptxfsizT {}
unsafe impl crate::common::Write<Hptxfsiz> for HptxfsizT {}
impl Hptxfsiz {
    #[doc = "Host periodic TxFIFO start\n              address"]
    #[inline(always)]
    pub fn ptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Hptxfsiz, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Hptxfsiz,common::RW>::from_register(self,0)
    }

    #[doc = "Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Hptxfsiz, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Hptxfsiz,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Hptxfsiz> for HptxfsizT {
    #[inline(always)]
    fn reset_value(&self) -> Hptxfsiz {
        Hptxfsiz::new(33555968)
    }
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dieptxf1 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Dieptxf1T;
unsafe impl crate::common::AsPtr for Dieptxf1T {}
impl crate::common::Reg<Dieptxf1> for Dieptxf1T {}

unsafe impl crate::common::Read<Dieptxf1> for Dieptxf1T {}
unsafe impl crate::common::Write<Dieptxf1> for Dieptxf1T {}
impl Dieptxf1 {
    #[doc = "IN endpoint FIFOx transmit RAM start\n              address"]
    #[inline(always)]
    pub fn ineptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dieptxf1, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dieptxf1,common::RW>::from_register(self,0)
    }

    #[doc = "IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dieptxf1, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dieptxf1,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dieptxf1> for Dieptxf1T {
    #[inline(always)]
    fn reset_value(&self) -> Dieptxf1 {
        Dieptxf1::new(33555456)
    }
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dieptxf2 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Dieptxf2T;
unsafe impl crate::common::AsPtr for Dieptxf2T {}
impl crate::common::Reg<Dieptxf2> for Dieptxf2T {}

unsafe impl crate::common::Read<Dieptxf2> for Dieptxf2T {}
unsafe impl crate::common::Write<Dieptxf2> for Dieptxf2T {}
impl Dieptxf2 {
    #[doc = "IN endpoint FIFOx transmit RAM start\n              address"]
    #[inline(always)]
    pub fn ineptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dieptxf2, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dieptxf2,common::RW>::from_register(self,0)
    }

    #[doc = "IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dieptxf2, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dieptxf2,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dieptxf2> for Dieptxf2T {
    #[inline(always)]
    fn reset_value(&self) -> Dieptxf2 {
        Dieptxf2::new(33555456)
    }
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dieptxf3 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Dieptxf3T;
unsafe impl crate::common::AsPtr for Dieptxf3T {}
impl crate::common::Reg<Dieptxf3> for Dieptxf3T {}

unsafe impl crate::common::Read<Dieptxf3> for Dieptxf3T {}
unsafe impl crate::common::Write<Dieptxf3> for Dieptxf3T {}
impl Dieptxf3 {
    #[doc = "IN endpoint FIFOx transmit RAM start\n              address"]
    #[inline(always)]
    pub fn ineptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dieptxf3, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dieptxf3,common::RW>::from_register(self,0)
    }

    #[doc = "IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dieptxf3, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dieptxf3,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dieptxf3> for Dieptxf3T {
    #[inline(always)]
    fn reset_value(&self) -> Dieptxf3 {
        Dieptxf3::new(33555456)
    }
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dieptxf4 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Dieptxf4T;
unsafe impl crate::common::AsPtr for Dieptxf4T {}
impl crate::common::Reg<Dieptxf4> for Dieptxf4T {}

unsafe impl crate::common::Read<Dieptxf4> for Dieptxf4T {}
unsafe impl crate::common::Write<Dieptxf4> for Dieptxf4T {}
impl Dieptxf4 {
    #[doc = "IN endpoint FIFOx transmit RAM start\n              address"]
    #[inline(always)]
    pub fn ineptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dieptxf4, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dieptxf4,common::RW>::from_register(self,0)
    }

    #[doc = "IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dieptxf4, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dieptxf4,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dieptxf4> for Dieptxf4T {
    #[inline(always)]
    fn reset_value(&self) -> Dieptxf4 {
        Dieptxf4::new(33555456)
    }
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dieptxf5 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Dieptxf5T;
unsafe impl crate::common::AsPtr for Dieptxf5T {}
impl crate::common::Reg<Dieptxf5> for Dieptxf5T {}

unsafe impl crate::common::Read<Dieptxf5> for Dieptxf5T {}
unsafe impl crate::common::Write<Dieptxf5> for Dieptxf5T {}
impl Dieptxf5 {
    #[doc = "IN endpoint FIFOx transmit RAM start\n              address"]
    #[inline(always)]
    pub fn ineptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dieptxf5, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dieptxf5,common::RW>::from_register(self,0)
    }

    #[doc = "IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dieptxf5, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dieptxf5,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dieptxf5> for Dieptxf5T {
    #[inline(always)]
    fn reset_value(&self) -> Dieptxf5 {
        Dieptxf5::new(33555456)
    }
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dieptxf6 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Dieptxf6T;
unsafe impl crate::common::AsPtr for Dieptxf6T {}
impl crate::common::Reg<Dieptxf6> for Dieptxf6T {}

unsafe impl crate::common::Read<Dieptxf6> for Dieptxf6T {}
unsafe impl crate::common::Write<Dieptxf6> for Dieptxf6T {}
impl Dieptxf6 {
    #[doc = "IN endpoint FIFOx transmit RAM start\n              address"]
    #[inline(always)]
    pub fn ineptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dieptxf6, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dieptxf6,common::RW>::from_register(self,0)
    }

    #[doc = "IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dieptxf6, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dieptxf6,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dieptxf6> for Dieptxf6T {
    #[inline(always)]
    fn reset_value(&self) -> Dieptxf6 {
        Dieptxf6::new(33555456)
    }
}

#[doc = "OTG_HS device IN endpoint transmit FIFO size\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieptxf7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dieptxf7 {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct Dieptxf7T;
unsafe impl crate::common::AsPtr for Dieptxf7T {}
impl crate::common::Reg<Dieptxf7> for Dieptxf7T {}

unsafe impl crate::common::Read<Dieptxf7> for Dieptxf7T {}
unsafe impl crate::common::Write<Dieptxf7> for Dieptxf7T {}
impl Dieptxf7 {
    #[doc = "IN endpoint FIFOx transmit RAM start\n              address"]
    #[inline(always)]
    pub fn ineptxsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dieptxf7, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dieptxf7,common::RW>::from_register(self,0)
    }

    #[doc = "IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Dieptxf7, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Dieptxf7,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dieptxf7> for Dieptxf7T {
    #[inline(always)]
    fn reset_value(&self) -> Dieptxf7 {
        Dieptxf7::new(33555456)
    }
}

#[doc = "OTG_HS Receive status debug read register\n          (peripheral mode mode)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstsrPeripheral {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GrxstsrPeripheral {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GrxstsrPeripheralT;
unsafe impl crate::common::AsPtr for GrxstsrPeripheralT {}
impl crate::common::Reg<GrxstsrPeripheral> for GrxstsrPeripheralT {}

unsafe impl crate::common::Read<GrxstsrPeripheral> for GrxstsrPeripheralT {}
impl GrxstsrPeripheral {
    #[doc = "Endpoint number"]
    #[inline(always)]
    pub fn epnum(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, GrxstsrPeripheral, common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,GrxstsrPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Byte count"]
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<4, 0x7ff, 1, 0, u16, u16, GrxstsrPeripheral, common::R> {
        crate::common::RegisterField::<4,0x7ff,1,0,u16,u16,GrxstsrPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Data PID"]
    #[inline(always)]
    pub fn dpid(
        self,
    ) -> crate::common::RegisterField<15, 0x3, 1, 0, u8, u8, GrxstsrPeripheral, common::R> {
        crate::common::RegisterField::<15,0x3,1,0,u8,u8,GrxstsrPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Packet status"]
    #[inline(always)]
    pub fn pktsts(
        self,
    ) -> crate::common::RegisterField<17, 0xf, 1, 0, u8, u8, GrxstsrPeripheral, common::R> {
        crate::common::RegisterField::<17,0xf,1,0,u8,u8,GrxstsrPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Frame number"]
    #[inline(always)]
    pub fn frmnum(
        self,
    ) -> crate::common::RegisterField<21, 0xf, 1, 0, u8, u8, GrxstsrPeripheral, common::R> {
        crate::common::RegisterField::<21,0xf,1,0,u8,u8,GrxstsrPeripheral,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<GrxstsrPeripheral> for GrxstsrPeripheralT {
    #[inline(always)]
    fn reset_value(&self) -> GrxstsrPeripheral {
        GrxstsrPeripheral::new(0)
    }
}

#[doc = "OTG_HS status read and pop register\n          (peripheral mode)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrxstspPeripheral {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for GrxstspPeripheral {
    type DataType = u32;

    fn inner_mut(&mut self) -> (&mut Self::DataType, &mut Self::DataType) {
        (&mut self.data, &mut self.mask)
    }

    fn inner(&self) -> (Self::DataType, Self::DataType) {
        (self.data, self.mask)
    }

    fn new(data: Self::DataType) -> Self {
        Self { data, mask: 0x0 }
    }
}

#[doc(hidden)]
pub struct GrxstspPeripheralT;
unsafe impl crate::common::AsPtr for GrxstspPeripheralT {}
impl crate::common::Reg<GrxstspPeripheral> for GrxstspPeripheralT {}

unsafe impl crate::common::Read<GrxstspPeripheral> for GrxstspPeripheralT {}
impl GrxstspPeripheral {
    #[doc = "Endpoint number"]
    #[inline(always)]
    pub fn epnum(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, GrxstspPeripheral, common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,GrxstspPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Byte count"]
    #[inline(always)]
    pub fn bcnt(
        self,
    ) -> crate::common::RegisterField<4, 0x7ff, 1, 0, u16, u16, GrxstspPeripheral, common::R> {
        crate::common::RegisterField::<4,0x7ff,1,0,u16,u16,GrxstspPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Data PID"]
    #[inline(always)]
    pub fn dpid(
        self,
    ) -> crate::common::RegisterField<15, 0x3, 1, 0, u8, u8, GrxstspPeripheral, common::R> {
        crate::common::RegisterField::<15,0x3,1,0,u8,u8,GrxstspPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Packet status"]
    #[inline(always)]
    pub fn pktsts(
        self,
    ) -> crate::common::RegisterField<17, 0xf, 1, 0, u8, u8, GrxstspPeripheral, common::R> {
        crate::common::RegisterField::<17,0xf,1,0,u8,u8,GrxstspPeripheral,common::R>::from_register(self,0)
    }

    #[doc = "Frame number"]
    #[inline(always)]
    pub fn frmnum(
        self,
    ) -> crate::common::RegisterField<21, 0xf, 1, 0, u8, u8, GrxstspPeripheral, common::R> {
        crate::common::RegisterField::<21,0xf,1,0,u8,u8,GrxstspPeripheral,common::R>::from_register(self,0)
    }
}
impl crate::common::ResetValue<GrxstspPeripheral> for GrxstspPeripheralT {
    #[inline(always)]
    fn reset_value(&self) -> GrxstspPeripheral {
        GrxstspPeripheral::new(0)
    }
}
