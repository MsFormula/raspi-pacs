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
#[doc = r"Pin level and mux control"]
unsafe impl ::core::marker::Send for super::Gpio {}
unsafe impl ::core::marker::Sync for super::Gpio {}
impl super::Gpio {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "GPIO Function Select 0"]
    #[inline(always)]
    pub fn gpfsel0(&self) -> &'static self::Gpfsel0T {
        unsafe { self::Gpfsel0T::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "GPIO Function Select 1"]
    #[inline(always)]
    pub fn gpfsel1(&self) -> &'static self::Gpfsel1T {
        unsafe { self::Gpfsel1T::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "GPIO Function Select 2"]
    #[inline(always)]
    pub fn gpfsel2(&self) -> &'static self::Gpfsel2T {
        unsafe { self::Gpfsel2T::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "GPIO Function Select 3"]
    #[inline(always)]
    pub fn gpfsel3(&self) -> &'static self::Gpfsel3T {
        unsafe { self::Gpfsel3T::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "GPIO Function Select 4"]
    #[inline(always)]
    pub fn gpfsel4(&self) -> &'static self::Gpfsel4T {
        unsafe { self::Gpfsel4T::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "GPIO Function Select 5"]
    #[inline(always)]
    pub fn gpfsel5(&self) -> &'static self::Gpfsel5T {
        unsafe { self::Gpfsel5T::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "GPIO Pin Output Set 0"]
    #[inline(always)]
    pub fn gpset0(&self) -> &'static self::Gpset0T {
        unsafe { self::Gpset0T::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "GPIO Pin Output Set 1"]
    #[inline(always)]
    pub fn gpset1(&self) -> &'static self::Gpset1T {
        unsafe { self::Gpset1T::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "GPIO Pin Output Clear 0"]
    #[inline(always)]
    pub fn gpclr0(&self) -> &'static self::Gpclr0T {
        unsafe { self::Gpclr0T::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "GPIO Pin Output Clear 1"]
    #[inline(always)]
    pub fn gpclr1(&self) -> &'static self::Gpclr1T {
        unsafe { self::Gpclr1T::from_ptr(self._svd2pac_as_ptr().add(44usize)) }
    }

    #[doc = "GPIO Pin Level 0"]
    #[inline(always)]
    pub fn gplev0(&self) -> &'static self::Gplev0T {
        unsafe { self::Gplev0T::from_ptr(self._svd2pac_as_ptr().add(52usize)) }
    }

    #[doc = "GPIO Pin Level 1"]
    #[inline(always)]
    pub fn gplev1(&self) -> &'static self::Gplev1T {
        unsafe { self::Gplev1T::from_ptr(self._svd2pac_as_ptr().add(56usize)) }
    }

    #[doc = "GPIO Pin Event Detect Status 0"]
    #[inline(always)]
    pub fn gpeds0(&self) -> &'static self::Gpeds0T {
        unsafe { self::Gpeds0T::from_ptr(self._svd2pac_as_ptr().add(64usize)) }
    }

    #[doc = "GPIO Pin Event Detect Status 1"]
    #[inline(always)]
    pub fn gpeds1(&self) -> &'static self::Gpeds1T {
        unsafe { self::Gpeds1T::from_ptr(self._svd2pac_as_ptr().add(68usize)) }
    }

    #[doc = "GPIO Pin Rising Edge Detect Enable 0"]
    #[inline(always)]
    pub fn gpren0(&self) -> &'static self::Gpren0T {
        unsafe { self::Gpren0T::from_ptr(self._svd2pac_as_ptr().add(76usize)) }
    }

    #[doc = "GPIO Pin Rising Edge Detect Enable 1"]
    #[inline(always)]
    pub fn gpren1(&self) -> &'static self::Gpren1T {
        unsafe { self::Gpren1T::from_ptr(self._svd2pac_as_ptr().add(80usize)) }
    }

    #[doc = "GPIO Pin Falling Edge Detect Enable 0"]
    #[inline(always)]
    pub fn gpfen0(&self) -> &'static self::Gpfen0T {
        unsafe { self::Gpfen0T::from_ptr(self._svd2pac_as_ptr().add(88usize)) }
    }

    #[doc = "GPIO Pin Falling Edge Detect Enable 1"]
    #[inline(always)]
    pub fn gpfen1(&self) -> &'static self::Gpfen1T {
        unsafe { self::Gpfen1T::from_ptr(self._svd2pac_as_ptr().add(92usize)) }
    }

    #[doc = "GPIO Pin High Detect Enable 0"]
    #[inline(always)]
    pub fn gphen0(&self) -> &'static self::Gphen0T {
        unsafe { self::Gphen0T::from_ptr(self._svd2pac_as_ptr().add(100usize)) }
    }

    #[doc = "GPIO Pin High Detect Enable 1"]
    #[inline(always)]
    pub fn gphen1(&self) -> &'static self::Gphen1T {
        unsafe { self::Gphen1T::from_ptr(self._svd2pac_as_ptr().add(104usize)) }
    }

    #[doc = "GPIO Pin Low Detect Enable 0"]
    #[inline(always)]
    pub fn gplen0(&self) -> &'static self::Gplen0T {
        unsafe { self::Gplen0T::from_ptr(self._svd2pac_as_ptr().add(112usize)) }
    }

    #[doc = "GPIO Pin Low Detect Enable 1"]
    #[inline(always)]
    pub fn gplen1(&self) -> &'static self::Gplen1T {
        unsafe { self::Gplen1T::from_ptr(self._svd2pac_as_ptr().add(116usize)) }
    }

    #[doc = "GPIO Pin Async. Rising Edge Detect 0"]
    #[inline(always)]
    pub fn gparen0(&self) -> &'static self::Gparen0T {
        unsafe { self::Gparen0T::from_ptr(self._svd2pac_as_ptr().add(124usize)) }
    }

    #[doc = "GPIO Pin Async. Rising Edge Detect 1"]
    #[inline(always)]
    pub fn gparen1(&self) -> &'static self::Gparen1T {
        unsafe { self::Gparen1T::from_ptr(self._svd2pac_as_ptr().add(128usize)) }
    }

    #[doc = "GPIO Pin Async. Falling Edge Detect 0"]
    #[inline(always)]
    pub fn gpafen0(&self) -> &'static self::Gpafen0T {
        unsafe { self::Gpafen0T::from_ptr(self._svd2pac_as_ptr().add(136usize)) }
    }

    #[doc = "GPIO Pin Async. Falling Edge Detect 1"]
    #[inline(always)]
    pub fn gpafen1(&self) -> &'static self::Gpafen1T {
        unsafe { self::Gpafen1T::from_ptr(self._svd2pac_as_ptr().add(140usize)) }
    }

    #[doc = "Undocumented multiplexing bits"]
    #[inline(always)]
    pub fn extra_mux(&self) -> &'static self::ExtraMuxT {
        unsafe { self::ExtraMuxT::from_ptr(self._svd2pac_as_ptr().add(208usize)) }
    }

    #[doc = "GPIO Pin Pull-up/down Enable"]
    #[inline(always)]
    pub fn gppud(&self) -> &'static self::GppudT {
        unsafe { self::GppudT::from_ptr(self._svd2pac_as_ptr().add(148usize)) }
    }

    #[doc = "GPIO Pin Pull-up/down Enable Clock 0"]
    #[inline(always)]
    pub fn gppudclk0(&self) -> &'static self::Gppudclk0T {
        unsafe { self::Gppudclk0T::from_ptr(self._svd2pac_as_ptr().add(152usize)) }
    }

    #[doc = "GPIO Pin Pull-up/down Enable Clock 1"]
    #[inline(always)]
    pub fn gppudclk1(&self) -> &'static self::Gppudclk1T {
        unsafe { self::Gppudclk1T::from_ptr(self._svd2pac_as_ptr().add(156usize)) }
    }
}

#[doc = "GPIO Function Select 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfsel0 {
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
pub struct Gpfsel0T;
unsafe impl crate::common::AsPtr for Gpfsel0T {}
impl crate::common::Reg<Gpfsel0> for Gpfsel0T {}

