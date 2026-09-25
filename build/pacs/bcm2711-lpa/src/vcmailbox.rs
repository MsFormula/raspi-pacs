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
#[doc = r"Mailboxes for talking to/from VideoCore"]
unsafe impl ::core::marker::Send for super::Vcmailbox {}
unsafe impl ::core::marker::Sync for super::Vcmailbox {}
impl super::Vcmailbox {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Read messages from the VideoCore"]
    #[inline(always)]
    pub fn read(&self) -> &'static self::ReadT {
        unsafe { self::ReadT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[inline(always)]
    pub fn peek0(&self) -> &'static self::Peek0T {
        unsafe { self::Peek0T::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[inline(always)]
    pub fn sender0(&self) -> &'static self::Sender0T {
        unsafe { self::Sender0T::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[inline(always)]
    pub fn status0(&self) -> &'static self::Status0T {
        unsafe { self::Status0T::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[inline(always)]
    pub fn config0(&self) -> &'static self::Config0T {
        unsafe { self::Config0T::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "Write messages to the VideoCore"]
    #[inline(always)]
    pub fn write(&self) -> &'static self::WriteT {
        unsafe { self::WriteT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[inline(always)]
    pub fn peek1(&self) -> &'static self::Peek1T {
        unsafe { self::Peek1T::from_ptr(self._svd2pac_as_ptr().add(48usize)) }
    }

    #[inline(always)]
    pub fn sender1(&self) -> &'static self::Sender1T {
        unsafe { self::Sender1T::from_ptr(self._svd2pac_as_ptr().add(52usize)) }
    }

    #[inline(always)]
    pub fn status1(&self) -> &'static self::Status1T {
        unsafe { self::Status1T::from_ptr(self._svd2pac_as_ptr().add(56usize)) }
    }

    #[inline(always)]
    pub fn config1(&self) -> &'static self::Config1T {
        unsafe { self::Config1T::from_ptr(self._svd2pac_as_ptr().add(60usize)) }
    }
}

#[doc = "Read messages from the VideoCore"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Read {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Read {
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
pub struct ReadT;
unsafe impl crate::common::AsPtr for ReadT {}
impl crate::common::Reg<Read> for ReadT {}

unsafe impl crate::common::Read<Read> for ReadT {}

impl crate::common::NoBitfieldReg for Read {}
impl crate::common::ResetValue<Read> for ReadT {
    #[inline(always)]
    fn reset_value(&self) -> Read {
        Read::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Peek0 {
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
pub struct Peek0T;
unsafe impl crate::common::AsPtr for Peek0T {}
impl crate::common::Reg<Peek0> for Peek0T {}

unsafe impl crate::common::Read<Peek0> for Peek0T {}
unsafe impl crate::common::Write<Peek0> for Peek0T {}

impl crate::common::NoBitfieldReg for Peek0 {}
impl crate::common::ResetValue<Peek0> for Peek0T {
    #[inline(always)]
    fn reset_value(&self) -> Peek0 {
        Peek0::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sender0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Sender0 {
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
pub struct Sender0T;
unsafe impl crate::common::AsPtr for Sender0T {}
impl crate::common::Reg<Sender0> for Sender0T {}

unsafe impl crate::common::Read<Sender0> for Sender0T {}
unsafe impl crate::common::Write<Sender0> for Sender0T {}

impl crate::common::NoBitfieldReg for Sender0 {}
impl crate::common::ResetValue<Sender0> for Sender0T {
    #[inline(always)]
    fn reset_value(&self) -> Sender0 {
        Sender0::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Status0 {
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
pub struct Status0T;
unsafe impl crate::common::AsPtr for Status0T {}
impl crate::common::Reg<Status0> for Status0T {}

unsafe impl crate::common::Read<Status0> for Status0T {}
unsafe impl crate::common::Write<Status0> for Status0T {}
impl Status0 {
    #[inline(always)]
    pub fn full(self) -> crate::common::RegisterFieldBool<31, 1, 0, Status0, common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Status0, common::R>::from_register(self, 0)
    }

    #[inline(always)]
    pub fn empty(self) -> crate::common::RegisterFieldBool<30, 1, 0, Status0, common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Status0, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Status0> for Status0T {
    #[inline(always)]
    fn reset_value(&self) -> Status0 {
        Status0::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Config0 {
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
pub struct Config0T;
unsafe impl crate::common::AsPtr for Config0T {}
impl crate::common::Reg<Config0> for Config0T {}

unsafe impl crate::common::Read<Config0> for Config0T {}
unsafe impl crate::common::Write<Config0> for Config0T {}
impl Config0 {
    #[doc = "Enable the interrupt when data is available"]
    #[inline(always)]
    pub fn irqen(self) -> crate::common::RegisterFieldBool<0, 1, 0, Config0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Config0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Config0> for Config0T {
    #[inline(always)]
    fn reset_value(&self) -> Config0 {
        Config0::new(0)
    }
}

#[doc = "Write messages to the VideoCore"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Write {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Write {
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
pub struct WriteT;
unsafe impl crate::common::AsPtr for WriteT {}
impl crate::common::Reg<Write> for WriteT {}

unsafe impl crate::common::Write<Write> for WriteT {}

impl crate::common::NoBitfieldReg for Write {}
impl crate::common::ResetValue<Write> for WriteT {
    #[inline(always)]
    fn reset_value(&self) -> Write {
        Write::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Peek1 {
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
pub struct Peek1T;
unsafe impl crate::common::AsPtr for Peek1T {}
impl crate::common::Reg<Peek1> for Peek1T {}

unsafe impl crate::common::Read<Peek1> for Peek1T {}
unsafe impl crate::common::Write<Peek1> for Peek1T {}

impl crate::common::NoBitfieldReg for Peek1 {}
impl crate::common::ResetValue<Peek1> for Peek1T {
    #[inline(always)]
    fn reset_value(&self) -> Peek1 {
        Peek1::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sender1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Sender1 {
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
pub struct Sender1T;
unsafe impl crate::common::AsPtr for Sender1T {}
impl crate::common::Reg<Sender1> for Sender1T {}

unsafe impl crate::common::Read<Sender1> for Sender1T {}
unsafe impl crate::common::Write<Sender1> for Sender1T {}

impl crate::common::NoBitfieldReg for Sender1 {}
impl crate::common::ResetValue<Sender1> for Sender1T {
    #[inline(always)]
    fn reset_value(&self) -> Sender1 {
        Sender1::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Status1 {
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
pub struct Status1T;
unsafe impl crate::common::AsPtr for Status1T {}
impl crate::common::Reg<Status1> for Status1T {}

unsafe impl crate::common::Read<Status1> for Status1T {}
unsafe impl crate::common::Write<Status1> for Status1T {}

impl crate::common::NoBitfieldReg for Status1 {}
impl crate::common::ResetValue<Status1> for Status1T {
    #[inline(always)]
    fn reset_value(&self) -> Status1 {
        Status1::new(0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Config1 {
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
pub struct Config1T;
unsafe impl crate::common::AsPtr for Config1T {}
impl crate::common::Reg<Config1> for Config1T {}

unsafe impl crate::common::Read<Config1> for Config1T {}
unsafe impl crate::common::Write<Config1> for Config1T {}

impl crate::common::NoBitfieldReg for Config1 {}
impl crate::common::ResetValue<Config1> for Config1T {
    #[inline(always)]
    fn reset_value(&self) -> Config1 {
        Config1::new(0)
    }
}
