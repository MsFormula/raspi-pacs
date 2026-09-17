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
    pub const fn hcfg(&self) -> &'static crate::common::Reg<self::Hcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "OTG_HS Host frame interval\n          register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &'static crate::common::Reg<self::Hfir_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hfir_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "OTG_HS host frame number/frame time\n          remaining register"]
    #[inline(always)]
    pub const fn hfnum(&self) -> &'static crate::common::Reg<self::Hfnum_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Hfnum_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Host periodic transmit FIFO/queue\n          status register"]
    #[inline(always)]
    pub const fn hptxsts(
        &self,
    ) -> &'static crate::common::Reg<self::Hptxsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hptxsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "OTG_HS Host all channels interrupt\n          register"]
    #[inline(always)]
    pub const fn haint(&self) -> &'static crate::common::Reg<self::Haint_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Haint_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "OTG_HS host all channels interrupt mask\n          register"]
    #[inline(always)]
    pub const fn haintmsk(
        &self,
    ) -> &'static crate::common::Reg<self::Haintmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Haintmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "OTG_HS host port control and status\n          register"]
    #[inline(always)]
    pub const fn hprt(&self) -> &'static crate::common::Reg<self::Hprt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hprt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Host channel %s"]
    #[inline(always)]
    pub fn host_channel(
        self,
    ) -> &'static crate::common::ClusterRegisterArray<crate::usb_otg_host::_HostChannel, 12, 0x20>
    {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcfg_SPEC;
impl crate::sealed::RegSpec for Hcfg_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS host configuration\n          register"]
pub type Hcfg = crate::RegValueT<Hcfg_SPEC>;

impl Hcfg {
    #[doc = "FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Hcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Hcfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(self) -> crate::common::RegisterFieldBool<2, 1, 0, Hcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Hcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Hcfg {
    #[inline(always)]
    fn default() -> Hcfg {
        <crate::RegValueT<Hcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfir_SPEC;
impl crate::sealed::RegSpec for Hfir_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS Host frame interval\n          register"]
pub type Hfir = crate::RegValueT<Hfir_SPEC>;

impl NoBitfieldReg<Hfir_SPEC> for Hfir {}
impl ::core::default::Default for Hfir {
    #[inline(always)]
    fn default() -> Hfir {
        <crate::RegValueT<Hfir_SPEC> as RegisterValue<_>>::new(60000)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfnum_SPEC;
impl crate::sealed::RegSpec for Hfnum_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS host frame number/frame time\n          remaining register"]
pub type Hfnum = crate::RegValueT<Hfnum_SPEC>;

impl NoBitfieldReg<Hfnum_SPEC> for Hfnum {}
impl ::core::default::Default for Hfnum {
    #[inline(always)]
    fn default() -> Hfnum {
        <crate::RegValueT<Hfnum_SPEC> as RegisterValue<_>>::new(16383)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hptxsts_SPEC;
impl crate::sealed::RegSpec for Hptxsts_SPEC {
    type DataType = u32;
}

#[doc = "Host periodic transmit FIFO/queue\n          status register"]
pub type Hptxsts = crate::RegValueT<Hptxsts_SPEC>;

impl Hptxsts {
    #[doc = "Periodic transmit data FIFO space\n              available"]
    #[inline(always)]
    pub fn ptxfsavl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Hptxsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Hptxsts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Periodic transmit request queue space\n              available"]
    #[inline(always)]
    pub fn ptxqsav(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Hptxsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Hptxsts_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Top of the periodic transmit request\n              queue"]
    #[inline(always)]
    pub fn ptxqtop(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Hptxsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Hptxsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Hptxsts {
    #[inline(always)]
    fn default() -> Hptxsts {
        <crate::RegValueT<Hptxsts_SPEC> as RegisterValue<_>>::new(524544)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haint_SPEC;
impl crate::sealed::RegSpec for Haint_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS Host all channels interrupt\n          register"]
pub type Haint = crate::RegValueT<Haint_SPEC>;

impl NoBitfieldReg<Haint_SPEC> for Haint {}
impl ::core::default::Default for Haint {
    #[inline(always)]
    fn default() -> Haint {
        <crate::RegValueT<Haint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haintmsk_SPEC;
impl crate::sealed::RegSpec for Haintmsk_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS host all channels interrupt mask\n          register"]
pub type Haintmsk = crate::RegValueT<Haintmsk_SPEC>;

impl NoBitfieldReg<Haintmsk_SPEC> for Haintmsk {}
impl ::core::default::Default for Haintmsk {
    #[inline(always)]
    fn default() -> Haintmsk {
        <crate::RegValueT<Haintmsk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hprt_SPEC;
impl crate::sealed::RegSpec for Hprt_SPEC {
    type DataType = u32;
}

#[doc = "OTG_HS host port control and status\n          register"]
pub type Hprt = crate::RegValueT<Hprt_SPEC>;

impl Hprt {
    #[doc = "Port connect status"]
    #[inline(always)]
    pub fn pcsts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Hprt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Hprt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Port connect detected"]
    #[inline(always)]
    pub fn pcdet(self) -> crate::common::RegisterFieldBool<1, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port enable"]
    #[inline(always)]
    pub fn pena(self) -> crate::common::RegisterFieldBool<2, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port enable/disable change"]
    #[inline(always)]
    pub fn penchng(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port overcurrent active"]
    #[inline(always)]
    pub fn poca(self) -> crate::common::RegisterFieldBool<4, 1, 0, Hprt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Hprt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Port overcurrent change"]
    #[inline(always)]
    pub fn pocchng(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port resume"]
    #[inline(always)]
    pub fn pres(self) -> crate::common::RegisterFieldBool<6, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port suspend"]
    #[inline(always)]
    pub fn psusp(self) -> crate::common::RegisterFieldBool<7, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port reset"]
    #[inline(always)]
    pub fn prst(self) -> crate::common::RegisterFieldBool<8, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port line status"]
    #[inline(always)]
    pub fn plsts(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, Hprt_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,Hprt_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Port power"]
    #[inline(always)]
    pub fn ppwr(self) -> crate::common::RegisterFieldBool<12, 1, 0, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Hprt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Port test control"]
    #[inline(always)]
    pub fn ptctl(
        self,
    ) -> crate::common::RegisterField<13, 0xf, 1, 0, u8, u8, Hprt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0xf,1,0,u8,u8,Hprt_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Port speed"]
    #[inline(always)]
    pub fn pspd(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, u8, Hprt_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x3,1,0,u8,u8,Hprt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Hprt {
    #[inline(always)]
    fn default() -> Hprt {
        <crate::RegValueT<Hprt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "Host channel %s"]
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
    pub const fn hcchar(
        &self,
    ) -> &'static crate::common::Reg<host_channel::Hcchar_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<host_channel::Hcchar_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Split control register"]
    #[inline(always)]
    pub const fn hcsplt(
        &self,
    ) -> &'static crate::common::Reg<host_channel::Hcsplt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<host_channel::Hcsplt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt register"]
    #[inline(always)]
    pub const fn hcint(
        &self,
    ) -> &'static crate::common::Reg<host_channel::Hcint_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<host_channel::Hcint_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn hcintmsk(
        &self,
    ) -> &'static crate::common::Reg<host_channel::Hcintmsk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<host_channel::Hcintmsk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Transfer size"]
    #[inline(always)]
    pub const fn hctsiz(
        &self,
    ) -> &'static crate::common::Reg<host_channel::Hctsiz_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<host_channel::Hctsiz_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DMA address"]
    #[inline(always)]
    pub const fn hcdma(
        &self,
    ) -> &'static crate::common::Reg<host_channel::Hcdma_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<host_channel::Hcdma_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }
}

unsafe impl AsPtr for _HostChannel {
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
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar_SPEC;
    impl crate::sealed::RegSpec for Hcchar_SPEC {
        type DataType = u32;
    }

    #[doc = "Characteristics register"]
    pub type Hcchar = crate::RegValueT<Hcchar_SPEC>;

    impl NoBitfieldReg<Hcchar_SPEC> for Hcchar {}
    impl ::core::default::Default for Hcchar {
        #[inline(always)]
        fn default() -> Hcchar {
            <crate::RegValueT<Hcchar_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcsplt_SPEC;
    impl crate::sealed::RegSpec for Hcsplt_SPEC {
        type DataType = u32;
    }

    #[doc = "Split control register"]
    pub type Hcsplt = crate::RegValueT<Hcsplt_SPEC>;

    impl NoBitfieldReg<Hcsplt_SPEC> for Hcsplt {}
    impl ::core::default::Default for Hcsplt {
        #[inline(always)]
        fn default() -> Hcsplt {
            <crate::RegValueT<Hcsplt_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint_SPEC;
    impl crate::sealed::RegSpec for Hcint_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt register"]
    pub type Hcint = crate::RegValueT<Hcint_SPEC>;

    impl NoBitfieldReg<Hcint_SPEC> for Hcint {}
    impl ::core::default::Default for Hcint {
        #[inline(always)]
        fn default() -> Hcint {
            <crate::RegValueT<Hcint_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk_SPEC;
    impl crate::sealed::RegSpec for Hcintmsk_SPEC {
        type DataType = u32;
    }

    #[doc = "Interrupt mask"]
    pub type Hcintmsk = crate::RegValueT<Hcintmsk_SPEC>;

    impl NoBitfieldReg<Hcintmsk_SPEC> for Hcintmsk {}
    impl ::core::default::Default for Hcintmsk {
        #[inline(always)]
        fn default() -> Hcintmsk {
            <crate::RegValueT<Hcintmsk_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz_SPEC;
    impl crate::sealed::RegSpec for Hctsiz_SPEC {
        type DataType = u32;
    }

    #[doc = "Transfer size"]
    pub type Hctsiz = crate::RegValueT<Hctsiz_SPEC>;

    impl NoBitfieldReg<Hctsiz_SPEC> for Hctsiz {}
    impl ::core::default::Default for Hctsiz {
        #[inline(always)]
        fn default() -> Hctsiz {
            <crate::RegValueT<Hctsiz_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcdma_SPEC;
    impl crate::sealed::RegSpec for Hcdma_SPEC {
        type DataType = u32;
    }

    #[doc = "DMA address"]
    pub type Hcdma = crate::RegValueT<Hcdma_SPEC>;

    impl NoBitfieldReg<Hcdma_SPEC> for Hcdma {}
    impl ::core::default::Default for Hcdma {
        #[inline(always)]
        fn default() -> Hcdma {
            <crate::RegValueT<Hcdma_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
