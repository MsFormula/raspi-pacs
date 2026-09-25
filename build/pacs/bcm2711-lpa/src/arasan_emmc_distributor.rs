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
#[doc = r"Arasan SD3.0 Host AHB eMMC 4.4"]
unsafe impl ::core::marker::Send for super::ArasanEmmcDistributor {}
unsafe impl ::core::marker::Sync for super::ArasanEmmcDistributor {}
impl super::ArasanEmmcDistributor {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Argument for ACMD23 command"]
    #[inline(always)]
    pub fn arg2(&self) -> &'static self::Arg2T {
        unsafe { self::Arg2T::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "Numer and size in bytes for data block to be transferred"]
    #[inline(always)]
    pub fn blksizecnt(&self) -> &'static self::BlksizecntT {
        unsafe { self::BlksizecntT::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "Argument for everything but ACMD23"]
    #[inline(always)]
    pub fn arg1(&self) -> &'static self::Arg1T {
        unsafe { self::Arg1T::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "Issue commands to the card"]
    #[inline(always)]
    pub fn cmdtm(&self) -> &'static self::CmdtmT {
        unsafe { self::CmdtmT::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "Status bits of the response"]
    #[inline(always)]
    pub fn resp0(&self) -> &'static self::Resp0T {
        unsafe { self::Resp0T::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "Bits 63:32 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub fn resp1(&self) -> &'static self::Resp1T {
        unsafe { self::Resp1T::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "Bits 95:64 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub fn resp2(&self) -> &'static self::Resp2T {
        unsafe { self::Resp2T::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "Bits 127:96 of CMD2 and CMD10 responses"]
    #[inline(always)]
    pub fn resp3(&self) -> &'static self::Resp3T {
        unsafe { self::Resp3T::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "Data to/from the card"]
    #[inline(always)]
    pub fn data(&self) -> &'static self::DataT {
        unsafe { self::DataT::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "Status info for debugging"]
    #[inline(always)]
    pub fn status(&self) -> &'static self::StatusT {
        unsafe { self::StatusT::from_ptr(self._svd2pac_as_ptr().add(36usize)) }
    }

    #[doc = "Control"]
    #[inline(always)]
    pub fn control0(&self) -> &'static self::Control0T {
        unsafe { self::Control0T::from_ptr(self._svd2pac_as_ptr().add(40usize)) }
    }

    #[doc = "Configure"]
    #[inline(always)]
    pub fn control1(&self) -> &'static self::Control1T {
        unsafe { self::Control1T::from_ptr(self._svd2pac_as_ptr().add(44usize)) }
    }

    #[doc = "Interrupt flags"]
    #[inline(always)]
    pub fn interrupt(&self) -> &'static self::InterruptT {
        unsafe { self::InterruptT::from_ptr(self._svd2pac_as_ptr().add(48usize)) }
    }

    #[doc = "Mask interrupts that change in INTERRUPT"]
    #[inline(always)]
    pub fn irpt_mask(&self) -> &'static self::IrptMaskT {
        unsafe { self::IrptMaskT::from_ptr(self._svd2pac_as_ptr().add(52usize)) }
    }

    #[doc = "Enable interrupt to core"]
    #[inline(always)]
    pub fn irpt_en(&self) -> &'static self::IrptEnT {
        unsafe { self::IrptEnT::from_ptr(self._svd2pac_as_ptr().add(56usize)) }
    }

    #[doc = "Control 2"]
    #[inline(always)]
    pub fn control2(&self) -> &'static self::Control2T {
        unsafe { self::Control2T::from_ptr(self._svd2pac_as_ptr().add(60usize)) }
    }

    #[doc = "Force an interrupt"]
    #[inline(always)]
    pub fn force_irpt(&self) -> &'static self::ForceIrptT {
        unsafe { self::ForceIrptT::from_ptr(self._svd2pac_as_ptr().add(80usize)) }
    }

    #[doc = "Number of SD clock cycles to wait for boot"]
    #[inline(always)]
    pub fn boot_timeout(&self) -> &'static self::BootTimeoutT {
        unsafe { self::BootTimeoutT::from_ptr(self._svd2pac_as_ptr().add(112usize)) }
    }

    #[doc = "What submodules are accessed by the debug bus"]
    #[inline(always)]
    pub fn dbg_sel(&self) -> &'static self::DbgSelT {
        unsafe { self::DbgSelT::from_ptr(self._svd2pac_as_ptr().add(116usize)) }
    }

    #[doc = "Fine tune DMA request generation"]
    #[inline(always)]
    pub fn exrdfifo_cfg(&self) -> &'static self::ExrdfifoCfgT {
        unsafe { self::ExrdfifoCfgT::from_ptr(self._svd2pac_as_ptr().add(128usize)) }
    }

    #[doc = "Enable the extension data register"]
    #[inline(always)]
    pub fn exrdfifo_en(&self) -> &'static self::ExrdfifoEnT {
        unsafe { self::ExrdfifoEnT::from_ptr(self._svd2pac_as_ptr().add(132usize)) }
    }

    #[doc = "Sample clock delay step duration"]
    #[inline(always)]
    pub fn tune_step(&self) -> &'static self::TuneStepT {
        unsafe { self::TuneStepT::from_ptr(self._svd2pac_as_ptr().add(136usize)) }
    }

    #[doc = "Sample clock delay step count for SDR"]
    #[inline(always)]
    pub fn tune_steps_std(&self) -> &'static self::TuneStepsStdT {
        unsafe { self::TuneStepsStdT::from_ptr(self._svd2pac_as_ptr().add(140usize)) }
    }

    #[doc = "Sample clock delay step count for DDR"]
    #[inline(always)]
    pub fn tune_steps_ddr(&self) -> &'static self::TuneStepsDdrT {
        unsafe { self::TuneStepsDdrT::from_ptr(self._svd2pac_as_ptr().add(144usize)) }
    }

    #[doc = "Interrupts in SPI mode depend on CS"]
    #[inline(always)]
    pub fn spi_int_spt(&self) -> &'static self::SpiIntSptT {
        unsafe { self::SpiIntSptT::from_ptr(self._svd2pac_as_ptr().add(240usize)) }
    }

    #[doc = "Version information and slot interrupt status"]
    #[inline(always)]
    pub fn slotisr_ver(&self) -> &'static self::SlotisrVerT {
        unsafe { self::SlotisrVerT::from_ptr(self._svd2pac_as_ptr().add(252usize)) }
    }
}

