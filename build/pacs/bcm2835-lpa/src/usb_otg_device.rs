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
// Generated from SVD A, with svd2pac 0.8.0 on Fri, 25 Sep 2026 21:30:43 +0000

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
unsafe impl ::core::marker::Send for super::UsbOtgDevice {}
unsafe impl ::core::marker::Sync for super::UsbOtgDevice {}
impl super::UsbOtgDevice {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "OTG_HS device configuration\n          register"]
    #[inline(always)]
    pub fn dcfg(&self) -> &'static self::DcfgT {
        unsafe { self::DcfgT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "OTG_HS device control register"]
    #[inline(always)]
    pub fn dctl(&self) -> &'static self::DctlT {
        unsafe { self::DctlT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "OTG_HS device status register"]
    #[inline(always)]
    pub fn dsts(&self) -> &'static self::DstsT {
        unsafe { self::DstsT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "OTG_HS device IN endpoint common interrupt\n          mask register"]
    #[inline(always)]
    pub fn diepmsk(&self) -> &'static self::DiepmskT {
        unsafe { self::DiepmskT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "OTG_HS device OUT endpoint common interrupt\n          mask register"]
    #[inline(always)]
    pub fn doepmsk(&self) -> &'static self::DoepmskT {
        unsafe { self::DoepmskT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "OTG_HS device all endpoints interrupt\n          register"]
    #[inline(always)]
    pub fn daint(&self) -> &'static self::DaintT {
        unsafe { self::DaintT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "OTG_HS all endpoints interrupt mask\n          register"]
    #[inline(always)]
    pub fn daintmsk(&self) -> &'static self::DaintmskT {
        unsafe { self::DaintmskT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "OTG_HS device VBUS discharge time\n          register"]
    #[inline(always)]
    pub fn dvbusdis(&self) -> &'static self::DvbusdisT {
        unsafe { self::DvbusdisT::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "OTG_HS device VBUS pulsing time\n          register"]
    #[inline(always)]
    pub fn dvbuspulse(&self) -> &'static self::DvbuspulseT {
        unsafe { self::DvbuspulseT::from_ptr(self._svd2pac_as_ptr().add(44usize)) }
    }

    #[doc = "OTG_HS Device threshold control\n          register"]
    #[inline(always)]
    pub fn dthrctl(&self) -> &'static self::DthrctlT {
        unsafe { self::DthrctlT::from_ptr(self._svd2pac_as_ptr().add(48usize)) }
    }

    #[doc = "OTG_HS device IN endpoint FIFO empty\n          interrupt mask register"]
    #[inline(always)]
    pub fn diepempmsk(&self) -> &'static self::DiepempmskT {
        unsafe { self::DiepempmskT::from_ptr(self._svd2pac_as_ptr().add(52usize)) }
    }

    #[doc = "OTG_HS device each endpoint interrupt\n          register"]
    #[inline(always)]
    pub fn deachint(&self) -> &'static self::DeachintT {
        unsafe { self::DeachintT::from_ptr(self._svd2pac_as_ptr().add(56usize)) }
    }

    #[doc = "OTG_HS device each endpoint interrupt\n          register mask"]
    #[inline(always)]
    pub fn deachintmsk(&self) -> &'static self::DeachintmskT {
        unsafe { self::DeachintmskT::from_ptr(self._svd2pac_as_ptr().add(60usize)) }
    }

    #[doc = "OTG_HS device each in endpoint-1 interrupt\n          register"]
    #[inline(always)]
    pub fn diepeachmsk1(&self) -> &'static self::Diepeachmsk1T {
        unsafe { self::Diepeachmsk1T::from_ptr(self._svd2pac_as_ptr().add(64usize)) }
    }

    #[doc = "OTG_HS device each OUT endpoint-1 interrupt\n          register"]
    #[inline(always)]
    pub fn doepeachmsk1(&self) -> &'static self::Doepeachmsk1T {
        unsafe { self::Doepeachmsk1T::from_ptr(self._svd2pac_as_ptr().add(128usize)) }
    }

    #[doc = "IN Endpoint %s"]
    #[inline(always)]
    pub fn in_endpoint(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::usb_otg_device::_InEndpoint, 12, 0x20>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[doc = "OUT Endpoint %s"]
    #[inline(always)]
    pub fn out_endpoint(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::usb_otg_device::_OutEndpoint, 12, 0x20>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
}

#[doc = "OTG_HS device configuration\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcfg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dcfg {
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
pub struct DcfgT;
unsafe impl crate::common::AsPtr for DcfgT {}
impl crate::common::Reg<Dcfg> for DcfgT {}

unsafe impl crate::common::Read<Dcfg> for DcfgT {}
unsafe impl crate::common::Write<Dcfg> for DcfgT {}
impl Dcfg {
    #[doc = "Device speed"]
    #[inline(always)]
    pub fn dspd(self) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Dcfg, common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, u8, Dcfg, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Nonzero-length status OUT\n              handshake"]
    #[inline(always)]
    pub fn nzlsohsk(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dcfg, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dcfg, common::RW>::from_register(self, 0)
    }

    #[doc = "Device address"]
    #[inline(always)]
    pub fn dad(self) -> crate::common::RegisterField<4, 0x7f, 1, 0, u8, u8, Dcfg, common::RW> {
        crate::common::RegisterField::<4, 0x7f, 1, 0, u8, u8, Dcfg, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Periodic (micro)frame\n              interval"]
    #[inline(always)]
    pub fn pfivl(self) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, Dcfg, common::RW> {
        crate::common::RegisterField::<11, 0x3, 1, 0, u8, u8, Dcfg, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Periodic scheduling\n              interval"]
    #[inline(always)]
    pub fn perschivl(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, Dcfg, common::RW> {
        crate::common::RegisterField::<24, 0x3, 1, 0, u8, u8, Dcfg, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Dcfg> for DcfgT {
    #[inline(always)]
    fn reset_value(&self) -> Dcfg {
        Dcfg::new(35651584)
    }
}

#[doc = "OTG_HS device control register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dctl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dctl {
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
pub struct DctlT;
unsafe impl crate::common::AsPtr for DctlT {}
impl crate::common::Reg<Dctl> for DctlT {}

unsafe impl crate::common::Read<Dctl> for DctlT {}
unsafe impl crate::common::Write<Dctl> for DctlT {}
impl Dctl {
    #[doc = "Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dctl, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Soft disconnect"]
    #[inline(always)]
    pub fn sdis(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dctl, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dctl, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dctl, common::R>::from_register(self, 0)
    }

    #[doc = "Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dctl, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dctl, common::R>::from_register(self, 0)
    }

    #[doc = "Test control"]
    #[inline(always)]
    pub fn tctl(self) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Dctl, common::RW> {
        crate::common::RegisterField::<4, 0x7, 1, 0, u8, u8, Dctl, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(self) -> crate::common::RegisterFieldBool<7, 1, 0, Dctl, common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dctl, common::W>::from_register(self, 0)
    }

    #[doc = "Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dctl, common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dctl, common::W>::from_register(self, 0)
    }

    #[doc = "Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(self) -> crate::common::RegisterFieldBool<9, 1, 0, Dctl, common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Dctl, common::W>::from_register(self, 0)
    }

    #[doc = "Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(self) -> crate::common::RegisterFieldBool<10, 1, 0, Dctl, common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Dctl, common::W>::from_register(self, 0)
    }

    #[doc = "Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(self) -> crate::common::RegisterFieldBool<11, 1, 0, Dctl, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Dctl, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Dctl> for DctlT {
    #[inline(always)]
    fn reset_value(&self) -> Dctl {
        Dctl::new(0)
    }
}

#[doc = "OTG_HS device status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsts {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dsts {
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
pub struct DstsT;
unsafe impl crate::common::AsPtr for DstsT {}
impl crate::common::Reg<Dsts> for DstsT {}

unsafe impl crate::common::Read<Dsts> for DstsT {}
impl Dsts {
    #[doc = "Suspend status"]
    #[inline(always)]
    pub fn suspsts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dsts, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dsts, common::R>::from_register(self, 0)
    }

    #[doc = "Enumerated speed"]
    #[inline(always)]
    pub fn enumspd(self) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dsts, common::R> {
        crate::common::RegisterField::<1, 0x3, 1, 0, u8, u8, Dsts, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Erratic error"]
    #[inline(always)]
    pub fn eerr(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dsts, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dsts, common::R>::from_register(self, 0)
    }

    #[doc = "Frame number of the received\n              SOF"]
    #[inline(always)]
    pub fn fnsof(self) -> crate::common::RegisterField<8, 0x3fff, 1, 0, u16, u16, Dsts, common::R> {
        crate::common::RegisterField::<8, 0x3fff, 1, 0, u16, u16, Dsts, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Dsts> for DstsT {
    #[inline(always)]
    fn reset_value(&self) -> Dsts {
        Dsts::new(16)
    }
}

#[doc = "OTG_HS device IN endpoint common interrupt\n          mask register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepmsk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Diepmsk {
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
pub struct DiepmskT;
unsafe impl crate::common::AsPtr for DiepmskT {}
impl crate::common::Reg<Diepmsk> for DiepmskT {}

unsafe impl crate::common::Read<Diepmsk> for DiepmskT {}
unsafe impl crate::common::Write<Diepmsk> for DiepmskT {}
impl Diepmsk {
    #[doc = "Transfer completed interrupt\n              mask"]
    #[inline(always)]
    pub fn xfrcm(self) -> crate::common::RegisterFieldBool<0, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Endpoint disabled interrupt\n              mask"]
    #[inline(always)]
    pub fn epdm(self) -> crate::common::RegisterFieldBool<1, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Timeout condition mask (nonisochronous\n              endpoints)"]
    #[inline(always)]
    pub fn tom(self) -> crate::common::RegisterFieldBool<3, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "IN token received when TxFIFO empty\n              mask"]
    #[inline(always)]
    pub fn ittxfemsk(self) -> crate::common::RegisterFieldBool<4, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "IN token received with EP mismatch\n              mask"]
    #[inline(always)]
    pub fn inepnmm(self) -> crate::common::RegisterFieldBool<5, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "IN endpoint NAK effective\n              mask"]
    #[inline(always)]
    pub fn inepnem(self) -> crate::common::RegisterFieldBool<6, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "FIFO underrun mask"]
    #[inline(always)]
    pub fn txfurm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(self) -> crate::common::RegisterFieldBool<9, 1, 0, Diepmsk, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Diepmsk, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Diepmsk> for DiepmskT {
    #[inline(always)]
    fn reset_value(&self) -> Diepmsk {
        Diepmsk::new(0)
    }
}

#[doc = "OTG_HS device OUT endpoint common interrupt\n          mask register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doepmsk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Doepmsk {
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
pub struct DoepmskT;
unsafe impl crate::common::AsPtr for DoepmskT {}
impl crate::common::Reg<Doepmsk> for DoepmskT {}

unsafe impl crate::common::Read<Doepmsk> for DoepmskT {}
unsafe impl crate::common::Write<Doepmsk> for DoepmskT {}
impl Doepmsk {
    #[doc = "Transfer completed interrupt\n              mask"]
    #[inline(always)]
    pub fn xfrcm(self) -> crate::common::RegisterFieldBool<0, 1, 0, Doepmsk, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Doepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Endpoint disabled interrupt\n              mask"]
    #[inline(always)]
    pub fn epdm(self) -> crate::common::RegisterFieldBool<1, 1, 0, Doepmsk, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Doepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Doepmsk, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Doepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "OUT token received when endpoint\n              disabled mask"]
    #[inline(always)]
    pub fn otepdm(self) -> crate::common::RegisterFieldBool<4, 1, 0, Doepmsk, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Doepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "Back-to-back SETUP packets received\n              mask"]
    #[inline(always)]
    pub fn b2bstup(self) -> crate::common::RegisterFieldBool<6, 1, 0, Doepmsk, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Doepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "OUT packet error mask"]
    #[inline(always)]
    pub fn opem(self) -> crate::common::RegisterFieldBool<8, 1, 0, Doepmsk, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Doepmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "BNA interrupt mask"]
    #[inline(always)]
    pub fn boim(self) -> crate::common::RegisterFieldBool<9, 1, 0, Doepmsk, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Doepmsk, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Doepmsk> for DoepmskT {
    #[inline(always)]
    fn reset_value(&self) -> Doepmsk {
        Doepmsk::new(0)
    }
}

