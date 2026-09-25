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
#[doc = r"Aux SPI"]
unsafe impl ::core::marker::Send for super::Spi1 {}
unsafe impl ::core::marker::Sync for super::Spi1 {}
impl super::Spi1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Control 0"]
    #[inline(always)]
    pub fn cntl0(&self) -> &'static self::Cntl0T {
        unsafe { self::Cntl0T::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Control 1"]
    #[inline(always)]
    pub fn cntl1(&self) -> &'static self::Cntl1T {
        unsafe { self::Cntl1T::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub fn stat(&self) -> &'static self::StatT {
        unsafe { self::StatT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Read the RXFIFO without removing an entry"]
    #[inline(always)]
    pub fn peek(&self) -> &'static self::PeekT {
        unsafe { self::PeekT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "Writing to the FIFO will deassert CS at the end of the access"]
    #[inline(always)]
    pub const fn io(&self) -> &'static crate::common::ClusterRegisterArray<self::IoT, 4, 0x4> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }

    #[doc = "Writing to the FIFO will maintain CS at the end of the access"]
    #[inline(always)]
    pub const fn txhold(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<self::TxholdT, 4, 0x4> {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x30usize))
        }
    }
}

#[doc = "Control 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntl0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cntl0 {
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
pub struct Cntl0T;
unsafe impl crate::common::AsPtr for Cntl0T {}
impl crate::common::Reg<Cntl0> for Cntl0T {}

