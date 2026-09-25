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
#[doc = r"Broadcom System Timer"]
unsafe impl ::core::marker::Send for super::Systmr {}
unsafe impl ::core::marker::Sync for super::Systmr {}
impl super::Systmr {
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

    #[doc = "Lower 32 bits for the free running counter"]
    #[inline(always)]
    pub fn clo(&self) -> &'static self::CloT {
        unsafe { self::CloT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Higher 32 bits for the free running counter"]
    #[inline(always)]
    pub fn chi(&self) -> &'static self::ChiT {
        unsafe { self::ChiT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Compare channel 0"]
    #[inline(always)]
    pub fn c0(&self) -> &'static self::C0T {
        unsafe { self::C0T::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "Compare channel 1"]
    #[inline(always)]
    pub fn c1(&self) -> &'static self::C1T {
        unsafe { self::C1T::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "Compare channel 2"]
    #[inline(always)]
    pub fn c2(&self) -> &'static self::C2T {
        unsafe { self::C2T::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "Compare channel 3"]
    #[inline(always)]
    pub fn c3(&self) -> &'static self::C3T {
        unsafe { self::C3T::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
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
    #[doc = "System timer match 3"]
    #[inline(always)]
    pub fn m3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "System timer match 2"]
    #[inline(always)]
    pub fn m2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "System timer match 1"]
    #[inline(always)]
    pub fn m1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs, common::RW>::from_register(self, 0)
    }

    #[doc = "System timer match 0"]
    #[inline(always)]
    pub fn m0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs> for CsT {
    #[inline(always)]
    fn reset_value(&self) -> Cs {
        Cs::new(0)
    }
}

#[doc = "Lower 32 bits for the free running counter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clo {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Clo {
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
pub struct CloT;
unsafe impl crate::common::AsPtr for CloT {}
impl crate::common::Reg<Clo> for CloT {}

unsafe impl crate::common::Read<Clo> for CloT {}

impl crate::common::NoBitfieldReg for Clo {}
impl crate::common::ResetValue<Clo> for CloT {
    #[inline(always)]
    fn reset_value(&self) -> Clo {
        Clo::new(0)
    }
}

#[doc = "Higher 32 bits for the free running counter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chi {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Chi {
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
pub struct ChiT;
unsafe impl crate::common::AsPtr for ChiT {}
impl crate::common::Reg<Chi> for ChiT {}

unsafe impl crate::common::Read<Chi> for ChiT {}

impl crate::common::NoBitfieldReg for Chi {}
impl crate::common::ResetValue<Chi> for ChiT {
    #[inline(always)]
    fn reset_value(&self) -> Chi {
        Chi::new(0)
    }
}

#[doc = "Compare channel 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for C0 {
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
pub struct C0T;
unsafe impl crate::common::AsPtr for C0T {}
impl crate::common::Reg<C0> for C0T {}

unsafe impl crate::common::Read<C0> for C0T {}
unsafe impl crate::common::Write<C0> for C0T {}

impl crate::common::NoBitfieldReg for C0 {}
impl crate::common::ResetValue<C0> for C0T {
    #[inline(always)]
    fn reset_value(&self) -> C0 {
        C0::new(0)
    }
}

#[doc = "Compare channel 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for C1 {
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
pub struct C1T;
unsafe impl crate::common::AsPtr for C1T {}
impl crate::common::Reg<C1> for C1T {}

unsafe impl crate::common::Read<C1> for C1T {}
unsafe impl crate::common::Write<C1> for C1T {}

impl crate::common::NoBitfieldReg for C1 {}
impl crate::common::ResetValue<C1> for C1T {
    #[inline(always)]
    fn reset_value(&self) -> C1 {
        C1::new(0)
    }
}

#[doc = "Compare channel 2"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for C2 {
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
pub struct C2T;
unsafe impl crate::common::AsPtr for C2T {}
impl crate::common::Reg<C2> for C2T {}

unsafe impl crate::common::Read<C2> for C2T {}
unsafe impl crate::common::Write<C2> for C2T {}

impl crate::common::NoBitfieldReg for C2 {}
impl crate::common::ResetValue<C2> for C2T {
    #[inline(always)]
    fn reset_value(&self) -> C2 {
        C2::new(0)
    }
}

#[doc = "Compare channel 3"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct C3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for C3 {
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
pub struct C3T;
unsafe impl crate::common::AsPtr for C3T {}
impl crate::common::Reg<C3> for C3T {}

unsafe impl crate::common::Read<C3> for C3T {}
unsafe impl crate::common::Write<C3> for C3T {}

impl crate::common::NoBitfieldReg for C3 {}
impl crate::common::ResetValue<C3> for C3T {
    #[inline(always)]
    fn reset_value(&self) -> C3 {
        C3::new(0)
    }
}
