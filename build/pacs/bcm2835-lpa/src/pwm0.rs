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
#[doc = r"Broadcom PWM"]
unsafe impl ::core::marker::Send for super::Pwm0 {}
unsafe impl ::core::marker::Sync for super::Pwm0 {}
impl super::Pwm0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control"]
    #[inline(always)]
    pub fn ctl(&self) -> &'static self::CtlT {
        unsafe { self::CtlT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub fn sta(&self) -> &'static self::StaT {
        unsafe { self::StaT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "DMA control"]
    #[inline(always)]
    pub fn dmac(&self) -> &'static self::DmacT {
        unsafe { self::DmacT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Range for channel 1"]
    #[inline(always)]
    pub fn rng1(&self) -> &'static self::Rng1T {
        unsafe { self::Rng1T::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "Channel 1 data"]
    #[inline(always)]
    pub fn dat1(&self) -> &'static self::Dat1T {
        unsafe { self::Dat1T::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "FIFO input"]
    #[inline(always)]
    pub fn fif1(&self) -> &'static self::Fif1T {
        unsafe { self::Fif1T::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "Range for channel 2"]
    #[inline(always)]
    pub fn rng2(&self) -> &'static self::Rng2T {
        unsafe { self::Rng2T::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "Channel 2 data"]
    #[inline(always)]
    pub fn dat2(&self) -> &'static self::Dat2T {
        unsafe { self::Dat2T::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }
}

#[doc = "Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ctl {
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
pub struct CtlT;
unsafe impl crate::common::AsPtr for CtlT {}
impl crate::common::Reg<Ctl> for CtlT {}

unsafe impl crate::common::Read<Ctl> for CtlT {}
unsafe impl crate::common::Write<Ctl> for CtlT {}
impl Ctl {
    #[doc = "M/S mode for channel 2"]
    #[inline(always)]
    pub fn msen2(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Use FIFO for channel 2"]
    #[inline(always)]
    pub fn usef2(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 2 polarity inverted"]
    #[inline(always)]
    pub fn pola2(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "State when not transmitting on channel 2"]
    #[inline(always)]
    pub fn sbit2(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Repeat last value from FIFO for channel 2"]
    #[inline(always)]
    pub fn rptl2(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 2 mode"]
    #[inline(always)]
    pub fn mode2(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ctl::Mode2, ctl::Mode2, Ctl, common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,ctl::Mode2,ctl::Mode2,Ctl,common::RW>::from_register(self,0)
    }

    #[doc = "Enable channel 2"]
    #[inline(always)]
    pub fn pwen2(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "M/S mode for channel 1"]
    #[inline(always)]
    pub fn msen1(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Clear FIFO"]
    #[inline(always)]
    pub fn clrf1(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Use FIFO for channel 1"]
    #[inline(always)]
    pub fn usef1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 polarity inverted"]
    #[inline(always)]
    pub fn pola1(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "State when not transmitting on channel 1"]
    #[inline(always)]
    pub fn sbit1(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Repeat last value from FIFO for channel 1"]
    #[inline(always)]
    pub fn rptl1(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ctl::Mode1, ctl::Mode1, Ctl, common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ctl::Mode1,ctl::Mode1,Ctl,common::RW>::from_register(self,0)
    }

    #[doc = "Enable channel 1"]
    #[inline(always)]
    pub fn pwen1(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ctl, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ctl, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ctl> for CtlT {
    #[inline(always)]
    fn reset_value(&self) -> Ctl {
        Ctl::new(0)
    }
}
pub mod ctl {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Mode2(u8);

    impl Mode2 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Mode2 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Mode2 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Mode2> for u64 {
        #[inline(always)]
        fn from(value: Mode2) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Mode2 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Mode2 {
        pub const PWM: Self = Self(0);

        pub const SERIAL: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Mode1(u8);

    impl Mode1 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Mode1 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Mode1 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Mode1> for u64 {
        #[inline(always)]
        fn from(value: Mode1) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Mode1 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Mode1 {
        pub const PWM: Self = Self(0);

        pub const SERIAL: Self = Self(1);
    }
}

#[doc = "Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sta {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Sta {
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
pub struct StaT;
unsafe impl crate::common::AsPtr for StaT {}
impl crate::common::Reg<Sta> for StaT {}

unsafe impl crate::common::Read<Sta> for StaT {}
unsafe impl crate::common::Write<Sta> for StaT {}
impl Sta {
    #[doc = "Channel 4 state"]
    #[inline(always)]
    pub fn sta4(self) -> crate::common::RegisterFieldBool<12, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 3 state"]
    #[inline(always)]
    pub fn sta3(self) -> crate::common::RegisterFieldBool<11, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 2 state"]
    #[inline(always)]
    pub fn sta2(self) -> crate::common::RegisterFieldBool<10, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 state"]
    #[inline(always)]
    pub fn sta1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Bus error"]
    #[inline(always)]
    pub fn berr(self) -> crate::common::RegisterFieldBool<8, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 4 gap occurred"]
    #[inline(always)]
    pub fn gapo4(self) -> crate::common::RegisterFieldBool<7, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 3 gap occurred"]
    #[inline(always)]
    pub fn gapo3(self) -> crate::common::RegisterFieldBool<6, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 2 gap occurred"]
    #[inline(always)]
    pub fn gapo2(self) -> crate::common::RegisterFieldBool<5, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "Channel 1 gap occurred"]
    #[inline(always)]
    pub fn gapo1(self) -> crate::common::RegisterFieldBool<4, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "FIFO read error"]
    #[inline(always)]
    pub fn rerr1(self) -> crate::common::RegisterFieldBool<3, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "FIFO write error"]
    #[inline(always)]
    pub fn werr1(self) -> crate::common::RegisterFieldBool<2, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn empt1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sta, common::RW>::from_register(self, 0)
    }

    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn full1(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sta, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sta, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Sta> for StaT {
    #[inline(always)]
    fn reset_value(&self) -> Sta {
        Sta::new(0)
    }
}

#[doc = "DMA control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dmac {
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
pub struct DmacT;
unsafe impl crate::common::AsPtr for DmacT {}
impl crate::common::Reg<Dmac> for DmacT {}

unsafe impl crate::common::Read<Dmac> for DmacT {}
unsafe impl crate::common::Write<Dmac> for DmacT {}
impl Dmac {
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn enab(self) -> crate::common::RegisterFieldBool<31, 1, 0, Dmac, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Dmac, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA threshold for panic signal"]
    #[inline(always)]
    pub fn panic(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Dmac, common::RW> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Dmac, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA threshold for DREQ signal"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dmac, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, Dmac, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Dmac> for DmacT {
    #[inline(always)]
    fn reset_value(&self) -> Dmac {
        Dmac::new(0)
    }
}

#[doc = "Range for channel 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Rng1 {
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
pub struct Rng1T;
unsafe impl crate::common::AsPtr for Rng1T {}
impl crate::common::Reg<Rng1> for Rng1T {}

unsafe impl crate::common::Read<Rng1> for Rng1T {}
unsafe impl crate::common::Write<Rng1> for Rng1T {}

impl crate::common::NoBitfieldReg for Rng1 {}
impl crate::common::ResetValue<Rng1> for Rng1T {
    #[inline(always)]
    fn reset_value(&self) -> Rng1 {
        Rng1::new(32)
    }
}

#[doc = "Channel 1 data"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dat1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dat1 {
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
pub struct Dat1T;
unsafe impl crate::common::AsPtr for Dat1T {}
impl crate::common::Reg<Dat1> for Dat1T {}

unsafe impl crate::common::Read<Dat1> for Dat1T {}
unsafe impl crate::common::Write<Dat1> for Dat1T {}

impl crate::common::NoBitfieldReg for Dat1 {}
impl crate::common::ResetValue<Dat1> for Dat1T {
    #[inline(always)]
    fn reset_value(&self) -> Dat1 {
        Dat1::new(0)
    }
}

#[doc = "FIFO input"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fif1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Fif1 {
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
pub struct Fif1T;
unsafe impl crate::common::AsPtr for Fif1T {}
impl crate::common::Reg<Fif1> for Fif1T {}

unsafe impl crate::common::Write<Fif1> for Fif1T {}

impl crate::common::NoBitfieldReg for Fif1 {}
impl crate::common::ResetValue<Fif1> for Fif1T {
    #[inline(always)]
    fn reset_value(&self) -> Fif1 {
        Fif1::new(0)
    }
}

#[doc = "Range for channel 2"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Rng2 {
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
pub struct Rng2T;
unsafe impl crate::common::AsPtr for Rng2T {}
impl crate::common::Reg<Rng2> for Rng2T {}

unsafe impl crate::common::Read<Rng2> for Rng2T {}
unsafe impl crate::common::Write<Rng2> for Rng2T {}

impl crate::common::NoBitfieldReg for Rng2 {}
impl crate::common::ResetValue<Rng2> for Rng2T {
    #[inline(always)]
    fn reset_value(&self) -> Rng2 {
        Rng2::new(32)
    }
}

#[doc = "Channel 2 data"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dat2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Dat2 {
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
pub struct Dat2T;
unsafe impl crate::common::AsPtr for Dat2T {}
impl crate::common::Reg<Dat2> for Dat2T {}

unsafe impl crate::common::Read<Dat2> for Dat2T {}
unsafe impl crate::common::Write<Dat2> for Dat2T {}

impl crate::common::NoBitfieldReg for Dat2 {}
impl crate::common::ResetValue<Dat2> for Dat2T {
    #[inline(always)]
    fn reset_value(&self) -> Dat2 {
        Dat2::new(0)
    }
}