unsafe impl crate::common::Read<Cntl0> for Cntl0T {}
unsafe impl crate::common::Write<Cntl0> for Cntl0T {}
impl Cntl0 {
    #[doc = "SPI clock speed. clk = sys / 2 * (SPEED + 1)"]
    #[inline(always)]
    pub fn speed(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, u16, Cntl0, common::RW> {
        crate::common::RegisterField::<20, 0xfff, 1, 0, u16, u16, Cntl0, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "The CS pattern when active"]
    #[inline(always)]
    pub fn chip_selects(
        self,
    ) -> crate::common::RegisterField<17, 0x7, 1, 0, u8, u8, Cntl0, common::RW> {
        crate::common::RegisterField::<17, 0x7, 1, 0, u8, u8, Cntl0, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Post input mode"]
    #[inline(always)]
    pub fn post_input(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Take CS pattern and data from TX FIFO (along with VARIABLE_WIDTH)"]
    #[inline(always)]
    pub fn variable_cs(self) -> crate::common::RegisterFieldBool<15, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Take shift length and data from FIFO"]
    #[inline(always)]
    pub fn variable_width(self) -> crate::common::RegisterFieldBool<14, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Controls extra DOUT hold time in system clock cycles"]
    #[inline(always)]
    pub fn dout_hold_time(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        cntl0::DoutHoldTime,
        cntl0::DoutHoldTime,
        Cntl0,
        common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            cntl0::DoutHoldTime,
            cntl0::DoutHoldTime,
            Cntl0,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable the interface"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::RegisterFieldBool<11, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Data is clocked in on rising edge of CLK"]
    #[inline(always)]
    pub fn in_rising(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Clear FIFOs"]
    #[inline(always)]
    pub fn clear_fifos(self) -> crate::common::RegisterFieldBool<9, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Data is clocked out on rising edge of CLK"]
    #[inline(always)]
    pub fn out_rising(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Idle clock high"]
    #[inline(always)]
    pub fn invert_clk(self) -> crate::common::RegisterFieldBool<7, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Shift out the most significant bit (MSB) first"]
    #[inline(always)]
    pub fn msb_first(self) -> crate::common::RegisterFieldBool<6, 1, 0, Cntl0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cntl0, common::RW>::from_register(self, 0)
    }

    #[doc = "Number of bits to shift"]
    #[inline(always)]
    pub fn shift_length(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Cntl0, common::RW> {
        crate::common::RegisterField::<0, 0x3f, 1, 0, u8, u8, Cntl0, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Cntl0> for Cntl0T {
    #[inline(always)]
    fn reset_value(&self) -> Cntl0 {
        Cntl0::new(917504)
    }
}
pub mod cntl0 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct DoutHoldTime(u8);

    impl DoutHoldTime {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for DoutHoldTime {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for DoutHoldTime {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<DoutHoldTime> for u64 {
        #[inline(always)]
        fn from(value: DoutHoldTime) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for DoutHoldTime {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl DoutHoldTime {
        pub const _0: Self = Self(0);

        pub const _1: Self = Self(1);

        pub const _4: Self = Self(2);

        pub const _7: Self = Self(3);
    }
}

#[doc = "Control 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntl1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cntl1 {
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
pub struct Cntl1T;
unsafe impl crate::common::AsPtr for Cntl1T {}
impl crate::common::Reg<Cntl1> for Cntl1T {}

unsafe impl crate::common::Read<Cntl1> for Cntl1T {}
unsafe impl crate::common::Write<Cntl1> for Cntl1T {}
impl Cntl1 {
    #[doc = "Additional SPI clock cycles where CS is high"]
    #[inline(always)]
    pub fn cs_high_time(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, Cntl1, common::RW> {
        crate::common::RegisterField::<8, 0x7, 1, 0, u8, u8, Cntl1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable TX empty interrupt"]
    #[inline(always)]
    pub fn txe_enable(self) -> crate::common::RegisterFieldBool<7, 1, 0, Cntl1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cntl1, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable DONE interrupt"]
    #[inline(always)]
    pub fn done_enable(self) -> crate::common::RegisterFieldBool<6, 1, 0, Cntl1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cntl1, common::RW>::from_register(self, 0)
    }

    #[doc = "Shift the most significant bit first (MSB)"]
    #[inline(always)]
    pub fn msb_first(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cntl1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cntl1, common::RW>::from_register(self, 0)
    }

    #[doc = "Don\'t clear the RX shift register before a new transaction"]
    #[inline(always)]
    pub fn keep_input(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cntl1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cntl1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cntl1> for Cntl1T {
    #[inline(always)]
    fn reset_value(&self) -> Cntl1 {
        Cntl1::new(0)
    }
}

#[doc = "Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stat {
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
pub struct StatT;
unsafe impl crate::common::AsPtr for StatT {}
impl crate::common::Reg<Stat> for StatT {}

unsafe impl crate::common::Read<Stat> for StatT {}
unsafe impl crate::common::Write<Stat> for StatT {}
impl Stat {
    #[doc = "Number of entries in TX FIFO"]
    #[inline(always)]
    pub fn tx_level(self) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Stat, common::RW> {
        crate::common::RegisterField::<24, 0xf, 1, 0, u8, u8, Stat, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Number of entries in RX FIFO"]
    #[inline(always)]
    pub fn rx_level(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Stat, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Stat, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX FIFO is full"]
    #[inline(always)]
    pub fn tx_full(self) -> crate::common::RegisterFieldBool<10, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn tx_empty(self) -> crate::common::RegisterFieldBool<9, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn rx_full(self) -> crate::common::RegisterFieldBool<8, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "RX FIFO is empty"]
    #[inline(always)]
    pub fn rx_empty(self) -> crate::common::RegisterFieldBool<7, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Indicates a transfer is ongoing"]
    #[inline(always)]
    pub fn busy(self) -> crate::common::RegisterFieldBool<6, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Number of bits left to be processed."]
    #[inline(always)]
    pub fn bit_count(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Stat, common::RW> {
        crate::common::RegisterField::<0, 0x3f, 1, 0, u8, u8, Stat, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Stat> for StatT {
    #[inline(always)]
    fn reset_value(&self) -> Stat {
        Stat::new(0)
    }
}

#[doc = "Read the RXFIFO without removing an entry"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peek {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Peek {
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
pub struct PeekT;
unsafe impl crate::common::AsPtr for PeekT {}
impl crate::common::Reg<Peek> for PeekT {}

unsafe impl crate::common::Read<Peek> for PeekT {}
impl Peek {
    #[doc = "FIFO data access"]
    #[inline(always)]
    pub fn data(self) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Peek, common::R> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Peek, common::R>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Peek> for PeekT {
    #[inline(always)]
    fn reset_value(&self) -> Peek {
        Peek::new(0)
    }
}

#[doc = "Writing to the FIFO will deassert CS at the end of the access"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Io {
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
pub struct IoT;
unsafe impl crate::common::AsPtr for IoT {}
impl crate::common::Reg<Io> for IoT {}

unsafe impl crate::common::Read<Io> for IoT {}
unsafe impl crate::common::Write<Io> for IoT {}
impl Io {
    #[doc = "FIFO data access"]
    #[inline(always)]
    pub fn data(self) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Io, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Io, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Io> for IoT {
    #[inline(always)]
    fn reset_value(&self) -> Io {
        Io::new(0)
    }
}

#[doc = "Writing to the FIFO will maintain CS at the end of the access"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txhold {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Txhold {
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
pub struct TxholdT;
unsafe impl crate::common::AsPtr for TxholdT {}
impl crate::common::Reg<Txhold> for TxholdT {}

unsafe impl crate::common::Read<Txhold> for TxholdT {}
unsafe impl crate::common::Write<Txhold> for TxholdT {}
impl Txhold {
    #[doc = "FIFO data access"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Txhold, common::RW> {
        crate::common::RegisterField::<0, 0xffff, 1, 0, u16, u16, Txhold, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Txhold> for TxholdT {
    #[inline(always)]
    fn reset_value(&self) -> Txhold {
        Txhold::new(0)
    }
}
