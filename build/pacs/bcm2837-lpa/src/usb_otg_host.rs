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
// Generated from SVD A, with svd2pac 0.8.0 on Fri, 25 Sep 2026 21:30:44 +0000

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
unsafe impl ::core::marker::Send for super::UsbOtgHost {}
unsafe impl ::core::marker::Sync for super::UsbOtgHost {}
impl super::UsbOtgHost {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "OTG_HS host configuration\n          register"]
    #[inline(always)]
    pub fn hcfg(&self) -> &'static self::HcfgT {
        unsafe { self::HcfgT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "OTG_HS Host frame interval\n          register"]
    #[inline(always)]
    pub fn hfir(&self) -> &'static self::HfirT {
        unsafe { self::HfirT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "OTG_HS host frame number/frame time\n          remaining register"]
    #[inline(always)]
    pub fn hfnum(&self) -> &'static self::HfnumT {
        unsafe { self::HfnumT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Host periodic transmit FIFO/queue\n          status register"]
    #[inline(always)]
    pub fn hptxsts(&self) -> &'static self::HptxstsT {
        unsafe { self::HptxstsT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "OTG_HS Host all channels interrupt\n          register"]
    #[inline(always)]
    pub fn haint(&self) -> &'static self::HaintT {
        unsafe { self::HaintT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "OTG_HS host all channels interrupt mask\n          register"]
    #[inline(always)]
    pub fn haintmsk(&self) -> &'static self::HaintmskT {
        unsafe { self::HaintmskT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "OTG_HS host port control and status\n          register"]
    #[inline(always)]
    pub fn hprt(&self) -> &'static self::HprtT {
        unsafe { self::HprtT::from_ptr(self._svd2pac_as_ptr().add(64usize)) }
    }

    #[doc = "Host channel %s"]
    #[inline(always)]
    pub fn host_channel(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::usb_otg_host::_HostChannel, 12, 0x20>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
}

#[doc = "OTG_HS host configuration\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcfg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Hcfg {
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
pub struct HcfgT;
unsafe impl crate::common::AsPtr for HcfgT {}
impl crate::common::Reg<Hcfg> for HcfgT {}

unsafe impl crate::common::Read<Hcfg> for HcfgT {}
unsafe impl crate::common::Write<Hcfg> for HcfgT {}
impl Hcfg {
    #[doc = "FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(self) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Hcfg, common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, u8, Hcfg, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(self) -> crate::common::RegisterFieldBool<2, 1, 0, Hcfg, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Hcfg, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Hcfg> for HcfgT {
    #[inline(always)]
    fn reset_value(&self) -> Hcfg {
        Hcfg::new(0)
    }
}

#[doc = "OTG_HS Host frame interval\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfir {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Hfir {
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
pub struct HfirT;
unsafe impl crate::common::AsPtr for HfirT {}
impl crate::common::Reg<Hfir> for HfirT {}

