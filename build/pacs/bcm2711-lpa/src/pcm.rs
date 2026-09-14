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
// Generated from SVD A, with svd2pac 0.7.0 on Mon, 14 Sep 2026 00:08:44 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r""]
unsafe impl ::core::marker::Send for super::Pcm {}
unsafe impl ::core::marker::Sync for super::Pcm {}
impl super::Pcm {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "PCM Control and Status"]
    #[inline(always)]
    pub const fn cs_a(&self) -> &'static crate::common::Reg<self::CsA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CsA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "This is the FIFO port of the PCM"]
    #[inline(always)]
    pub const fn fifo_a(&self) -> &'static crate::common::Reg<self::FifoA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::FifoA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "This register defines the basic PCM Operating Mode"]
    #[inline(always)]
    pub const fn mode_a(&self) -> &'static crate::common::Reg<self::ModeA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ModeA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Sets the Channel configurations for Receiving"]
    #[inline(always)]
    pub const fn rxc_a(&self) -> &'static crate::common::Reg<self::RxcA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RxcA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Sets the Channel configurations for Transmitting"]
    #[inline(always)]
    pub const fn txc_a(&self) -> &'static crate::common::Reg<self::TxcA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxcA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Set the DMA DREQ and Panic thresholds"]
    #[inline(always)]
    pub const fn dreq_a(&self) -> &'static crate::common::Reg<self::DreqA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DreqA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Set the reasons for generating an Interrupt"]
    #[inline(always)]
    pub const fn inten_a(
        &self,
    ) -> &'static crate::common::Reg<self::IntenA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IntenA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "This register is used to read and clear the PCM interrupt status"]
    #[inline(always)]
    pub const fn intstc_a(
        &self,
    ) -> &'static crate::common::Reg<self::IntstcA_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::IntstcA_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "This register is used to control the gray mode generation"]
    #[inline(always)]
    pub const fn gray(&self) -> &'static crate::common::Reg<self::Gray_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gray_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsA_SPEC;
impl crate::sealed::RegSpec for CsA_SPEC {
    type DataType = u32;
}

#[doc = "PCM Control and Status"]
pub type CsA = crate::RegValueT<CsA_SPEC>;

