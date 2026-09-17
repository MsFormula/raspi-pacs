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
    pub const fn gpfsel0(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfsel0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfsel0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "GPIO Function Select 1"]
    #[inline(always)]
    pub const fn gpfsel1(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfsel1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfsel1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "GPIO Function Select 2"]
    #[inline(always)]
    pub const fn gpfsel2(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfsel2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfsel2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "GPIO Function Select 3"]
    #[inline(always)]
    pub const fn gpfsel3(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfsel3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfsel3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "GPIO Function Select 4"]
    #[inline(always)]
    pub const fn gpfsel4(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfsel4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfsel4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "GPIO Function Select 5"]
    #[inline(always)]
    pub const fn gpfsel5(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfsel5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfsel5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "GPIO Pin Output Set 0"]
    #[inline(always)]
    pub const fn gpset0(&self) -> &'static crate::common::Reg<self::Gpset0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gpset0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "GPIO Pin Output Set 1"]
    #[inline(always)]
    pub const fn gpset1(&self) -> &'static crate::common::Reg<self::Gpset1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gpset1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "GPIO Pin Output Clear 0"]
    #[inline(always)]
    pub const fn gpclr0(&self) -> &'static crate::common::Reg<self::Gpclr0_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gpclr0_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "GPIO Pin Output Clear 1"]
    #[inline(always)]
    pub const fn gpclr1(&self) -> &'static crate::common::Reg<self::Gpclr1_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gpclr1_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "GPIO Pin Level 0"]
    #[inline(always)]
    pub const fn gplev0(&self) -> &'static crate::common::Reg<self::Gplev0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gplev0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "GPIO Pin Level 1"]
    #[inline(always)]
    pub const fn gplev1(&self) -> &'static crate::common::Reg<self::Gplev1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gplev1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "GPIO Pin Event Detect Status 0"]
    #[inline(always)]
    pub const fn gpeds0(
        &self,
    ) -> &'static crate::common::Reg<self::Gpeds0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpeds0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "GPIO Pin Event Detect Status 1"]
    #[inline(always)]
    pub const fn gpeds1(
        &self,
    ) -> &'static crate::common::Reg<self::Gpeds1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpeds1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "GPIO Pin Rising Edge Detect Enable 0"]
    #[inline(always)]
    pub const fn gpren0(
        &self,
    ) -> &'static crate::common::Reg<self::Gpren0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpren0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "GPIO Pin Rising Edge Detect Enable 1"]
    #[inline(always)]
    pub const fn gpren1(
        &self,
    ) -> &'static crate::common::Reg<self::Gpren1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpren1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "GPIO Pin Falling Edge Detect Enable 0"]
    #[inline(always)]
    pub const fn gpfen0(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "GPIO Pin Falling Edge Detect Enable 1"]
    #[inline(always)]
    pub const fn gpfen1(
        &self,
    ) -> &'static crate::common::Reg<self::Gpfen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpfen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "GPIO Pin High Detect Enable 0"]
    #[inline(always)]
    pub const fn gphen0(
        &self,
    ) -> &'static crate::common::Reg<self::Gphen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gphen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "GPIO Pin High Detect Enable 1"]
    #[inline(always)]
    pub const fn gphen1(
        &self,
    ) -> &'static crate::common::Reg<self::Gphen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gphen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "GPIO Pin Low Detect Enable 0"]
    #[inline(always)]
    pub const fn gplen0(
        &self,
    ) -> &'static crate::common::Reg<self::Gplen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gplen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "GPIO Pin Low Detect Enable 1"]
    #[inline(always)]
    pub const fn gplen1(
        &self,
    ) -> &'static crate::common::Reg<self::Gplen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gplen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "GPIO Pin Async. Rising Edge Detect 0"]
    #[inline(always)]
    pub const fn gparen0(
        &self,
    ) -> &'static crate::common::Reg<self::Gparen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gparen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[doc = "GPIO Pin Async. Rising Edge Detect 1"]
    #[inline(always)]
    pub const fn gparen1(
        &self,
    ) -> &'static crate::common::Reg<self::Gparen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gparen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "GPIO Pin Async. Falling Edge Detect 0"]
    #[inline(always)]
    pub const fn gpafen0(
        &self,
    ) -> &'static crate::common::Reg<self::Gpafen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpafen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "GPIO Pin Async. Falling Edge Detect 1"]
    #[inline(always)]
    pub const fn gpafen1(
        &self,
    ) -> &'static crate::common::Reg<self::Gpafen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gpafen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Undocumented multiplexing bits"]
    #[inline(always)]
    pub const fn extra_mux(
        &self,
    ) -> &'static crate::common::Reg<self::ExtraMux_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ExtraMux_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "GPIO Pin Pull-up/down Enable"]
    #[inline(always)]
    pub const fn gppud(&self) -> &'static crate::common::Reg<self::Gppud_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gppud_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "GPIO Pin Pull-up/down Enable Clock 0"]
    #[inline(always)]
    pub const fn gppudclk0(
        &self,
    ) -> &'static crate::common::Reg<self::Gppudclk0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gppudclk0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "GPIO Pin Pull-up/down Enable Clock 1"]
    #[inline(always)]
    pub const fn gppudclk1(
        &self,
    ) -> &'static crate::common::Reg<self::Gppudclk1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gppudclk1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel0_SPEC;