unsafe impl crate::common::Read<Hfir> for HfirT {}
unsafe impl crate::common::Write<Hfir> for HfirT {}
impl Hfir {
    #[doc = "Frame interval"]
    #[inline(always)]
    pub fn frivl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Hfir, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Hfir, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Hfir> for HfirT {
    #[inline(always)]
    fn reset_value(&self) -> Hfir {
        Hfir::new(60000)
    }
}

#[doc = "OTG_HS host frame number/frame time\n          remaining register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfnum {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Hfnum {
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
pub struct HfnumT;
unsafe impl crate::common::AsPtr for HfnumT {}
impl crate::common::Reg<Hfnum> for HfnumT {}

unsafe impl crate::common::Read<Hfnum> for HfnumT {}
impl Hfnum {
    #[doc = "Frame number"]
    #[inline(always)]
    pub fn frnum(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Hfnum, common::R> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Hfnum, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Hfnum, common::R> {
        crate::common::RegisterField::<16, 0xffff, 1, 0, u16, u16, Hfnum, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Hfnum> for HfnumT {
    #[inline(always)]
    fn reset_value(&self) -> Hfnum {
        Hfnum::new(16383)
    }
}

#[doc = "Host periodic transmit FIFO/queue\n          status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptxsts {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Hptxsts {
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
pub struct HptxstsT;
unsafe impl crate::common::AsPtr for HptxstsT {}
impl crate::common::Reg<Hptxsts> for HptxstsT {}

unsafe impl crate::common::Read<Hptxsts> for HptxstsT {}
unsafe impl crate::common::Write<Hptxsts> for HptxstsT {}
impl Hptxsts {
    #[doc = "Periodic transmit data FIFO space\n              available"]
    #[inline(always)]
    pub fn ptxfsavl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Hptxsts, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Hptxsts,common::RW>::from_register(self,0)
    }

    #[doc = "Periodic transmit request queue space\n              available"]
    #[inline(always)]
    pub fn ptxqsav(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Hptxsts, common::R> {
        crate::common::RegisterField::<16, 0xff, 1, 0, u8, u8, Hptxsts, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Top of the periodic transmit request\n              queue"]
    #[inline(always)]
    pub fn ptxqtop(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Hptxsts, common::R> {
        crate::common::RegisterField::<24, 0xff, 1, 0, u8, u8, Hptxsts, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Hptxsts> for HptxstsT {
    #[inline(always)]
    fn reset_value(&self) -> Hptxsts {
        Hptxsts::new(524544)
    }
}

#[doc = "OTG_HS Host all channels interrupt\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haint {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Haint {
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
pub struct HaintT;
unsafe impl crate::common::AsPtr for HaintT {}
impl crate::common::Reg<Haint> for HaintT {}

unsafe impl crate::common::Read<Haint> for HaintT {}
impl Haint {
    #[doc = "Channel interrupts"]
    #[inline(always)]
    pub fn haint(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Haint, common::R> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Haint, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Haint> for HaintT {
    #[inline(always)]
    fn reset_value(&self) -> Haint {
        Haint::new(0)
    }
}

#[doc = "OTG_HS host all channels interrupt mask\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haintmsk {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Haintmsk {
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
pub struct HaintmskT;
unsafe impl crate::common::AsPtr for HaintmskT {}
impl crate::common::Reg<Haintmsk> for HaintmskT {}

unsafe impl crate::common::Read<Haintmsk> for HaintmskT {}
unsafe impl crate::common::Write<Haintmsk> for HaintmskT {}
impl Haintmsk {
    #[doc = "Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Haintmsk, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Haintmsk,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Haintmsk> for HaintmskT {
    #[inline(always)]
    fn reset_value(&self) -> Haintmsk {
        Haintmsk::new(0)
    }
}

#[doc = "OTG_HS host port control and status\n          register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hprt {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Hprt {
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
pub struct HprtT;
unsafe impl crate::common::AsPtr for HprtT {}
impl crate::common::Reg<Hprt> for HprtT {}

unsafe impl crate::common::Read<Hprt> for HprtT {}
unsafe impl crate::common::Write<Hprt> for HprtT {}
impl Hprt {
    #[doc = "Port connect status"]
    #[inline(always)]
    pub fn pcsts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Hprt, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Hprt, common::R>::from_register(self, 0)
    }

    #[doc = "Port connect detected"]
    #[inline(always)]
    pub fn pcdet(self) -> crate::common::RegisterFieldBool<1, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port enable"]
    #[inline(always)]
    pub fn pena(self) -> crate::common::RegisterFieldBool<2, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port enable/disable change"]
    #[inline(always)]
    pub fn penchng(self) -> crate::common::RegisterFieldBool<3, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port overcurrent active"]
    #[inline(always)]
    pub fn poca(self) -> crate::common::RegisterFieldBool<4, 1, 0, Hprt, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Hprt, common::R>::from_register(self, 0)
    }

    #[doc = "Port overcurrent change"]
    #[inline(always)]
    pub fn pocchng(self) -> crate::common::RegisterFieldBool<5, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port resume"]
    #[inline(always)]
    pub fn pres(self) -> crate::common::RegisterFieldBool<6, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port suspend"]
    #[inline(always)]
    pub fn psusp(self) -> crate::common::RegisterFieldBool<7, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port reset"]
    #[inline(always)]
    pub fn prst(self) -> crate::common::RegisterFieldBool<8, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port line status"]
    #[inline(always)]
    pub fn plsts(self) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, Hprt, common::R> {
        crate::common::RegisterField::<10, 0x3, 1, 0, u8, u8, Hprt, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Port power"]
    #[inline(always)]
    pub fn ppwr(self) -> crate::common::RegisterFieldBool<12, 1, 0, Hprt, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Hprt, common::RW>::from_register(self, 0)
    }

    #[doc = "Port test control"]
    #[inline(always)]
    pub fn ptctl(self) -> crate::common::RegisterField<13, 0xf, 1, 0, u8, u8, Hprt, common::RW> {
        crate::common::RegisterField::<13, 0xf, 1, 0, u8, u8, Hprt, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port speed"]
    #[inline(always)]
    pub fn pspd(self) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, Hprt, common::R> {
        crate::common::RegisterField::<17, 0x3, 1, 0, u8, u8, Hprt, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Hprt> for HprtT {
    #[inline(always)]
    fn reset_value(&self) -> Hprt {
        Hprt::new(0)
    }
}

#[doc(hidden)]
#[non_exhaustive]
pub struct _HostChannel;

#[doc = "Host channel %s"]
pub type HostChannel = &'static _HostChannel;

unsafe impl ::core::marker::Sync for _HostChannel {}
impl _HostChannel {
    #[inline(always)]
    pub(crate) const unsafe fn _svd2pac_from_ptr(ptr: *mut u8) -> &'static Self {
        &*(ptr as *const _)
    }

    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self as *const Self as *mut u8
    }

    #[doc = "Characteristics register"]
    #[inline(always)]
    pub fn hcchar(&self) -> &'static host_channel::HccharT {
        unsafe { host_channel::HccharT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Split control register"]
    #[inline(always)]
    pub fn hcsplt(&self) -> &'static host_channel::HcspltT {
        unsafe { host_channel::HcspltT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Interrupt register"]
    #[inline(always)]
    pub fn hcint(&self) -> &'static host_channel::HcintT {
        unsafe { host_channel::HcintT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub fn hcintmsk(&self) -> &'static host_channel::HcintmskT {
        unsafe { host_channel::HcintmskT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "Transfer size"]
    #[inline(always)]
    pub fn hctsiz(&self) -> &'static host_channel::HctsizT {
        unsafe { host_channel::HctsizT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "DMA address"]
    #[inline(always)]
    pub fn hcdma(&self) -> &'static host_channel::HcdmaT {
        unsafe { host_channel::HcdmaT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }
}

unsafe impl crate::common::AsPtr for _HostChannel {
    fn as_ptr(&self) -> *mut u8 {
        self._svd2pac_as_ptr()
    }

    #[inline(always)]
    unsafe fn from_ptr(ptr: *mut u8) -> &'static Self {
        Self::_svd2pac_from_ptr(ptr)
    }
}

pub mod host_channel {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _, ResetValue as _,
        Write as _,
    };

    #[doc = "Characteristics register"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Hcchar {
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
    pub struct HccharT;
    unsafe impl crate::common::AsPtr for HccharT {}
    impl crate::common::Reg<Hcchar> for HccharT {}

    unsafe impl crate::common::Read<Hcchar> for HccharT {}
    unsafe impl crate::common::Write<Hcchar> for HccharT {}
    impl Hcchar {
        #[doc = "Maximum packet size"]
        #[inline(always)]
        pub fn mpsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Hcchar, common::RW> {
            crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Hcchar,common::RW>::from_register(self,0)
        }

        #[doc = "Endpoint number"]
        #[inline(always)]
        pub fn epnum(
            self,
        ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, u8, Hcchar, common::RW> {
            crate::common::RegisterField::<11, 0xf, 1, 0, u8, u8, Hcchar, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Endpoint direction"]
        #[inline(always)]
        pub fn epdir(self) -> crate::common::RegisterFieldBool<15, 1, 0, Hcchar, common::RW> {
            crate::common::RegisterFieldBool::<15, 1, 0, Hcchar, common::RW>::from_register(self, 0)
        }

        #[doc = "Low-speed device"]
        #[inline(always)]
        pub fn lsdev(self) -> crate::common::RegisterFieldBool<17, 1, 0, Hcchar, common::RW> {
            crate::common::RegisterFieldBool::<17, 1, 0, Hcchar, common::RW>::from_register(self, 0)
        }

        #[doc = "Endpoint type"]
        #[inline(always)]
        pub fn eptyp(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, u8, Hcchar, common::RW> {
            crate::common::RegisterField::<18, 0x3, 1, 0, u8, u8, Hcchar, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Multi Count (MC) / Error Count\n                (EC)"]
        #[inline(always)]
        pub fn mc(self) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, u8, Hcchar, common::RW> {
            crate::common::RegisterField::<20, 0x3, 1, 0, u8, u8, Hcchar, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Device address"]
        #[inline(always)]
        pub fn dad(
            self,
        ) -> crate::common::RegisterField<22, 0x7f, 1, 0, u8, u8, Hcchar, common::RW> {
            crate::common::RegisterField::<22,0x7f,1,0,u8,u8,Hcchar,common::RW>::from_register(self,0)
        }

        #[doc = "Odd frame"]
        #[inline(always)]
        pub fn oddfrm(self) -> crate::common::RegisterFieldBool<29, 1, 0, Hcchar, common::RW> {
            crate::common::RegisterFieldBool::<29, 1, 0, Hcchar, common::RW>::from_register(self, 0)
        }

        #[doc = "Channel disable"]
        #[inline(always)]
        pub fn chdis(self) -> crate::common::RegisterFieldBool<30, 1, 0, Hcchar, common::RW> {
            crate::common::RegisterFieldBool::<30, 1, 0, Hcchar, common::RW>::from_register(self, 0)
        }

        #[doc = "Channel enable"]
        #[inline(always)]
        pub fn chena(self) -> crate::common::RegisterFieldBool<31, 1, 0, Hcchar, common::RW> {
            crate::common::RegisterFieldBool::<31, 1, 0, Hcchar, common::RW>::from_register(self, 0)
        }
    }
    impl crate::common::ResetValue<Hcchar> for HccharT {
        #[inline(always)]
        fn reset_value(&self) -> Hcchar {
            Hcchar::new(0)
        }
    }

    #[doc = "Split control register"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Hcsplt {
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
    pub struct HcspltT;
    unsafe impl crate::common::AsPtr for HcspltT {}
    impl crate::common::Reg<Hcsplt> for HcspltT {}

    unsafe impl crate::common::Read<Hcsplt> for HcspltT {}
    unsafe impl crate::common::Write<Hcsplt> for HcspltT {}
    impl Hcsplt {
        #[doc = "Port address"]
        #[inline(always)]
        pub fn prtaddr(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Hcsplt, common::RW> {
            crate::common::RegisterField::<0, 0x7f, 1, 0, u8, u8, Hcsplt, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Hub address"]
        #[inline(always)]
        pub fn hubaddr(
            self,
        ) -> crate::common::RegisterField<7, 0x7f, 1, 0, u8, u8, Hcsplt, common::RW> {
            crate::common::RegisterField::<7, 0x7f, 1, 0, u8, u8, Hcsplt, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "XACTPOS"]
        #[inline(always)]
        pub fn xactpos(
            self,
        ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Hcsplt, common::RW> {
            crate::common::RegisterField::<14, 0x3, 1, 0, u8, u8, Hcsplt, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Do complete split"]
        #[inline(always)]
        pub fn complsplt(self) -> crate::common::RegisterFieldBool<16, 1, 0, Hcsplt, common::RW> {
            crate::common::RegisterFieldBool::<16, 1, 0, Hcsplt, common::RW>::from_register(self, 0)
        }

        #[doc = "Split enable"]
        #[inline(always)]
        pub fn spliten(self) -> crate::common::RegisterFieldBool<31, 1, 0, Hcsplt, common::RW> {
            crate::common::RegisterFieldBool::<31, 1, 0, Hcsplt, common::RW>::from_register(self, 0)
        }
    }
    impl crate::common::ResetValue<Hcsplt> for HcspltT {
        #[inline(always)]
        fn reset_value(&self) -> Hcsplt {
            Hcsplt::new(0)
        }
    }

    #[doc = "Interrupt register"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Hcint {
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
    pub struct HcintT;
    unsafe impl crate::common::AsPtr for HcintT {}
    impl crate::common::Reg<Hcint> for HcintT {}

    unsafe impl crate::common::Read<Hcint> for HcintT {}
    unsafe impl crate::common::Write<Hcint> for HcintT {}
    impl Hcint {
        #[doc = "Transfer completed"]
        #[inline(always)]
        pub fn xfrc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "Channel halted"]
        #[inline(always)]
        pub fn chh(self) -> crate::common::RegisterFieldBool<1, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "AHB error"]
        #[inline(always)]
        pub fn ahberr(self) -> crate::common::RegisterFieldBool<2, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "STALL response received\n                interrupt"]
        #[inline(always)]
        pub fn stall(self) -> crate::common::RegisterFieldBool<3, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "NAK response received\n                interrupt"]
        #[inline(always)]
        pub fn nak(self) -> crate::common::RegisterFieldBool<4, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<4, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "ACK response received/transmitted\n                interrupt"]
        #[inline(always)]
        pub fn ack(self) -> crate::common::RegisterFieldBool<5, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<5, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "Response received\n                interrupt"]
        #[inline(always)]
        pub fn nyet(self) -> crate::common::RegisterFieldBool<6, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "Transaction error"]
        #[inline(always)]
        pub fn txerr(self) -> crate::common::RegisterFieldBool<7, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<7, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "Babble error"]
        #[inline(always)]
        pub fn bberr(self) -> crate::common::RegisterFieldBool<8, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<8, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "Frame overrun"]
        #[inline(always)]
        pub fn frmor(self) -> crate::common::RegisterFieldBool<9, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<9, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }

        #[doc = "Data toggle error"]
        #[inline(always)]
        pub fn dterr(self) -> crate::common::RegisterFieldBool<10, 1, 0, Hcint, common::RW> {
            crate::common::RegisterFieldBool::<10, 1, 0, Hcint, common::RW>::from_register(self, 0)
        }
    }
    impl crate::common::ResetValue<Hcint> for HcintT {
        #[inline(always)]
        fn reset_value(&self) -> Hcint {
            Hcint::new(0)
        }
    }

    #[doc = "Interrupt mask"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Hcintmsk {
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
    pub struct HcintmskT;
    unsafe impl crate::common::AsPtr for HcintmskT {}
    impl crate::common::Reg<Hcintmsk> for HcintmskT {}

    unsafe impl crate::common::Read<Hcintmsk> for HcintmskT {}
    unsafe impl crate::common::Write<Hcintmsk> for HcintmskT {}
    impl Hcintmsk {
        #[doc = "Transfer completed mask"]
        #[inline(always)]
        pub fn xfrcm(self) -> crate::common::RegisterFieldBool<0, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Channel halted mask"]
        #[inline(always)]
        pub fn chhm(self) -> crate::common::RegisterFieldBool<1, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "AHB error"]
        #[inline(always)]
        pub fn ahberr(self) -> crate::common::RegisterFieldBool<2, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "STALL response received interrupt\n                mask"]
        #[inline(always)]
        pub fn stallm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "NAK response received interrupt\n                mask"]
        #[inline(always)]
        pub fn nakm(self) -> crate::common::RegisterFieldBool<4, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<4, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "ACK response received/transmitted\n                interrupt mask"]
        #[inline(always)]
        pub fn ackm(self) -> crate::common::RegisterFieldBool<5, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<5, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "response received interrupt\n                mask"]
        #[inline(always)]
        pub fn nyet(self) -> crate::common::RegisterFieldBool<6, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Transaction error mask"]
        #[inline(always)]
        pub fn txerrm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<7, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Babble error mask"]
        #[inline(always)]
        pub fn bberrm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<8, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Frame overrun mask"]
        #[inline(always)]
        pub fn frmorm(self) -> crate::common::RegisterFieldBool<9, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<9, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }

        #[doc = "Data toggle error mask"]
        #[inline(always)]
        pub fn dterrm(self) -> crate::common::RegisterFieldBool<10, 1, 0, Hcintmsk, common::RW> {
            crate::common::RegisterFieldBool::<10, 1, 0, Hcintmsk, common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl crate::common::ResetValue<Hcintmsk> for HcintmskT {
        #[inline(always)]
        fn reset_value(&self) -> Hcintmsk {
            Hcintmsk::new(0)
        }
    }

    #[doc = "Transfer size"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Hctsiz {
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
    pub struct HctsizT;
    unsafe impl crate::common::AsPtr for HctsizT {}
    impl crate::common::Reg<Hctsiz> for HctsizT {}

    unsafe impl crate::common::Read<Hctsiz> for HctsizT {}
    unsafe impl crate::common::Write<Hctsiz> for HctsizT {}
    impl Hctsiz {
        #[doc = "Transfer size"]
        #[inline(always)]
        pub fn xfrsiz(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, u32, Hctsiz, common::RW> {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32,u32,Hctsiz,common::RW>::from_register(self,0)
        }

        #[doc = "Packet count"]
        #[inline(always)]
        pub fn pktcnt(
            self,
        ) -> crate::common::RegisterField<19, 0x3ff, 1, 0, u16, u16, Hctsiz, common::RW> {
            crate::common::RegisterField::<19,0x3ff,1,0,u16,u16,Hctsiz,common::RW>::from_register(self,0)
        }

        #[doc = "Data PID"]
        #[inline(always)]
        pub fn dpid(
            self,
        ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, u8, Hctsiz, common::RW> {
            crate::common::RegisterField::<29, 0x3, 1, 0, u8, u8, Hctsiz, common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl crate::common::ResetValue<Hctsiz> for HctsizT {
        #[inline(always)]
        fn reset_value(&self) -> Hctsiz {
            Hctsiz::new(0)
        }
    }

    #[doc = "DMA address"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcdma {
        pub(crate) data: u32,
        pub(crate) mask: u32,
    }

    impl crate::common::RegisterValue for Hcdma {
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
    pub struct HcdmaT;
    unsafe impl crate::common::AsPtr for HcdmaT {}
    impl crate::common::Reg<Hcdma> for HcdmaT {}

    unsafe impl crate::common::Read<Hcdma> for HcdmaT {}
    unsafe impl crate::common::Write<Hcdma> for HcdmaT {}
    impl Hcdma {
        #[doc = "DMA address"]
        #[inline(always)]
        pub fn dmaaddr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Hcdma, common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Hcdma,common::RW>::from_register(self,0)
        }
    }
    impl crate::common::ResetValue<Hcdma> for HcdmaT {
        #[inline(always)]
        fn reset_value(&self) -> Hcdma {
            Hcdma::new(0)
        }
    }
}