impl CsA {
    #[doc = "Enable the PCM Audio Interface"]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable reception"]
    #[inline(always)]
    pub fn rxon(self) -> crate::common::RegisterFieldBool<1, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable transmission"]
    #[inline(always)]
    pub fn txon(self) -> crate::common::RegisterFieldBool<2, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Clear the TX FIFO"]
    #[inline(always)]
    pub fn txclr(self) -> crate::common::RegisterFieldBool<3, 1, 0, CsA_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, CsA_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Clear the RX FIFO"]
    #[inline(always)]
    pub fn rxclr(self) -> crate::common::RegisterFieldBool<4, 1, 0, CsA_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, CsA_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Sets the TX FIFO threshold at which point the TXW flag is set"]
    #[inline(always)]
    pub fn txthr(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,CsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the RX FIFO threshold at which point the RXR flag is set"]
    #[inline(always)]
    pub fn rxthr(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,CsA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DMA DREQ Enable"]
    #[inline(always)]
    pub fn dmaen(self) -> crate::common::RegisterFieldBool<9, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX FIFO Sync"]
    #[inline(always)]
    pub fn txsync(self) -> crate::common::RegisterFieldBool<13, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX FIFO Sync"]
    #[inline(always)]
    pub fn rxsync(self) -> crate::common::RegisterFieldBool<14, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TX FIFO Error"]
    #[inline(always)]
    pub fn txerr(self) -> crate::common::RegisterFieldBool<15, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX FIFO Error"]
    #[inline(always)]
    pub fn rxerr(self) -> crate::common::RegisterFieldBool<16, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that the TX FIFO needs Writing"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<17, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that the RX FIFO needs Reading"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<18, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that the TX FIFO can accept data"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::RegisterFieldBool<19, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that the RX FIFO contains data"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::RegisterFieldBool<20, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TX FIFO is Empty"]
    #[inline(always)]
    pub fn txe(self) -> crate::common::RegisterFieldBool<21, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX FIFO is Full"]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<22, 1, 0, CsA_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, CsA_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Sign Extend"]
    #[inline(always)]
    pub fn rxsex(self) -> crate::common::RegisterFieldBool<23, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM Clock sync helper"]
    #[inline(always)]
    pub fn sync(self) -> crate::common::RegisterFieldBool<24, 1, 0, CsA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, CsA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for CsA {
    #[inline(always)]
    fn default() -> CsA {
        <crate::RegValueT<CsA_SPEC> as RegisterValue<_>>::new(2752512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoA_SPEC;
impl crate::sealed::RegSpec for FifoA_SPEC {
    type DataType = u32;
}

#[doc = "This is the FIFO port of the PCM"]
pub type FifoA = crate::RegValueT<FifoA_SPEC>;

impl FifoA {
    #[doc = "FIFO"]
    #[inline(always)]
    pub fn fifo(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, FifoA_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,FifoA_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FifoA {
    #[inline(always)]
    fn default() -> FifoA {
        <crate::RegValueT<FifoA_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModeA_SPEC;
impl crate::sealed::RegSpec for ModeA_SPEC {
    type DataType = u32;
}

#[doc = "This register defines the basic PCM Operating Mode"]
pub type ModeA = crate::RegValueT<ModeA_SPEC>;

impl ModeA {
    #[doc = "Frame Sync Length"]
    #[inline(always)]
    pub fn fslen(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,ModeA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Frame Length"]
    #[inline(always)]
    pub fn flen(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, u16, ModeA_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3ff,1,0,u16,u16,ModeA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Frame Sync Invert This logically inverts the frame sync signal"]
    #[inline(always)]
    pub fn fsi(self) -> crate::common::RegisterFieldBool<20, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Sync Mode"]
    #[inline(always)]
    pub fn fsm(self) -> crate::common::RegisterFieldBool<21, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Clock Invert this logically inverts the PCM_CLK signal"]
    #[inline(always)]
    pub fn clki(self) -> crate::common::RegisterFieldBool<22, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM Clock Mode"]
    #[inline(always)]
    pub fn clkm(self) -> crate::common::RegisterFieldBool<23, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Transmit Frame Packed Mode"]
    #[inline(always)]
    pub fn ftxp(self) -> crate::common::RegisterFieldBool<24, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Receive Frame Packed Mode"]
    #[inline(always)]
    pub fn frxp(self) -> crate::common::RegisterFieldBool<25, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PDM Input Mode Enable"]
    #[inline(always)]
    pub fn pdme(self) -> crate::common::RegisterFieldBool<26, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PDM Decimation Factor (N)"]
    #[inline(always)]
    pub fn pdmn(self) -> crate::common::RegisterFieldBool<27, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PCM Clock Disable"]
    #[inline(always)]
    pub fn clk_dis(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, ModeA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, ModeA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for ModeA {
    #[inline(always)]
    fn default() -> ModeA {
        <crate::RegValueT<ModeA_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxcA_SPEC;
impl crate::sealed::RegSpec for RxcA_SPEC {
    type DataType = u32;
}

#[doc = "Sets the Channel configurations for Receiving"]
pub type RxcA = crate::RegValueT<RxcA_SPEC>;

impl RxcA {
    #[doc = "Channel 2 Width"]
    #[inline(always)]
    pub fn ch2wid(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,RxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 2 Position"]
    #[inline(always)]
    pub fn ch2pos(
        self,
    ) -> crate::common::RegisterField<4, 0x3ff, 1, 0, u16, u16, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3ff,1,0,u16,u16,RxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 2 Enable"]
    #[inline(always)]
    pub fn ch2en(self) -> crate::common::RegisterFieldBool<14, 1, 0, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, RxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 2 Width Extension Bit"]
    #[inline(always)]
    pub fn ch2wex(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, RxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Width"]
    #[inline(always)]
    pub fn ch1wid(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,RxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 1 Position"]
    #[inline(always)]
    pub fn ch1pos(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,RxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(self) -> crate::common::RegisterFieldBool<30, 1, 0, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, RxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Width Extension Bit"]
    #[inline(always)]
    pub fn ch1wex(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, RxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for RxcA {
    #[inline(always)]
    fn default() -> RxcA {
        <crate::RegValueT<RxcA_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxcA_SPEC;
impl crate::sealed::RegSpec for TxcA_SPEC {
    type DataType = u32;
}

#[doc = "Sets the Channel configurations for Transmitting"]
pub type TxcA = crate::RegValueT<TxcA_SPEC>;

impl TxcA {
    #[doc = "Channel 2 Width"]
    #[inline(always)]
    pub fn ch2wid(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,TxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 2 Position"]
    #[inline(always)]
    pub fn ch2pos(
        self,
    ) -> crate::common::RegisterField<4, 0x3ff, 1, 0, u16, u16, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3ff,1,0,u16,u16,TxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 2 Enable"]
    #[inline(always)]
    pub fn ch2en(self) -> crate::common::RegisterFieldBool<14, 1, 0, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, TxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 2 Width Extension Bit"]
    #[inline(always)]
    pub fn ch2wex(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, TxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Width"]
    #[inline(always)]
    pub fn ch1wid(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,TxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 1 Position"]
    #[inline(always)]
    pub fn ch1pos(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, u16, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16,u16,TxcA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(self) -> crate::common::RegisterFieldBool<30, 1, 0, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, TxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Channel 1 Width Extension Bit"]
    #[inline(always)]
    pub fn ch1wex(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, TxcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, TxcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for TxcA {
    #[inline(always)]
    fn default() -> TxcA {
        <crate::RegValueT<TxcA_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DreqA_SPEC;
impl crate::sealed::RegSpec for DreqA_SPEC {
    type DataType = u32;
}

#[doc = "Set the DMA DREQ and Panic thresholds"]
pub type DreqA = crate::RegValueT<DreqA_SPEC>;

impl DreqA {
    #[doc = "RX Request Level"]
    #[inline(always)]
    pub fn rx_req(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, DreqA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,DreqA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TX Request Level"]
    #[inline(always)]
    pub fn tx_req(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, DreqA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,DreqA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RX Panic Level"]
    #[inline(always)]
    pub fn rx_panic(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, DreqA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,DreqA_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TX Panic Level"]
    #[inline(always)]
    pub fn tx_panic(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, u8, DreqA_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7f,1,0,u8,u8,DreqA_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DreqA {
    #[inline(always)]
    fn default() -> DreqA {
        <crate::RegValueT<DreqA_SPEC> as RegisterValue<_>>::new(271593504)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntenA_SPEC;
impl crate::sealed::RegSpec for IntenA_SPEC {
    type DataType = u32;
}

#[doc = "Set the reasons for generating an Interrupt"]
pub type IntenA = crate::RegValueT<IntenA_SPEC>;

impl IntenA {
    #[doc = "TX Write Interrupt Enable"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntenA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntenA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Read Interrupt Enable"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntenA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntenA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Error Interrupt"]
    #[inline(always)]
    pub fn txerr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntenA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntenA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Error Interrupt"]
    #[inline(always)]
    pub fn rxerr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntenA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntenA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for IntenA {
    #[inline(always)]
    fn default() -> IntenA {
        <crate::RegValueT<IntenA_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntstcA_SPEC;
impl crate::sealed::RegSpec for IntstcA_SPEC {
    type DataType = u32;
}

#[doc = "This register is used to read and clear the PCM interrupt status"]
pub type IntstcA = crate::RegValueT<IntstcA_SPEC>;

impl IntstcA {
    #[doc = "TX Write Interrupt Status / Clear"]
    #[inline(always)]
    pub fn txw(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntstcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntstcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Read Interrupt Status / Clear"]
    #[inline(always)]
    pub fn rxr(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntstcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntstcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TX Error Interrupt Status / Clear"]
    #[inline(always)]
    pub fn txerr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntstcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntstcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RX Error Interrupt Status / Clear"]
    #[inline(always)]
    pub fn rxerr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntstcA_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntstcA_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for IntstcA {
    #[inline(always)]
    fn default() -> IntstcA {
        <crate::RegValueT<IntstcA_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gray_SPEC;
impl crate::sealed::RegSpec for Gray_SPEC {
    type DataType = u32;
}

#[doc = "This register is used to control the gray mode generation"]
pub type Gray = crate::RegValueT<Gray_SPEC>;

impl Gray {
    #[doc = "Enable GRAY Mode"]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gray_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gray_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Clear the GRAY Mode Logic"]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gray_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gray_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Flush the RX Buffer into the RX FIFO"]
    #[inline(always)]
    pub fn flush(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gray_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gray_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "The Current fill level of the RX Buffer"]
    #[inline(always)]
    pub fn rxlevel(
        self,
    ) -> crate::common::RegisterField<4, 0x3f, 1, 0, u8, u8, Gray_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3f,1,0,u8,u8,Gray_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "The Number of bits that were flushed into the RXFIFO"]
    #[inline(always)]
    pub fn flushed(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Gray_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Gray_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "The Current level of the RXFIFO"]
    #[inline(always)]
    pub fn rxfifolevel(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Gray_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Gray_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gray {
    #[inline(always)]
    fn default() -> Gray {
        <crate::RegValueT<Gray_SPEC> as RegisterValue<_>>::new(0)
    }
}