impl crate::sealed::RegSpec for Gpfsel0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Function Select 0"]
pub type Gpfsel0 = crate::RegValueT<Gpfsel0_SPEC>;

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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel0::Fsel0,
            gpfsel0::Fsel0,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel0::Fsel1,
            gpfsel0::Fsel1,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel0::Fsel2,
            gpfsel0::Fsel2,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel0::Fsel3,
            gpfsel0::Fsel3,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel0::Fsel4,
            gpfsel0::Fsel4,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel0::Fsel5,
            gpfsel0::Fsel5,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel0::Fsel6,
            gpfsel0::Fsel6,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel0::Fsel7,
            gpfsel0::Fsel7,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel0::Fsel8,
            gpfsel0::Fsel8,
            Gpfsel0_SPEC,
            crate::common::RW,
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
        Gpfsel0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel0::Fsel9,
            gpfsel0::Fsel9,
            Gpfsel0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gpfsel0 {
    #[inline(always)]
    fn default() -> Gpfsel0 {
        <crate::RegValueT<Gpfsel0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gpfsel0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel0_SPEC;
    pub type Fsel0 = crate::EnumBitfieldStruct<u8, Fsel0_SPEC>;
    impl Fsel0 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SDA0"]
        pub const SDA_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA5"]
        pub const SA_5: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel1_SPEC;
    pub type Fsel1 = crate::EnumBitfieldStruct<u8, Fsel1_SPEC>;
    impl Fsel1 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SCL0"]
        pub const SCL_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA4"]
        pub const SA_4: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel2_SPEC;
    pub type Fsel2 = crate::EnumBitfieldStruct<u8, Fsel2_SPEC>;
    impl Fsel2 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SDA1"]
        pub const SDA_1: Self = Self::new(4);

        #[doc = "Pin is connected to SA3"]
        pub const SA_3: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel3_SPEC;
    pub type Fsel3 = crate::EnumBitfieldStruct<u8, Fsel3_SPEC>;
    impl Fsel3 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SCL1"]
        pub const SCL_1: Self = Self::new(4);

        #[doc = "Pin is connected to SA2"]
        pub const SA_2: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel4_SPEC;
    pub type Fsel4 = crate::EnumBitfieldStruct<u8, Fsel4_SPEC>;
    impl Fsel4 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA1"]
        pub const SA_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to ARM_TDI"]
        pub const ARM_TDI: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel5_SPEC;
    pub type Fsel5 = crate::EnumBitfieldStruct<u8, Fsel5_SPEC>;
    impl Fsel5 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self::new(4);

        #[doc = "Pin is connected to SA0"]
        pub const SA_0: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to ARM_TDO"]
        pub const ARM_TDO: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel6_SPEC;
    pub type Fsel6 = crate::EnumBitfieldStruct<u8, Fsel6_SPEC>;
    impl Fsel6 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK2"]
        pub const GPCLK_2: Self = Self::new(4);

        #[doc = "Pin is connected to SOE_N"]
        pub const SOE_N: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to ARM_RTCK"]
        pub const ARM_RTCK: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel7_SPEC;
    pub type Fsel7 = crate::EnumBitfieldStruct<u8, Fsel7_SPEC>;
    impl Fsel7 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_CE1_N"]
        pub const SPI_0_CE_1_N: Self = Self::new(4);

        #[doc = "Pin is connected to SWE_N"]
        pub const SWE_N: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel8_SPEC;
    pub type Fsel8 = crate::EnumBitfieldStruct<u8, Fsel8_SPEC>;
    impl Fsel8 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_CE0_N"]
        pub const SPI_0_CE_0_N: Self = Self::new(4);

        #[doc = "Pin is connected to SD0"]
        pub const SD_0: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel9_SPEC;
    pub type Fsel9 = crate::EnumBitfieldStruct<u8, Fsel9_SPEC>;
    impl Fsel9 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_MISO"]
        pub const SPI_0_MISO: Self = Self::new(4);

        #[doc = "Pin is connected to SD1"]
        pub const SD_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel1_SPEC;
impl crate::sealed::RegSpec for Gpfsel1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Function Select 1"]
pub type Gpfsel1 = crate::RegValueT<Gpfsel1_SPEC>;

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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel1::Fsel10,
            gpfsel1::Fsel10,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel1::Fsel11,
            gpfsel1::Fsel11,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel1::Fsel12,
            gpfsel1::Fsel12,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel1::Fsel13,
            gpfsel1::Fsel13,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel1::Fsel14,
            gpfsel1::Fsel14,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel1::Fsel15,
            gpfsel1::Fsel15,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel1::Fsel16,
            gpfsel1::Fsel16,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel1::Fsel17,
            gpfsel1::Fsel17,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel1::Fsel18,
            gpfsel1::Fsel18,
            Gpfsel1_SPEC,
            crate::common::RW,
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
        Gpfsel1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel1::Fsel19,
            gpfsel1::Fsel19,
            Gpfsel1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gpfsel1 {
    #[inline(always)]
    fn default() -> Gpfsel1 {
        <crate::RegValueT<Gpfsel1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gpfsel1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel10_SPEC;
    pub type Fsel10 = crate::EnumBitfieldStruct<u8, Fsel10_SPEC>;
    impl Fsel10 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_MOSI"]
        pub const SPI_0_MOSI: Self = Self::new(4);

        #[doc = "Pin is connected to SD2"]
        pub const SD_2: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel11_SPEC;
    pub type Fsel11 = crate::EnumBitfieldStruct<u8, Fsel11_SPEC>;
    impl Fsel11 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_SCLK"]
        pub const SPI_0_SCLK: Self = Self::new(4);

        #[doc = "Pin is connected to SD3"]
        pub const SD_3: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel12_SPEC;
    pub type Fsel12 = crate::EnumBitfieldStruct<u8, Fsel12_SPEC>;
    impl Fsel12 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PWM0_0"]
        pub const PWM_0_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD4"]
        pub const SD_4: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to ARM_TMS"]
        pub const ARM_TMS: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel13_SPEC;
    pub type Fsel13 = crate::EnumBitfieldStruct<u8, Fsel13_SPEC>;
    impl Fsel13 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self::new(4);

        #[doc = "Pin is connected to SD5"]
        pub const SD_5: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to ARM_TCK"]
        pub const ARM_TCK: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel14_SPEC;
    pub type Fsel14 = crate::EnumBitfieldStruct<u8, Fsel14_SPEC>;
    impl Fsel14 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to TXD0"]
        pub const TXD_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD6"]
        pub const SD_6: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to TXD1"]
        pub const TXD_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel15_SPEC;
    pub type Fsel15 = crate::EnumBitfieldStruct<u8, Fsel15_SPEC>;
    impl Fsel15 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to RXD0"]
        pub const RXD_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD7"]
        pub const SD_7: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to RXD1"]
        pub const RXD_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel16_SPEC;
    pub type Fsel16 = crate::EnumBitfieldStruct<u8, Fsel16_SPEC>;
    impl Fsel16 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD8"]
        pub const SD_8: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to CTS0"]
        pub const CTS_0: Self = Self::new(7);

        #[doc = "Pin is connected to SPI1_CE2_N"]
        pub const SPI_1_CE_2_N: Self = Self::new(3);

        #[doc = "Pin is connected to CTS1"]
        pub const CTS_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel17_SPEC;
    pub type Fsel17 = crate::EnumBitfieldStruct<u8, Fsel17_SPEC>;
    impl Fsel17 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD9"]
        pub const SD_9: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to RTS0"]
        pub const RTS_0: Self = Self::new(7);

        #[doc = "Pin is connected to SPI1_CE1_N"]
        pub const SPI_1_CE_1_N: Self = Self::new(3);

        #[doc = "Pin is connected to RTS1"]
        pub const RTS_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel18_SPEC;
    pub type Fsel18 = crate::EnumBitfieldStruct<u8, Fsel18_SPEC>;
    impl Fsel18 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PCM_CLK"]
        pub const PCM_CLK: Self = Self::new(4);

        #[doc = "Pin is connected to SD10"]
        pub const SD_10: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI1_CE0_N"]
        pub const SPI_1_CE_0_N: Self = Self::new(3);

        #[doc = "Pin is connected to PWM0_0"]
        pub const PWM_0_0: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel19_SPEC;
    pub type Fsel19 = crate::EnumBitfieldStruct<u8, Fsel19_SPEC>;
    impl Fsel19 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PCM_FS"]
        pub const PCM_FS: Self = Self::new(4);

        #[doc = "Pin is connected to SD11"]
        pub const SD_11: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI1_MISO"]
        pub const SPI_1_MISO: Self = Self::new(3);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel2_SPEC;
impl crate::sealed::RegSpec for Gpfsel2_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Function Select 2"]
pub type Gpfsel2 = crate::RegValueT<Gpfsel2_SPEC>;

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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel2::Fsel20,
            gpfsel2::Fsel20,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel2::Fsel21,
            gpfsel2::Fsel21,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel2::Fsel22,
            gpfsel2::Fsel22,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel2::Fsel23,
            gpfsel2::Fsel23,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel2::Fsel24,
            gpfsel2::Fsel24,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel2::Fsel25,
            gpfsel2::Fsel25,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel2::Fsel26,
            gpfsel2::Fsel26,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel2::Fsel27,
            gpfsel2::Fsel27,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel2::Fsel28,
            gpfsel2::Fsel28,
            Gpfsel2_SPEC,
            crate::common::RW,
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
        Gpfsel2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel2::Fsel29,
            gpfsel2::Fsel29,
            Gpfsel2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gpfsel2 {
    #[inline(always)]
    fn default() -> Gpfsel2 {
        <crate::RegValueT<Gpfsel2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gpfsel2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel20_SPEC;
    pub type Fsel20 = crate::EnumBitfieldStruct<u8, Fsel20_SPEC>;
    impl Fsel20 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PCM_DIN"]
        pub const PCM_DIN: Self = Self::new(4);

        #[doc = "Pin is connected to SD12"]
        pub const SD_12: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI1_MOSI"]
        pub const SPI_1_MOSI: Self = Self::new(3);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel21_SPEC;
    pub type Fsel21 = crate::EnumBitfieldStruct<u8, Fsel21_SPEC>;
    impl Fsel21 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PCM_DOUT"]
        pub const PCM_DOUT: Self = Self::new(4);

        #[doc = "Pin is connected to SD13"]
        pub const SD_13: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI1_SCLK"]
        pub const SPI_1_SCLK: Self = Self::new(3);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel22_SPEC;
    pub type Fsel22 = crate::EnumBitfieldStruct<u8, Fsel22_SPEC>;
    impl Fsel22 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD14"]
        pub const SD_14: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_CLK"]
        pub const SD_1_CLK: Self = Self::new(7);

        #[doc = "Pin is connected to ARM_TRST"]
        pub const ARM_TRST: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel23_SPEC;
    pub type Fsel23 = crate::EnumBitfieldStruct<u8, Fsel23_SPEC>;
    impl Fsel23 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD15"]
        pub const SD_15: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_CMD"]
        pub const SD_1_CMD: Self = Self::new(7);

        #[doc = "Pin is connected to ARM_RTCK"]
        pub const ARM_RTCK: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel24_SPEC;
    pub type Fsel24 = crate::EnumBitfieldStruct<u8, Fsel24_SPEC>;
    impl Fsel24 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD16"]
        pub const SD_16: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT0"]
        pub const SD_1_DAT_0: Self = Self::new(7);

        #[doc = "Pin is connected to ARM_TDO"]
        pub const ARM_TDO: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel25_SPEC;
    pub type Fsel25 = crate::EnumBitfieldStruct<u8, Fsel25_SPEC>;
    impl Fsel25 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD17"]
        pub const SD_17: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT1"]
        pub const SD_1_DAT_1: Self = Self::new(7);

        #[doc = "Pin is connected to ARM_TCK"]
        pub const ARM_TCK: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel26_SPEC;
    pub type Fsel26 = crate::EnumBitfieldStruct<u8, Fsel26_SPEC>;
    impl Fsel26 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT2"]
        pub const SD_1_DAT_2: Self = Self::new(7);

        #[doc = "Pin is connected to ARM_TDI"]
        pub const ARM_TDI: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel27_SPEC;
    pub type Fsel27 = crate::EnumBitfieldStruct<u8, Fsel27_SPEC>;
    impl Fsel27 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT3"]
        pub const SD_1_DAT_3: Self = Self::new(7);

        #[doc = "Pin is connected to ARM_TMS"]
        pub const ARM_TMS: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel28_SPEC;
    pub type Fsel28 = crate::EnumBitfieldStruct<u8, Fsel28_SPEC>;
    impl Fsel28 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SDA0"]
        pub const SDA_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA5"]
        pub const SA_5: Self = Self::new(5);

        #[doc = "Pin is connected to PCM_CLK"]
        pub const PCM_CLK: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel29_SPEC;
    pub type Fsel29 = crate::EnumBitfieldStruct<u8, Fsel29_SPEC>;
    impl Fsel29 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SCL0"]
        pub const SCL_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA4"]
        pub const SA_4: Self = Self::new(5);

        #[doc = "Pin is connected to PCM_FS"]
        pub const PCM_FS: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel3_SPEC;
impl crate::sealed::RegSpec for Gpfsel3_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Function Select 3"]
pub type Gpfsel3 = crate::RegValueT<Gpfsel3_SPEC>;

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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel3::Fsel30,
            gpfsel3::Fsel30,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel3::Fsel31,
            gpfsel3::Fsel31,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel3::Fsel32,
            gpfsel3::Fsel32,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel3::Fsel33,
            gpfsel3::Fsel33,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel3::Fsel34,
            gpfsel3::Fsel34,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel3::Fsel35,
            gpfsel3::Fsel35,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel3::Fsel36,
            gpfsel3::Fsel36,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel3::Fsel37,
            gpfsel3::Fsel37,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel3::Fsel38,
            gpfsel3::Fsel38,
            Gpfsel3_SPEC,
            crate::common::RW,
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
        Gpfsel3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel3::Fsel39,
            gpfsel3::Fsel39,
            Gpfsel3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gpfsel3 {
    #[inline(always)]
    fn default() -> Gpfsel3 {
        <crate::RegValueT<Gpfsel3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gpfsel3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel30_SPEC;
    pub type Fsel30 = crate::EnumBitfieldStruct<u8, Fsel30_SPEC>;
    impl Fsel30 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA3"]
        pub const SA_3: Self = Self::new(5);

        #[doc = "Pin is connected to PCM_DIN"]
        pub const PCM_DIN: Self = Self::new(6);

        #[doc = "Pin is connected to CTS0"]
        pub const CTS_0: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to CTS1"]
        pub const CTS_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel31_SPEC;
    pub type Fsel31 = crate::EnumBitfieldStruct<u8, Fsel31_SPEC>;
    impl Fsel31 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA2"]
        pub const SA_2: Self = Self::new(5);

        #[doc = "Pin is connected to PCM_DOUT"]
        pub const PCM_DOUT: Self = Self::new(6);

        #[doc = "Pin is connected to RTS0"]
        pub const RTS_0: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to RTS1"]
        pub const RTS_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel32_SPEC;
    pub type Fsel32 = crate::EnumBitfieldStruct<u8, Fsel32_SPEC>;
    impl Fsel32 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA1"]
        pub const SA_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to TXD0"]
        pub const TXD_0: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to TXD1"]
        pub const TXD_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel33_SPEC;
    pub type Fsel33 = crate::EnumBitfieldStruct<u8, Fsel33_SPEC>;
    impl Fsel33 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Pin is connected to SA0"]
        pub const SA_0: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to RXD0"]
        pub const RXD_0: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Pin is connected to RXD1"]
        pub const RXD_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel34_SPEC;
    pub type Fsel34 = crate::EnumBitfieldStruct<u8, Fsel34_SPEC>;
    impl Fsel34 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK0"]
        pub const GPCLK_0: Self = Self::new(4);

        #[doc = "Pin is connected to SOE_N"]
        pub const SOE_N: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel35_SPEC;
    pub type Fsel35 = crate::EnumBitfieldStruct<u8, Fsel35_SPEC>;
    impl Fsel35 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_CE1_N"]
        pub const SPI_0_CE_1_N: Self = Self::new(4);

        #[doc = "Pin is connected to SWE_N"]
        pub const SWE_N: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel36_SPEC;
    pub type Fsel36 = crate::EnumBitfieldStruct<u8, Fsel36_SPEC>;
    impl Fsel36 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_CE0_N"]
        pub const SPI_0_CE_0_N: Self = Self::new(4);

        #[doc = "Pin is connected to SD0"]
        pub const SD_0: Self = Self::new(5);

        #[doc = "Pin is connected to TXD0"]
        pub const TXD_0: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel37_SPEC;
    pub type Fsel37 = crate::EnumBitfieldStruct<u8, Fsel37_SPEC>;
    impl Fsel37 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_MISO"]
        pub const SPI_0_MISO: Self = Self::new(4);

        #[doc = "Pin is connected to SD1"]
        pub const SD_1: Self = Self::new(5);

        #[doc = "Pin is connected to RXD0"]
        pub const RXD_0: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel38_SPEC;
    pub type Fsel38 = crate::EnumBitfieldStruct<u8, Fsel38_SPEC>;
    impl Fsel38 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_MOSI"]
        pub const SPI_0_MOSI: Self = Self::new(4);

        #[doc = "Pin is connected to SD2"]
        pub const SD_2: Self = Self::new(5);

        #[doc = "Pin is connected to CTS0"]
        pub const CTS_0: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel39_SPEC;
    pub type Fsel39 = crate::EnumBitfieldStruct<u8, Fsel39_SPEC>;
    impl Fsel39 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to SPI0_SCLK"]
        pub const SPI_0_SCLK: Self = Self::new(4);

        #[doc = "Pin is connected to SD3"]
        pub const SD_3: Self = Self::new(5);

        #[doc = "Pin is connected to RTS0"]
        pub const RTS_0: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel4_SPEC;
impl crate::sealed::RegSpec for Gpfsel4_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Function Select 4"]
pub type Gpfsel4 = crate::RegValueT<Gpfsel4_SPEC>;

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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel4::Fsel40,
            gpfsel4::Fsel40,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel4::Fsel41,
            gpfsel4::Fsel41,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel4::Fsel42,
            gpfsel4::Fsel42,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel4::Fsel43,
            gpfsel4::Fsel43,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            gpfsel4::Fsel44,
            gpfsel4::Fsel44,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x7,
            1,
            0,
            gpfsel4::Fsel45,
            gpfsel4::Fsel45,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x7,
            1,
            0,
            gpfsel4::Fsel46,
            gpfsel4::Fsel46,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x7,
            1,
            0,
            gpfsel4::Fsel47,
            gpfsel4::Fsel47,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            gpfsel4::Fsel48,
            gpfsel4::Fsel48,
            Gpfsel4_SPEC,
            crate::common::RW,
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
        Gpfsel4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x7,
            1,
            0,
            gpfsel4::Fsel49,
            gpfsel4::Fsel49,
            Gpfsel4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gpfsel4 {
    #[inline(always)]
    fn default() -> Gpfsel4 {
        <crate::RegValueT<Gpfsel4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gpfsel4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel40_SPEC;
    pub type Fsel40 = crate::EnumBitfieldStruct<u8, Fsel40_SPEC>;
    impl Fsel40 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PWM0_0"]
        pub const PWM_0_0: Self = Self::new(4);

        #[doc = "Pin is connected to SD4"]
        pub const SD_4: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI2_MISO"]
        pub const SPI_2_MISO: Self = Self::new(3);

        #[doc = "Pin is connected to TXD1"]
        pub const TXD_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel41_SPEC;
    pub type Fsel41 = crate::EnumBitfieldStruct<u8, Fsel41_SPEC>;
    impl Fsel41 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self::new(4);

        #[doc = "Pin is connected to SD5"]
        pub const SD_5: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI2_MOSI"]
        pub const SPI_2_MOSI: Self = Self::new(3);

        #[doc = "Pin is connected to RXD1"]
        pub const RXD_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel42_SPEC;
    pub type Fsel42 = crate::EnumBitfieldStruct<u8, Fsel42_SPEC>;
    impl Fsel42 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self::new(4);

        #[doc = "Pin is connected to SD6"]
        pub const SD_6: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI2_SCLK"]
        pub const SPI_2_SCLK: Self = Self::new(3);

        #[doc = "Pin is connected to CTS1"]
        pub const CTS_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel43_SPEC;
    pub type Fsel43 = crate::EnumBitfieldStruct<u8, Fsel43_SPEC>;
    impl Fsel43 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK2"]
        pub const GPCLK_2: Self = Self::new(4);

        #[doc = "Pin is connected to SD7"]
        pub const SD_7: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI2_CE0_N"]
        pub const SPI_2_CE_0_N: Self = Self::new(3);

        #[doc = "Pin is connected to RTS1"]
        pub const RTS_1: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel44_SPEC;
    pub type Fsel44 = crate::EnumBitfieldStruct<u8, Fsel44_SPEC>;
    impl Fsel44 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to GPCLK1"]
        pub const GPCLK_1: Self = Self::new(4);

        #[doc = "Pin is connected to SDA0"]
        pub const SDA_0: Self = Self::new(5);

        #[doc = "Pin is connected to SDA1"]
        pub const SDA_1: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI2_CE1_N"]
        pub const SPI_2_CE_1_N: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel45_SPEC;
    pub type Fsel45 = crate::EnumBitfieldStruct<u8, Fsel45_SPEC>;
    impl Fsel45 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Pin is connected to PWM0_1"]
        pub const PWM_0_1: Self = Self::new(4);

        #[doc = "Pin is connected to SCL0"]
        pub const SCL_0: Self = Self::new(5);

        #[doc = "Pin is connected to SCL1"]
        pub const SCL_1: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Pin is connected to SPI2_CE2_N"]
        pub const SPI_2_CE_2_N: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel46_SPEC;
    pub type Fsel46 = crate::EnumBitfieldStruct<u8, Fsel46_SPEC>;
    impl Fsel46 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel47_SPEC;
    pub type Fsel47 = crate::EnumBitfieldStruct<u8, Fsel47_SPEC>;
    impl Fsel47 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Alt function 3 reserved"]
        pub const RESERVED_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel48_SPEC;
    pub type Fsel48 = crate::EnumBitfieldStruct<u8, Fsel48_SPEC>;
    impl Fsel48 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_CLK"]
        pub const SD_1_CLK: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel49_SPEC;
    pub type Fsel49 = crate::EnumBitfieldStruct<u8, Fsel49_SPEC>;
    impl Fsel49 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_CMD"]
        pub const SD_1_CMD: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfsel5_SPEC;
impl crate::sealed::RegSpec for Gpfsel5_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Function Select 5"]
pub type Gpfsel5 = crate::RegValueT<Gpfsel5_SPEC>;

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
        Gpfsel5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            gpfsel5::Fsel50,
            gpfsel5::Fsel50,
            Gpfsel5_SPEC,
            crate::common::RW,
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
        Gpfsel5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            gpfsel5::Fsel51,
            gpfsel5::Fsel51,
            Gpfsel5_SPEC,
            crate::common::RW,
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
        Gpfsel5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x7,
            1,
            0,
            gpfsel5::Fsel52,
            gpfsel5::Fsel52,
            Gpfsel5_SPEC,
            crate::common::RW,
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
        Gpfsel5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            gpfsel5::Fsel53,
            gpfsel5::Fsel53,
            Gpfsel5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gpfsel5 {
    #[inline(always)]
    fn default() -> Gpfsel5 {
        <crate::RegValueT<Gpfsel5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gpfsel5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel50_SPEC;
    pub type Fsel50 = crate::EnumBitfieldStruct<u8, Fsel50_SPEC>;
    impl Fsel50 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT0"]
        pub const SD_1_DAT_0: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel51_SPEC;
    pub type Fsel51 = crate::EnumBitfieldStruct<u8, Fsel51_SPEC>;
    impl Fsel51 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT1"]
        pub const SD_1_DAT_1: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel52_SPEC;
    pub type Fsel52 = crate::EnumBitfieldStruct<u8, Fsel52_SPEC>;
    impl Fsel52 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT2"]
        pub const SD_1_DAT_2: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsel53_SPEC;
    pub type Fsel53 = crate::EnumBitfieldStruct<u8, Fsel53_SPEC>;
    impl Fsel53 {
        #[doc = "Pin is an input"]
        pub const INPUT: Self = Self::new(0);

        #[doc = "Pin is an output"]
        pub const OUTPUT: Self = Self::new(1);

        #[doc = "Alt function 0 reserved"]
        pub const RESERVED_0: Self = Self::new(4);

        #[doc = "Alt function 1 reserved"]
        pub const RESERVED_1: Self = Self::new(5);

        #[doc = "Alt function 2 reserved"]
        pub const RESERVED_2: Self = Self::new(6);

        #[doc = "Pin is connected to SD1_DAT3"]
        pub const SD_1_DAT_3: Self = Self::new(7);

        #[doc = "Alt function 4 reserved"]
        pub const RESERVED_4: Self = Self::new(3);

        #[doc = "Alt function 5 reserved"]
        pub const RESERVED_5: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpset0_SPEC;
impl crate::sealed::RegSpec for Gpset0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Output Set 0"]
pub type Gpset0 = crate::RegValueT<Gpset0_SPEC>;

impl NoBitfieldReg<Gpset0_SPEC> for Gpset0 {}
impl ::core::default::Default for Gpset0 {
    #[inline(always)]
    fn default() -> Gpset0 {
        <crate::RegValueT<Gpset0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpset1_SPEC;
impl crate::sealed::RegSpec for Gpset1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Output Set 1"]
pub type Gpset1 = crate::RegValueT<Gpset1_SPEC>;

impl NoBitfieldReg<Gpset1_SPEC> for Gpset1 {}
impl ::core::default::Default for Gpset1 {
    #[inline(always)]
    fn default() -> Gpset1 {
        <crate::RegValueT<Gpset1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr0_SPEC;
impl crate::sealed::RegSpec for Gpclr0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Output Clear 0"]
pub type Gpclr0 = crate::RegValueT<Gpclr0_SPEC>;

impl NoBitfieldReg<Gpclr0_SPEC> for Gpclr0 {}
impl ::core::default::Default for Gpclr0 {
    #[inline(always)]
    fn default() -> Gpclr0 {
        <crate::RegValueT<Gpclr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr1_SPEC;
impl crate::sealed::RegSpec for Gpclr1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Output Clear 1"]
pub type Gpclr1 = crate::RegValueT<Gpclr1_SPEC>;

impl NoBitfieldReg<Gpclr1_SPEC> for Gpclr1 {}
impl ::core::default::Default for Gpclr1 {
    #[inline(always)]
    fn default() -> Gpclr1 {
        <crate::RegValueT<Gpclr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplev0_SPEC;
impl crate::sealed::RegSpec for Gplev0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Level 0"]
pub type Gplev0 = crate::RegValueT<Gplev0_SPEC>;

impl NoBitfieldReg<Gplev0_SPEC> for Gplev0 {}
impl ::core::default::Default for Gplev0 {
    #[inline(always)]
    fn default() -> Gplev0 {
        <crate::RegValueT<Gplev0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplev1_SPEC;
impl crate::sealed::RegSpec for Gplev1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Level 1"]
pub type Gplev1 = crate::RegValueT<Gplev1_SPEC>;

impl NoBitfieldReg<Gplev1_SPEC> for Gplev1 {}
impl ::core::default::Default for Gplev1 {
    #[inline(always)]
    fn default() -> Gplev1 {
        <crate::RegValueT<Gplev1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpeds0_SPEC;
impl crate::sealed::RegSpec for Gpeds0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Event Detect Status 0"]
pub type Gpeds0 = crate::RegValueT<Gpeds0_SPEC>;

impl NoBitfieldReg<Gpeds0_SPEC> for Gpeds0 {}
impl ::core::default::Default for Gpeds0 {
    #[inline(always)]
    fn default() -> Gpeds0 {
        <crate::RegValueT<Gpeds0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpeds1_SPEC;
impl crate::sealed::RegSpec for Gpeds1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Event Detect Status 1"]
pub type Gpeds1 = crate::RegValueT<Gpeds1_SPEC>;

impl NoBitfieldReg<Gpeds1_SPEC> for Gpeds1 {}
impl ::core::default::Default for Gpeds1 {
    #[inline(always)]
    fn default() -> Gpeds1 {
        <crate::RegValueT<Gpeds1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpren0_SPEC;
impl crate::sealed::RegSpec for Gpren0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Rising Edge Detect Enable 0"]
pub type Gpren0 = crate::RegValueT<Gpren0_SPEC>;

impl NoBitfieldReg<Gpren0_SPEC> for Gpren0 {}
impl ::core::default::Default for Gpren0 {
    #[inline(always)]
    fn default() -> Gpren0 {
        <crate::RegValueT<Gpren0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpren1_SPEC;
impl crate::sealed::RegSpec for Gpren1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Rising Edge Detect Enable 1"]
pub type Gpren1 = crate::RegValueT<Gpren1_SPEC>;

impl NoBitfieldReg<Gpren1_SPEC> for Gpren1 {}
impl ::core::default::Default for Gpren1 {
    #[inline(always)]
    fn default() -> Gpren1 {
        <crate::RegValueT<Gpren1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfen0_SPEC;
impl crate::sealed::RegSpec for Gpfen0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Falling Edge Detect Enable 0"]
pub type Gpfen0 = crate::RegValueT<Gpfen0_SPEC>;

impl NoBitfieldReg<Gpfen0_SPEC> for Gpfen0 {}
impl ::core::default::Default for Gpfen0 {
    #[inline(always)]
    fn default() -> Gpfen0 {
        <crate::RegValueT<Gpfen0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpfen1_SPEC;
impl crate::sealed::RegSpec for Gpfen1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Falling Edge Detect Enable 1"]
pub type Gpfen1 = crate::RegValueT<Gpfen1_SPEC>;

impl NoBitfieldReg<Gpfen1_SPEC> for Gpfen1 {}
impl ::core::default::Default for Gpfen1 {
    #[inline(always)]
    fn default() -> Gpfen1 {
        <crate::RegValueT<Gpfen1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gphen0_SPEC;
impl crate::sealed::RegSpec for Gphen0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin High Detect Enable 0"]
pub type Gphen0 = crate::RegValueT<Gphen0_SPEC>;

impl NoBitfieldReg<Gphen0_SPEC> for Gphen0 {}
impl ::core::default::Default for Gphen0 {
    #[inline(always)]
    fn default() -> Gphen0 {
        <crate::RegValueT<Gphen0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gphen1_SPEC;
impl crate::sealed::RegSpec for Gphen1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin High Detect Enable 1"]
pub type Gphen1 = crate::RegValueT<Gphen1_SPEC>;

impl NoBitfieldReg<Gphen1_SPEC> for Gphen1 {}
impl ::core::default::Default for Gphen1 {
    #[inline(always)]
    fn default() -> Gphen1 {
        <crate::RegValueT<Gphen1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplen0_SPEC;
impl crate::sealed::RegSpec for Gplen0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Low Detect Enable 0"]
pub type Gplen0 = crate::RegValueT<Gplen0_SPEC>;

impl NoBitfieldReg<Gplen0_SPEC> for Gplen0 {}
impl ::core::default::Default for Gplen0 {
    #[inline(always)]
    fn default() -> Gplen0 {
        <crate::RegValueT<Gplen0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gplen1_SPEC;
impl crate::sealed::RegSpec for Gplen1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Low Detect Enable 1"]
pub type Gplen1 = crate::RegValueT<Gplen1_SPEC>;

impl NoBitfieldReg<Gplen1_SPEC> for Gplen1 {}
impl ::core::default::Default for Gplen1 {
    #[inline(always)]
    fn default() -> Gplen1 {
        <crate::RegValueT<Gplen1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gparen0_SPEC;
impl crate::sealed::RegSpec for Gparen0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Async. Rising Edge Detect 0"]
pub type Gparen0 = crate::RegValueT<Gparen0_SPEC>;

impl NoBitfieldReg<Gparen0_SPEC> for Gparen0 {}
impl ::core::default::Default for Gparen0 {
    #[inline(always)]
    fn default() -> Gparen0 {
        <crate::RegValueT<Gparen0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gparen1_SPEC;
impl crate::sealed::RegSpec for Gparen1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Async. Rising Edge Detect 1"]
pub type Gparen1 = crate::RegValueT<Gparen1_SPEC>;

impl NoBitfieldReg<Gparen1_SPEC> for Gparen1 {}
impl ::core::default::Default for Gparen1 {
    #[inline(always)]
    fn default() -> Gparen1 {
        <crate::RegValueT<Gparen1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpafen0_SPEC;
impl crate::sealed::RegSpec for Gpafen0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Async. Falling Edge Detect 0"]
pub type Gpafen0 = crate::RegValueT<Gpafen0_SPEC>;

impl NoBitfieldReg<Gpafen0_SPEC> for Gpafen0 {}
impl ::core::default::Default for Gpafen0 {
    #[inline(always)]
    fn default() -> Gpafen0 {
        <crate::RegValueT<Gpafen0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpafen1_SPEC;
impl crate::sealed::RegSpec for Gpafen1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Async. Falling Edge Detect 1"]
pub type Gpafen1 = crate::RegValueT<Gpafen1_SPEC>;

impl NoBitfieldReg<Gpafen1_SPEC> for Gpafen1 {}
impl ::core::default::Default for Gpafen1 {
    #[inline(always)]
    fn default() -> Gpafen1 {
        <crate::RegValueT<Gpafen1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtraMux_SPEC;
impl crate::sealed::RegSpec for ExtraMux_SPEC {
    type DataType = u32;
}

#[doc = "Undocumented multiplexing bits"]
pub type ExtraMux = crate::RegValueT<ExtraMux_SPEC>;

impl NoBitfieldReg<ExtraMux_SPEC> for ExtraMux {}
impl ::core::default::Default for ExtraMux {
    #[inline(always)]
    fn default() -> ExtraMux {
        <crate::RegValueT<ExtraMux_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gppud_SPEC;
impl crate::sealed::RegSpec for Gppud_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Pull-up/down Enable"]
pub type Gppud = crate::RegValueT<Gppud_SPEC>;

impl NoBitfieldReg<Gppud_SPEC> for Gppud {}
impl ::core::default::Default for Gppud {
    #[inline(always)]
    fn default() -> Gppud {
        <crate::RegValueT<Gppud_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gppudclk0_SPEC;
impl crate::sealed::RegSpec for Gppudclk0_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Pull-up/down Enable Clock 0"]
pub type Gppudclk0 = crate::RegValueT<Gppudclk0_SPEC>;

impl NoBitfieldReg<Gppudclk0_SPEC> for Gppudclk0 {}
impl ::core::default::Default for Gppudclk0 {
    #[inline(always)]
    fn default() -> Gppudclk0 {
        <crate::RegValueT<Gppudclk0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gppudclk1_SPEC;
impl crate::sealed::RegSpec for Gppudclk1_SPEC {
    type DataType = u32;
}

#[doc = "GPIO Pin Pull-up/down Enable Clock 1"]
pub type Gppudclk1 = crate::RegValueT<Gppudclk1_SPEC>;

impl NoBitfieldReg<Gppudclk1_SPEC> for Gppudclk1 {}
impl ::core::default::Default for Gppudclk1 {
    #[inline(always)]
    fn default() -> Gppudclk1 {
        <crate::RegValueT<Gppudclk1_SPEC> as RegisterValue<_>>::new(0)
    }
}
