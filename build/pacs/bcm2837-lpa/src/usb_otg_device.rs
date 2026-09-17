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
    pub const fn dcfg(&self) -> &'static crate::common::Reg<self::Dcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "OTG_HS device control register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &'static crate::common::Reg<self::Dctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "OTG_HS device status register"]
    #[inline(always)]
    pub const fn dsts(&self) -> &'static crate::common::Reg<self::Dsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint common interrupt\n          mask register"]
    #[inline(always)]
    pub const fn diepmsk(
        &self,
    ) -> &'static crate::common::Reg<self::Diepmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Diepmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "OTG_HS device OUT endpoint common interrupt\n          mask register"]
    #[inline(always)]
    pub const fn doepmsk(
        &self,
    ) -> &'static crate::common::Reg<self::Doepmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Doepmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "OTG_HS device all endpoints interrupt\n          register"]
    #[inline(always)]
    pub const fn daint(&self) -> &'static crate::common::Reg<self::Daint_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Daint_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "OTG_HS all endpoints interrupt mask\n          register"]
    #[inline(always)]
    pub const fn daintmsk(
        &self,
    ) -> &'static crate::common::Reg<self::Daintmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Daintmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "OTG_HS device VBUS discharge time\n          register"]
    #[inline(always)]
    pub const fn dvbusdis(
        &self,
    ) -> &'static crate::common::Reg<self::Dvbusdis_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dvbusdis_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "OTG_HS device VBUS pulsing time\n          register"]
    #[inline(always)]
    pub const fn dvbuspulse(
        &self,
    ) -> &'static crate::common::Reg<self::Dvbuspulse_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dvbuspulse_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "OTG_HS Device threshold control\n          register"]
    #[inline(always)]
    pub const fn dthrctl(
        &self,
    ) -> &'static crate::common::Reg<self::Dthrctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dthrctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "OTG_HS device IN endpoint FIFO empty\n          interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(
        &self,
    ) -> &'static crate::common::Reg<self::Diepempmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Diepempmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "OTG_HS device each endpoint interrupt\n          register"]
    #[inline(always)]
    pub const fn deachint(
        &self,
    ) -> &'static crate::common::Reg<self::Deachint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Deachint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "OTG_HS device each endpoint interrupt\n          register mask"]
    #[inline(always)]
    pub const fn deachintmsk(
        &self,
    ) -> &'static crate::common::Reg<self::Deachintmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Deachintmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "OTG_HS device each in endpoint-1 interrupt\n          register"]
    #[inline(always)]
    pub const fn diepeachmsk1(
        &self,
    ) -> &'static crate::common::Reg<self::Diepeachmsk1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Diepeachmsk1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "OTG_HS device each OUT endpoint-1 interrupt\n          register"]
    #[inline(always)]
    pub const fn doepeachmsk1(
        &self,
    ) -> &'static crate::common::Reg<self::Doepeachmsk1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Doepeachmsk1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "IN Endpoint %s"]
    #[inline(always)]
    pub fn in_endpoint(
        self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::usb_otg_device::_InEndpoint, 12, 0x20>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[doc = "OUT Endpoint %s"]
    #[inline(always)]
    pub fn out_endpoint(
        self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::usb_otg_device::_OutEndpoint, 12, 0x20>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x300usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcfg_SPEC;
impl crate::sealed::RegSpec for Dcfg_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device configuration\n          register"]
pub type Dcfg = crate::RegValueT<Dcfg_SPEC>;

impl NoBitfieldReg<Dcfg_SPEC> for Dcfg {}
impl ::core::default::Default for Dcfg {
    #[inline(always)]
    fn default() -> Dcfg {
        <crate::RegValueT<Dcfg_SPEC> as RegisterValue<_>>::new(35651584)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dctl_SPEC;
impl crate::sealed::RegSpec for Dctl_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device control register"]
pub type Dctl = crate::RegValueT<Dctl_SPEC>;

impl Dctl {
    #[doc = "Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Soft disconnect"]
    #[inline(always)]
    pub fn sdis(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Test control"]
    #[inline(always)]
    pub fn tctl(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, Dctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,Dctl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(self) -> crate::common::RegisterFieldBool<7, 1, 0, Dctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(self) -> crate::common::RegisterFieldBool<9, 1, 0, Dctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Dctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(self) -> crate::common::RegisterFieldBool<10, 1, 0, Dctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Dctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Dctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dctl {
    #[inline(always)]
    fn default() -> Dctl {
        <crate::RegValueT<Dctl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsts_SPEC;
impl crate::sealed::RegSpec for Dsts_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device status register"]
pub type Dsts = crate::RegValueT<Dsts_SPEC>;

impl NoBitfieldReg<Dsts_SPEC> for Dsts {}
impl ::core::default::Default for Dsts {
    #[inline(always)]
    fn default() -> Dsts {
        <crate::RegValueT<Dsts_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepmsk_SPEC;
impl crate::sealed::RegSpec for Diepmsk_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint common interrupt\n          mask register"]
pub type Diepmsk = crate::RegValueT<Diepmsk_SPEC>;

impl NoBitfieldReg<Diepmsk_SPEC> for Diepmsk {}
impl ::core::default::Default for Diepmsk {
    #[inline(always)]
    fn default() -> Diepmsk {
        <crate::RegValueT<Diepmsk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doepmsk_SPEC;
impl crate::sealed::RegSpec for Doepmsk_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device OUT endpoint common interrupt\n          mask register"]
pub type Doepmsk = crate::RegValueT<Doepmsk_SPEC>;

impl NoBitfieldReg<Doepmsk_SPEC> for Doepmsk {}
impl ::core::default::Default for Doepmsk {
    #[inline(always)]
    fn default() -> Doepmsk {
        <crate::RegValueT<Doepmsk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daint_SPEC;
impl crate::sealed::RegSpec for Daint_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device all endpoints interrupt\n          register"]
pub type Daint = crate::RegValueT<Daint_SPEC>;

impl NoBitfieldReg<Daint_SPEC> for Daint {}
impl ::core::default::Default for Daint {
    #[inline(always)]
    fn default() -> Daint {
        <crate::RegValueT<Daint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daintmsk_SPEC;
impl crate::sealed::RegSpec for Daintmsk_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS all endpoints interrupt mask\n          register"]
pub type Daintmsk = crate::RegValueT<Daintmsk_SPEC>;

impl NoBitfieldReg<Daintmsk_SPEC> for Daintmsk {}
impl ::core::default::Default for Daintmsk {
    #[inline(always)]
    fn default() -> Daintmsk {
        <crate::RegValueT<Daintmsk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvbusdis_SPEC;
impl crate::sealed::RegSpec for Dvbusdis_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device VBUS discharge time\n          register"]
pub type Dvbusdis = crate::RegValueT<Dvbusdis_SPEC>;

impl NoBitfieldReg<Dvbusdis_SPEC> for Dvbusdis {}
impl ::core::default::Default for Dvbusdis {
    #[inline(always)]
    fn default() -> Dvbusdis {
        <crate::RegValueT<Dvbusdis_SPEC> as RegisterValue<_>>::new(6103)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvbuspulse_SPEC;
impl crate::sealed::RegSpec for Dvbuspulse_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device VBUS pulsing time\n          register"]
pub type Dvbuspulse = crate::RegValueT<Dvbuspulse_SPEC>;

impl NoBitfieldReg<Dvbuspulse_SPEC> for Dvbuspulse {}
impl ::core::default::Default for Dvbuspulse {
    #[inline(always)]
    fn default() -> Dvbuspulse {
        <crate::RegValueT<Dvbuspulse_SPEC> as RegisterValue<_>>::new(1464)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dthrctl_SPEC;
impl crate::sealed::RegSpec for Dthrctl_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS Device threshold control\n          register"]
pub type Dthrctl = crate::RegValueT<Dthrctl_SPEC>;

impl NoBitfieldReg<Dthrctl_SPEC> for Dthrctl {}
impl ::core::default::Default for Dthrctl {
    #[inline(always)]
    fn default() -> Dthrctl {
        <crate::RegValueT<Dthrctl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepempmsk_SPEC;
impl crate::sealed::RegSpec for Diepempmsk_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device IN endpoint FIFO empty\n          interrupt mask register"]
pub type Diepempmsk = crate::RegValueT<Diepempmsk_SPEC>;

impl NoBitfieldReg<Diepempmsk_SPEC> for Diepempmsk {}
impl ::core::default::Default for Diepempmsk {
    #[inline(always)]
    fn default() -> Diepempmsk {
        <crate::RegValueT<Diepempmsk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deachint_SPEC;
impl crate::sealed::RegSpec for Deachint_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device each endpoint interrupt\n          register"]
pub type Deachint = crate::RegValueT<Deachint_SPEC>;

impl NoBitfieldReg<Deachint_SPEC> for Deachint {}
impl ::core::default::Default for Deachint {
    #[inline(always)]
    fn default() -> Deachint {
        <crate::RegValueT<Deachint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deachintmsk_SPEC;
impl crate::sealed::RegSpec for Deachintmsk_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device each endpoint interrupt\n          register mask"]
pub type Deachintmsk = crate::RegValueT<Deachintmsk_SPEC>;

impl NoBitfieldReg<Deachintmsk_SPEC> for Deachintmsk {}
impl ::core::default::Default for Deachintmsk {
    #[inline(always)]
    fn default() -> Deachintmsk {
        <crate::RegValueT<Deachintmsk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diepeachmsk1_SPEC;
impl crate::sealed::RegSpec for Diepeachmsk1_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device each in endpoint-1 interrupt\n          register"]
pub type Diepeachmsk1 = crate::RegValueT<Diepeachmsk1_SPEC>;

impl NoBitfieldReg<Diepeachmsk1_SPEC> for Diepeachmsk1 {}
impl ::core::default::Default for Diepeachmsk1 {
    #[inline(always)]
    fn default() -> Diepeachmsk1 {
        <crate::RegValueT<Diepeachmsk1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Doepeachmsk1_SPEC;
impl crate::sealed::RegSpec for Doepeachmsk1_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS device each OUT endpoint-1 interrupt\n          register"]
pub type Doepeachmsk1 = crate::RegValueT<Doepeachmsk1_SPEC>;

impl NoBitfieldReg<Doepeachmsk1_SPEC> for Doepeachmsk1 {}
impl ::core::default::Default for Doepeachmsk1 {
    #[inline(always)]
    fn default() -> Doepeachmsk1 {
        <crate::RegValueT<Doepeachmsk1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "IN Endpoint %s"]
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
    pub const fn diepctl0(
        &self,
    ) -> &'static crate::common::Reg<in_endpoint::Diepctl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<in_endpoint::Diepctl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn diepint(
        &self,
    ) -> &'static crate::common::Reg<in_endpoint::Diepint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<in_endpoint::Diepint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Transfer size"]
    #[inline(always)]
    pub const fn dieptsiz(
        &self,
    ) -> &'static crate::common::Reg<in_endpoint::Dieptsiz_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<in_endpoint::Dieptsiz_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DMA address"]
    #[inline(always)]
    pub const fn diepdma(
        &self,
    ) -> &'static crate::common::Reg<in_endpoint::Diepdma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<in_endpoint::Diepdma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Transmit FIFO status"]
    #[inline(always)]
    pub const fn dtxfsts(
        &self,
    ) -> &'static crate::common::Reg<in_endpoint::Dtxfsts_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<in_endpoint::Dtxfsts_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}

unsafe impl AsPtr for _InEndpoint {
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
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl0_SPEC;
    impl crate::sealed::RegSpec for Diepctl0_SPEC {
        type DataType = u32;
    }

    #[doc = "Control"]
    pub type Diepctl0 = crate::RegValueT<Diepctl0_SPEC>;

    impl Diepctl0 {
        #[doc = "Maximum packet size"]
        #[inline(always)]
        pub fn mpsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Diepctl0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Diepctl0_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "USB active endpoint"]
        #[inline(always)]
        pub fn usbaep(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Diepctl0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,Diepctl0_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Even/odd frame"]
        #[inline(always)]
        pub fn eonum_dpid(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, Diepctl0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,Diepctl0_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "NAK status"]
        #[inline(always)]
        pub fn naksts(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, Diepctl0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,Diepctl0_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "Endpoint type"]
        #[inline(always)]
        pub fn eptyp(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, Diepctl0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3,1,0,u8,u8,Diepctl0_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "STALL handshake"]
        #[inline(always)]
        pub fn stall(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, Diepctl0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,Diepctl0_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "TxFIFO number"]
        #[inline(always)]
        pub fn txfnum(
            self,
        ) -> crate::common::RegisterField<22, 0xf, 1, 0, u8, u8, Diepctl0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0xf,1,0,u8,u8,Diepctl0_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Clear NAK"]
        #[inline(always)]
        pub fn cnak(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, Diepctl0_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,Diepctl0_SPEC,crate::common::W>::from_register(self,0)
        }

        #[doc = "Set NAK"]
        #[inline(always)]
        pub fn snak(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, Diepctl0_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<27,1,0,Diepctl0_SPEC,crate::common::W>::from_register(self,0)
        }

        #[doc = "Set DATA0 PID"]
        #[inline(always)]
        pub fn sd0pid_sevnfrm(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, Diepctl0_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28,1,0,Diepctl0_SPEC,crate::common::W>::from_register(self,0)
        }

        #[doc = "Set odd frame"]
        #[inline(always)]
        pub fn soddfrm(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, Diepctl0_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<29,1,0,Diepctl0_SPEC,crate::common::W>::from_register(self,0)
        }

        #[doc = "Endpoint disable"]
        #[inline(always)]
        pub fn epdis(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, Diepctl0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,Diepctl0_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Endpoint enable"]
        #[inline(always)]
        pub fn epena(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Diepctl0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,Diepctl0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Diepctl0 {
        #[inline(always)]
        fn default() -> Diepctl0 {
            <crate::RegValueT<Diepctl0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint_SPEC;
    impl crate::sealed::RegSpec for Diepint_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt"]
    pub type Diepint = crate::RegValueT<Diepint_SPEC>;

    impl Diepint {
        #[doc = "Transfer completed\n                interrupt"]
        #[inline(always)]
        pub fn xfrc(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Endpoint disabled\n                interrupt"]
        #[inline(always)]
        pub fn epdisd(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Timeout condition"]
        #[inline(always)]
        pub fn toc(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "IN token received when TxFIFO is\n                empty"]
        #[inline(always)]
        pub fn ittxfe(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "IN endpoint NAK effective"]
        #[inline(always)]
        pub fn inepne(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Transmit FIFO empty"]
        #[inline(always)]
        pub fn txfe(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, Diepint_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,Diepint_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "Transmit Fifo Underrun"]
        #[inline(always)]
        pub fn txfifoudrn(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Buffer not available\n                interrupt"]
        #[inline(always)]
        pub fn bna(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Packet dropped status"]
        #[inline(always)]
        pub fn pktdrpsts(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Babble error interrupt"]
        #[inline(always)]
        pub fn berr(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "NAK interrupt"]
        #[inline(always)]
        pub fn nak(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, Diepint_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,Diepint_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Diepint {
        #[inline(always)]
        fn default() -> Diepint {
            <crate::RegValueT<Diepint_SPEC> as RegisterValue<_>>::new(128)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz_SPEC;
    impl crate::sealed::RegSpec for Dieptsiz_SPEC {
        type DataType = u32;
    }

    #[doc = "Transfer size"]
    pub type Dieptsiz = crate::RegValueT<Dieptsiz_SPEC>;

    impl NoBitfieldReg<Dieptsiz_SPEC> for Dieptsiz {}
    impl ::core::default::Default for Dieptsiz {
        #[inline(always)]
        fn default() -> Dieptsiz {
            <crate::RegValueT<Dieptsiz_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepdma_SPEC;
    impl crate::sealed::RegSpec for Diepdma_SPEC {
        type DataType = u32;
    }

    #[doc = "DMA address"]
    pub type Diepdma = crate::RegValueT<Diepdma_SPEC>;

    impl NoBitfieldReg<Diepdma_SPEC> for Diepdma {}
    impl ::core::default::Default for Diepdma {
        #[inline(always)]
        fn default() -> Diepdma {
            <crate::RegValueT<Diepdma_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts_SPEC;
    impl crate::sealed::RegSpec for Dtxfsts_SPEC {
        type DataType = u32;
    }

    #[doc = "Transmit FIFO status"]
    pub type Dtxfsts = crate::RegValueT<Dtxfsts_SPEC>;

    impl NoBitfieldReg<Dtxfsts_SPEC> for Dtxfsts {}
    impl ::core::default::Default for Dtxfsts {
        #[inline(always)]
        fn default() -> Dtxfsts {
            <crate::RegValueT<Dtxfsts_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}

#[doc = "OUT Endpoint %s"]
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
    pub const fn doepctl(
        &self,
    ) -> &'static crate::common::Reg<out_endpoint::Doepctl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<out_endpoint::Doepctl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn doepint(
        &self,
    ) -> &'static crate::common::Reg<out_endpoint::Doepint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<out_endpoint::Doepint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Transfer size"]
    #[inline(always)]
    pub const fn doeptsiz(
        &self,
    ) -> &'static crate::common::Reg<out_endpoint::Doeptsiz_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<out_endpoint::Doeptsiz_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DMA address"]
    #[inline(always)]
    pub const fn doepdma(
        &self,
    ) -> &'static crate::common::Reg<out_endpoint::Doepdma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<out_endpoint::Doepdma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}

unsafe impl AsPtr for _OutEndpoint {
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
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl_SPEC;
    impl crate::sealed::RegSpec for Doepctl_SPEC {
        type DataType = u32;
    }

    #[doc = "Control"]
    pub type Doepctl = crate::RegValueT<Doepctl_SPEC>;

    impl Doepctl {
        #[doc = "Maximum packet size"]
        #[inline(always)]
        pub fn mpsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Doepctl_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8,u8,Doepctl_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "USB active endpoint"]
        #[inline(always)]
        pub fn usbaep(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Doepctl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,Doepctl_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "NAK status"]
        #[inline(always)]
        pub fn naksts(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, Doepctl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,Doepctl_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "Endpoint type"]
        #[inline(always)]
        pub fn eptyp(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, Doepctl_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<18,0x3,1,0,u8,u8,Doepctl_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "Snoop mode"]
        #[inline(always)]
        pub fn snpm(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, Doepctl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,Doepctl_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "STALL handshake"]
        #[inline(always)]
        pub fn stall(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, Doepctl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,Doepctl_SPEC,crate::common::RW>::from_register(self,0)
        }

        #[doc = "Clear NAK"]
        #[inline(always)]
        pub fn cnak(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, Doepctl_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,Doepctl_SPEC,crate::common::W>::from_register(self,0)
        }

        #[doc = "Set NAK"]
        #[inline(always)]
        pub fn snak(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, Doepctl_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<27,1,0,Doepctl_SPEC,crate::common::W>::from_register(self,0)
        }

        #[doc = "Endpoint disable"]
        #[inline(always)]
        pub fn epdis(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, Doepctl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,Doepctl_SPEC,crate::common::R>::from_register(self,0)
        }

        #[doc = "Endpoint enable"]
        #[inline(always)]
        pub fn epena(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Doepctl_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,Doepctl_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Doepctl {
        #[inline(always)]
        fn default() -> Doepctl {
            <crate::RegValueT<Doepctl_SPEC> as RegisterValue<_>>::new(32768)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint_SPEC;
    impl crate::sealed::RegSpec for Doepint_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt"]
    pub type Doepint = crate::RegValueT<Doepint_SPEC>;

    impl NoBitfieldReg<Doepint_SPEC> for Doepint {}
    impl ::core::default::Default for Doepint {
        #[inline(always)]
        fn default() -> Doepint {
            <crate::RegValueT<Doepint_SPEC> as RegisterValue<_>>::new(128)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz_SPEC;
    impl crate::sealed::RegSpec for Doeptsiz_SPEC {
        type DataType = u32;
    }

    #[doc = "Transfer size"]
    pub type Doeptsiz = crate::RegValueT<Doeptsiz_SPEC>;

    impl NoBitfieldReg<Doeptsiz_SPEC> for Doeptsiz {}
    impl ::core::default::Default for Doeptsiz {
        #[inline(always)]
        fn default() -> Doeptsiz {
            <crate::RegValueT<Doeptsiz_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepdma_SPEC;
    impl crate::sealed::RegSpec for Doepdma_SPEC {
        type DataType = u32;
    }

    #[doc = "DMA address"]
    pub type Doepdma = crate::RegValueT<Doepdma_SPEC>;

    impl NoBitfieldReg<Doepdma_SPEC> for Doepdma {}
    impl ::core::default::Default for Doepdma {
        #[inline(always)]
        fn default() -> Doepdma {
            <crate::RegValueT<Doepdma_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