unsafe impl crate::common::Read<Gpfsel0> for Gpfsel0T {}
unsafe impl crate::common::Write<Gpfsel0> for Gpfsel0T {}
impl Gpfsel0 {
    #[doc = "Function Select 0"]
    #[inline(always)]
    pub fn fsel0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gpfsel0::Fsel0,
        gpfsel0::Fsel0,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel0::Fsel0,
            gpfsel0::Fsel0,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 1"]
    #[inline(always)]
    pub fn fsel1(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        gpfsel0::Fsel1,
        gpfsel0::Fsel1,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel0::Fsel1,
            gpfsel0::Fsel1,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 2"]
    #[inline(always)]
    pub fn fsel2(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x7,
        1,
        0,
        gpfsel0::Fsel2,
        gpfsel0::Fsel2,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel0::Fsel2,
            gpfsel0::Fsel2,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 3"]
    #[inline(always)]
    pub fn fsel3(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7,
        1,
        0,
        gpfsel0::Fsel3,
        gpfsel0::Fsel3,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel0::Fsel3,
            gpfsel0::Fsel3,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 4"]
    #[inline(always)]
    pub fn fsel4(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        gpfsel0::Fsel4,
        gpfsel0::Fsel4,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel0::Fsel4,
            gpfsel0::Fsel4,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 5"]
    #[inline(always)]
    pub fn fsel5(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x7,
        1,
        0,
        gpfsel0::Fsel5,
        gpfsel0::Fsel5,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel0::Fsel5,
            gpfsel0::Fsel5,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 6"]
    #[inline(always)]
    pub fn fsel6(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x7,
        1,
        0,
        gpfsel0::Fsel6,
        gpfsel0::Fsel6,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel0::Fsel6,
            gpfsel0::Fsel6,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 7"]
    #[inline(always)]
    pub fn fsel7(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7,
        1,
        0,
        gpfsel0::Fsel7,
        gpfsel0::Fsel7,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel0::Fsel7,
            gpfsel0::Fsel7,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 8"]
    #[inline(always)]
    pub fn fsel8(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        gpfsel0::Fsel8,
        gpfsel0::Fsel8,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel0::Fsel8,
            gpfsel0::Fsel8,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 9"]
    #[inline(always)]
    pub fn fsel9(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x7,
        1,
        0,
        gpfsel0::Fsel9,
        gpfsel0::Fsel9,
        Gpfsel0,
        common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel0::Fsel9,
            gpfsel0::Fsel9,
            Gpfsel0,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfsel0> for Gpfsel0T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfsel0 {
        Gpfsel0::new(0)
    }
}
pub mod gpfsel0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel0(u8);

    impl Fsel0 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel0 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel0 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel0> for u64 {
        #[inline(always)]
        fn from(value: Fsel0) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel0 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel0 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SDA0"]
        pub const SDA_0: Self = Self(4);

        #[doc = "Pin is connected to SA5"]
        pub const SA_5: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel1(u8);

    impl Fsel1 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel1 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel1 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel1> for u64 {
        #[inline(always)]
        fn from(value: Fsel1) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel1 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel1 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SCL0"]
        pub const SCL_0: Self = Self(4);

        #[doc = "Pin is connected to SA4"]
        pub const SA_4: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel2(u8);

    impl Fsel2 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel2 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel2 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel2> for u64 {
        #[inline(always)]
        fn from(value: Fsel2) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel2 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel2 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SDA1"]
        pub const SDA_1: Self = Self(4);

        #[doc = "Pin is connected to SA3"]
        pub const SA_3: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel3(u8);

    impl Fsel3 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel3 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel3 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel3> for u64 {
        #[inline(always)]
        fn from(value: Fsel3) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel3 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel3 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SCL1"]
        pub const SCL_1: Self = Self(4);

        #[doc = "Pin is connected to SA2"]
        pub const SA_2: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel4(u8);

    impl Fsel4 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel4 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel4 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel4> for u64 {
        #[inline(always)]
        fn from(value: Fsel4) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel4 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel4 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self(4);

        #[doc = "Pin is connected to SA1"]
        pub const SA_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to ARM_TDI"]
        pub const ARM_TDI: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel5(u8);

    impl Fsel5 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel5 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel5 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel5> for u64 {
        #[inline(always)]
        fn from(value: Fsel5) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel5 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel5 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self(4);

        #[doc = "Pin is connected to SA0"]
        pub const SA_0: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to ARM_TDO"]
        pub const ARM_TDO: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel6(u8);

    impl Fsel6 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel6 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel6 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel6> for u64 {
        #[inline(always)]
        fn from(value: Fsel6) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel6 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel6 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK2"]
        pub const GPCLK_2: Self = Self(4);

        #[doc = "Pin is connected to SOE_N"]
        pub const SOE_N: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to ARM_RTCK"]
        pub const ARM_RTCK: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel7(u8);

    impl Fsel7 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel7 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel7 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel7> for u64 {
        #[inline(always)]
        fn from(value: Fsel7) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel7 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel7 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_CE1_N"]
        pub const SPI_0_CE_1_N: Self = Self(4);

        #[doc = "Pin is connected to SWE_N"]
        pub const SWE_N: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel8(u8);

    impl Fsel8 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel8 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel8 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel8> for u64 {
        #[inline(always)]
        fn from(value: Fsel8) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel8 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel8 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_CE0_N"]
        pub const SPI_0_CE_0_N: Self = Self(4);

        #[doc = "Pin is connected to SD0"]
        pub const SD_0: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel9(u8);

    impl Fsel9 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel9 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel9 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel9> for u64 {
        #[inline(always)]
        fn from(value: Fsel9) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel9 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel9 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_MISO"]
        pub const SPI_0_MISO: Self = Self(4);

        #[doc = "Pin is connected to SD1"]
        pub const SD_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
}

#[doc = "GPIO Function Select 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfsel1 {
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
pub struct Gpfsel1T;
unsafe impl crate::common::AsPtr for Gpfsel1T {}
impl crate::common::Reg<Gpfsel1> for Gpfsel1T {}

unsafe impl crate::common::Read<Gpfsel1> for Gpfsel1T {}
unsafe impl crate::common::Write<Gpfsel1> for Gpfsel1T {}
impl Gpfsel1 {
    #[doc = "Function Select 10"]
    #[inline(always)]
    pub fn fsel10(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gpfsel1::Fsel10,
        gpfsel1::Fsel10,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel1::Fsel10,
            gpfsel1::Fsel10,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 11"]
    #[inline(always)]
    pub fn fsel11(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        gpfsel1::Fsel11,
        gpfsel1::Fsel11,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel1::Fsel11,
            gpfsel1::Fsel11,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 12"]
    #[inline(always)]
    pub fn fsel12(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x7,
        1,
        0,
        gpfsel1::Fsel12,
        gpfsel1::Fsel12,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel1::Fsel12,
            gpfsel1::Fsel12,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 13"]
    #[inline(always)]
    pub fn fsel13(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7,
        1,
        0,
        gpfsel1::Fsel13,
        gpfsel1::Fsel13,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel1::Fsel13,
            gpfsel1::Fsel13,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 14"]
    #[inline(always)]
    pub fn fsel14(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        gpfsel1::Fsel14,
        gpfsel1::Fsel14,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel1::Fsel14,
            gpfsel1::Fsel14,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 15"]
    #[inline(always)]
    pub fn fsel15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x7,
        1,
        0,
        gpfsel1::Fsel15,
        gpfsel1::Fsel15,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel1::Fsel15,
            gpfsel1::Fsel15,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 16"]
    #[inline(always)]
    pub fn fsel16(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x7,
        1,
        0,
        gpfsel1::Fsel16,
        gpfsel1::Fsel16,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel1::Fsel16,
            gpfsel1::Fsel16,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 17"]
    #[inline(always)]
    pub fn fsel17(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7,
        1,
        0,
        gpfsel1::Fsel17,
        gpfsel1::Fsel17,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel1::Fsel17,
            gpfsel1::Fsel17,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 18"]
    #[inline(always)]
    pub fn fsel18(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        gpfsel1::Fsel18,
        gpfsel1::Fsel18,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel1::Fsel18,
            gpfsel1::Fsel18,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 19"]
    #[inline(always)]
    pub fn fsel19(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x7,
        1,
        0,
        gpfsel1::Fsel19,
        gpfsel1::Fsel19,
        Gpfsel1,
        common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel1::Fsel19,
            gpfsel1::Fsel19,
            Gpfsel1,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfsel1> for Gpfsel1T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfsel1 {
        Gpfsel1::new(0)
    }
}
pub mod gpfsel1 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel10(u8);

    impl Fsel10 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel10 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel10 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel10> for u64 {
        #[inline(always)]
        fn from(value: Fsel10) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel10 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel10 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_MOSI"]
        pub const SPI_0_MOSI: Self = Self(4);

        #[doc = "Pin is connected to SD2"]
        pub const SD_2: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel11(u8);

    impl Fsel11 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel11 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel11 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel11> for u64 {
        #[inline(always)]
        fn from(value: Fsel11) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel11 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel11 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_SCLK"]
        pub const SPI_0_SCLK: Self = Self(4);

        #[doc = "Pin is connected to SD3"]
        pub const SD_3: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel12(u8);

    impl Fsel12 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel12 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel12 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel12> for u64 {
        #[inline(always)]
        fn from(value: Fsel12) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel12 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel12 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PWM0_0"]
        pub const PWM_0_0: Self = Self(4);

        #[doc = "Pin is connected to SD4"]
        pub const SD_4: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to ARM_TMS"]
        pub const ARM_TMS: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel13(u8);

    impl Fsel13 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel13 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel13 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel13> for u64 {
        #[inline(always)]
        fn from(value: Fsel13) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel13 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel13 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self(4);

        #[doc = "Pin is connected to SD5"]
        pub const SD_5: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to ARM_TCK"]
        pub const ARM_TCK: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel14(u8);

    impl Fsel14 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel14 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel14 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel14> for u64 {
        #[inline(always)]
        fn from(value: Fsel14) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel14 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel14 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to TXD0"]
        pub const TXD_0: Self = Self(4);

        #[doc = "Pin is connected to SD6"]
        pub const SD_6: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to TXD1"]
        pub const TXD_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel15(u8);

    impl Fsel15 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel15 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel15 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel15> for u64 {
        #[inline(always)]
        fn from(value: Fsel15) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel15 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel15 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to RXD0"]
        pub const RXD_0: Self = Self(4);

        #[doc = "Pin is connected to SD7"]
        pub const SD_7: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to RXD1"]
        pub const RXD_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel16(u8);

    impl Fsel16 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel16 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel16 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel16> for u64 {
        #[inline(always)]
        fn from(value: Fsel16) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel16 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel16 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SD8"]
        pub const SD_8: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to CTS0"]
        pub const CTS_0: Self = Self(7);

        #[doc = "Pin is connected to SPI1_CE2_N"]
        pub const SPI_1_CE_2_N: Self = Self(3);

        #[doc = "Pin is connected to CTS1"]
        pub const CTS_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel17(u8);

    impl Fsel17 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel17 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel17 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel17> for u64 {
        #[inline(always)]
        fn from(value: Fsel17) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel17 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel17 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SD9"]
        pub const SD_9: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to RTS0"]
        pub const RTS_0: Self = Self(7);

        #[doc = "Pin is connected to SPI1_CE1_N"]
        pub const SPI_1_CE_1_N: Self = Self(3);

        #[doc = "Pin is connected to RTS1"]
        pub const RTS_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel18(u8);

    impl Fsel18 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel18 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel18 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel18> for u64 {
        #[inline(always)]
        fn from(value: Fsel18) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel18 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel18 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PCM_CLK"]
        pub const PCM_CLK: Self = Self(4);

        #[doc = "Pin is connected to SD10"]
        pub const SD_10: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI1_CE0_N"]
        pub const SPI_1_CE_0_N: Self = Self(3);

        #[doc = "Pin is connected to PWM0_0"]
        pub const PWM_0_0: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel19(u8);

    impl Fsel19 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel19 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel19 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel19> for u64 {
        #[inline(always)]
        fn from(value: Fsel19) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel19 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel19 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PCM_FS"]
        pub const PCM_FS: Self = Self(4);

        #[doc = "Pin is connected to SD11"]
        pub const SD_11: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI1_MISO"]
        pub const SPI_1_MISO: Self = Self(3);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self(2);
    }
}

#[doc = "GPIO Function Select 2"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfsel2 {
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
pub struct Gpfsel2T;
unsafe impl crate::common::AsPtr for Gpfsel2T {}
impl crate::common::Reg<Gpfsel2> for Gpfsel2T {}

unsafe impl crate::common::Read<Gpfsel2> for Gpfsel2T {}
unsafe impl crate::common::Write<Gpfsel2> for Gpfsel2T {}
impl Gpfsel2 {
    #[doc = "Function Select 20"]
    #[inline(always)]
    pub fn fsel20(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gpfsel2::Fsel20,
        gpfsel2::Fsel20,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel2::Fsel20,
            gpfsel2::Fsel20,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 21"]
    #[inline(always)]
    pub fn fsel21(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        gpfsel2::Fsel21,
        gpfsel2::Fsel21,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel2::Fsel21,
            gpfsel2::Fsel21,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 22"]
    #[inline(always)]
    pub fn fsel22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x7,
        1,
        0,
        gpfsel2::Fsel22,
        gpfsel2::Fsel22,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel2::Fsel22,
            gpfsel2::Fsel22,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 23"]
    #[inline(always)]
    pub fn fsel23(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7,
        1,
        0,
        gpfsel2::Fsel23,
        gpfsel2::Fsel23,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel2::Fsel23,
            gpfsel2::Fsel23,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 24"]
    #[inline(always)]
    pub fn fsel24(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        gpfsel2::Fsel24,
        gpfsel2::Fsel24,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel2::Fsel24,
            gpfsel2::Fsel24,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 25"]
    #[inline(always)]
    pub fn fsel25(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x7,
        1,
        0,
        gpfsel2::Fsel25,
        gpfsel2::Fsel25,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel2::Fsel25,
            gpfsel2::Fsel25,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 26"]
    #[inline(always)]
    pub fn fsel26(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x7,
        1,
        0,
        gpfsel2::Fsel26,
        gpfsel2::Fsel26,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel2::Fsel26,
            gpfsel2::Fsel26,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 27"]
    #[inline(always)]
    pub fn fsel27(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7,
        1,
        0,
        gpfsel2::Fsel27,
        gpfsel2::Fsel27,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel2::Fsel27,
            gpfsel2::Fsel27,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 28"]
    #[inline(always)]
    pub fn fsel28(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        gpfsel2::Fsel28,
        gpfsel2::Fsel28,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel2::Fsel28,
            gpfsel2::Fsel28,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 29"]
    #[inline(always)]
    pub fn fsel29(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x7,
        1,
        0,
        gpfsel2::Fsel29,
        gpfsel2::Fsel29,
        Gpfsel2,
        common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel2::Fsel29,
            gpfsel2::Fsel29,
            Gpfsel2,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfsel2> for Gpfsel2T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfsel2 {
        Gpfsel2::new(0)
    }
}
pub mod gpfsel2 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel20(u8);

    impl Fsel20 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel20 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel20 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel20> for u64 {
        #[inline(always)]
        fn from(value: Fsel20) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel20 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel20 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PCM_DIN"]
        pub const PCM_DIN: Self = Self(4);

        #[doc = "Pin is connected to SD12"]
        pub const SD_12: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI1_MOSI"]
        pub const SPI_1_MOSI: Self = Self(3);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel21(u8);

    impl Fsel21 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel21 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel21 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel21> for u64 {
        #[inline(always)]
        fn from(value: Fsel21) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel21 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel21 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PCM_DOUT"]
        pub const PCM_DOUT: Self = Self(4);

        #[doc = "Pin is connected to SD13"]
        pub const SD_13: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI1_SCLK"]
        pub const SPI_1_SCLK: Self = Self(3);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel22(u8);

    impl Fsel22 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel22 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel22 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel22> for u64 {
        #[inline(always)]
        fn from(value: Fsel22) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel22 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel22 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SD14"]
        pub const SD_14: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_CLK"]
        pub const SD_1_CLK: Self = Self(7);

        #[doc = "Pin is connected to ARM_TRST"]
        pub const ARM_TRST: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel23(u8);

    impl Fsel23 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel23 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel23 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel23> for u64 {
        #[inline(always)]
        fn from(value: Fsel23) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel23 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel23 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SD15"]
        pub const SD_15: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_CMD"]
        pub const SD_1_CMD: Self = Self(7);

        #[doc = "Pin is connected to ARM_RTCK"]
        pub const ARM_RTCK: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel24(u8);

    impl Fsel24 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel24 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel24 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel24> for u64 {
        #[inline(always)]
        fn from(value: Fsel24) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel24 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel24 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SD16"]
        pub const SD_16: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT0"]
        pub const SD_1_DAT_0: Self = Self(7);

        #[doc = "Pin is connected to ARM_TDO"]
        pub const ARM_TDO: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel25(u8);

    impl Fsel25 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel25 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel25 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel25> for u64 {
        #[inline(always)]
        fn from(value: Fsel25) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel25 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel25 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SD17"]
        pub const SD_17: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT1"]
        pub const SD_1_DAT_1: Self = Self(7);

        #[doc = "Pin is connected to ARM_TCK"]
        pub const ARM_TCK: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel26(u8);

    impl Fsel26 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel26 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel26 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel26> for u64 {
        #[inline(always)]
        fn from(value: Fsel26) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel26 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel26 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT2"]
        pub const SD_1_DAT_2: Self = Self(7);

        #[doc = "Pin is connected to ARM_TDI"]
        pub const ARM_TDI: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel27(u8);

    impl Fsel27 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel27 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel27 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel27> for u64 {
        #[inline(always)]
        fn from(value: Fsel27) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel27 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel27 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT3"]
        pub const SD_1_DAT_3: Self = Self(7);

        #[doc = "Pin is connected to ARM_TMS"]
        pub const ARM_TMS: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel28(u8);

    impl Fsel28 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel28 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel28 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel28> for u64 {
        #[inline(always)]
        fn from(value: Fsel28) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel28 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel28 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SDA0"]
        pub const SDA_0: Self = Self(4);

        #[doc = "Pin is connected to SA5"]
        pub const SA_5: Self = Self(5);

        #[doc = "Pin is connected to PCM_CLK"]
        pub const PCM_CLK: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel29(u8);

    impl Fsel29 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel29 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel29 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel29> for u64 {
        #[inline(always)]
        fn from(value: Fsel29) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel29 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel29 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SCL0"]
        pub const SCL_0: Self = Self(4);

        #[doc = "Pin is connected to SA4"]
        pub const SA_4: Self = Self(5);

        #[doc = "Pin is connected to PCM_FS"]
        pub const PCM_FS: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
}

#[doc = "GPIO Function Select 3"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfsel3 {
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
pub struct Gpfsel3T;
unsafe impl crate::common::AsPtr for Gpfsel3T {}
impl crate::common::Reg<Gpfsel3> for Gpfsel3T {}

unsafe impl crate::common::Read<Gpfsel3> for Gpfsel3T {}
unsafe impl crate::common::Write<Gpfsel3> for Gpfsel3T {}
impl Gpfsel3 {
    #[doc = "Function Select 30"]
    #[inline(always)]
    pub fn fsel30(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gpfsel3::Fsel30,
        gpfsel3::Fsel30,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel3::Fsel30,
            gpfsel3::Fsel30,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 31"]
    #[inline(always)]
    pub fn fsel31(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        gpfsel3::Fsel31,
        gpfsel3::Fsel31,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel3::Fsel31,
            gpfsel3::Fsel31,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 32"]
    #[inline(always)]
    pub fn fsel32(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x7,
        1,
        0,
        gpfsel3::Fsel32,
        gpfsel3::Fsel32,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel3::Fsel32,
            gpfsel3::Fsel32,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 33"]
    #[inline(always)]
    pub fn fsel33(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7,
        1,
        0,
        gpfsel3::Fsel33,
        gpfsel3::Fsel33,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel3::Fsel33,
            gpfsel3::Fsel33,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 34"]
    #[inline(always)]
    pub fn fsel34(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        gpfsel3::Fsel34,
        gpfsel3::Fsel34,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel3::Fsel34,
            gpfsel3::Fsel34,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 35"]
    #[inline(always)]
    pub fn fsel35(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x7,
        1,
        0,
        gpfsel3::Fsel35,
        gpfsel3::Fsel35,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel3::Fsel35,
            gpfsel3::Fsel35,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 36"]
    #[inline(always)]
    pub fn fsel36(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x7,
        1,
        0,
        gpfsel3::Fsel36,
        gpfsel3::Fsel36,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel3::Fsel36,
            gpfsel3::Fsel36,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 37"]
    #[inline(always)]
    pub fn fsel37(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7,
        1,
        0,
        gpfsel3::Fsel37,
        gpfsel3::Fsel37,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel3::Fsel37,
            gpfsel3::Fsel37,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 38"]
    #[inline(always)]
    pub fn fsel38(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        gpfsel3::Fsel38,
        gpfsel3::Fsel38,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel3::Fsel38,
            gpfsel3::Fsel38,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 39"]
    #[inline(always)]
    pub fn fsel39(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x7,
        1,
        0,
        gpfsel3::Fsel39,
        gpfsel3::Fsel39,
        Gpfsel3,
        common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel3::Fsel39,
            gpfsel3::Fsel39,
            Gpfsel3,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfsel3> for Gpfsel3T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfsel3 {
        Gpfsel3::new(0)
    }
}
pub mod gpfsel3 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel30(u8);

    impl Fsel30 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel30 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel30 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel30> for u64 {
        #[inline(always)]
        fn from(value: Fsel30) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel30 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel30 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SA3"]
        pub const SA_3: Self = Self(5);

        #[doc = "Pin is connected to PCM_DIN"]
        pub const PCM_DIN: Self = Self(6);

        #[doc = "Pin is connected to CTS0"]
        pub const CTS_0: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to CTS1"]
        pub const CTS_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel31(u8);

    impl Fsel31 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel31 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel31 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel31> for u64 {
        #[inline(always)]
        fn from(value: Fsel31) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel31 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel31 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SA2"]
        pub const SA_2: Self = Self(5);

        #[doc = "Pin is connected to PCM_DOUT"]
        pub const PCM_DOUT: Self = Self(6);

        #[doc = "Pin is connected to RTS0"]
        pub const RTS_0: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to RTS1"]
        pub const RTS_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel32(u8);

    impl Fsel32 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel32 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel32 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel32> for u64 {
        #[inline(always)]
        fn from(value: Fsel32) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel32 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel32 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self(4);

        #[doc = "Pin is connected to SA1"]
        pub const SA_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to TXD0"]
        pub const TXD_0: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to TXD1"]
        pub const TXD_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel33(u8);

    impl Fsel33 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel33 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel33 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel33> for u64 {
        #[inline(always)]
        fn from(value: Fsel33) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel33 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel33 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Pin is connected to SA0"]
        pub const SA_0: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to RXD0"]
        pub const RXD_0: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Pin is connected to RXD1"]
        pub const RXD_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel34(u8);

    impl Fsel34 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel34 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel34 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel34> for u64 {
        #[inline(always)]
        fn from(value: Fsel34) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel34 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel34 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self(4);

        #[doc = "Pin is connected to SOE_N"]
        pub const SOE_N: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel35(u8);

    impl Fsel35 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel35 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel35 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel35> for u64 {
        #[inline(always)]
        fn from(value: Fsel35) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel35 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel35 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_CE1_N"]
        pub const SPI_0_CE_1_N: Self = Self(4);

        #[doc = "Pin is connected to SWE_N"]
        pub const SWE_N: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel36(u8);

    impl Fsel36 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel36 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel36 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel36> for u64 {
        #[inline(always)]
        fn from(value: Fsel36) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel36 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel36 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_CE0_N"]
        pub const SPI_0_CE_0_N: Self = Self(4);

        #[doc = "Pin is connected to SD0"]
        pub const SD_0: Self = Self(5);

        #[doc = "Pin is connected to TXD0"]
        pub const TXD_0: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel37(u8);

    impl Fsel37 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel37 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel37 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel37> for u64 {
        #[inline(always)]
        fn from(value: Fsel37) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel37 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel37 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_MISO"]
        pub const SPI_0_MISO: Self = Self(4);

        #[doc = "Pin is connected to SD1"]
        pub const SD_1: Self = Self(5);

        #[doc = "Pin is connected to RXD0"]
        pub const RXD_0: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel38(u8);

    impl Fsel38 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel38 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel38 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel38> for u64 {
        #[inline(always)]
        fn from(value: Fsel38) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel38 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel38 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_MOSI"]
        pub const SPI_0_MOSI: Self = Self(4);

        #[doc = "Pin is connected to SD2"]
        pub const SD_2: Self = Self(5);

        #[doc = "Pin is connected to CTS0"]
        pub const CTS_0: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel39(u8);

    impl Fsel39 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel39 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel39 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel39> for u64 {
        #[inline(always)]
        fn from(value: Fsel39) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel39 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel39 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to SPI0_SCLK"]
        pub const SPI_0_SCLK: Self = Self(4);

        #[doc = "Pin is connected to SD3"]
        pub const SD_3: Self = Self(5);

        #[doc = "Pin is connected to RTS0"]
        pub const RTS_0: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
}

#[doc = "GPIO Function Select 4"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfsel4 {
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
pub struct Gpfsel4T;
unsafe impl crate::common::AsPtr for Gpfsel4T {}
impl crate::common::Reg<Gpfsel4> for Gpfsel4T {}

unsafe impl crate::common::Read<Gpfsel4> for Gpfsel4T {}
unsafe impl crate::common::Write<Gpfsel4> for Gpfsel4T {}
impl Gpfsel4 {
    #[doc = "Function Select 40"]
    #[inline(always)]
    pub fn fsel40(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gpfsel4::Fsel40,
        gpfsel4::Fsel40,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel4::Fsel40,
            gpfsel4::Fsel40,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 41"]
    #[inline(always)]
    pub fn fsel41(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        gpfsel4::Fsel41,
        gpfsel4::Fsel41,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel4::Fsel41,
            gpfsel4::Fsel41,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 42"]
    #[inline(always)]
    pub fn fsel42(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x7,
        1,
        0,
        gpfsel4::Fsel42,
        gpfsel4::Fsel42,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel4::Fsel42,
            gpfsel4::Fsel42,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 43"]
    #[inline(always)]
    pub fn fsel43(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7,
        1,
        0,
        gpfsel4::Fsel43,
        gpfsel4::Fsel43,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel4::Fsel43,
            gpfsel4::Fsel43,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 44"]
    #[inline(always)]
    pub fn fsel44(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7,
        1,
        0,
        gpfsel4::Fsel44,
        gpfsel4::Fsel44,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel4::Fsel44,
            gpfsel4::Fsel44,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 45"]
    #[inline(always)]
    pub fn fsel45(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x7,
        1,
        0,
        gpfsel4::Fsel45,
        gpfsel4::Fsel45,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel4::Fsel45,
            gpfsel4::Fsel45,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 46"]
    #[inline(always)]
    pub fn fsel46(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x7,
        1,
        0,
        gpfsel4::Fsel46,
        gpfsel4::Fsel46,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel4::Fsel46,
            gpfsel4::Fsel46,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 47"]
    #[inline(always)]
    pub fn fsel47(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x7,
        1,
        0,
        gpfsel4::Fsel47,
        gpfsel4::Fsel47,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel4::Fsel47,
            gpfsel4::Fsel47,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 48"]
    #[inline(always)]
    pub fn fsel48(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        gpfsel4::Fsel48,
        gpfsel4::Fsel48,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel4::Fsel48,
            gpfsel4::Fsel48,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 49"]
    #[inline(always)]
    pub fn fsel49(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x7,
        1,
        0,
        gpfsel4::Fsel49,
        gpfsel4::Fsel49,
        Gpfsel4,
        common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel4::Fsel49,
            gpfsel4::Fsel49,
            Gpfsel4,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfsel4> for Gpfsel4T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfsel4 {
        Gpfsel4::new(0)
    }
}
pub mod gpfsel4 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel40(u8);

    impl Fsel40 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel40 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel40 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel40> for u64 {
        #[inline(always)]
        fn from(value: Fsel40) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel40 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel40 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PWM0_0"]
        pub const PWM_0_0: Self = Self(4);

        #[doc = "Pin is connected to SD4"]
        pub const SD_4: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI2_MISO"]
        pub const SPI_2_MISO: Self = Self(3);

        #[doc = "Pin is connected to TXD1"]
        pub const TXD_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel41(u8);

    impl Fsel41 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel41 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel41 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel41> for u64 {
        #[inline(always)]
        fn from(value: Fsel41) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel41 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel41 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self(4);

        #[doc = "Pin is connected to SD5"]
        pub const SD_5: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI2_MOSI"]
        pub const SPI_2_MOSI: Self = Self(3);

        #[doc = "Pin is connected to RXD1"]
        pub const RXD_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel42(u8);

    impl Fsel42 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel42 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel42 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel42> for u64 {
        #[inline(always)]
        fn from(value: Fsel42) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel42 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel42 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self(4);

        #[doc = "Pin is connected to SD6"]
        pub const SD_6: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI2_SCLK"]
        pub const SPI_2_SCLK: Self = Self(3);

        #[doc = "Pin is connected to CTS1"]
        pub const CTS_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel43(u8);

    impl Fsel43 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel43 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel43 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel43> for u64 {
        #[inline(always)]
        fn from(value: Fsel43) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel43 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel43 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK2"]
        pub const GPCLK_2: Self = Self(4);

        #[doc = "Pin is connected to SD7"]
        pub const SD_7: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI2_CE0_N"]
        pub const SPI_2_CE_0_N: Self = Self(3);

        #[doc = "Pin is connected to RTS1"]
        pub const RTS_1: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel44(u8);

    impl Fsel44 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel44 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel44 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel44> for u64 {
        #[inline(always)]
        fn from(value: Fsel44) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel44 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel44 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self(4);

        #[doc = "Pin is connected to SDA0"]
        pub const SDA_0: Self = Self(5);

        #[doc = "Pin is connected to SDA1"]
        pub const SDA_1: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI2_CE1_N"]
        pub const SPI_2_CE_1_N: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel45(u8);

    impl Fsel45 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel45 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel45 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel45> for u64 {
        #[inline(always)]
        fn from(value: Fsel45) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel45 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel45 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self(4);

        #[doc = "Pin is connected to SCL0"]
        pub const SCL_0: Self = Self(5);

        #[doc = "Pin is connected to SCL1"]
        pub const SCL_1: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Pin is connected to SPI2_CE2_N"]
        pub const SPI_2_CE_2_N: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel46(u8);

    impl Fsel46 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel46 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel46 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel46> for u64 {
        #[inline(always)]
        fn from(value: Fsel46) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel46 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel46 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel47(u8);

    impl Fsel47 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel47 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel47 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel47> for u64 {
        #[inline(always)]
        fn from(value: Fsel47) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel47 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel47 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel48(u8);

    impl Fsel48 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel48 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel48 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel48> for u64 {
        #[inline(always)]
        fn from(value: Fsel48) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel48 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel48 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_CLK"]
        pub const SD_1_CLK: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel49(u8);

    impl Fsel49 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel49 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel49 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel49> for u64 {
        #[inline(always)]
        fn from(value: Fsel49) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel49 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel49 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_CMD"]
        pub const SD_1_CMD: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
}

#[doc = "GPIO Function Select 5"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfsel5 {
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
pub struct Gpfsel5T;
unsafe impl crate::common::AsPtr for Gpfsel5T {}
impl crate::common::Reg<Gpfsel5> for Gpfsel5T {}

unsafe impl crate::common::Read<Gpfsel5> for Gpfsel5T {}
unsafe impl crate::common::Write<Gpfsel5> for Gpfsel5T {}
impl Gpfsel5 {
    #[doc = "Function Select 50"]
    #[inline(always)]
    pub fn fsel50(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        gpfsel5::Fsel50,
        gpfsel5::Fsel50,
        Gpfsel5,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel5::Fsel50,
            gpfsel5::Fsel50,
            Gpfsel5,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 51"]
    #[inline(always)]
    pub fn fsel51(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        gpfsel5::Fsel51,
        gpfsel5::Fsel51,
        Gpfsel5,
        common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel5::Fsel51,
            gpfsel5::Fsel51,
            Gpfsel5,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 52"]
    #[inline(always)]
    pub fn fsel52(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x7,
        1,
        0,
        gpfsel5::Fsel52,
        gpfsel5::Fsel52,
        Gpfsel5,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel5::Fsel52,
            gpfsel5::Fsel52,
            Gpfsel5,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Function Select 53"]
    #[inline(always)]
    pub fn fsel53(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7,
        1,
        0,
        gpfsel5::Fsel53,
        gpfsel5::Fsel53,
        Gpfsel5,
        common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel5::Fsel53,
            gpfsel5::Fsel53,
            Gpfsel5,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfsel5> for Gpfsel5T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfsel5 {
        Gpfsel5::new(0)
    }
}
pub mod gpfsel5 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel50(u8);

    impl Fsel50 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel50 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel50 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel50> for u64 {
        #[inline(always)]
        fn from(value: Fsel50) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel50 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel50 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT0"]
        pub const SD_1_DAT_0: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel51(u8);

    impl Fsel51 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel51 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel51 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel51> for u64 {
        #[inline(always)]
        fn from(value: Fsel51) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel51 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel51 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT1"]
        pub const SD_1_DAT_1: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel52(u8);

    impl Fsel52 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel52 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel52 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel52> for u64 {
        #[inline(always)]
        fn from(value: Fsel52) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel52 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel52 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT2"]
        pub const SD_1_DAT_2: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Fsel53(u8);

    impl Fsel53 {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Fsel53 {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Fsel53 {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Fsel53> for u64 {
        #[inline(always)]
        fn from(value: Fsel53) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Fsel53 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Fsel53 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self(6);

        #[doc = "Pin is connected to SD1_DAT3"]
        pub const SD_1_DAT_3: Self = Self(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self(2);
    }
}

#[doc = "GPIO Pin Output Set 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpset0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpset0 {
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
pub struct Gpset0T;
unsafe impl crate::common::AsPtr for Gpset0T {}
impl crate::common::Reg<Gpset0> for Gpset0T {}

unsafe impl crate::common::Write<Gpset0> for Gpset0T {}
impl Gpset0 {
    #[doc = "Set 0"]
    #[inline(always)]
    pub fn set0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 1"]
    #[inline(always)]
    pub fn set1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 2"]
    #[inline(always)]
    pub fn set2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 3"]
    #[inline(always)]
    pub fn set3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 4"]
    #[inline(always)]
    pub fn set4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 5"]
    #[inline(always)]
    pub fn set5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 6"]
    #[inline(always)]
    pub fn set6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 7"]
    #[inline(always)]
    pub fn set7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 8"]
    #[inline(always)]
    pub fn set8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 9"]
    #[inline(always)]
    pub fn set9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 10"]
    #[inline(always)]
    pub fn set10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 11"]
    #[inline(always)]
    pub fn set11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 12"]
    #[inline(always)]
    pub fn set12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 13"]
    #[inline(always)]
    pub fn set13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 14"]
    #[inline(always)]
    pub fn set14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 15"]
    #[inline(always)]
    pub fn set15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 16"]
    #[inline(always)]
    pub fn set16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 17"]
    #[inline(always)]
    pub fn set17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 18"]
    #[inline(always)]
    pub fn set18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 19"]
    #[inline(always)]
    pub fn set19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 20"]
    #[inline(always)]
    pub fn set20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 21"]
    #[inline(always)]
    pub fn set21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 22"]
    #[inline(always)]
    pub fn set22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 23"]
    #[inline(always)]
    pub fn set23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 24"]
    #[inline(always)]
    pub fn set24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 25"]
    #[inline(always)]
    pub fn set25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 26"]
    #[inline(always)]
    pub fn set26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 27"]
    #[inline(always)]
    pub fn set27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 28"]
    #[inline(always)]
    pub fn set28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 29"]
    #[inline(always)]
    pub fn set29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 30"]
    #[inline(always)]
    pub fn set30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }

    #[doc = "Set 31"]
    #[inline(always)]
    pub fn set31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gpset0, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gpset0, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpset0> for Gpset0T {
    #[inline(always)]
    fn reset_value(&self) -> Gpset0 {
        Gpset0::new(0)
    }
}

#[doc = "GPIO Pin Output Set 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpset1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpset1 {
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
pub struct Gpset1T;
unsafe impl crate::common::AsPtr for Gpset1T {}
impl crate::common::Reg<Gpset1> for Gpset1T {}

unsafe impl crate::common::Write<Gpset1> for Gpset1T {}
impl Gpset1 {
    #[doc = "Set 32"]
    #[inline(always)]
    pub fn set32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 33"]
    #[inline(always)]
    pub fn set33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 34"]
    #[inline(always)]
    pub fn set34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 35"]
    #[inline(always)]
    pub fn set35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 36"]
    #[inline(always)]
    pub fn set36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 37"]
    #[inline(always)]
    pub fn set37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 38"]
    #[inline(always)]
    pub fn set38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 39"]
    #[inline(always)]
    pub fn set39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 40"]
    #[inline(always)]
    pub fn set40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 41"]
    #[inline(always)]
    pub fn set41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 42"]
    #[inline(always)]
    pub fn set42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 43"]
    #[inline(always)]
    pub fn set43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 44"]
    #[inline(always)]
    pub fn set44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 45"]
    #[inline(always)]
    pub fn set45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 46"]
    #[inline(always)]
    pub fn set46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 47"]
    #[inline(always)]
    pub fn set47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 48"]
    #[inline(always)]
    pub fn set48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 49"]
    #[inline(always)]
    pub fn set49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 50"]
    #[inline(always)]
    pub fn set50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 51"]
    #[inline(always)]
    pub fn set51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 52"]
    #[inline(always)]
    pub fn set52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }

    #[doc = "Set 53"]
    #[inline(always)]
    pub fn set53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpset1, common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpset1, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpset1> for Gpset1T {
    #[inline(always)]
    fn reset_value(&self) -> Gpset1 {
        Gpset1::new(0)
    }
}

#[doc = "GPIO Pin Output Clear 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpclr0 {
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
pub struct Gpclr0T;
unsafe impl crate::common::AsPtr for Gpclr0T {}
impl crate::common::Reg<Gpclr0> for Gpclr0T {}

unsafe impl crate::common::Write<Gpclr0> for Gpclr0T {}
impl Gpclr0 {
    #[doc = "Clear 0"]
    #[inline(always)]
    pub fn clr0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 1"]
    #[inline(always)]
    pub fn clr1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 2"]
    #[inline(always)]
    pub fn clr2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 3"]
    #[inline(always)]
    pub fn clr3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 4"]
    #[inline(always)]
    pub fn clr4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 5"]
    #[inline(always)]
    pub fn clr5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 6"]
    #[inline(always)]
    pub fn clr6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 7"]
    #[inline(always)]
    pub fn clr7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 8"]
    #[inline(always)]
    pub fn clr8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 9"]
    #[inline(always)]
    pub fn clr9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 10"]
    #[inline(always)]
    pub fn clr10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 11"]
    #[inline(always)]
    pub fn clr11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 12"]
    #[inline(always)]
    pub fn clr12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 13"]
    #[inline(always)]
    pub fn clr13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 14"]
    #[inline(always)]
    pub fn clr14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 15"]
    #[inline(always)]
    pub fn clr15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 16"]
    #[inline(always)]
    pub fn clr16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 17"]
    #[inline(always)]
    pub fn clr17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 18"]
    #[inline(always)]
    pub fn clr18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 19"]
    #[inline(always)]
    pub fn clr19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 20"]
    #[inline(always)]
    pub fn clr20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 21"]
    #[inline(always)]
    pub fn clr21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 22"]
    #[inline(always)]
    pub fn clr22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 23"]
    #[inline(always)]
    pub fn clr23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 24"]
    #[inline(always)]
    pub fn clr24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 25"]
    #[inline(always)]
    pub fn clr25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 26"]
    #[inline(always)]
    pub fn clr26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 27"]
    #[inline(always)]
    pub fn clr27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 28"]
    #[inline(always)]
    pub fn clr28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 29"]
    #[inline(always)]
    pub fn clr29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 30"]
    #[inline(always)]
    pub fn clr30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 31"]
    #[inline(always)]
    pub fn clr31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gpclr0, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gpclr0, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpclr0> for Gpclr0T {
    #[inline(always)]
    fn reset_value(&self) -> Gpclr0 {
        Gpclr0::new(0)
    }
}

#[doc = "GPIO Pin Output Clear 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpclr1 {
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
pub struct Gpclr1T;
unsafe impl crate::common::AsPtr for Gpclr1T {}
impl crate::common::Reg<Gpclr1> for Gpclr1T {}

unsafe impl crate::common::Write<Gpclr1> for Gpclr1T {}
impl Gpclr1 {
    #[doc = "Clear 32"]
    #[inline(always)]
    pub fn clr32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 33"]
    #[inline(always)]
    pub fn clr33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 34"]
    #[inline(always)]
    pub fn clr34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 35"]
    #[inline(always)]
    pub fn clr35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 36"]
    #[inline(always)]
    pub fn clr36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 37"]
    #[inline(always)]
    pub fn clr37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 38"]
    #[inline(always)]
    pub fn clr38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 39"]
    #[inline(always)]
    pub fn clr39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 40"]
    #[inline(always)]
    pub fn clr40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 41"]
    #[inline(always)]
    pub fn clr41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 42"]
    #[inline(always)]
    pub fn clr42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 43"]
    #[inline(always)]
    pub fn clr43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 44"]
    #[inline(always)]
    pub fn clr44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 45"]
    #[inline(always)]
    pub fn clr45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 46"]
    #[inline(always)]
    pub fn clr46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 47"]
    #[inline(always)]
    pub fn clr47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 48"]
    #[inline(always)]
    pub fn clr48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 49"]
    #[inline(always)]
    pub fn clr49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 50"]
    #[inline(always)]
    pub fn clr50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 51"]
    #[inline(always)]
    pub fn clr51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 52"]
    #[inline(always)]
    pub fn clr52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }

    #[doc = "Clear 53"]
    #[inline(always)]
    pub fn clr53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpclr1, common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpclr1, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpclr1> for Gpclr1T {
    #[inline(always)]
    fn reset_value(&self) -> Gpclr1 {
        Gpclr1::new(0)
    }
}

#[doc = "GPIO Pin Level 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplev0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gplev0 {
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
pub struct Gplev0T;
unsafe impl crate::common::AsPtr for Gplev0T {}
impl crate::common::Reg<Gplev0> for Gplev0T {}

unsafe impl crate::common::Read<Gplev0> for Gplev0T {}
impl Gplev0 {
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn lev0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 1"]
    #[inline(always)]
    pub fn lev1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 2"]
    #[inline(always)]
    pub fn lev2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 3"]
    #[inline(always)]
    pub fn lev3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 4"]
    #[inline(always)]
    pub fn lev4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 5"]
    #[inline(always)]
    pub fn lev5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 6"]
    #[inline(always)]
    pub fn lev6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 7"]
    #[inline(always)]
    pub fn lev7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 8"]
    #[inline(always)]
    pub fn lev8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 9"]
    #[inline(always)]
    pub fn lev9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 10"]
    #[inline(always)]
    pub fn lev10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 11"]
    #[inline(always)]
    pub fn lev11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 12"]
    #[inline(always)]
    pub fn lev12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 13"]
    #[inline(always)]
    pub fn lev13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 14"]
    #[inline(always)]
    pub fn lev14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 15"]
    #[inline(always)]
    pub fn lev15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 16"]
    #[inline(always)]
    pub fn lev16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 17"]
    #[inline(always)]
    pub fn lev17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 18"]
    #[inline(always)]
    pub fn lev18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 19"]
    #[inline(always)]
    pub fn lev19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 20"]
    #[inline(always)]
    pub fn lev20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 21"]
    #[inline(always)]
    pub fn lev21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 22"]
    #[inline(always)]
    pub fn lev22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 23"]
    #[inline(always)]
    pub fn lev23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 24"]
    #[inline(always)]
    pub fn lev24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 25"]
    #[inline(always)]
    pub fn lev25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 26"]
    #[inline(always)]
    pub fn lev26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 27"]
    #[inline(always)]
    pub fn lev27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 28"]
    #[inline(always)]
    pub fn lev28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 29"]
    #[inline(always)]
    pub fn lev29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 30"]
    #[inline(always)]
    pub fn lev30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }

    #[doc = "Level 31"]
    #[inline(always)]
    pub fn lev31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gplev0, common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gplev0, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gplev0> for Gplev0T {
    #[inline(always)]
    fn reset_value(&self) -> Gplev0 {
        Gplev0::new(0)
    }
}

#[doc = "GPIO Pin Level 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplev1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gplev1 {
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
pub struct Gplev1T;
unsafe impl crate::common::AsPtr for Gplev1T {}
impl crate::common::Reg<Gplev1> for Gplev1T {}

unsafe impl crate::common::Read<Gplev1> for Gplev1T {}
impl Gplev1 {
    #[doc = "Level 32"]
    #[inline(always)]
    pub fn lev32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 33"]
    #[inline(always)]
    pub fn lev33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 34"]
    #[inline(always)]
    pub fn lev34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 35"]
    #[inline(always)]
    pub fn lev35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 36"]
    #[inline(always)]
    pub fn lev36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 37"]
    #[inline(always)]
    pub fn lev37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 38"]
    #[inline(always)]
    pub fn lev38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 39"]
    #[inline(always)]
    pub fn lev39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 40"]
    #[inline(always)]
    pub fn lev40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 41"]
    #[inline(always)]
    pub fn lev41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 42"]
    #[inline(always)]
    pub fn lev42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 43"]
    #[inline(always)]
    pub fn lev43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 44"]
    #[inline(always)]
    pub fn lev44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 45"]
    #[inline(always)]
    pub fn lev45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 46"]
    #[inline(always)]
    pub fn lev46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 47"]
    #[inline(always)]
    pub fn lev47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 48"]
    #[inline(always)]
    pub fn lev48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 49"]
    #[inline(always)]
    pub fn lev49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 50"]
    #[inline(always)]
    pub fn lev50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 51"]
    #[inline(always)]
    pub fn lev51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 52"]
    #[inline(always)]
    pub fn lev52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }

    #[doc = "Level 53"]
    #[inline(always)]
    pub fn lev53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gplev1, common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gplev1, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gplev1> for Gplev1T {
    #[inline(always)]
    fn reset_value(&self) -> Gplev1 {
        Gplev1::new(0)
    }
}

#[doc = "GPIO Pin Event Detect Status 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpeds0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpeds0 {
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
pub struct Gpeds0T;
unsafe impl crate::common::AsPtr for Gpeds0T {}
impl crate::common::Reg<Gpeds0> for Gpeds0T {}

unsafe impl crate::common::Read<Gpeds0> for Gpeds0T {}
unsafe impl crate::common::Write<Gpeds0> for Gpeds0T {}
impl Gpeds0 {
    #[doc = "Event detected 0"]
    #[inline(always)]
    pub fn eds0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 1"]
    #[inline(always)]
    pub fn eds1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 2"]
    #[inline(always)]
    pub fn eds2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 3"]
    #[inline(always)]
    pub fn eds3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 4"]
    #[inline(always)]
    pub fn eds4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 5"]
    #[inline(always)]
    pub fn eds5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 6"]
    #[inline(always)]
    pub fn eds6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 7"]
    #[inline(always)]
    pub fn eds7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 8"]
    #[inline(always)]
    pub fn eds8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 9"]
    #[inline(always)]
    pub fn eds9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 10"]
    #[inline(always)]
    pub fn eds10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 11"]
    #[inline(always)]
    pub fn eds11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 12"]
    #[inline(always)]
    pub fn eds12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 13"]
    #[inline(always)]
    pub fn eds13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 14"]
    #[inline(always)]
    pub fn eds14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 15"]
    #[inline(always)]
    pub fn eds15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 16"]
    #[inline(always)]
    pub fn eds16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 17"]
    #[inline(always)]
    pub fn eds17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 18"]
    #[inline(always)]
    pub fn eds18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 19"]
    #[inline(always)]
    pub fn eds19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 20"]
    #[inline(always)]
    pub fn eds20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 21"]
    #[inline(always)]
    pub fn eds21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 22"]
    #[inline(always)]
    pub fn eds22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 23"]
    #[inline(always)]
    pub fn eds23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 24"]
    #[inline(always)]
    pub fn eds24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 25"]
    #[inline(always)]
    pub fn eds25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 26"]
    #[inline(always)]
    pub fn eds26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 27"]
    #[inline(always)]
    pub fn eds27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 28"]
    #[inline(always)]
    pub fn eds28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 29"]
    #[inline(always)]
    pub fn eds29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 30"]
    #[inline(always)]
    pub fn eds30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 31"]
    #[inline(always)]
    pub fn eds31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gpeds0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gpeds0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpeds0> for Gpeds0T {
    #[inline(always)]
    fn reset_value(&self) -> Gpeds0 {
        Gpeds0::new(0)
    }
}

#[doc = "GPIO Pin Event Detect Status 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpeds1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpeds1 {
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
pub struct Gpeds1T;
unsafe impl crate::common::AsPtr for Gpeds1T {}
impl crate::common::Reg<Gpeds1> for Gpeds1T {}

unsafe impl crate::common::Read<Gpeds1> for Gpeds1T {}
unsafe impl crate::common::Write<Gpeds1> for Gpeds1T {}
impl Gpeds1 {
    #[doc = "Event detected 32"]
    #[inline(always)]
    pub fn eds32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 33"]
    #[inline(always)]
    pub fn eds33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 34"]
    #[inline(always)]
    pub fn eds34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 35"]
    #[inline(always)]
    pub fn eds35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 36"]
    #[inline(always)]
    pub fn eds36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 37"]
    #[inline(always)]
    pub fn eds37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 38"]
    #[inline(always)]
    pub fn eds38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 39"]
    #[inline(always)]
    pub fn eds39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 40"]
    #[inline(always)]
    pub fn eds40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 41"]
    #[inline(always)]
    pub fn eds41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 42"]
    #[inline(always)]
    pub fn eds42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 43"]
    #[inline(always)]
    pub fn eds43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 44"]
    #[inline(always)]
    pub fn eds44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 45"]
    #[inline(always)]
    pub fn eds45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 46"]
    #[inline(always)]
    pub fn eds46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 47"]
    #[inline(always)]
    pub fn eds47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 48"]
    #[inline(always)]
    pub fn eds48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 49"]
    #[inline(always)]
    pub fn eds49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 50"]
    #[inline(always)]
    pub fn eds50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 51"]
    #[inline(always)]
    pub fn eds51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 52"]
    #[inline(always)]
    pub fn eds52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }

    #[doc = "Event detected 53"]
    #[inline(always)]
    pub fn eds53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpeds1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpeds1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpeds1> for Gpeds1T {
    #[inline(always)]
    fn reset_value(&self) -> Gpeds1 {
        Gpeds1::new(0)
    }
}

#[doc = "GPIO Pin Rising Edge Detect Enable 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpren0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpren0 {
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
pub struct Gpren0T;
unsafe impl crate::common::AsPtr for Gpren0T {}
impl crate::common::Reg<Gpren0> for Gpren0T {}

unsafe impl crate::common::Read<Gpren0> for Gpren0T {}
unsafe impl crate::common::Write<Gpren0> for Gpren0T {}
impl Gpren0 {
    #[doc = "Rising edge enabled 0"]
    #[inline(always)]
    pub fn ren0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 1"]
    #[inline(always)]
    pub fn ren1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 2"]
    #[inline(always)]
    pub fn ren2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 3"]
    #[inline(always)]
    pub fn ren3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 4"]
    #[inline(always)]
    pub fn ren4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 5"]
    #[inline(always)]
    pub fn ren5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 6"]
    #[inline(always)]
    pub fn ren6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 7"]
    #[inline(always)]
    pub fn ren7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 8"]
    #[inline(always)]
    pub fn ren8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 9"]
    #[inline(always)]
    pub fn ren9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 10"]
    #[inline(always)]
    pub fn ren10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 11"]
    #[inline(always)]
    pub fn ren11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 12"]
    #[inline(always)]
    pub fn ren12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 13"]
    #[inline(always)]
    pub fn ren13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 14"]
    #[inline(always)]
    pub fn ren14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 15"]
    #[inline(always)]
    pub fn ren15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 16"]
    #[inline(always)]
    pub fn ren16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 17"]
    #[inline(always)]
    pub fn ren17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 18"]
    #[inline(always)]
    pub fn ren18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 19"]
    #[inline(always)]
    pub fn ren19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 20"]
    #[inline(always)]
    pub fn ren20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 21"]
    #[inline(always)]
    pub fn ren21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 22"]
    #[inline(always)]
    pub fn ren22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 23"]
    #[inline(always)]
    pub fn ren23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 24"]
    #[inline(always)]
    pub fn ren24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 25"]
    #[inline(always)]
    pub fn ren25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 26"]
    #[inline(always)]
    pub fn ren26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 27"]
    #[inline(always)]
    pub fn ren27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 28"]
    #[inline(always)]
    pub fn ren28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 29"]
    #[inline(always)]
    pub fn ren29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 30"]
    #[inline(always)]
    pub fn ren30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 31"]
    #[inline(always)]
    pub fn ren31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gpren0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gpren0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpren0> for Gpren0T {
    #[inline(always)]
    fn reset_value(&self) -> Gpren0 {
        Gpren0::new(0)
    }
}

#[doc = "GPIO Pin Rising Edge Detect Enable 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpren1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpren1 {
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
pub struct Gpren1T;
unsafe impl crate::common::AsPtr for Gpren1T {}
impl crate::common::Reg<Gpren1> for Gpren1T {}

unsafe impl crate::common::Read<Gpren1> for Gpren1T {}
unsafe impl crate::common::Write<Gpren1> for Gpren1T {}
impl Gpren1 {
    #[doc = "Rising edge enabled 32"]
    #[inline(always)]
    pub fn ren32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 33"]
    #[inline(always)]
    pub fn ren33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 34"]
    #[inline(always)]
    pub fn ren34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 35"]
    #[inline(always)]
    pub fn ren35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 36"]
    #[inline(always)]
    pub fn ren36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 37"]
    #[inline(always)]
    pub fn ren37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 38"]
    #[inline(always)]
    pub fn ren38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 39"]
    #[inline(always)]
    pub fn ren39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 40"]
    #[inline(always)]
    pub fn ren40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 41"]
    #[inline(always)]
    pub fn ren41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 42"]
    #[inline(always)]
    pub fn ren42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 43"]
    #[inline(always)]
    pub fn ren43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 44"]
    #[inline(always)]
    pub fn ren44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 45"]
    #[inline(always)]
    pub fn ren45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 46"]
    #[inline(always)]
    pub fn ren46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 47"]
    #[inline(always)]
    pub fn ren47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 48"]
    #[inline(always)]
    pub fn ren48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 49"]
    #[inline(always)]
    pub fn ren49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 50"]
    #[inline(always)]
    pub fn ren50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 51"]
    #[inline(always)]
    pub fn ren51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 52"]
    #[inline(always)]
    pub fn ren52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }

    #[doc = "Rising edge enabled 53"]
    #[inline(always)]
    pub fn ren53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpren1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpren1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpren1> for Gpren1T {
    #[inline(always)]
    fn reset_value(&self) -> Gpren1 {
        Gpren1::new(0)
    }
}

#[doc = "GPIO Pin Falling Edge Detect Enable 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfen0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfen0 {
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
pub struct Gpfen0T;
unsafe impl crate::common::AsPtr for Gpfen0T {}
impl crate::common::Reg<Gpfen0> for Gpfen0T {}

unsafe impl crate::common::Read<Gpfen0> for Gpfen0T {}
unsafe impl crate::common::Write<Gpfen0> for Gpfen0T {}
impl Gpfen0 {
    #[doc = "Falling edge enabled 0"]
    #[inline(always)]
    pub fn fen0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 1"]
    #[inline(always)]
    pub fn fen1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 2"]
    #[inline(always)]
    pub fn fen2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 3"]
    #[inline(always)]
    pub fn fen3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 4"]
    #[inline(always)]
    pub fn fen4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 5"]
    #[inline(always)]
    pub fn fen5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 6"]
    #[inline(always)]
    pub fn fen6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 7"]
    #[inline(always)]
    pub fn fen7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 8"]
    #[inline(always)]
    pub fn fen8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 9"]
    #[inline(always)]
    pub fn fen9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 10"]
    #[inline(always)]
    pub fn fen10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 11"]
    #[inline(always)]
    pub fn fen11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 12"]
    #[inline(always)]
    pub fn fen12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 13"]
    #[inline(always)]
    pub fn fen13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 14"]
    #[inline(always)]
    pub fn fen14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 15"]
    #[inline(always)]
    pub fn fen15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 16"]
    #[inline(always)]
    pub fn fen16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 17"]
    #[inline(always)]
    pub fn fen17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 18"]
    #[inline(always)]
    pub fn fen18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 19"]
    #[inline(always)]
    pub fn fen19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 20"]
    #[inline(always)]
    pub fn fen20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 21"]
    #[inline(always)]
    pub fn fen21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 22"]
    #[inline(always)]
    pub fn fen22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 23"]
    #[inline(always)]
    pub fn fen23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 24"]
    #[inline(always)]
    pub fn fen24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 25"]
    #[inline(always)]
    pub fn fen25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 26"]
    #[inline(always)]
    pub fn fen26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 27"]
    #[inline(always)]
    pub fn fen27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 28"]
    #[inline(always)]
    pub fn fen28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 29"]
    #[inline(always)]
    pub fn fen29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 30"]
    #[inline(always)]
    pub fn fen30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 31"]
    #[inline(always)]
    pub fn fen31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gpfen0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gpfen0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfen0> for Gpfen0T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfen0 {
        Gpfen0::new(0)
    }
}

#[doc = "GPIO Pin Falling Edge Detect Enable 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfen1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpfen1 {
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
pub struct Gpfen1T;
unsafe impl crate::common::AsPtr for Gpfen1T {}
impl crate::common::Reg<Gpfen1> for Gpfen1T {}

unsafe impl crate::common::Read<Gpfen1> for Gpfen1T {}
unsafe impl crate::common::Write<Gpfen1> for Gpfen1T {}
impl Gpfen1 {
    #[doc = "Falling edge enabled 32"]
    #[inline(always)]
    pub fn fen32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 33"]
    #[inline(always)]
    pub fn fen33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 34"]
    #[inline(always)]
    pub fn fen34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 35"]
    #[inline(always)]
    pub fn fen35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 36"]
    #[inline(always)]
    pub fn fen36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 37"]
    #[inline(always)]
    pub fn fen37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 38"]
    #[inline(always)]
    pub fn fen38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 39"]
    #[inline(always)]
    pub fn fen39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 40"]
    #[inline(always)]
    pub fn fen40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 41"]
    #[inline(always)]
    pub fn fen41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 42"]
    #[inline(always)]
    pub fn fen42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 43"]
    #[inline(always)]
    pub fn fen43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 44"]
    #[inline(always)]
    pub fn fen44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 45"]
    #[inline(always)]
    pub fn fen45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 46"]
    #[inline(always)]
    pub fn fen46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 47"]
    #[inline(always)]
    pub fn fen47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 48"]
    #[inline(always)]
    pub fn fen48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 49"]
    #[inline(always)]
    pub fn fen49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 50"]
    #[inline(always)]
    pub fn fen50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 51"]
    #[inline(always)]
    pub fn fen51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 52"]
    #[inline(always)]
    pub fn fen52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Falling edge enabled 53"]
    #[inline(always)]
    pub fn fen53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpfen1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpfen1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpfen1> for Gpfen1T {
    #[inline(always)]
    fn reset_value(&self) -> Gpfen1 {
        Gpfen1::new(0)
    }
}

#[doc = "GPIO Pin High Detect Enable 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gphen0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gphen0 {
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
pub struct Gphen0T;
unsafe impl crate::common::AsPtr for Gphen0T {}
impl crate::common::Reg<Gphen0> for Gphen0T {}

unsafe impl crate::common::Read<Gphen0> for Gphen0T {}
unsafe impl crate::common::Write<Gphen0> for Gphen0T {}
impl Gphen0 {
    #[doc = "High detect enabled 0"]
    #[inline(always)]
    pub fn hen0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 1"]
    #[inline(always)]
    pub fn hen1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 2"]
    #[inline(always)]
    pub fn hen2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 3"]
    #[inline(always)]
    pub fn hen3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 4"]
    #[inline(always)]
    pub fn hen4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 5"]
    #[inline(always)]
    pub fn hen5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 6"]
    #[inline(always)]
    pub fn hen6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 7"]
    #[inline(always)]
    pub fn hen7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 8"]
    #[inline(always)]
    pub fn hen8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 9"]
    #[inline(always)]
    pub fn hen9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 10"]
    #[inline(always)]
    pub fn hen10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 11"]
    #[inline(always)]
    pub fn hen11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 12"]
    #[inline(always)]
    pub fn hen12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 13"]
    #[inline(always)]
    pub fn hen13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 14"]
    #[inline(always)]
    pub fn hen14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 15"]
    #[inline(always)]
    pub fn hen15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 16"]
    #[inline(always)]
    pub fn hen16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 17"]
    #[inline(always)]
    pub fn hen17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 18"]
    #[inline(always)]
    pub fn hen18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 19"]
    #[inline(always)]
    pub fn hen19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 20"]
    #[inline(always)]
    pub fn hen20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 21"]
    #[inline(always)]
    pub fn hen21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 22"]
    #[inline(always)]
    pub fn hen22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 23"]
    #[inline(always)]
    pub fn hen23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 24"]
    #[inline(always)]
    pub fn hen24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 25"]
    #[inline(always)]
    pub fn hen25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 26"]
    #[inline(always)]
    pub fn hen26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 27"]
    #[inline(always)]
    pub fn hen27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 28"]
    #[inline(always)]
    pub fn hen28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 29"]
    #[inline(always)]
    pub fn hen29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 30"]
    #[inline(always)]
    pub fn hen30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 31"]
    #[inline(always)]
    pub fn hen31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gphen0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gphen0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gphen0> for Gphen0T {
    #[inline(always)]
    fn reset_value(&self) -> Gphen0 {
        Gphen0::new(0)
    }
}

#[doc = "GPIO Pin High Detect Enable 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gphen1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gphen1 {
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
pub struct Gphen1T;
unsafe impl crate::common::AsPtr for Gphen1T {}
impl crate::common::Reg<Gphen1> for Gphen1T {}

unsafe impl crate::common::Read<Gphen1> for Gphen1T {}
unsafe impl crate::common::Write<Gphen1> for Gphen1T {}
impl Gphen1 {
    #[doc = "High detect enabled 32"]
    #[inline(always)]
    pub fn hen32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 33"]
    #[inline(always)]
    pub fn hen33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 34"]
    #[inline(always)]
    pub fn hen34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 35"]
    #[inline(always)]
    pub fn hen35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 36"]
    #[inline(always)]
    pub fn hen36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 37"]
    #[inline(always)]
    pub fn hen37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 38"]
    #[inline(always)]
    pub fn hen38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 39"]
    #[inline(always)]
    pub fn hen39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 40"]
    #[inline(always)]
    pub fn hen40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 41"]
    #[inline(always)]
    pub fn hen41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 42"]
    #[inline(always)]
    pub fn hen42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 43"]
    #[inline(always)]
    pub fn hen43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 44"]
    #[inline(always)]
    pub fn hen44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 45"]
    #[inline(always)]
    pub fn hen45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 46"]
    #[inline(always)]
    pub fn hen46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 47"]
    #[inline(always)]
    pub fn hen47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 48"]
    #[inline(always)]
    pub fn hen48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 49"]
    #[inline(always)]
    pub fn hen49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 50"]
    #[inline(always)]
    pub fn hen50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 51"]
    #[inline(always)]
    pub fn hen51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 52"]
    #[inline(always)]
    pub fn hen52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }

    #[doc = "High detect enabled 53"]
    #[inline(always)]
    pub fn hen53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gphen1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gphen1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gphen1> for Gphen1T {
    #[inline(always)]
    fn reset_value(&self) -> Gphen1 {
        Gphen1::new(0)
    }
}

#[doc = "GPIO Pin Low Detect Enable 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplen0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gplen0 {
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
pub struct Gplen0T;
unsafe impl crate::common::AsPtr for Gplen0T {}
impl crate::common::Reg<Gplen0> for Gplen0T {}

unsafe impl crate::common::Read<Gplen0> for Gplen0T {}
unsafe impl crate::common::Write<Gplen0> for Gplen0T {}
impl Gplen0 {
    #[doc = "Low detect enabled 0"]
    #[inline(always)]
    pub fn len0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 1"]
    #[inline(always)]
    pub fn len1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 2"]
    #[inline(always)]
    pub fn len2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 3"]
    #[inline(always)]
    pub fn len3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 4"]
    #[inline(always)]
    pub fn len4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 5"]
    #[inline(always)]
    pub fn len5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 6"]
    #[inline(always)]
    pub fn len6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 7"]
    #[inline(always)]
    pub fn len7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 8"]
    #[inline(always)]
    pub fn len8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 9"]
    #[inline(always)]
    pub fn len9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 10"]
    #[inline(always)]
    pub fn len10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 11"]
    #[inline(always)]
    pub fn len11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 12"]
    #[inline(always)]
    pub fn len12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 13"]
    #[inline(always)]
    pub fn len13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 14"]
    #[inline(always)]
    pub fn len14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 15"]
    #[inline(always)]
    pub fn len15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 16"]
    #[inline(always)]
    pub fn len16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 17"]
    #[inline(always)]
    pub fn len17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 18"]
    #[inline(always)]
    pub fn len18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 19"]
    #[inline(always)]
    pub fn len19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 20"]
    #[inline(always)]
    pub fn len20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 21"]
    #[inline(always)]
    pub fn len21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 22"]
    #[inline(always)]
    pub fn len22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 23"]
    #[inline(always)]
    pub fn len23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 24"]
    #[inline(always)]
    pub fn len24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 25"]
    #[inline(always)]
    pub fn len25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 26"]
    #[inline(always)]
    pub fn len26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 27"]
    #[inline(always)]
    pub fn len27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 28"]
    #[inline(always)]
    pub fn len28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 29"]
    #[inline(always)]
    pub fn len29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 30"]
    #[inline(always)]
    pub fn len30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 31"]
    #[inline(always)]
    pub fn len31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gplen0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gplen0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gplen0> for Gplen0T {
    #[inline(always)]
    fn reset_value(&self) -> Gplen0 {
        Gplen0::new(0)
    }
}

#[doc = "GPIO Pin Low Detect Enable 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplen1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gplen1 {
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
pub struct Gplen1T;
unsafe impl crate::common::AsPtr for Gplen1T {}
impl crate::common::Reg<Gplen1> for Gplen1T {}

unsafe impl crate::common::Read<Gplen1> for Gplen1T {}
unsafe impl crate::common::Write<Gplen1> for Gplen1T {}
impl Gplen1 {
    #[doc = "Low detect enabled 32"]
    #[inline(always)]
    pub fn len32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 33"]
    #[inline(always)]
    pub fn len33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 34"]
    #[inline(always)]
    pub fn len34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 35"]
    #[inline(always)]
    pub fn len35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 36"]
    #[inline(always)]
    pub fn len36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 37"]
    #[inline(always)]
    pub fn len37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 38"]
    #[inline(always)]
    pub fn len38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 39"]
    #[inline(always)]
    pub fn len39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 40"]
    #[inline(always)]
    pub fn len40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 41"]
    #[inline(always)]
    pub fn len41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 42"]
    #[inline(always)]
    pub fn len42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 43"]
    #[inline(always)]
    pub fn len43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 44"]
    #[inline(always)]
    pub fn len44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 45"]
    #[inline(always)]
    pub fn len45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 46"]
    #[inline(always)]
    pub fn len46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 47"]
    #[inline(always)]
    pub fn len47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 48"]
    #[inline(always)]
    pub fn len48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 49"]
    #[inline(always)]
    pub fn len49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 50"]
    #[inline(always)]
    pub fn len50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 51"]
    #[inline(always)]
    pub fn len51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 52"]
    #[inline(always)]
    pub fn len52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Low detect enabled 53"]
    #[inline(always)]
    pub fn len53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gplen1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gplen1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gplen1> for Gplen1T {
    #[inline(always)]
    fn reset_value(&self) -> Gplen1 {
        Gplen1::new(0)
    }
}

#[doc = "GPIO Pin Async. Rising Edge Detect 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gparen0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gparen0 {
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
pub struct Gparen0T;
unsafe impl crate::common::AsPtr for Gparen0T {}
impl crate::common::Reg<Gparen0> for Gparen0T {}

unsafe impl crate::common::Read<Gparen0> for Gparen0T {}
unsafe impl crate::common::Write<Gparen0> for Gparen0T {}
impl Gparen0 {
    #[doc = "Async rising enabled 0"]
    #[inline(always)]
    pub fn aren0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 1"]
    #[inline(always)]
    pub fn aren1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 2"]
    #[inline(always)]
    pub fn aren2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 3"]
    #[inline(always)]
    pub fn aren3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 4"]
    #[inline(always)]
    pub fn aren4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 5"]
    #[inline(always)]
    pub fn aren5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 6"]
    #[inline(always)]
    pub fn aren6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 7"]
    #[inline(always)]
    pub fn aren7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 8"]
    #[inline(always)]
    pub fn aren8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 9"]
    #[inline(always)]
    pub fn aren9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 10"]
    #[inline(always)]
    pub fn aren10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 11"]
    #[inline(always)]
    pub fn aren11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 12"]
    #[inline(always)]
    pub fn aren12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 13"]
    #[inline(always)]
    pub fn aren13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 14"]
    #[inline(always)]
    pub fn aren14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 15"]
    #[inline(always)]
    pub fn aren15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 16"]
    #[inline(always)]
    pub fn aren16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 17"]
    #[inline(always)]
    pub fn aren17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 18"]
    #[inline(always)]
    pub fn aren18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 19"]
    #[inline(always)]
    pub fn aren19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 20"]
    #[inline(always)]
    pub fn aren20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 21"]
    #[inline(always)]
    pub fn aren21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 22"]
    #[inline(always)]
    pub fn aren22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 23"]
    #[inline(always)]
    pub fn aren23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 24"]
    #[inline(always)]
    pub fn aren24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 25"]
    #[inline(always)]
    pub fn aren25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 26"]
    #[inline(always)]
    pub fn aren26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 27"]
    #[inline(always)]
    pub fn aren27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 28"]
    #[inline(always)]
    pub fn aren28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 29"]
    #[inline(always)]
    pub fn aren29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 30"]
    #[inline(always)]
    pub fn aren30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 31"]
    #[inline(always)]
    pub fn aren31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gparen0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gparen0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gparen0> for Gparen0T {
    #[inline(always)]
    fn reset_value(&self) -> Gparen0 {
        Gparen0::new(0)
    }
}

#[doc = "GPIO Pin Async. Rising Edge Detect 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gparen1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gparen1 {
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
pub struct Gparen1T;
unsafe impl crate::common::AsPtr for Gparen1T {}
impl crate::common::Reg<Gparen1> for Gparen1T {}

unsafe impl crate::common::Read<Gparen1> for Gparen1T {}
unsafe impl crate::common::Write<Gparen1> for Gparen1T {}
impl Gparen1 {
    #[doc = "Async rising enabled 32"]
    #[inline(always)]
    pub fn aren32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 33"]
    #[inline(always)]
    pub fn aren33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 34"]
    #[inline(always)]
    pub fn aren34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 35"]
    #[inline(always)]
    pub fn aren35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 36"]
    #[inline(always)]
    pub fn aren36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 37"]
    #[inline(always)]
    pub fn aren37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 38"]
    #[inline(always)]
    pub fn aren38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 39"]
    #[inline(always)]
    pub fn aren39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 40"]
    #[inline(always)]
    pub fn aren40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 41"]
    #[inline(always)]
    pub fn aren41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 42"]
    #[inline(always)]
    pub fn aren42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 43"]
    #[inline(always)]
    pub fn aren43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 44"]
    #[inline(always)]
    pub fn aren44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 45"]
    #[inline(always)]
    pub fn aren45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 46"]
    #[inline(always)]
    pub fn aren46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 47"]
    #[inline(always)]
    pub fn aren47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 48"]
    #[inline(always)]
    pub fn aren48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 49"]
    #[inline(always)]
    pub fn aren49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 50"]
    #[inline(always)]
    pub fn aren50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 51"]
    #[inline(always)]
    pub fn aren51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 52"]
    #[inline(always)]
    pub fn aren52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async rising enabled 53"]
    #[inline(always)]
    pub fn aren53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gparen1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gparen1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gparen1> for Gparen1T {
    #[inline(always)]
    fn reset_value(&self) -> Gparen1 {
        Gparen1::new(0)
    }
}

#[doc = "GPIO Pin Async. Falling Edge Detect 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpafen0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpafen0 {
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
pub struct Gpafen0T;
unsafe impl crate::common::AsPtr for Gpafen0T {}
impl crate::common::Reg<Gpafen0> for Gpafen0T {}

unsafe impl crate::common::Read<Gpafen0> for Gpafen0T {}
unsafe impl crate::common::Write<Gpafen0> for Gpafen0T {}
impl Gpafen0 {
    #[doc = "Async falling enabled 0"]
    #[inline(always)]
    pub fn afen0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 1"]
    #[inline(always)]
    pub fn afen1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 2"]
    #[inline(always)]
    pub fn afen2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 3"]
    #[inline(always)]
    pub fn afen3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 4"]
    #[inline(always)]
    pub fn afen4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 5"]
    #[inline(always)]
    pub fn afen5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 6"]
    #[inline(always)]
    pub fn afen6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 7"]
    #[inline(always)]
    pub fn afen7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 8"]
    #[inline(always)]
    pub fn afen8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 9"]
    #[inline(always)]
    pub fn afen9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 10"]
    #[inline(always)]
    pub fn afen10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 11"]
    #[inline(always)]
    pub fn afen11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 12"]
    #[inline(always)]
    pub fn afen12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 13"]
    #[inline(always)]
    pub fn afen13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 14"]
    #[inline(always)]
    pub fn afen14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 15"]
    #[inline(always)]
    pub fn afen15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 16"]
    #[inline(always)]
    pub fn afen16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 17"]
    #[inline(always)]
    pub fn afen17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 18"]
    #[inline(always)]
    pub fn afen18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 19"]
    #[inline(always)]
    pub fn afen19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 20"]
    #[inline(always)]
    pub fn afen20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 21"]
    #[inline(always)]
    pub fn afen21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 22"]
    #[inline(always)]
    pub fn afen22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 23"]
    #[inline(always)]
    pub fn afen23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 24"]
    #[inline(always)]
    pub fn afen24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 25"]
    #[inline(always)]
    pub fn afen25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 26"]
    #[inline(always)]
    pub fn afen26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 27"]
    #[inline(always)]
    pub fn afen27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 28"]
    #[inline(always)]
    pub fn afen28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 29"]
    #[inline(always)]
    pub fn afen29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 30"]
    #[inline(always)]
    pub fn afen30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 31"]
    #[inline(always)]
    pub fn afen31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gpafen0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gpafen0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpafen0> for Gpafen0T {
    #[inline(always)]
    fn reset_value(&self) -> Gpafen0 {
        Gpafen0::new(0)
    }
}

#[doc = "GPIO Pin Async. Falling Edge Detect 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpafen1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gpafen1 {
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
pub struct Gpafen1T;
unsafe impl crate::common::AsPtr for Gpafen1T {}
impl crate::common::Reg<Gpafen1> for Gpafen1T {}

unsafe impl crate::common::Read<Gpafen1> for Gpafen1T {}
unsafe impl crate::common::Write<Gpafen1> for Gpafen1T {}
impl Gpafen1 {
    #[doc = "Async falling enabled 32"]
    #[inline(always)]
    pub fn afen32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 33"]
    #[inline(always)]
    pub fn afen33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 34"]
    #[inline(always)]
    pub fn afen34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 35"]
    #[inline(always)]
    pub fn afen35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 36"]
    #[inline(always)]
    pub fn afen36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 37"]
    #[inline(always)]
    pub fn afen37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 38"]
    #[inline(always)]
    pub fn afen38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 39"]
    #[inline(always)]
    pub fn afen39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 40"]
    #[inline(always)]
    pub fn afen40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 41"]
    #[inline(always)]
    pub fn afen41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 42"]
    #[inline(always)]
    pub fn afen42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 43"]
    #[inline(always)]
    pub fn afen43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 44"]
    #[inline(always)]
    pub fn afen44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 45"]
    #[inline(always)]
    pub fn afen45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 46"]
    #[inline(always)]
    pub fn afen46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 47"]
    #[inline(always)]
    pub fn afen47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 48"]
    #[inline(always)]
    pub fn afen48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 49"]
    #[inline(always)]
    pub fn afen49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 50"]
    #[inline(always)]
    pub fn afen50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 51"]
    #[inline(always)]
    pub fn afen51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 52"]
    #[inline(always)]
    pub fn afen52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }

    #[doc = "Async falling enabled 53"]
    #[inline(always)]
    pub fn afen53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gpafen1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gpafen1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gpafen1> for Gpafen1T {
    #[inline(always)]
    fn reset_value(&self) -> Gpafen1 {
        Gpafen1::new(0)
    }
}

#[doc = "Undocumented multiplexing bits"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtraMux {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ExtraMux {
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
pub struct ExtraMuxT;
unsafe impl crate::common::AsPtr for ExtraMuxT {}
impl crate::common::Reg<ExtraMux> for ExtraMuxT {}

unsafe impl crate::common::Read<ExtraMux> for ExtraMuxT {}
unsafe impl crate::common::Write<ExtraMux> for ExtraMuxT {}
impl ExtraMux {
    #[doc = "Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
    #[inline(always)]
    pub fn sdio(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        extra_mux::Sdio,
        extra_mux::Sdio,
        ExtraMux,
        common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            extra_mux::Sdio,
            extra_mux::Sdio,
            ExtraMux,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<ExtraMux> for ExtraMuxT {
    #[inline(always)]
    fn reset_value(&self) -> ExtraMux {
        ExtraMux::new(0)
    }
}
pub mod extra_mux {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Sdio(u8);

    impl Sdio {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Sdio {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Sdio {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Sdio> for u64 {
        #[inline(always)]
        fn from(value: Sdio) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Sdio {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Sdio {
        #[doc = "Connect the newer SD host"]
        pub const SDHOST: Self = Self(0);

        #[doc = "Connect Arasan SD/EMMC host"]
        pub const ARASAN: Self = Self(1);
    }
}

#[doc = "GPIO Pin Pull-up/down Enable"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gppud {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gppud {
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
pub struct GppudT;
unsafe impl crate::common::AsPtr for GppudT {}
impl crate::common::Reg<Gppud> for GppudT {}

unsafe impl crate::common::Read<Gppud> for GppudT {}
unsafe impl crate::common::Write<Gppud> for GppudT {}
impl Gppud {
    #[doc = "GPIO Pin Pull-up/down"]
    #[inline(always)]
    pub fn pud(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, gppud::Pud, gppud::Pud, Gppud, common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,gppud::Pud,gppud::Pud,Gppud,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Gppud> for GppudT {
    #[inline(always)]
    fn reset_value(&self) -> Gppud {
        Gppud::new(0)
    }
}
pub mod gppud {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Pud(u8);

    impl Pud {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Pud {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Pud {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Pud> for u64 {
        #[inline(always)]
        fn from(value: Pud) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Pud {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Pud {
        #[doc = "No pull"]
        pub const BP_PULL_NONE: Self = Self(0);

        #[doc = "Pull down"]
        pub const BP_PULL_DOWN: Self = Self(1);

        #[doc = "Pull up"]
        pub const BP_PULL_UP: Self = Self(2);
    }
}

#[doc = "GPIO Pin Pull-up/down Enable Clock 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gppudclk0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gppudclk0 {
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
pub struct Gppudclk0T;
unsafe impl crate::common::AsPtr for Gppudclk0T {}
impl crate::common::Reg<Gppudclk0> for Gppudclk0T {}

unsafe impl crate::common::Read<Gppudclk0> for Gppudclk0T {}
unsafe impl crate::common::Write<Gppudclk0> for Gppudclk0T {}
impl Gppudclk0 {
    #[doc = "Assert Clock on line 0"]
    #[inline(always)]
    pub fn pudclk0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 1"]
    #[inline(always)]
    pub fn pudclk1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 2"]
    #[inline(always)]
    pub fn pudclk2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 3"]
    #[inline(always)]
    pub fn pudclk3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 4"]
    #[inline(always)]
    pub fn pudclk4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 5"]
    #[inline(always)]
    pub fn pudclk5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 6"]
    #[inline(always)]
    pub fn pudclk6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 7"]
    #[inline(always)]
    pub fn pudclk7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 8"]
    #[inline(always)]
    pub fn pudclk8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 9"]
    #[inline(always)]
    pub fn pudclk9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 10"]
    #[inline(always)]
    pub fn pudclk10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 11"]
    #[inline(always)]
    pub fn pudclk11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 12"]
    #[inline(always)]
    pub fn pudclk12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 13"]
    #[inline(always)]
    pub fn pudclk13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 14"]
    #[inline(always)]
    pub fn pudclk14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 15"]
    #[inline(always)]
    pub fn pudclk15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 16"]
    #[inline(always)]
    pub fn pudclk16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 17"]
    #[inline(always)]
    pub fn pudclk17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 18"]
    #[inline(always)]
    pub fn pudclk18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 19"]
    #[inline(always)]
    pub fn pudclk19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 20"]
    #[inline(always)]
    pub fn pudclk20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 21"]
    #[inline(always)]
    pub fn pudclk21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 22"]
    #[inline(always)]
    pub fn pudclk22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 23"]
    #[inline(always)]
    pub fn pudclk23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 24"]
    #[inline(always)]
    pub fn pudclk24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 25"]
    #[inline(always)]
    pub fn pudclk25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 26"]
    #[inline(always)]
    pub fn pudclk26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 27"]
    #[inline(always)]
    pub fn pudclk27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 28"]
    #[inline(always)]
    pub fn pudclk28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 29"]
    #[inline(always)]
    pub fn pudclk29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 30"]
    #[inline(always)]
    pub fn pudclk30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 31"]
    #[inline(always)]
    pub fn pudclk31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gppudclk0, common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gppudclk0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gppudclk0> for Gppudclk0T {
    #[inline(always)]
    fn reset_value(&self) -> Gppudclk0 {
        Gppudclk0::new(0)
    }
}

#[doc = "GPIO Pin Pull-up/down Enable Clock 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gppudclk1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Gppudclk1 {
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
pub struct Gppudclk1T;
unsafe impl crate::common::AsPtr for Gppudclk1T {}
impl crate::common::Reg<Gppudclk1> for Gppudclk1T {}

unsafe impl crate::common::Read<Gppudclk1> for Gppudclk1T {}
unsafe impl crate::common::Write<Gppudclk1> for Gppudclk1T {}
impl Gppudclk1 {
    #[doc = "Assert Clock on line 32"]
    #[inline(always)]
    pub fn pudclk32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 33"]
    #[inline(always)]
    pub fn pudclk33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 34"]
    #[inline(always)]
    pub fn pudclk34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 35"]
    #[inline(always)]
    pub fn pudclk35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 36"]
    #[inline(always)]
    pub fn pudclk36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 37"]
    #[inline(always)]
    pub fn pudclk37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 38"]
    #[inline(always)]
    pub fn pudclk38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 39"]
    #[inline(always)]
    pub fn pudclk39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 40"]
    #[inline(always)]
    pub fn pudclk40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 41"]
    #[inline(always)]
    pub fn pudclk41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 42"]
    #[inline(always)]
    pub fn pudclk42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 43"]
    #[inline(always)]
    pub fn pudclk43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 44"]
    #[inline(always)]
    pub fn pudclk44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 45"]
    #[inline(always)]
    pub fn pudclk45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 46"]
    #[inline(always)]
    pub fn pudclk46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 47"]
    #[inline(always)]
    pub fn pudclk47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 48"]
    #[inline(always)]
    pub fn pudclk48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 49"]
    #[inline(always)]
    pub fn pudclk49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 50"]
    #[inline(always)]
    pub fn pudclk50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 51"]
    #[inline(always)]
    pub fn pudclk51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 52"]
    #[inline(always)]
    pub fn pudclk52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }

    #[doc = "Assert Clock on line 53"]
    #[inline(always)]
    pub fn pudclk53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gppudclk1, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gppudclk1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Gppudclk1> for Gppudclk1T {
    #[inline(always)]
    fn reset_value(&self) -> Gppudclk1 {
        Gppudclk1::new(0)
    }
}
