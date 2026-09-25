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
#[doc = r"Broadcom Power Manager"]
unsafe impl ::core::marker::Send for super::Pm {}
unsafe impl ::core::marker::Sync for super::Pm {}
impl super::Pm {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Reset Control"]
    #[inline(always)]
    pub fn rstc(&self) -> &'static self::RstcT {
        unsafe { self::RstcT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "Watchdog control"]
    #[inline(always)]
    pub fn wdog(&self) -> &'static self::WdogT {
        unsafe { self::WdogT::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }
}

#[doc = "Reset Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstc {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Rstc {
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
pub struct RstcT;
unsafe impl crate::common::AsPtr for RstcT {}
impl crate::common::Reg<Rstc> for RstcT {}

unsafe impl crate::common::Read<Rstc> for RstcT {}
unsafe impl crate::common::Write<Rstc> for RstcT {}
impl Rstc {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, rstc::Passwd, rstc::Passwd, Rstc, common::W>
    {
        crate::common::RegisterField::<24,0xff,1,0,rstc::Passwd,rstc::Passwd,Rstc,common::W>::from_register(self,0)
    }

    #[doc = "Watchdog reset config"]
    #[inline(always)]
    pub fn wrcfg(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, rstc::Wrcfg, rstc::Wrcfg, Rstc, common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,rstc::Wrcfg,rstc::Wrcfg,Rstc,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Rstc> for RstcT {
    #[inline(always)]
    fn reset_value(&self) -> Rstc {
        Rstc::new(258)
    }
}
pub mod rstc {
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
    pub struct Wrcfg(u8);

    impl Wrcfg {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Wrcfg {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Wrcfg {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Wrcfg> for u64 {
        #[inline(always)]
        fn from(value: Wrcfg) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Wrcfg {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Wrcfg {
        pub const FULL_RESET: Self = Self(2);
    }
}

#[doc = "Watchdog control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdog {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Wdog {
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
pub struct WdogT;
unsafe impl crate::common::AsPtr for WdogT {}
impl crate::common::Reg<Wdog> for WdogT {}

unsafe impl crate::common::Read<Wdog> for WdogT {}
unsafe impl crate::common::Write<Wdog> for WdogT {}
impl Wdog {
    #[doc = "Password. Always 0x5a"]
    #[inline(always)]
    pub fn passwd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, wdog::Passwd, wdog::Passwd, Wdog, common::W>
    {
        crate::common::RegisterField::<24,0xff,1,0,wdog::Passwd,wdog::Passwd,Wdog,common::W>::from_register(self,0)
    }

    #[doc = "Time until watchdog alarm"]
    #[inline(always)]
    pub fn time(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, u32, Wdog, common::RW> {
        crate::common::RegisterField::<0, 0xfffff, 1, 0, u32, u32, Wdog, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Wdog> for WdogT {
    #[inline(always)]
    fn reset_value(&self) -> Wdog {
        Wdog::new(0)
    }
}
pub mod wdog {
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