#[doc = "Argument for ACMD23 command"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arg2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Arg2 {
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
pub struct Arg2T;
unsafe impl crate::common::AsPtr for Arg2T {}
impl crate::common::Reg<Arg2> for Arg2T {}

unsafe impl crate::common::Read<Arg2> for Arg2T {}
unsafe impl crate::common::Write<Arg2> for Arg2T {}

impl crate::common::NoBitfieldReg for Arg2 {}
impl crate::common::ResetValue<Arg2> for Arg2T {
    #[inline(always)]
    fn reset_value(&self) -> Arg2 {
        Arg2::new(0)
    }
}

#[doc = "Numer and size in bytes for data block to be transferred"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blksizecnt {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Blksizecnt {
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
pub struct BlksizecntT;
unsafe impl crate::common::AsPtr for BlksizecntT {}
impl crate::common::Reg<Blksizecnt> for BlksizecntT {}

unsafe impl crate::common::Read<Blksizecnt> for BlksizecntT {}
unsafe impl crate::common::Write<Blksizecnt> for BlksizecntT {}
impl Blksizecnt {
    #[doc = "Number of blocks to be transferred"]
    #[inline(always)]
    pub fn blkcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Blksizecnt, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Blksizecnt,common::RW>::from_register(self,0)
    }

    #[doc = "Block size in bytes"]
    #[inline(always)]
    pub fn blksize(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, Blksizecnt, common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,Blksizecnt,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Blksizecnt> for BlksizecntT {
    #[inline(always)]
    fn reset_value(&self) -> Blksizecnt {
        Blksizecnt::new(0)
    }
}

#[doc = "Argument for everything but ACMD23"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arg1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Arg1 {
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
pub struct Arg1T;
unsafe impl crate::common::AsPtr for Arg1T {}
impl crate::common::Reg<Arg1> for Arg1T {}

unsafe impl crate::common::Read<Arg1> for Arg1T {}
unsafe impl crate::common::Write<Arg1> for Arg1T {}

impl crate::common::NoBitfieldReg for Arg1 {}
impl crate::common::ResetValue<Arg1> for Arg1T {
    #[inline(always)]
    fn reset_value(&self) -> Arg1 {
        Arg1::new(0)
    }
}

#[doc = "Issue commands to the card"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdtm {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cmdtm {
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
pub struct CmdtmT;
unsafe impl crate::common::AsPtr for CmdtmT {}
impl crate::common::Reg<Cmdtm> for CmdtmT {}

unsafe impl crate::common::Read<Cmdtm> for CmdtmT {}
unsafe impl crate::common::Write<Cmdtm> for CmdtmT {}
impl Cmdtm {
    #[doc = "Command index to be issued"]
    #[inline(always)]
    pub fn cmd_index(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Cmdtm, common::RW> {
        crate::common::RegisterField::<24, 0x3f, 1, 0, u8, u8, Cmdtm, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Type of command to be issued"]
    #[inline(always)]
    pub fn cmd_type(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        cmdtm::CmdType,
        cmdtm::CmdType,
        Cmdtm,
        common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            cmdtm::CmdType,
            cmdtm::CmdType,
            Cmdtm,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command involves data"]
    #[inline(always)]
    pub fn cmd_isdata(self) -> crate::common::RegisterFieldBool<21, 1, 0, Cmdtm, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Cmdtm, common::RW>::from_register(self, 0)
    }

    #[doc = "Check that the response has the same command index"]
    #[inline(always)]
    pub fn cmd_ixchk_en(self) -> crate::common::RegisterFieldBool<20, 1, 0, Cmdtm, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Cmdtm, common::RW>::from_register(self, 0)
    }

    #[doc = "Check the responses CRC"]
    #[inline(always)]
    pub fn cmd_crcchk_en(self) -> crate::common::RegisterFieldBool<19, 1, 0, Cmdtm, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Cmdtm, common::RW>::from_register(self, 0)
    }

    #[doc = "Type of expected response"]
    #[inline(always)]
    pub fn cmd_rspns_type(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        cmdtm::CmdRspnsType,
        cmdtm::CmdRspnsType,
        Cmdtm,
        common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            cmdtm::CmdRspnsType,
            cmdtm::CmdRspnsType,
            Cmdtm,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Type of data transfer"]
    #[inline(always)]
    pub fn tm_multi_block(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cmdtm::TmMultiBlock,
        cmdtm::TmMultiBlock,
        Cmdtm,
        common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cmdtm::TmMultiBlock,
            cmdtm::TmMultiBlock,
            Cmdtm,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Direction of data transfer"]
    #[inline(always)]
    pub fn tm_dat_dir(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cmdtm::TmDatDir,
        cmdtm::TmDatDir,
        Cmdtm,
        common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cmdtm::TmDatDir,
            cmdtm::TmDatDir,
            Cmdtm,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Command after completion"]
    #[inline(always)]
    pub fn tm_auto_cmd_en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        cmdtm::TmAutoCmdEn,
        cmdtm::TmAutoCmdEn,
        Cmdtm,
        common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            cmdtm::TmAutoCmdEn,
            cmdtm::TmAutoCmdEn,
            Cmdtm,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable block counter"]
    #[inline(always)]
    pub fn tm_blkcnt_en(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cmdtm, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cmdtm, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cmdtm> for CmdtmT {
    #[inline(always)]
    fn reset_value(&self) -> Cmdtm {
        Cmdtm::new(0)
    }
}
pub mod cmdtm {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct CmdType(u8);

    impl CmdType {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for CmdType {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for CmdType {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<CmdType> for u64 {
        #[inline(always)]
        fn from(value: CmdType) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for CmdType {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl CmdType {
        pub const NORMAL: Self = Self(0);

        pub const SUSPEND: Self = Self(1);

        pub const RESUME: Self = Self(2);

        pub const ABORT: Self = Self(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct CmdRspnsType(u8);

    impl CmdRspnsType {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for CmdRspnsType {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for CmdRspnsType {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<CmdRspnsType> for u64 {
        #[inline(always)]
        fn from(value: CmdRspnsType) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for CmdRspnsType {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl CmdRspnsType {
        pub const RESPONSE_NONE: Self = Self(0);

        pub const RESPONSE_BITS_136: Self = Self(1);

        pub const RESPONSE_BITS_48: Self = Self(2);

        pub const RESPONSE_BITS_48_USING_BUSY: Self = Self(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct TmMultiBlock(u8);

    impl TmMultiBlock {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for TmMultiBlock {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for TmMultiBlock {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<TmMultiBlock> for u64 {
        #[inline(always)]
        fn from(value: TmMultiBlock) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for TmMultiBlock {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl TmMultiBlock {
        pub const SINGLE: Self = Self(0);

        pub const MULTIPLE: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct TmDatDir(u8);

    impl TmDatDir {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for TmDatDir {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for TmDatDir {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<TmDatDir> for u64 {
        #[inline(always)]
        fn from(value: TmDatDir) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for TmDatDir {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl TmDatDir {
        pub const HOST_TO_CARD: Self = Self(0);

        pub const CARD_TO_HOST: Self = Self(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct TmAutoCmdEn(u8);

    impl TmAutoCmdEn {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for TmAutoCmdEn {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for TmAutoCmdEn {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<TmAutoCmdEn> for u64 {
        #[inline(always)]
        fn from(value: TmAutoCmdEn) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for TmAutoCmdEn {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl TmAutoCmdEn {
        pub const NONE: Self = Self(0);

        pub const CMD_12: Self = Self(1);

        pub const CMD_23: Self = Self(2);
    }
}

#[doc = "Status bits of the response"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Resp0 {
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
pub struct Resp0T;
unsafe impl crate::common::AsPtr for Resp0T {}
impl crate::common::Reg<Resp0> for Resp0T {}

unsafe impl crate::common::Read<Resp0> for Resp0T {}
unsafe impl crate::common::Write<Resp0> for Resp0T {}

impl crate::common::NoBitfieldReg for Resp0 {}
impl crate::common::ResetValue<Resp0> for Resp0T {
    #[inline(always)]
    fn reset_value(&self) -> Resp0 {
        Resp0::new(0)
    }
}

#[doc = "Bits 63:32 of CMD2 and CMD10 responses"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Resp1 {
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
pub struct Resp1T;
unsafe impl crate::common::AsPtr for Resp1T {}
impl crate::common::Reg<Resp1> for Resp1T {}

unsafe impl crate::common::Read<Resp1> for Resp1T {}
unsafe impl crate::common::Write<Resp1> for Resp1T {}

impl crate::common::NoBitfieldReg for Resp1 {}
impl crate::common::ResetValue<Resp1> for Resp1T {
    #[inline(always)]
    fn reset_value(&self) -> Resp1 {
        Resp1::new(0)
    }
}

#[doc = "Bits 95:64 of CMD2 and CMD10 responses"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Resp2 {
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
pub struct Resp2T;
unsafe impl crate::common::AsPtr for Resp2T {}
impl crate::common::Reg<Resp2> for Resp2T {}

unsafe impl crate::common::Read<Resp2> for Resp2T {}
unsafe impl crate::common::Write<Resp2> for Resp2T {}

impl crate::common::NoBitfieldReg for Resp2 {}
impl crate::common::ResetValue<Resp2> for Resp2T {
    #[inline(always)]
    fn reset_value(&self) -> Resp2 {
        Resp2::new(0)
    }
}

#[doc = "Bits 127:96 of CMD2 and CMD10 responses"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Resp3 {
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
pub struct Resp3T;
unsafe impl crate::common::AsPtr for Resp3T {}
impl crate::common::Reg<Resp3> for Resp3T {}

unsafe impl crate::common::Read<Resp3> for Resp3T {}
unsafe impl crate::common::Write<Resp3> for Resp3T {}

impl crate::common::NoBitfieldReg for Resp3 {}
impl crate::common::ResetValue<Resp3> for Resp3T {
    #[inline(always)]
    fn reset_value(&self) -> Resp3 {
        Resp3::new(0)
    }
}

#[doc = "Data to/from the card"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Data {
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
pub struct DataT;
unsafe impl crate::common::AsPtr for DataT {}
impl crate::common::Reg<Data> for DataT {}

unsafe impl crate::common::Read<Data> for DataT {}
unsafe impl crate::common::Write<Data> for DataT {}

impl crate::common::NoBitfieldReg for Data {}
impl crate::common::ResetValue<Data> for DataT {
    #[inline(always)]
    fn reset_value(&self) -> Data {
        Data::new(0)
    }
}

#[doc = "Status info for debugging"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Status {
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
pub struct StatusT;
unsafe impl crate::common::AsPtr for StatusT {}
impl crate::common::Reg<Status> for StatusT {}

unsafe impl crate::common::Read<Status> for StatusT {}
unsafe impl crate::common::Write<Status> for StatusT {}
impl Status {
    #[doc = "Value of DAT\\[7:4\\]"]
    #[inline(always)]
    pub fn dat_level1(
        self,
    ) -> crate::common::RegisterField<25, 0xf, 1, 0, u8, u8, Status, common::RW> {
        crate::common::RegisterField::<25, 0xf, 1, 0, u8, u8, Status, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Value of CMD"]
    #[inline(always)]
    pub fn cmd_level(self) -> crate::common::RegisterFieldBool<24, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Status, common::RW>::from_register(self, 0)
    }

    #[doc = "Value of DAT\\[3:0\\]"]
    #[inline(always)]
    pub fn dat_level0(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Status, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Status, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "New data is available to read"]
    #[inline(always)]
    pub fn buffer_read_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Status, common::RW>::from_register(self, 0)
    }

    #[doc = "The buffer has space for new data"]
    #[inline(always)]
    pub fn buffer_write_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Status, common::RW>::from_register(self, 0)
    }

    #[doc = "Read transfer is active"]
    #[inline(always)]
    pub fn read_transfer(self) -> crate::common::RegisterFieldBool<9, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Status, common::RW>::from_register(self, 0)
    }

    #[doc = "Write transfer is active"]
    #[inline(always)]
    pub fn write_transfer(self) -> crate::common::RegisterFieldBool<8, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Status, common::RW>::from_register(self, 0)
    }

    #[doc = "At least one data line is active"]
    #[inline(always)]
    pub fn dat_active(self) -> crate::common::RegisterFieldBool<2, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Status, common::RW>::from_register(self, 0)
    }

    #[doc = "Data lines still in use"]
    #[inline(always)]
    pub fn dat_inhibit(self) -> crate::common::RegisterFieldBool<1, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Status, common::RW>::from_register(self, 0)
    }

    #[doc = "Command line still in use"]
    #[inline(always)]
    pub fn cmd_inhibit(self) -> crate::common::RegisterFieldBool<0, 1, 0, Status, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Status, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Status> for StatusT {
    #[inline(always)]
    fn reset_value(&self) -> Status {
        Status::new(0)
    }
}

#[doc = "Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Control0 {
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
pub struct Control0T;
unsafe impl crate::common::AsPtr for Control0T {}
impl crate::common::Reg<Control0> for Control0T {}

unsafe impl crate::common::Read<Control0> for Control0T {}
unsafe impl crate::common::Write<Control0> for Control0T {}
impl Control0 {
    #[doc = "Enable alternate boot mode"]
    #[inline(always)]
    pub fn alt_boot_en(self) -> crate::common::RegisterFieldBool<22, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot mode enabled"]
    #[inline(always)]
    pub fn boot_en(self) -> crate::common::RegisterFieldBool<21, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable SPI mode"]
    #[inline(always)]
    pub fn spi_mode(self) -> crate::common::RegisterFieldBool<20, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable interrupt on block gap"]
    #[inline(always)]
    pub fn gap_ien(self) -> crate::common::RegisterFieldBool<19, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Use DAT2 read/wait protocol"]
    #[inline(always)]
    pub fn readwait_en(self) -> crate::common::RegisterFieldBool<18, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Restart a transaction stopped by GAP_STOP"]
    #[inline(always)]
    pub fn gap_restart(self) -> crate::common::RegisterFieldBool<17, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Stop the current transaction at the next block gap"]
    #[inline(always)]
    pub fn gap_stop(self) -> crate::common::RegisterFieldBool<16, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Use 8 data lines"]
    #[inline(always)]
    pub fn hctl_8bit(self) -> crate::common::RegisterFieldBool<5, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable high speed mode"]
    #[inline(always)]
    pub fn hctl_hs_en(self) -> crate::common::RegisterFieldBool<2, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Control0, common::RW>::from_register(self, 0)
    }

    #[doc = "Use 4 data lines"]
    #[inline(always)]
    pub fn hctl_dwidth(self) -> crate::common::RegisterFieldBool<1, 1, 0, Control0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Control0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Control0> for Control0T {
    #[inline(always)]
    fn reset_value(&self) -> Control0 {
        Control0::new(0)
    }
}

#[doc = "Configure"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Control1 {
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
pub struct Control1T;
unsafe impl crate::common::AsPtr for Control1T {}
impl crate::common::Reg<Control1> for Control1T {}

unsafe impl crate::common::Read<Control1> for Control1T {}
unsafe impl crate::common::Write<Control1> for Control1T {}
impl Control1 {
    #[doc = "Reset the data handling circuit"]
    #[inline(always)]
    pub fn srst_data(self) -> crate::common::RegisterFieldBool<26, 1, 0, Control1, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Control1, common::RW>::from_register(self, 0)
    }

    #[doc = "Reset the command handling circuit"]
    #[inline(always)]
    pub fn srst_cmd(self) -> crate::common::RegisterFieldBool<25, 1, 0, Control1, common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Control1, common::RW>::from_register(self, 0)
    }

    #[doc = "Reset the complete host circuit"]
    #[inline(always)]
    pub fn srst_hc(self) -> crate::common::RegisterFieldBool<24, 1, 0, Control1, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Control1, common::RW>::from_register(self, 0)
    }

    #[doc = "Data timeout exponent (TMCLK * 2 ** (x + 13)) 1111 disabled"]
    #[inline(always)]
    pub fn data_tounit(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Control1, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Control1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Clock base divider LSB"]
    #[inline(always)]
    pub fn clk_freq8(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Control1, common::RW> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Control1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Clock base divider MSBs"]
    #[inline(always)]
    pub fn clk_freq_ms2(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, Control1, common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, u8, Control1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Mode of clock generation"]
    #[inline(always)]
    pub fn clk_gensel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        control1::ClkGensel,
        control1::ClkGensel,
        Control1,
        common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            control1::ClkGensel,
            control1::ClkGensel,
            Control1,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "SD Clock enable"]
    #[inline(always)]
    pub fn clk_en(self) -> crate::common::RegisterFieldBool<2, 1, 0, Control1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Control1, common::RW>::from_register(self, 0)
    }

    #[doc = "SD Clock stable"]
    #[inline(always)]
    pub fn clk_stable(self) -> crate::common::RegisterFieldBool<1, 1, 0, Control1, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Control1, common::R>::from_register(self, 0)
    }

    #[doc = "Enable internal clock"]
    #[inline(always)]
    pub fn clk_intlen(self) -> crate::common::RegisterFieldBool<0, 1, 0, Control1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Control1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Control1> for Control1T {
    #[inline(always)]
    fn reset_value(&self) -> Control1 {
        Control1::new(0)
    }
}
pub mod control1 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct ClkGensel(u8);

    impl ClkGensel {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for ClkGensel {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for ClkGensel {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<ClkGensel> for u64 {
        #[inline(always)]
        fn from(value: ClkGensel) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for ClkGensel {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl ClkGensel {
        pub const DIVIDED: Self = Self(0);

        pub const PROGRAMMABLE: Self = Self(1);
    }
}

#[doc = "Interrupt flags"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interrupt {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Interrupt {
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
pub struct InterruptT;
unsafe impl crate::common::AsPtr for InterruptT {}
impl crate::common::Reg<Interrupt> for InterruptT {}

unsafe impl crate::common::Read<Interrupt> for InterruptT {}
unsafe impl crate::common::Write<Interrupt> for InterruptT {}
impl Interrupt {
    #[doc = "Auto command error"]
    #[inline(always)]
    pub fn acmd_err(self) -> crate::common::RegisterFieldBool<24, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(self) -> crate::common::RegisterFieldBool<22, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(self) -> crate::common::RegisterFieldBool<21, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data timeout"]
    #[inline(always)]
    pub fn dto_err(self) -> crate::common::RegisterFieldBool<20, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(self) -> crate::common::RegisterFieldBool<19, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(self) -> crate::common::RegisterFieldBool<18, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(self) -> crate::common::RegisterFieldBool<17, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command timeout"]
    #[inline(always)]
    pub fn cto_err(self) -> crate::common::RegisterFieldBool<16, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "An error has occured"]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<15, 1, 0, Interrupt, common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Interrupt, common::R>::from_register(self, 0)
    }

    #[doc = "Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(self) -> crate::common::RegisterFieldBool<14, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(self) -> crate::common::RegisterFieldBool<13, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Clock retune request"]
    #[inline(always)]
    pub fn retune(self) -> crate::common::RegisterFieldBool<12, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Card made interrupt request"]
    #[inline(always)]
    pub fn card(self) -> crate::common::RegisterFieldBool<8, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(self) -> crate::common::RegisterFieldBool<5, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(self) -> crate::common::RegisterFieldBool<4, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(self) -> crate::common::RegisterFieldBool<2, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(self) -> crate::common::RegisterFieldBool<1, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command has finished"]
    #[inline(always)]
    pub fn cmd_done(self) -> crate::common::RegisterFieldBool<0, 1, 0, Interrupt, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Interrupt, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Interrupt> for InterruptT {
    #[inline(always)]
    fn reset_value(&self) -> Interrupt {
        Interrupt::new(0)
    }
}

#[doc = "Mask interrupts that change in INTERRUPT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrptMask {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for IrptMask {
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
pub struct IrptMaskT;
unsafe impl crate::common::AsPtr for IrptMaskT {}
impl crate::common::Reg<IrptMask> for IrptMaskT {}

unsafe impl crate::common::Read<IrptMask> for IrptMaskT {}
unsafe impl crate::common::Write<IrptMask> for IrptMaskT {}
impl IrptMask {
    #[doc = "Auto command error"]
    #[inline(always)]
    pub fn acmd_err(self) -> crate::common::RegisterFieldBool<24, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(self) -> crate::common::RegisterFieldBool<22, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(self) -> crate::common::RegisterFieldBool<21, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Data timeout"]
    #[inline(always)]
    pub fn dto_err(self) -> crate::common::RegisterFieldBool<20, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(self) -> crate::common::RegisterFieldBool<19, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(self) -> crate::common::RegisterFieldBool<18, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(self) -> crate::common::RegisterFieldBool<17, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Command timeout"]
    #[inline(always)]
    pub fn cto_err(self) -> crate::common::RegisterFieldBool<16, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(self) -> crate::common::RegisterFieldBool<14, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(self) -> crate::common::RegisterFieldBool<13, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Clock retune request"]
    #[inline(always)]
    pub fn retune(self) -> crate::common::RegisterFieldBool<12, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Card made interrupt request"]
    #[inline(always)]
    pub fn card(self) -> crate::common::RegisterFieldBool<8, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(self) -> crate::common::RegisterFieldBool<5, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(self) -> crate::common::RegisterFieldBool<4, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(self) -> crate::common::RegisterFieldBool<2, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(self) -> crate::common::RegisterFieldBool<1, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }

    #[doc = "Command has finished"]
    #[inline(always)]
    pub fn cmd_done(self) -> crate::common::RegisterFieldBool<0, 1, 0, IrptMask, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IrptMask, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<IrptMask> for IrptMaskT {
    #[inline(always)]
    fn reset_value(&self) -> IrptMask {
        IrptMask::new(0)
    }
}

#[doc = "Enable interrupt to core"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrptEn {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for IrptEn {
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
pub struct IrptEnT;
unsafe impl crate::common::AsPtr for IrptEnT {}
impl crate::common::Reg<IrptEn> for IrptEnT {}

unsafe impl crate::common::Read<IrptEn> for IrptEnT {}
unsafe impl crate::common::Write<IrptEn> for IrptEnT {}
impl IrptEn {
    #[doc = "Auto command error"]
    #[inline(always)]
    pub fn acmd_err(self) -> crate::common::RegisterFieldBool<24, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(self) -> crate::common::RegisterFieldBool<22, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(self) -> crate::common::RegisterFieldBool<21, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Data timeout"]
    #[inline(always)]
    pub fn dto_err(self) -> crate::common::RegisterFieldBool<20, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(self) -> crate::common::RegisterFieldBool<19, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(self) -> crate::common::RegisterFieldBool<18, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(self) -> crate::common::RegisterFieldBool<17, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Command timeout"]
    #[inline(always)]
    pub fn cto_err(self) -> crate::common::RegisterFieldBool<16, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(self) -> crate::common::RegisterFieldBool<14, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(self) -> crate::common::RegisterFieldBool<13, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Clock retune request"]
    #[inline(always)]
    pub fn retune(self) -> crate::common::RegisterFieldBool<12, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Card made interrupt request"]
    #[inline(always)]
    pub fn card(self) -> crate::common::RegisterFieldBool<8, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(self) -> crate::common::RegisterFieldBool<5, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(self) -> crate::common::RegisterFieldBool<4, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(self) -> crate::common::RegisterFieldBool<2, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(self) -> crate::common::RegisterFieldBool<1, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }

    #[doc = "Command has finished"]
    #[inline(always)]
    pub fn cmd_done(self) -> crate::common::RegisterFieldBool<0, 1, 0, IrptEn, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IrptEn, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<IrptEn> for IrptEnT {
    #[inline(always)]
    fn reset_value(&self) -> IrptEn {
        IrptEn::new(0)
    }
}

#[doc = "Control 2"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Control2 {
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
pub struct Control2T;
unsafe impl crate::common::AsPtr for Control2T {}
impl crate::common::Reg<Control2> for Control2T {}

unsafe impl crate::common::Read<Control2> for Control2T {}
unsafe impl crate::common::Write<Control2> for Control2T {}
impl Control2 {
    #[doc = "Tuned clock is used for sampling data"]
    #[inline(always)]
    pub fn tuned(self) -> crate::common::RegisterFieldBool<23, 1, 0, Control2, common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Control2, common::RW>::from_register(self, 0)
    }

    #[doc = "SD Clock tune in progress"]
    #[inline(always)]
    pub fn tuneon(self) -> crate::common::RegisterFieldBool<22, 1, 0, Control2, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Control2, common::RW>::from_register(self, 0)
    }

    #[doc = "Select the speed of the SD card"]
    #[inline(always)]
    pub fn uhsmode(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        control2::Uhsmode,
        control2::Uhsmode,
        Control2,
        common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            control2::Uhsmode,
            control2::Uhsmode,
            Control2,
            common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Error during auto CMD12"]
    #[inline(always)]
    pub fn notc12_err(self) -> crate::common::RegisterFieldBool<7, 1, 0, Control2, common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Control2, common::R>::from_register(self, 0)
    }

    #[doc = "Command index error during auto command"]
    #[inline(always)]
    pub fn acbad_err(self) -> crate::common::RegisterFieldBool<4, 1, 0, Control2, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Control2, common::R>::from_register(self, 0)
    }

    #[doc = "End bit is not 1 during auto command"]
    #[inline(always)]
    pub fn acend_err(self) -> crate::common::RegisterFieldBool<3, 1, 0, Control2, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Control2, common::R>::from_register(self, 0)
    }

    #[doc = "Command CRC error during auto command"]
    #[inline(always)]
    pub fn accrc_err(self) -> crate::common::RegisterFieldBool<2, 1, 0, Control2, common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Control2, common::R>::from_register(self, 0)
    }

    #[doc = "Auto command timeout"]
    #[inline(always)]
    pub fn acto_err(self) -> crate::common::RegisterFieldBool<1, 1, 0, Control2, common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Control2, common::R>::from_register(self, 0)
    }

    #[doc = "Auto command not executed due to an error"]
    #[inline(always)]
    pub fn acnox_err(self) -> crate::common::RegisterFieldBool<0, 1, 0, Control2, common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Control2, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Control2> for Control2T {
    #[inline(always)]
    fn reset_value(&self) -> Control2 {
        Control2::new(0)
    }
}
pub mod control2 {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Uhsmode(u8);

    impl Uhsmode {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Uhsmode {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Uhsmode {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Uhsmode> for u64 {
        #[inline(always)]
        fn from(value: Uhsmode) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Uhsmode {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Uhsmode {
        pub const SDR_12: Self = Self(0);

        pub const SDR_25: Self = Self(1);

        pub const SDR_50: Self = Self(2);

        pub const SDR_104: Self = Self(3);

        pub const DDR_50: Self = Self(4);
    }
}

#[doc = "Force an interrupt"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceIrpt {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ForceIrpt {
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
pub struct ForceIrptT;
unsafe impl crate::common::AsPtr for ForceIrptT {}
impl crate::common::Reg<ForceIrpt> for ForceIrptT {}

unsafe impl crate::common::Read<ForceIrpt> for ForceIrptT {}
unsafe impl crate::common::Write<ForceIrpt> for ForceIrptT {}
impl ForceIrpt {
    #[doc = "Auto command error"]
    #[inline(always)]
    pub fn acmd_err(self) -> crate::common::RegisterFieldBool<24, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data end bit error (not 1)"]
    #[inline(always)]
    pub fn dend_err(self) -> crate::common::RegisterFieldBool<22, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn dcrc_err(self) -> crate::common::RegisterFieldBool<21, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data timeout"]
    #[inline(always)]
    pub fn dto_err(self) -> crate::common::RegisterFieldBool<20, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Incorrect response command index"]
    #[inline(always)]
    pub fn cbad_err(self) -> crate::common::RegisterFieldBool<19, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command end bit error (not 1)"]
    #[inline(always)]
    pub fn cend_err(self) -> crate::common::RegisterFieldBool<18, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command CRC error"]
    #[inline(always)]
    pub fn ccrc_err(self) -> crate::common::RegisterFieldBool<17, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command timeout"]
    #[inline(always)]
    pub fn cto_err(self) -> crate::common::RegisterFieldBool<16, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot operation has terminated"]
    #[inline(always)]
    pub fn endboot(self) -> crate::common::RegisterFieldBool<14, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Boot has been acknowledged"]
    #[inline(always)]
    pub fn bootack(self) -> crate::common::RegisterFieldBool<13, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Clock retune request"]
    #[inline(always)]
    pub fn retune(self) -> crate::common::RegisterFieldBool<12, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Card made interrupt request"]
    #[inline(always)]
    pub fn card(self) -> crate::common::RegisterFieldBool<8, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA contains data to be read"]
    #[inline(always)]
    pub fn read_rdy(self) -> crate::common::RegisterFieldBool<5, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "DATA can be written to"]
    #[inline(always)]
    pub fn write_rdy(self) -> crate::common::RegisterFieldBool<4, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has stopped at block gap"]
    #[inline(always)]
    pub fn block_gap(self) -> crate::common::RegisterFieldBool<2, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Data transfer has finished"]
    #[inline(always)]
    pub fn data_done(self) -> crate::common::RegisterFieldBool<1, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }

    #[doc = "Command has finished"]
    #[inline(always)]
    pub fn cmd_done(self) -> crate::common::RegisterFieldBool<0, 1, 0, ForceIrpt, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, ForceIrpt, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<ForceIrpt> for ForceIrptT {
    #[inline(always)]
    fn reset_value(&self) -> ForceIrpt {
        ForceIrpt::new(0)
    }
}

#[doc = "Number of SD clock cycles to wait for boot"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootTimeout {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for BootTimeout {
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
pub struct BootTimeoutT;
unsafe impl crate::common::AsPtr for BootTimeoutT {}
impl crate::common::Reg<BootTimeout> for BootTimeoutT {}

unsafe impl crate::common::Read<BootTimeout> for BootTimeoutT {}
unsafe impl crate::common::Write<BootTimeout> for BootTimeoutT {}

impl crate::common::NoBitfieldReg for BootTimeout {}
impl crate::common::ResetValue<BootTimeout> for BootTimeoutT {
    #[inline(always)]
    fn reset_value(&self) -> BootTimeout {
        BootTimeout::new(0)
    }
}

#[doc = "What submodules are accessed by the debug bus"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgSel {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DbgSel {
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
pub struct DbgSelT;
unsafe impl crate::common::AsPtr for DbgSelT {}
impl crate::common::Reg<DbgSel> for DbgSelT {}

unsafe impl crate::common::Read<DbgSel> for DbgSelT {}
unsafe impl crate::common::Write<DbgSel> for DbgSelT {}
impl DbgSel {
    #[inline(always)]
    pub fn select(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dbg_sel::Select,
        dbg_sel::Select,
        DbgSel,
        common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dbg_sel::Select,
            dbg_sel::Select,
            DbgSel,
            common::RW,
        >::from_register(self, 0)
    }
}
impl crate::common::ResetValue<DbgSel> for DbgSelT {
    #[inline(always)]
    fn reset_value(&self) -> DbgSel {
        DbgSel::new(0)
    }
}
pub mod dbg_sel {
    #[allow(unused_imports)]
    use crate::common;
    #[allow(unused_imports)]
    use crate::common::{
        AsPtr as _, CastFrom, EnumBitfieldStruct, NoBitfieldReg as _, Read as _, Reg as _,
        RegisterValue as _, ResetValue as _, Write as _,
    };

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    #[repr(transparent)]
    pub struct Select(u8);

    impl Select {
        pub fn new(value: u8) -> Self {
            Self(value)
        }
    }

    impl crate::common::EnumBitfieldStruct for Select {
        type RegNumberT = u8;

        fn value(&self) -> Self::RegNumberT {
            self.0
        }
    }

    impl From<u8> for Select {
        #[inline(always)]
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl From<Select> for u64 {
        #[inline(always)]
        fn from(value: Select) -> Self {
            value.value().into()
        }
    }

    impl CastFrom<u64> for Select {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            Self(u8::cast_from(val))
        }
    }

    impl Select {
        pub const RECEIVER_FIFO: Self = Self(0);

        pub const OTHERS: Self = Self(1);
    }
}

#[doc = "Fine tune DMA request generation"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExrdfifoCfg {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ExrdfifoCfg {
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
pub struct ExrdfifoCfgT;
unsafe impl crate::common::AsPtr for ExrdfifoCfgT {}
impl crate::common::Reg<ExrdfifoCfg> for ExrdfifoCfgT {}

unsafe impl crate::common::Read<ExrdfifoCfg> for ExrdfifoCfgT {}
unsafe impl crate::common::Write<ExrdfifoCfg> for ExrdfifoCfgT {}
impl ExrdfifoCfg {
    #[doc = "Read threshold in 32 bit words"]
    #[inline(always)]
    pub fn rd_thrsh(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, ExrdfifoCfg, common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, ExrdfifoCfg, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<ExrdfifoCfg> for ExrdfifoCfgT {
    #[inline(always)]
    fn reset_value(&self) -> ExrdfifoCfg {
        ExrdfifoCfg::new(0)
    }
}

#[doc = "Enable the extension data register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExrdfifoEn {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ExrdfifoEn {
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
pub struct ExrdfifoEnT;
unsafe impl crate::common::AsPtr for ExrdfifoEnT {}
impl crate::common::Reg<ExrdfifoEn> for ExrdfifoEnT {}

unsafe impl crate::common::Read<ExrdfifoEn> for ExrdfifoEnT {}
unsafe impl crate::common::Write<ExrdfifoEn> for ExrdfifoEnT {}
impl ExrdfifoEn {
    #[doc = "Enable the extension FIFO"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::RegisterFieldBool<0, 1, 0, ExrdfifoEn, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, ExrdfifoEn, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<ExrdfifoEn> for ExrdfifoEnT {
    #[inline(always)]
    fn reset_value(&self) -> ExrdfifoEn {
        ExrdfifoEn::new(0)
    }
}

#[doc = "Sample clock delay step duration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuneStep {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TuneStep {
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
pub struct TuneStepT;
unsafe impl crate::common::AsPtr for TuneStepT {}
impl crate::common::Reg<TuneStep> for TuneStepT {}

unsafe impl crate::common::Read<TuneStep> for TuneStepT {}
unsafe impl crate::common::Write<TuneStep> for TuneStepT {}
impl TuneStep {
    #[inline(always)]
    pub fn delay(self) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, TuneStep, common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, u8, TuneStep, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<TuneStep> for TuneStepT {
    #[inline(always)]
    fn reset_value(&self) -> TuneStep {
        TuneStep::new(0)
    }
}

#[doc = "Sample clock delay step count for SDR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuneStepsStd {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TuneStepsStd {
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
pub struct TuneStepsStdT;
unsafe impl crate::common::AsPtr for TuneStepsStdT {}
impl crate::common::Reg<TuneStepsStd> for TuneStepsStdT {}

unsafe impl crate::common::Read<TuneStepsStd> for TuneStepsStdT {}
unsafe impl crate::common::Write<TuneStepsStd> for TuneStepsStdT {}
impl TuneStepsStd {
    #[inline(always)]
    pub fn steps(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, TuneStepsStd, common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,TuneStepsStd,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TuneStepsStd> for TuneStepsStdT {
    #[inline(always)]
    fn reset_value(&self) -> TuneStepsStd {
        TuneStepsStd::new(0)
    }
}

#[doc = "Sample clock delay step count for DDR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuneStepsDdr {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TuneStepsDdr {
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
pub struct TuneStepsDdrT;
unsafe impl crate::common::AsPtr for TuneStepsDdrT {}
impl crate::common::Reg<TuneStepsDdr> for TuneStepsDdrT {}

unsafe impl crate::common::Read<TuneStepsDdr> for TuneStepsDdrT {}
unsafe impl crate::common::Write<TuneStepsDdr> for TuneStepsDdrT {}
impl TuneStepsDdr {
    #[inline(always)]
    pub fn steps(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, TuneStepsDdr, common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,TuneStepsDdr,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TuneStepsDdr> for TuneStepsDdrT {
    #[inline(always)]
    fn reset_value(&self) -> TuneStepsDdr {
        TuneStepsDdr::new(0)
    }
}

#[doc = "Interrupts in SPI mode depend on CS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiIntSpt {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SpiIntSpt {
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
pub struct SpiIntSptT;
unsafe impl crate::common::AsPtr for SpiIntSptT {}
impl crate::common::Reg<SpiIntSpt> for SpiIntSptT {}

unsafe impl crate::common::Read<SpiIntSpt> for SpiIntSptT {}
unsafe impl crate::common::Write<SpiIntSpt> for SpiIntSptT {}
impl SpiIntSpt {
    #[inline(always)]
    pub fn select(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SpiIntSpt, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, SpiIntSpt, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<SpiIntSpt> for SpiIntSptT {
    #[inline(always)]
    fn reset_value(&self) -> SpiIntSpt {
        SpiIntSpt::new(0)
    }
}

#[doc = "Version information and slot interrupt status"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlotisrVer {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SlotisrVer {
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
pub struct SlotisrVerT;
unsafe impl crate::common::AsPtr for SlotisrVerT {}
impl crate::common::Reg<SlotisrVer> for SlotisrVerT {}

unsafe impl crate::common::Read<SlotisrVer> for SlotisrVerT {}
unsafe impl crate::common::Write<SlotisrVer> for SlotisrVerT {}
impl SlotisrVer {
    #[doc = "Vendor version number"]
    #[inline(always)]
    pub fn vendor(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, SlotisrVer, common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,SlotisrVer,common::RW>::from_register(self,0)
    }

    #[doc = "Host controller specification version"]
    #[inline(always)]
    pub fn sdversion(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, SlotisrVer, common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,SlotisrVer,common::RW>::from_register(self,0)
    }

    #[doc = "OR of interrupt and wakeup signals for each slot"]
    #[inline(always)]
    pub fn slot_status(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SlotisrVer, common::RW> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, u8, SlotisrVer, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<SlotisrVer> for SlotisrVerT {
    #[inline(always)]
    fn reset_value(&self) -> SlotisrVer {
        SlotisrVer::new(0)
    }
}
