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
#[doc = r"Broadcom Clock Manager"]
unsafe impl ::core::marker::Send for super::CmPcm {}
unsafe impl ::core::marker::Sync for super::CmPcm {}
impl super::CmPcm {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control / Status"]
    #[inline(always)]
    pub fn cs(&self) -> &'static self::CsT {
        unsafe { self::CsT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Clock divisor"]
    #[inline(always)]
    pub fn div(&self) -> &'static self::DivT {
        unsafe { self::DivT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }
}

#[doc = "Control / Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs {
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
pub struct CsT;
unsafe impl crate::common::AsPtr for CsT {}
impl crate::common::Reg<Cs> for CsT {}

unsafe impl crate::common::Read<Cs> for CsT {}
unsafe impl crate::common::Write<Cs> for CsT {}
impl Cs {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, cs::Passwd, cs::Passwd, Cs, common::W> {
        crate::common::RegisterField::<24,0xff,1,0,cs::Passwd,cs::Passwd,Cs,common::W>::from_register(self,0)
    }

    #[doc = "MASH control, stage count"]
    #[inline(always)]
    pub fn mash(self) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, u8, Cs, common::RW> {
        crate::common::RegisterField::<9, 0x3, 1, 0, u8, u8, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Generate an edge on output. (For testing)"]
    #[inline(always)]
    pub fn flip(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Indicates the clock generator is running"]
    #[inline(always)]
    pub fn busy(self) -> crate::common::RegisterFieldBool<7, 1, 0, Cs, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cs, common::R>::from_register(self, 0)
    }

    #[doc = "Stop and reset the generator"]
    #[inline(always)]
    pub fn kill(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable the clock generator. (Switch SRC first.)"]
    #[inline(always)]
    pub fn enab(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "Clock source"]
    #[inline(always)]
    pub fn src(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, cs::Src, cs::Src, Cs, common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,cs::Src,cs::Src,Cs,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Cs> for CsT {
    #[inline(always)]
    fn reset_value(&self) -> Cs {
        Cs::new(0)
    }
}
pub mod cs {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Passwd(u8);

    impl Passwd {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Passwd {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Passwd {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Passwd> for u64 {
        #[inline(always)]
        fn from(value: Passwd) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Passwd {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Passwd {
        pub const PASSWD: Self = Self(90);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Src(u8);

    impl Src {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Src {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Src {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Src> for u64 {
        #[inline(always)]
        fn from(value: Src) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Src {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Src {
        pub const XOSC: Self = Self(1);

        pub const TEST_0: Self = Self(2);

        pub const TEST_1: Self = Self(3);

        pub const PLLA: Self = Self(4);

        pub const PLLB: Self = Self(5);

        pub const PLLC: Self = Self(6);

        pub const HDMI: Self = Self(7);
    }
}

#[doc = "Clock divisor"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Div {
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
pub struct DivT;
unsafe impl crate::common::AsPtr for DivT {}
impl crate::common::Reg<Div> for DivT {}

unsafe impl crate::common::Read<Div> for DivT {}
unsafe impl crate::common::Write<Div> for DivT {}
impl Div {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, div::Passwd, div::Passwd, Div, common::W>
    {
        crate::common::RegisterField::<24,0xff,1,0,div::Passwd,div::Passwd,Div,common::W>::from_register(self,0)
    }

    #[doc = "Integer part of divisor"]
    #[inline(always)]
    pub fn divi(self) -> crate::common::RegisterField<12, 0xfff, 1, 0, u16, u16, Div, common::RW> {
        crate::common::RegisterField::<12, 0xfff, 1, 0, u16, u16, Div, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fractional part of divisor"]
    #[inline(always)]
    pub fn divf(self) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Div, common::RW> {
        crate::common::RegisterField::<0, 0xfff, 1, 0, u16, u16, Div, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Div> for DivT {
    #[inline(always)]
    fn reset_value(&self) -> Div {
        Div::new(0)
    }
}
pub mod div {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Passwd(u8);

    impl Passwd {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Passwd {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Passwd {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Passwd> for u64 {
        #[inline(always)]
        fn from(value: Passwd) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Passwd {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Passwd {
        pub const PASSWD: Self = Self(90);
    }
}
