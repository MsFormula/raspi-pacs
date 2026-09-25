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
#[doc = r"Mini UART"]
unsafe impl ::core::marker::Send for super::Uart1 {}
unsafe impl ::core::marker::Sync for super::Uart1 {}
impl super::Uart1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "I/O Data"]
    #[inline(always)]
    pub fn io(&self) -> &'static self::IoT {
        unsafe { self::IoT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Lower bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub fn baudl(&self) -> &'static self::BaudlT {
        unsafe { self::BaudlT::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn ier(&self) -> &'static self::IerT {
        unsafe { self::IerT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "High bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub fn baudh(&self) -> &'static self::BaudhT {
        unsafe { self::BaudhT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Interrupt Identify"]
    #[inline(always)]
    pub fn iir(&self) -> &'static self::IirT {
        unsafe { self::IirT::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Line control"]
    #[inline(always)]
    pub fn lcr(&self) -> &'static self::LcrT {
        unsafe { self::LcrT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "Modem Control"]
    #[inline(always)]
    pub fn mcr(&self) -> &'static self::McrT {
        unsafe { self::McrT::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "Line Status"]
    #[inline(always)]
    pub fn lsr(&self) -> &'static self::LsrT {
        unsafe { self::LsrT::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "Modem Status"]
    #[inline(always)]
    pub fn msr(&self) -> &'static self::MsrT {
        unsafe { self::MsrT::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "Scratch"]
    #[inline(always)]
    pub fn scratch(&self) -> &'static self::ScratchT {
        unsafe { self::ScratchT::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "Control"]
    #[inline(always)]
    pub fn cntl(&self) -> &'static self::CntlT {
        unsafe { self::CntlT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "Status"]
    #[inline(always)]
    pub fn stat(&self) -> &'static self::StatT {
        unsafe { self::StatT::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }

    #[doc = "Baudrate"]
    #[inline(always)]
    pub fn baud(&self) -> &'static self::BaudT {
        unsafe { self::BaudT::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }
}

#[doc = "I/O Data"]
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
    #[doc = "FIFO access"]
    #[inline(always)]
    pub fn data(self) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Io, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, Io, common::RW>::from_register(
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

#[doc = "Lower bits of baudrate when DLAB is set"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baudl {
    pub(crate) data: u8,
    pub(crate) mask: u8,
}

impl crate::common::RegisterValue for Baudl {
    type DataType = u8;

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
pub struct BaudlT;
unsafe impl crate::common::AsPtr for BaudlT {}
impl crate::common::Reg<Baudl> for BaudlT {}

unsafe impl crate::common::Read<Baudl> for BaudlT {}
unsafe impl crate::common::Write<Baudl> for BaudlT {}

impl crate::common::NoBitfieldReg for Baudl {}
impl crate::common::ResetValue<Baudl> for BaudlT {
    #[inline(always)]
    fn reset_value(&self) -> Baudl {
        Baudl::new(0)
    }
}

#[doc = "Interrupt Enable"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ier {
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
pub struct IerT;
unsafe impl crate::common::AsPtr for IerT {}
impl crate::common::Reg<Ier> for IerT {}

unsafe impl crate::common::Read<Ier> for IerT {}
unsafe impl crate::common::Write<Ier> for IerT {}
impl Ier {
    #[doc = "Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ier, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ier, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ier, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ier, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ier> for IerT {
    #[inline(always)]
    fn reset_value(&self) -> Ier {
        Ier::new(0)
    }
}

#[doc = "High bits of baudrate when DLAB is set"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baudh {
    pub(crate) data: u8,
    pub(crate) mask: u8,
}

impl crate::common::RegisterValue for Baudh {
    type DataType = u8;

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
pub struct BaudhT;
unsafe impl crate::common::AsPtr for BaudhT {}
impl crate::common::Reg<Baudh> for BaudhT {}

unsafe impl crate::common::Read<Baudh> for BaudhT {}
unsafe impl crate::common::Write<Baudh> for BaudhT {}

impl crate::common::NoBitfieldReg for Baudh {}
impl crate::common::ResetValue<Baudh> for BaudhT {
    #[inline(always)]
    fn reset_value(&self) -> Baudh {
        Baudh::new(0)
    }
}

#[doc = "Interrupt Identify"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iir {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Iir {
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
pub struct IirT;
unsafe impl crate::common::AsPtr for IirT {}
impl crate::common::Reg<Iir> for IirT {}

unsafe impl crate::common::Read<Iir> for IirT {}
unsafe impl crate::common::Write<Iir> for IirT {}
impl Iir {
    #[doc = "Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(self) -> crate::common::RegisterFieldBool<2, 1, 0, Iir, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Iir, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(self) -> crate::common::RegisterFieldBool<1, 1, 0, Iir, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Iir, common::RW>::from_register(self, 0)
    }

    #[doc = "No pending interrupt"]
    #[inline(always)]
    pub fn npending(self) -> crate::common::RegisterFieldBool<0, 1, 0, Iir, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Iir, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Iir> for IirT {
    #[inline(always)]
    fn reset_value(&self) -> Iir {
        Iir::new(45057)
    }
}

#[doc = "Line control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Lcr {
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
pub struct LcrT;
unsafe impl crate::common::AsPtr for LcrT {}
impl crate::common::Reg<Lcr> for LcrT {}

unsafe impl crate::common::Read<Lcr> for LcrT {}
unsafe impl crate::common::Write<Lcr> for LcrT {}
impl Lcr {
    #[doc = "First two registers are baudrate"]
    #[inline(always)]
    pub fn dlab(self) -> crate::common::RegisterFieldBool<7, 1, 0, Lcr, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Lcr, common::RW>::from_register(self, 0)
    }

    #[doc = "Pull TX low continuously to send break"]
    #[inline(always)]
    pub fn r#break(self) -> crate::common::RegisterFieldBool<6, 1, 0, Lcr, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Lcr, common::RW>::from_register(self, 0)
    }

    #[doc = "UART word size"]
    #[inline(always)]
    pub fn data_size(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, lcr::DataSize, lcr::DataSize, Lcr, common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,lcr::DataSize,lcr::DataSize,Lcr,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Lcr> for LcrT {
    #[inline(always)]
    fn reset_value(&self) -> Lcr {
        Lcr::new(0)
    }
}
pub mod lcr {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct DataSize(u8);

    impl DataSize {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for DataSize {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for DataSize {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<DataSize> for u64 {
        #[inline(always)]
        fn from(value: DataSize) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for DataSize {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl DataSize {
        #[doc = "7 bit"]
        pub const MODE_7_BIT: Self = Self(0);

        #[doc = "8 bit"]
        pub const MODE_8_BIT: Self = Self(3);
    }
}

#[doc = "Modem Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Mcr {
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
pub struct McrT;
unsafe impl crate::common::AsPtr for McrT {}
impl crate::common::Reg<Mcr> for McrT {}

unsafe impl crate::common::Read<Mcr> for McrT {}
unsafe impl crate::common::Write<Mcr> for McrT {}
impl Mcr {
    #[doc = "RTS is low"]
    #[inline(always)]
    pub fn rts(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mcr, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mcr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Mcr> for McrT {
    #[inline(always)]
    fn reset_value(&self) -> Mcr {
        Mcr::new(0)
    }
}

#[doc = "Line Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lsr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Lsr {
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
pub struct LsrT;
unsafe impl crate::common::AsPtr for LsrT {}
impl crate::common::Reg<Lsr> for LsrT {}

unsafe impl crate::common::Read<Lsr> for LsrT {}
unsafe impl crate::common::Write<Lsr> for LsrT {}
impl Lsr {
    #[doc = "Transmit FIFO empty and all bits shifted out"]
    #[inline(always)]
    pub fn tx_idle(self) -> crate::common::RegisterFieldBool<6, 1, 0, Lsr, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Lsr, common::RW>::from_register(self, 0)
    }

    #[doc = "Transmit FIFO has room for at least one byte"]
    #[inline(always)]
    pub fn tx_empty(self) -> crate::common::RegisterFieldBool<5, 1, 0, Lsr, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Lsr, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive FIFO overrun"]
    #[inline(always)]
    pub fn rx_overrun(self) -> crate::common::RegisterFieldBool<1, 1, 0, Lsr, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Lsr, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive FIFO has at least one byte"]
    #[inline(always)]
    pub fn data_ready(self) -> crate::common::RegisterFieldBool<0, 1, 0, Lsr, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Lsr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Lsr> for LsrT {
    #[inline(always)]
    fn reset_value(&self) -> Lsr {
        Lsr::new(0)
    }
}

#[doc = "Modem Status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Msr {
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
pub struct MsrT;
unsafe impl crate::common::AsPtr for MsrT {}
impl crate::common::Reg<Msr> for MsrT {}

unsafe impl crate::common::Read<Msr> for MsrT {}
unsafe impl crate::common::Write<Msr> for MsrT {}
impl Msr {
    #[doc = "CTS is low"]
    #[inline(always)]
    pub fn cts(self) -> crate::common::RegisterFieldBool<4, 1, 0, Msr, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Msr, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Msr> for MsrT {
    #[inline(always)]
    fn reset_value(&self) -> Msr {
        Msr::new(0)
    }
}

#[doc = "Scratch"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scratch {
    pub(crate) data: u8,
    pub(crate) mask: u8,
}

impl crate::common::RegisterValue for Scratch {
    type DataType = u8;

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
pub struct ScratchT;
unsafe impl crate::common::AsPtr for ScratchT {}
impl crate::common::Reg<Scratch> for ScratchT {}

unsafe impl crate::common::Read<Scratch> for ScratchT {}
unsafe impl crate::common::Write<Scratch> for ScratchT {}

impl crate::common::NoBitfieldReg for Scratch {}
impl crate::common::ResetValue<Scratch> for ScratchT {
    #[inline(always)]
    fn reset_value(&self) -> Scratch {
        Scratch::new(0)
    }
}

#[doc = "Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntl {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cntl {
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
pub struct CntlT;
unsafe impl crate::common::AsPtr for CntlT {}
impl crate::common::Reg<Cntl> for CntlT {}

unsafe impl crate::common::Read<Cntl> for CntlT {}
unsafe impl crate::common::Write<Cntl> for CntlT {}
impl Cntl {
    #[doc = "CTS assert level"]
    #[inline(always)]
    pub fn cts_assert(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        cntl::CtsAssert,
        cntl::CtsAssert,
        Cntl,
        common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            cntl::CtsAssert,
            cntl::CtsAssert,
            Cntl,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "RTS assert level"]
    #[inline(always)]
    pub fn rts_assert(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        cntl::RtsAssert,
        cntl::RtsAssert,
        Cntl,
        common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            cntl::RtsAssert,
            cntl::RtsAssert,
            Cntl,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO level to de-assert RTS"]
    #[inline(always)]
    pub fn rts_fifo_level(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        cntl::RtsFifoLevel,
        cntl::RtsFifoLevel,
        Cntl,
        common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            cntl::RtsFifoLevel,
            cntl::RtsFifoLevel,
            Cntl,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable auto transmit flow control with CTS"]
    #[inline(always)]
    pub fn cts_enable(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cntl, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cntl, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable auto receive flow control with RTS"]
    #[inline(always)]
    pub fn rts_enable(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cntl, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cntl, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable transmit"]
    #[inline(always)]
    pub fn tx_enable(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cntl, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cntl, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable receive"]
    #[inline(always)]
    pub fn rx_enable(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cntl, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cntl, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cntl> for CntlT {
    #[inline(always)]
    fn reset_value(&self) -> Cntl {
        Cntl::new(0)
    }
}
pub mod cntl {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct CtsAssert(u8);

    impl CtsAssert {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for CtsAssert {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for CtsAssert {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<CtsAssert> for u64 {
        #[inline(always)]
        fn from(value: CtsAssert) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for CtsAssert {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl CtsAssert {
        #[doc = "Assert high"]
        pub const ASSERT_LEVEL_HIGH: Self = Self(0);

        #[doc = "Assert low"]
        pub const ASSERT_LEVEL_LOW: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct RtsAssert(u8);

    impl RtsAssert {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for RtsAssert {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for RtsAssert {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<RtsAssert> for u64 {
        #[inline(always)]
        fn from(value: RtsAssert) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for RtsAssert {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl RtsAssert {
        #[doc = "Assert high"]
        pub const ASSERT_LEVEL_HIGH: Self = Self(0);

        #[doc = "Assert low"]
        pub const ASSERT_LEVEL_LOW: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct RtsFifoLevel(u8);

    impl RtsFifoLevel {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for RtsFifoLevel {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for RtsFifoLevel {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<RtsFifoLevel> for u64 {
        #[inline(always)]
        fn from(value: RtsFifoLevel) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for RtsFifoLevel {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl RtsFifoLevel {
        #[doc = "3 empty spaces"]
        pub const FIFO_LEVEL_3_EMPTY: Self = Self(0);

        #[doc = "2 empty spaces"]
        pub const FIFO_LEVEL_2_EMPTY: Self = Self(1);

        #[doc = "1 empty spaces"]
        pub const FIFO_LEVEL_1_EMPTY: Self = Self(2);

        #[doc = "4 empty spaces"]
        pub const FIFO_LEVEL_4_EMPTY: Self = Self(3);
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
    #[doc = "How many entries are filled in the TX FIFO"]
    #[inline(always)]
    pub fn tx_fifo_level(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Stat, common::RW> {
        crate::common::RegisterField::<24, 0xf, 1, 0, u8, u8, Stat, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "How many entries are filled in the RX FIFO"]
    #[inline(always)]
    pub fn rx_fifo_level(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Stat, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Stat, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Transmit FIFO is empty and transmitter is idle"]
    #[inline(always)]
    pub fn tx_done(self) -> crate::common::RegisterFieldBool<9, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Transmit FIFO is completely empty"]
    #[inline(always)]
    pub fn tx_empty(self) -> crate::common::RegisterFieldBool<8, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "CTS state"]
    #[inline(always)]
    pub fn cts_status(self) -> crate::common::RegisterFieldBool<7, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "RTS state"]
    #[inline(always)]
    pub fn rts_status(self) -> crate::common::RegisterFieldBool<6, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Transmit FIFO is full"]
    #[inline(always)]
    pub fn tx_full(self) -> crate::common::RegisterFieldBool<5, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive FIFO overrun"]
    #[inline(always)]
    pub fn rx_overrun(self) -> crate::common::RegisterFieldBool<4, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Transmitter is idle"]
    #[inline(always)]
    pub fn tx_idle(self) -> crate::common::RegisterFieldBool<3, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Receiver is idle"]
    #[inline(always)]
    pub fn rx_idle(self) -> crate::common::RegisterFieldBool<2, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Transmit FIFO has space for at least one symbol"]
    #[inline(always)]
    pub fn tx_ready(self) -> crate::common::RegisterFieldBool<1, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Stat, common::RW>::from_register(self, 0)
    }

    #[doc = "Receive FIFO has at least one symbol"]
    #[inline(always)]
    pub fn data_ready(self) -> crate::common::RegisterFieldBool<0, 1, 0, Stat, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Stat, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Stat> for StatT {
    #[inline(always)]
    fn reset_value(&self) -> Stat {
        Stat::new(0)
    }
}

#[doc = "Baudrate"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baud {
    pub(crate) data: u16,
    pub(crate) mask: u16,
}

impl crate::common::RegisterValue for Baud {
    type DataType = u16;

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
pub struct BaudT;
unsafe impl crate::common::AsPtr for BaudT {}
impl crate::common::Reg<Baud> for BaudT {}

unsafe impl crate::common::Read<Baud> for BaudT {}
unsafe impl crate::common::Write<Baud> for BaudT {}

impl crate::common::NoBitfieldReg for Baud {}
impl crate::common::ResetValue<Baud> for BaudT {
    #[inline(always)]
    fn reset_value(&self) -> Baud {
        Baud::new(0)
    }
}