#[doc = "OTG_HS device all endpoints interrupt\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daint {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Daint {
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
pub struct DaintT;
unsafe impl crate::common::AsPtr for DaintT {}
impl crate::common::Reg<Daint> for DaintT {}

unsafe impl crate::common::Read<Daint> for DaintT {}
impl Daint {
    #[doc = "IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Daint, common::R> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Daint, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OUT endpoint interrupt\n              bits"]
    #[inline(always)]
    pub fn oepint(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Daint, common::R> {
        crate::common::RegisterField::<16, 0xffff, 1, 0, u16, u16, Daint, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Daint> for DaintT {
    #[inline(always)]
    fn reset_value(&self) -> Daint {
        Daint::new(0)
    }
}

#[doc = "OTG_HS all endpoints interrupt mask\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daintmsk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Daintmsk {
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
pub struct DaintmskT;
unsafe impl crate::common::AsPtr for DaintmskT {}
impl crate::common::Reg<Daintmsk> for DaintmskT {}

unsafe impl crate::common::Read<Daintmsk> for DaintmskT {}
unsafe impl crate::common::Write<Daintmsk> for DaintmskT {}
impl Daintmsk {
    #[doc = "IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Daintmsk, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Daintmsk,common::RW>::from_register(self,0)
    }

    #[doc = "OUT EP interrupt mask bits"]
    #[inline(always)]
    pub fn oepm(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Daintmsk, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Daintmsk,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Daintmsk> for DaintmskT {
    #[inline(always)]
    fn reset_value(&self) -> Daintmsk {
        Daintmsk::new(0)
    }
}

#[doc = "OTG_HS device VBUS discharge time\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvbusdis {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dvbusdis {
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
pub struct DvbusdisT;
unsafe impl crate::common::AsPtr for DvbusdisT {}
impl crate::common::Reg<Dvbusdis> for DvbusdisT {}

unsafe impl crate::common::Read<Dvbusdis> for DvbusdisT {}
unsafe impl crate::common::Write<Dvbusdis> for DvbusdisT {}
impl Dvbusdis {
    #[doc = "Device VBUS discharge time"]
    #[inline(always)]
    pub fn vbusdt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dvbusdis, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dvbusdis,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dvbusdis> for DvbusdisT {
    #[inline(always)]
    fn reset_value(&self) -> Dvbusdis {
        Dvbusdis::new(6103)
    }
}

#[doc = "OTG_HS device VBUS pulsing time\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvbuspulse {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dvbuspulse {
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
pub struct DvbuspulseT;
unsafe impl crate::common::AsPtr for DvbuspulseT {}
impl crate::common::Reg<Dvbuspulse> for DvbuspulseT {}

unsafe impl crate::common::Read<Dvbuspulse> for DvbuspulseT {}
unsafe impl crate::common::Write<Dvbuspulse> for DvbuspulseT {}
impl Dvbuspulse {
    #[doc = "Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Dvbuspulse, common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Dvbuspulse,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Dvbuspulse> for DvbuspulseT {
    #[inline(always)]
    fn reset_value(&self) -> Dvbuspulse {
        Dvbuspulse::new(1464)
    }
}

#[doc = "OTG_HS Device threshold control\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dthrctl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dthrctl {
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
pub struct DthrctlT;
unsafe impl crate::common::AsPtr for DthrctlT {}
impl crate::common::Reg<Dthrctl> for DthrctlT {}

unsafe impl crate::common::Read<Dthrctl> for DthrctlT {}
unsafe impl crate::common::Write<Dthrctl> for DthrctlT {}
impl Dthrctl {
    #[doc = "Nonisochronous IN endpoints threshold\n              enable"]
    #[inline(always)]
    pub fn nonisothren(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dthrctl, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dthrctl, common::RW>::from_register(self, 0)
    }

    #[doc = "ISO IN endpoint threshold\n              enable"]
    #[inline(always)]
    pub fn isothren(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dthrctl, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dthrctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Transmit threshold length"]
    #[inline(always)]
    pub fn txthrlen(
        self,
    ) -> crate::common::RegisterField<2, 0x1ff, 1, 0, u16, u16, Dthrctl, common::RW> {
        crate::common::RegisterField::<2, 0x1ff, 1, 0, u16, u16, Dthrctl, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Receive threshold enable"]
    #[inline(always)]
    pub fn rxthren(self) -> crate::common::RegisterFieldBool<16, 1, 0, Dthrctl, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dthrctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive threshold length"]
    #[inline(always)]
    pub fn rxthrlen(
        self,
    ) -> crate::common::RegisterField<17, 0x1ff, 1, 0, u16, u16, Dthrctl, common::RW> {
        crate::common::RegisterField::<17,0x1ff,1,0,u16,u16,Dthrctl,common::RW>::from_register(self,0)
    }

    #[doc = "Arbiter parking enable"]
    #[inline(always)]
    pub fn arpen(self) -> crate::common::RegisterFieldBool<27, 1, 0, Dthrctl, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Dthrctl, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Dthrctl> for DthrctlT {
    #[inline(always)]
    fn reset_value(&self) -> Dthrctl {
        Dthrctl::new(0)
    }
}

#[doc = "OTG_HS device IN endpoint FIFO empty\n          interrupt mask register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepempmsk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Diepempmsk {
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
pub struct DiepempmskT;
unsafe impl crate::common::AsPtr for DiepempmskT {}
impl crate::common::Reg<Diepempmsk> for DiepempmskT {}

unsafe impl crate::common::Read<Diepempmsk> for DiepempmskT {}
unsafe impl crate::common::Write<Diepempmsk> for DiepempmskT {}
impl Diepempmsk {
    #[doc = "IN EP Tx FIFO empty interrupt mask\n              bits"]
    #[inline(always)]
    pub fn ineptxfem(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Diepempmsk, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Diepempmsk,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Diepempmsk> for DiepempmskT {
    #[inline(always)]
    fn reset_value(&self) -> Diepempmsk {
        Diepempmsk::new(0)
    }
}

#[doc = "OTG_HS device each endpoint interrupt\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deachint {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Deachint {
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
pub struct DeachintT;
unsafe impl crate::common::AsPtr for DeachintT {}
impl crate::common::Reg<Deachint> for DeachintT {}

unsafe impl crate::common::Read<Deachint> for DeachintT {}
unsafe impl crate::common::Write<Deachint> for DeachintT {}
impl Deachint {
    #[doc = "IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(self) -> crate::common::RegisterFieldBool<1, 1, 0, Deachint, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Deachint, common::RW>::from_register(self, 0)
    }

    #[doc = "OUT endpoint 1 interrupt\n              bit"]
    #[inline(always)]
    pub fn oep1int(self) -> crate::common::RegisterFieldBool<17, 1, 0, Deachint, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Deachint, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Deachint> for DeachintT {
    #[inline(always)]
    fn reset_value(&self) -> Deachint {
        Deachint::new(0)
    }
}

#[doc = "OTG_HS device each endpoint interrupt\n          register mask"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deachintmsk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Deachintmsk {
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
pub struct DeachintmskT;
unsafe impl crate::common::AsPtr for DeachintmskT {}
impl crate::common::Reg<Deachintmsk> for DeachintmskT {}

unsafe impl crate::common::Read<Deachintmsk> for DeachintmskT {}
unsafe impl crate::common::Write<Deachintmsk> for DeachintmskT {}
impl Deachintmsk {
    #[doc = "IN Endpoint 1 interrupt mask\n              bit"]
    #[inline(always)]
    pub fn iep1intm(self) -> crate::common::RegisterFieldBool<1, 1, 0, Deachintmsk, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Deachintmsk, common::RW>::from_register(self, 0)
    }

    #[doc = "OUT Endpoint 1 interrupt mask\n              bit"]
    #[inline(always)]
    pub fn oep1intm(self) -> crate::common::RegisterFieldBool<17, 1, 0, Deachintmsk, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Deachintmsk, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Deachintmsk> for DeachintmskT {
    #[inline(always)]
    fn reset_value(&self) -> Deachintmsk {
        Deachintmsk::new(0)
    }
}

#[doc = "OTG_HS device each in endpoint-1 interrupt\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepeachmsk1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Diepeachmsk1 {
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
pub struct Diepeachmsk1T;
unsafe impl crate::common::AsPtr for Diepeachmsk1T {}
impl crate::common::Reg<Diepeachmsk1> for Diepeachmsk1T {}

unsafe impl crate::common::Read<Diepeachmsk1> for Diepeachmsk1T {}
unsafe impl crate::common::Write<Diepeachmsk1> for Diepeachmsk1T {}
impl Diepeachmsk1 {
    #[doc = "Transfer completed interrupt\n              mask"]
    #[inline(always)]
    pub fn xfrcm(self) -> crate::common::RegisterFieldBool<0, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Endpoint disabled interrupt\n              mask"]
    #[inline(always)]
    pub fn epdm(self) -> crate::common::RegisterFieldBool<1, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Timeout condition mask (nonisochronous\n              endpoints)"]
    #[inline(always)]
    pub fn tom(self) -> crate::common::RegisterFieldBool<3, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN token received when TxFIFO empty\n              mask"]
    #[inline(always)]
    pub fn ittxfemsk(self) -> crate::common::RegisterFieldBool<4, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN token received with EP mismatch\n              mask"]
    #[inline(always)]
    pub fn inepnmm(self) -> crate::common::RegisterFieldBool<5, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN endpoint NAK effective\n              mask"]
    #[inline(always)]
    pub fn inepnem(self) -> crate::common::RegisterFieldBool<6, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FIFO underrun mask"]
    #[inline(always)]
    pub fn txfurm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(self) -> crate::common::RegisterFieldBool<9, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(self) -> crate::common::RegisterFieldBool<13, 1, 0, Diepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Diepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Diepeachmsk1> for Diepeachmsk1T {
    #[inline(always)]
    fn reset_value(&self) -> Diepeachmsk1 {
        Diepeachmsk1::new(0)
    }
}

#[doc = "OTG_HS device each OUT endpoint-1 interrupt\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doepeachmsk1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Doepeachmsk1 {
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
pub struct Doepeachmsk1T;
unsafe impl crate::common::AsPtr for Doepeachmsk1T {}
impl crate::common::Reg<Doepeachmsk1> for Doepeachmsk1T {}

unsafe impl crate::common::Read<Doepeachmsk1> for Doepeachmsk1T {}
unsafe impl crate::common::Write<Doepeachmsk1> for Doepeachmsk1T {}
impl Doepeachmsk1 {
    #[doc = "Transfer completed interrupt\n              mask"]
    #[inline(always)]
    pub fn xfrcm(self) -> crate::common::RegisterFieldBool<0, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Endpoint disabled interrupt\n              mask"]
    #[inline(always)]
    pub fn epdm(self) -> crate::common::RegisterFieldBool<1, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Timeout condition mask"]
    #[inline(always)]
    pub fn tom(self) -> crate::common::RegisterFieldBool<3, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN token received when TxFIFO empty\n              mask"]
    #[inline(always)]
    pub fn ittxfemsk(self) -> crate::common::RegisterFieldBool<4, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN token received with EP mismatch\n              mask"]
    #[inline(always)]
    pub fn inepnmm(self) -> crate::common::RegisterFieldBool<5, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "IN endpoint NAK effective\n              mask"]
    #[inline(always)]
    pub fn inepnem(self) -> crate::common::RegisterFieldBool<6, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OUT packet error mask"]
    #[inline(always)]
    pub fn txfurm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(self) -> crate::common::RegisterFieldBool<9, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Bubble error interrupt\n              mask"]
    #[inline(always)]
    pub fn berrm(self) -> crate::common::RegisterFieldBool<12, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(self) -> crate::common::RegisterFieldBool<13, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetm(self) -> crate::common::RegisterFieldBool<14, 1, 0, Doepeachmsk1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Doepeachmsk1, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Doepeachmsk1> for Doepeachmsk1T {
    #[inline(always)]
    fn reset_value(&self) -> Doepeachmsk1 {
        Doepeachmsk1::new(0)
    }
}

#[doc(hidden)]
#[non_exhaustive]
pub struct _InEndpoint;

#[doc = "IN Endpoint %s"]
pub type InEndpoint = &'static _InEndpoint;

unsafe impl ::core::marker::Sync for _InEndpoint {}
impl _InEndpoint {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Control"]
    #[inline(always)]
    pub fn diepctl0(&self) -> &'static in_endpoint::Diepctl0T {
        unsafe { in_endpoint::Diepctl0T::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn diepint(&self) -> &'static in_endpoint::DiepintT {
        unsafe { in_endpoint::DiepintT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Transfer size"]
    #[inline(always)]
    pub fn dieptsiz(&self) -> &'static in_endpoint::DieptsizT {
        unsafe { in_endpoint::DieptsizT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "DMA address"]
    #[inline(always)]
    pub fn diepdma(&self) -> &'static in_endpoint::DiepdmaT {
        unsafe { in_endpoint::DiepdmaT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "Transmit FIFO status"]
    #[inline(always)]
    pub fn dtxfsts(&self) -> &'static in_endpoint::DtxfstsT {
        unsafe { in_endpoint::DtxfstsT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }
}

unsafe impl crate::common::AsPtr for _InEndpoint {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod in_endpoint {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[doc = "Control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl0 {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Diepctl0 {
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
    pub struct Diepctl0T;
    unsafe impl crate::common::AsPtr for Diepctl0T {}
    impl crate::common::Reg<Diepctl0> for Diepctl0T {}

    unsafe impl crate::common::Read<Diepctl0> for Diepctl0T {}
    unsafe impl crate::common::Write<Diepctl0> for Diepctl0T {}
    impl Diepctl0 {
        #[doc = "Maximum packet size"]
        #[inline(always)]
        pub fn mpsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Diepctl0, common::RW> {
            crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Diepctl0,common::RW>::from_register(self,0)
        }

        #[doc = "USB active endpoint"]
        #[inline(always)]
        pub fn usbaep(self) -> crate::common::RegisterFieldBool<15, 1, 0, Diepctl0, common::RW> {
            crate::common::RegisterFieldBool::<15, 1, 0, Diepctl0, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Even/odd frame"]
        #[inline(always)]
        pub fn eonum_dpid(self) -> crate::common::RegisterFieldBool<16, 1, 0, Diepctl0, common::R> {
            crate::common::RegisterFieldBool::<16, 1, 0, Diepctl0, common::R>::from_register(
                self, 0,
            )
        }

        #[doc = "NAK status"]
        #[inline(always)]
        pub fn naksts(self) -> crate::common::RegisterFieldBool<17, 1, 0, Diepctl0, common::R> {
            crate::common::RegisterFieldBool::<17, 1, 0, Diepctl0, common::R>::from_register(
                self, 0,
            )
        }

        #[doc = "Endpoint type"]
        #[inline(always)]
        pub fn eptyp(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, Diepctl0, common::RW> {
            crate::common::RegisterField::<18,0x3,1,0,u8,u8,Diepctl0,common::RW>::from_register(self,0)
        }

        #[doc = "STALL handshake"]
        #[inline(always)]
        pub fn stall(self) -> crate::common::RegisterFieldBool<21, 1, 0, Diepctl0, common::RW> {
            crate::common::RegisterFieldBool::<21, 1, 0, Diepctl0, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "TxFIFO number"]
        #[inline(always)]
        pub fn txfnum(
            self,
        ) -> crate::common::RegisterField<22, 0xf, 1, 0, u8, u8, Diepctl0, common::RW> {
            crate::common::RegisterField::<22,0xf,1,0,u8,u8,Diepctl0,common::RW>::from_register(self,0)
        }

        #[doc = "Clear NAK"]
        #[inline(always)]
        pub fn cnak(self) -> crate::common::RegisterFieldBool<26, 1, 0, Diepctl0, common::W> {
            crate::common::RegisterFieldBool::<26, 1, 0, Diepctl0, common::W>::from_register(
                self, 0,
            )
        }

        #[doc = "Set NAK"]
        #[inline(always)]
        pub fn snak(self) -> crate::common::RegisterFieldBool<27, 1, 0, Diepctl0, common::W> {
            crate::common::RegisterFieldBool::<27, 1, 0, Diepctl0, common::W>::from_register(
                self, 0,
            )
        }

        #[doc = "Set DATA0 PID"]
        #[inline(always)]
        pub fn sd0pid_sevnfrm(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, Diepctl0, common::W> {
            crate::common::RegisterFieldBool::<28, 1, 0, Diepctl0, common::W>::from_register(
                self, 0,
            )
        }

        #[doc = "Set odd frame"]
        #[inline(always)]
        pub fn soddfrm(self) -> crate::common::RegisterFieldBool<29, 1, 0, Diepctl0, common::W> {
            crate::common::RegisterFieldBool::<29, 1, 0, Diepctl0, common::W>::from_register(
                self, 0,
            )
        }

        #[doc = "Endpoint disable"]
        #[inline(always)]
        pub fn epdis(self) -> crate::common::RegisterFieldBool<30, 1, 0, Diepctl0, common::RW> {
            crate::common::RegisterFieldBool::<30, 1, 0, Diepctl0, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Endpoint enable"]
        #[inline(always)]
        pub fn epena(self) -> crate::common::RegisterFieldBool<31, 1, 0, Diepctl0, common::RW> {
            crate::common::RegisterFieldBool::<31, 1, 0, Diepctl0, common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl crate::common::ResetValue<Diepctl0> for Diepctl0T {
        #[inline(always)]
        fn reset_value(&self) -> Diepctl0 {
            Diepctl0::new(0)
        }
    }

    #[doc = "Interrupt"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Diepint {
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
    pub struct DiepintT;
    unsafe impl crate::common::AsPtr for DiepintT {}
    impl crate::common::Reg<Diepint> for DiepintT {}

    unsafe impl crate::common::Read<Diepint> for DiepintT {}
    unsafe impl crate::common::Write<Diepint> for DiepintT {}
    impl Diepint {
        #[doc = "Transfer completed\n                interrupt"]
        #[inline(always)]
        pub fn xfrc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, Diepint, common::RW>::from_register(self, 0)
        }

        #[doc = "Endpoint disabled\n                interrupt"]
        #[inline(always)]
        pub fn epdisd(self) -> crate::common::RegisterFieldBool<1, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, Diepint, common::RW>::from_register(self, 0)
        }

        #[doc = "Timeout condition"]
        #[inline(always)]
        pub fn toc(self) -> crate::common::RegisterFieldBool<3, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, Diepint, common::RW>::from_register(self, 0)
        }

        #[doc = "IN token received when TxFIFO is\n                empty"]
        #[inline(always)]
        pub fn ittxfe(self) -> crate::common::RegisterFieldBool<4, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<4, 1, 0, Diepint, common::RW>::from_register(self, 0)
        }

        #[doc = "IN endpoint NAK effective"]
        #[inline(always)]
        pub fn inepne(self) -> crate::common::RegisterFieldBool<6, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, Diepint, common::RW>::from_register(self, 0)
        }

        #[doc = "Transmit FIFO empty"]
        #[inline(always)]
        pub fn txfe(self) -> crate::common::RegisterFieldBool<7, 1, 0, Diepint, common::R> {
            crate::common::RegisterFieldBool::<7, 1, 0, Diepint, common::R>::from_register(self, 0)
        }

        #[doc = "Transmit Fifo Underrun"]
        #[inline(always)]
        pub fn txfifoudrn(self) -> crate::common::RegisterFieldBool<8, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<8, 1, 0, Diepint, common::RW>::from_register(self, 0)
        }

        #[doc = "Buffer not available\n                interrupt"]
        #[inline(always)]
        pub fn bna(self) -> crate::common::RegisterFieldBool<9, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<9, 1, 0, Diepint, common::RW>::from_register(self, 0)
        }

        #[doc = "Packet dropped status"]
        #[inline(always)]
        pub fn pktdrpsts(self) -> crate::common::RegisterFieldBool<11, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<11, 1, 0, Diepint, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Babble error interrupt"]
        #[inline(always)]
        pub fn berr(self) -> crate::common::RegisterFieldBool<12, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<12, 1, 0, Diepint, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "NAK interrupt"]
        #[inline(always)]
        pub fn nak(self) -> crate::common::RegisterFieldBool<13, 1, 0, Diepint, common::RW> {
            crate::common::RegisterFieldBool::<13, 1, 0, Diepint, common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl crate::common::ResetValue<Diepint> for DiepintT {
        #[inline(always)]
        fn reset_value(&self) -> Diepint {
            Diepint::new(128)
        }
    }

    #[doc = "Transfer size"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Dieptsiz {
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
    pub struct DieptsizT;
    unsafe impl crate::common::AsPtr for DieptsizT {}
    impl crate::common::Reg<Dieptsiz> for DieptsizT {}

    unsafe impl crate::common::Read<Dieptsiz> for DieptsizT {}
    unsafe impl crate::common::Write<Dieptsiz> for DieptsizT {}
    impl Dieptsiz {
        #[doc = "Transfer size"]
        #[inline(always)]
        pub fn xfrsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Dieptsiz, common::RW> {
            crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Dieptsiz,common::RW>::from_register(self,0)
        }

        #[doc = "Packet count"]
        #[inline(always)]
        pub fn pktcnt(
            self,
        ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, u8, Dieptsiz, common::RW> {
            crate::common::RegisterField::<19,0x3,1,0,u8,u8,Dieptsiz,common::RW>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Dieptsiz> for DieptsizT {
        #[inline(always)]
        fn reset_value(&self) -> Dieptsiz {
            Dieptsiz::new(0)
        }
    }

    #[doc = "DMA address"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepdma {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Diepdma {
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
    pub struct DiepdmaT;
    unsafe impl crate::common::AsPtr for DiepdmaT {}
    impl crate::common::Reg<Diepdma> for DiepdmaT {}

    unsafe impl crate::common::Read<Diepdma> for DiepdmaT {}
    unsafe impl crate::common::Write<Diepdma> for DiepdmaT {}
    impl Diepdma {
        #[doc = "DMA address"]
        #[inline(always)]
        pub fn dmaaddr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Diepdma, common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Diepdma,common::RW>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Diepdma> for DiepdmaT {
        #[inline(always)]
        fn reset_value(&self) -> Diepdma {
            Diepdma::new(0)
        }
    }

    #[doc = "Transmit FIFO status"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Dtxfsts {
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
    pub struct DtxfstsT;
    unsafe impl crate::common::AsPtr for DtxfstsT {}
    impl crate::common::Reg<Dtxfsts> for DtxfstsT {}

    unsafe impl crate::common::Read<Dtxfsts> for DtxfstsT {}
    impl Dtxfsts {
        #[doc = "IN endpoint TxFIFO space\n                avail"]
        #[inline(always)]
        pub fn ineptfsav(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Dtxfsts, common::R> {
            crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Dtxfsts,common::R>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Dtxfsts> for DtxfstsT {
        #[inline(always)]
        fn reset_value(&self) -> Dtxfsts {
            Dtxfsts::new(0)
        }
    }
}
#[doc(hidden)]
#[non_exhaustive]
pub struct _OutEndpoint;

#[doc = "OUT Endpoint %s"]
pub type OutEndpoint = &'static _OutEndpoint;

unsafe impl ::core::marker::Sync for _OutEndpoint {}
impl _OutEndpoint {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Control"]
    #[inline(always)]
    pub fn doepctl(&self) -> &'static out_endpoint::DoepctlT {
        unsafe { out_endpoint::DoepctlT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn doepint(&self) -> &'static out_endpoint::DoepintT {
        unsafe { out_endpoint::DoepintT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Transfer size"]
    #[inline(always)]
    pub fn doeptsiz(&self) -> &'static out_endpoint::DoeptsizT {
        unsafe { out_endpoint::DoeptsizT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "DMA address"]
    #[inline(always)]
    pub fn doepdma(&self) -> &'static out_endpoint::DoepdmaT {
        unsafe { out_endpoint::DoepdmaT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }
}

unsafe impl crate::common::AsPtr for _OutEndpoint {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod out_endpoint {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[doc = "Control"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Doepctl {
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
    pub struct DoepctlT;
    unsafe impl crate::common::AsPtr for DoepctlT {}
    impl crate::common::Reg<Doepctl> for DoepctlT {}

    unsafe impl crate::common::Read<Doepctl> for DoepctlT {}
    unsafe impl crate::common::Write<Doepctl> for DoepctlT {}
    impl Doepctl {
        #[doc = "Maximum packet size"]
        #[inline(always)]
        pub fn mpsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Doepctl, common::R> {
            crate::common::RegisterField::<0, 0x3, 1, 0, u8, u8, Doepctl, common::R>::from_register(
                self, 0,
            )
        }

        #[doc = "USB active endpoint"]
        #[inline(always)]
        pub fn usbaep(self) -> crate::common::RegisterFieldBool<15, 1, 0, Doepctl, common::R> {
            crate::common::RegisterFieldBool::<15, 1, 0, Doepctl, common::R>::from_register(self, 0)
        }

        #[doc = "NAK status"]
        #[inline(always)]
        pub fn naksts(self) -> crate::common::RegisterFieldBool<17, 1, 0, Doepctl, common::R> {
            crate::common::RegisterFieldBool::<17, 1, 0, Doepctl, common::R>::from_register(self, 0)
        }

        #[doc = "Endpoint type"]
        #[inline(always)]
        pub fn eptyp(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, Doepctl, common::R> {
            crate::common::RegisterField::<18, 0x3, 1, 0, u8, u8, Doepctl, common::R>::from_register(
                self, 0,
            )
        }

        #[doc = "Snoop mode"]
        #[inline(always)]
        pub fn snpm(self) -> crate::common::RegisterFieldBool<20, 1, 0, Doepctl, common::RW> {
            crate::common::RegisterFieldBool::<20, 1, 0, Doepctl, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "STALL handshake"]
        #[inline(always)]
        pub fn stall(self) -> crate::common::RegisterFieldBool<21, 1, 0, Doepctl, common::RW> {
            crate::common::RegisterFieldBool::<21, 1, 0, Doepctl, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Clear NAK"]
        #[inline(always)]
        pub fn cnak(self) -> crate::common::RegisterFieldBool<26, 1, 0, Doepctl, common::W> {
            crate::common::RegisterFieldBool::<26, 1, 0, Doepctl, common::W>::from_register(self, 0)
        }

        #[doc = "Set NAK"]
        #[inline(always)]
        pub fn snak(self) -> crate::common::RegisterFieldBool<27, 1, 0, Doepctl, common::W> {
            crate::common::RegisterFieldBool::<27, 1, 0, Doepctl, common::W>::from_register(self, 0)
        }

        #[doc = "Endpoint disable"]
        #[inline(always)]
        pub fn epdis(self) -> crate::common::RegisterFieldBool<30, 1, 0, Doepctl, common::R> {
            crate::common::RegisterFieldBool::<30, 1, 0, Doepctl, common::R>::from_register(self, 0)
        }

        #[doc = "Endpoint enable"]
        #[inline(always)]
        pub fn epena(self) -> crate::common::RegisterFieldBool<31, 1, 0, Doepctl, common::W> {
            crate::common::RegisterFieldBool::<31, 1, 0, Doepctl, common::W>::from_register(self, 0)
        }
    }
    impl crate::common::ResetValue<Doepctl> for DoepctlT {
        #[inline(always)]
        fn reset_value(&self) -> Doepctl {
            Doepctl::new(32768)
        }
    }

    #[doc = "Interrupt"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Doepint {
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
    pub struct DoepintT;
    unsafe impl crate::common::AsPtr for DoepintT {}
    impl crate::common::Reg<Doepint> for DoepintT {}

    unsafe impl crate::common::Read<Doepint> for DoepintT {}
    unsafe impl crate::common::Write<Doepint> for DoepintT {}
    impl Doepint {
        #[doc = "Transfer completed\n                interrupt"]
        #[inline(always)]
        pub fn xfrc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Doepint, common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, Doepint, common::RW>::from_register(self, 0)
        }

        #[doc = "Endpoint disabled\n                interrupt"]
        #[inline(always)]
        pub fn epdisd(self) -> crate::common::RegisterFieldBool<1, 1, 0, Doepint, common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, Doepint, common::RW>::from_register(self, 0)
        }

        #[doc = "SETUP phase done"]
        #[inline(always)]
        pub fn stup(self) -> crate::common::RegisterFieldBool<3, 1, 0, Doepint, common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, Doepint, common::RW>::from_register(self, 0)
        }

        #[doc = "OUT token received when endpoint\n                disabled"]
        #[inline(always)]
        pub fn otepdis(self) -> crate::common::RegisterFieldBool<4, 1, 0, Doepint, common::RW> {
            crate::common::RegisterFieldBool::<4, 1, 0, Doepint, common::RW>::from_register(self, 0)
        }

        #[doc = "Back-to-back SETUP packets\n                received"]
        #[inline(always)]
        pub fn b2bstup(self) -> crate::common::RegisterFieldBool<6, 1, 0, Doepint, common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, Doepint, common::RW>::from_register(self, 0)
        }

        #[doc = "NYET interrupt"]
        #[inline(always)]
        pub fn nyet(self) -> crate::common::RegisterFieldBool<14, 1, 0, Doepint, common::RW> {
            crate::common::RegisterFieldBool::<14, 1, 0, Doepint, common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl crate::common::ResetValue<Doepint> for DoepintT {
        #[inline(always)]
        fn reset_value(&self) -> Doepint {
            Doepint::new(128)
        }
    }

    #[doc = "Transfer size"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Doeptsiz {
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
    pub struct DoeptsizT;
    unsafe impl crate::common::AsPtr for DoeptsizT {}
    impl crate::common::Reg<Doeptsiz> for DoeptsizT {}

    unsafe impl crate::common::Read<Doeptsiz> for DoeptsizT {}
    unsafe impl crate::common::Write<Doeptsiz> for DoeptsizT {}
    impl Doeptsiz {
        #[doc = "Transfer size"]
        #[inline(always)]
        pub fn xfrsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Doeptsiz, common::RW> {
            crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Doeptsiz,common::RW>::from_register(self,0)
        }

        #[doc = "Packet count"]
        #[inline(always)]
        pub fn pktcnt(self) -> crate::common::RegisterFieldBool<19, 1, 0, Doeptsiz, common::RW> {
            crate::common::RegisterFieldBool::<19, 1, 0, Doeptsiz, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "SETUP packet count"]
        #[inline(always)]
        pub fn stupcnt(
            self,
        ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, u8, Doeptsiz, common::RW> {
            crate::common::RegisterField::<29,0x3,1,0,u8,u8,Doeptsiz,common::RW>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Doeptsiz> for DoeptsizT {
        #[inline(always)]
        fn reset_value(&self) -> Doeptsiz {
            Doeptsiz::new(0)
        }
    }

    #[doc = "DMA address"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepdma {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Doepdma {
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
    pub struct DoepdmaT;
    unsafe impl crate::common::AsPtr for DoepdmaT {}
    impl crate::common::Reg<Doepdma> for DoepdmaT {}

    unsafe impl crate::common::Read<Doepdma> for DoepdmaT {}
    unsafe impl crate::common::Write<Doepdma> for DoepdmaT {}
    impl Doepdma {
        #[doc = "DMA address"]
        #[inline(always)]
        pub fn dmaaddr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Doepdma, common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Doepdma,common::RW>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Doepdma> for DoepdmaT {
        #[inline(always)]
        fn reset_value(&self) -> Doepdma {
            Doepdma::new(0)
        }
    }
}
