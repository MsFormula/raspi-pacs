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
// Generated from SVD A, with svd2pac 0.7.0 on Mon, 14 Sep 2026 00:08:42 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ARM Prime Cell PL011"]
unsafe impl ::core::marker::Send for super::ArmUartPl011 {}
unsafe impl ::core::marker::Sync for super::ArmUartPl011 {}
impl super::ArmUartPl011 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Data Register"]
    #[inline(always)]
    pub const fn dr(&self) -> &'static crate::common::Reg<self::Dr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Receive Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &'static crate::common::Reg<self::Rsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Error Clear Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &'static crate::common::Reg<self::Ecr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ecr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Flag Register"]
    #[inline(always)]
    pub const fn fr(&self) -> &'static crate::common::Reg<self::Fr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Integer Baud Rate Register"]
    #[inline(always)]
    pub const fn ibrd(&self) -> &'static crate::common::Reg<self::Ibrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ibrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Fractional Baud Rate Register"]
    #[inline(always)]
    pub const fn fbrd(&self) -> &'static crate::common::Reg<self::Fbrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fbrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Line Control Register"]
    #[inline(always)]
    pub const fn lcr_h(&self) -> &'static crate::common::Reg<self::LcrH_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcrH_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &'static crate::common::Reg<self::Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Interrupt FIFO Level Select Register"]
    #[inline(always)]
    pub const fn ifls(&self) -> &'static crate::common::Reg<self::Ifls_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ifls_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Interrupt Mask set_Clear Register"]
    #[inline(always)]
    pub const fn imsc(&self) -> &'static crate::common::Reg<self::Imsc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Imsc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn ris(&self) -> &'static crate::common::Reg<self::Ris_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ris_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn mis(&self) -> &'static crate::common::Reg<self::Mis_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mis_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &'static crate::common::Reg<self::Icr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Icr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "DMA Control Register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &'static crate::common::Reg<self::Dmacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dmacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr_SPEC;
impl crate::sealed::RegSpec for Dr_SPEC {
    type DataType = u32;
}

#[doc = "Data Register"]
pub type Dr = crate::RegValueT<Dr_SPEC>;

impl Dr {
    #[doc = "DATA"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Dr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Dr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FE"]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PE"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Dr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Dr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "BE"]
    #[inline(always)]
    pub fn be(self) -> crate::common::RegisterFieldBool<10, 1, 0, Dr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Dr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<11, 1, 0, Dr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Dr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        <crate::RegValueT<Dr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsr_SPEC;
impl crate::sealed::RegSpec for Rsr_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Register"]
pub type Rsr = crate::RegValueT<Rsr_SPEC>;

impl Rsr {
    #[doc = "FE"]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "PE"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "BE"]
    #[inline(always)]
    pub fn be(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rsr {
    #[inline(always)]
    fn default() -> Rsr {
        <crate::RegValueT<Rsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr_SPEC;
impl crate::sealed::RegSpec for Ecr_SPEC {
    type DataType = u32;
}

#[doc = "Error Clear Register"]
pub type Ecr = crate::RegValueT<Ecr_SPEC>;

impl Ecr {
    #[doc = "FE"]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ecr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ecr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PE"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ecr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ecr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BE"]
    #[inline(always)]
    pub fn be(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ecr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ecr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ecr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ecr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        <crate::RegValueT<Ecr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fr_SPEC;
impl crate::sealed::RegSpec for Fr_SPEC {
    type DataType = u32;
}

#[doc = "Flag Register"]
pub type Fr = crate::RegValueT<Fr_SPEC>;

impl Fr {
    #[doc = "CTS"]
    #[inline(always)]
    pub fn cts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DSR"]
    #[inline(always)]
    pub fn dsr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DCD"]
    #[inline(always)]
    pub fn dcd(self) -> crate::common::RegisterFieldBool<2, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "BUSY"]
    #[inline(always)]
    pub fn busy(self) -> crate::common::RegisterFieldBool<3, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RXFE"]
    #[inline(always)]
    pub fn rxfe(self) -> crate::common::RegisterFieldBool<4, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TXFF"]
    #[inline(always)]
    pub fn txff(self) -> crate::common::RegisterFieldBool<5, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RXFF"]
    #[inline(always)]
    pub fn rxff(self) -> crate::common::RegisterFieldBool<6, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TXFE"]
    #[inline(always)]
    pub fn txfe(self) -> crate::common::RegisterFieldBool<7, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RI"]
    #[inline(always)]
    pub fn ri(self) -> crate::common::RegisterFieldBool<8, 1, 0, Fr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fr {
    #[inline(always)]
    fn default() -> Fr {
        <crate::RegValueT<Fr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibrd_SPEC;
impl crate::sealed::RegSpec for Ibrd_SPEC {
    type DataType = u32;
}

#[doc = "Integer Baud Rate Register"]
pub type Ibrd = crate::RegValueT<Ibrd_SPEC>;

impl Ibrd {
    #[doc = "BAUDDIVINT"]
    #[inline(always)]
    pub fn bauddivint(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Ibrd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Ibrd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ibrd {
    #[inline(always)]
    fn default() -> Ibrd {
        <crate::RegValueT<Ibrd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbrd_SPEC;
impl crate::sealed::RegSpec for Fbrd_SPEC {
    type DataType = u32;
}

#[doc = "Fractional Baud Rate Register"]
pub type Fbrd = crate::RegValueT<Fbrd_SPEC>;

impl Fbrd {
    #[doc = "BAUDDIVFRAC"]
    #[inline(always)]
    pub fn bauddivfrac(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Fbrd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Fbrd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbrd {
    #[inline(always)]
    fn default() -> Fbrd {
        <crate::RegValueT<Fbrd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcrH_SPEC;
impl crate::sealed::RegSpec for LcrH_SPEC {
    type DataType = u32;
}

#[doc = "Line Control Register"]
pub type LcrH = crate::RegValueT<LcrH_SPEC>;

impl LcrH {
    #[doc = "BRK"]
    #[inline(always)]
    pub fn brk(self) -> crate::common::RegisterFieldBool<0, 1, 0, LcrH_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, LcrH_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PEN"]
    #[inline(always)]
    pub fn pen(self) -> crate::common::RegisterFieldBool<1, 1, 0, LcrH_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, LcrH_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "EPS"]
    #[inline(always)]
    pub fn eps(self) -> crate::common::RegisterFieldBool<2, 1, 0, LcrH_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, LcrH_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "STP2"]
    #[inline(always)]
    pub fn stp2(self) -> crate::common::RegisterFieldBool<3, 1, 0, LcrH_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, LcrH_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FEN"]
    #[inline(always)]
    pub fn fen(self) -> crate::common::RegisterFieldBool<4, 1, 0, LcrH_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, LcrH_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "WLEN"]
    #[inline(always)]
    pub fn wlen(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, LcrH_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,LcrH_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "SPS"]
    #[inline(always)]
    pub fn sps(self) -> crate::common::RegisterFieldBool<7, 1, 0, LcrH_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, LcrH_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for LcrH {
    #[inline(always)]
    fn default() -> LcrH {
        <crate::RegValueT<LcrH_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr_SPEC;
impl crate::sealed::RegSpec for Cr_SPEC {
    type DataType = u32;
}

#[doc = "Control Register"]
pub type Cr = crate::RegValueT<Cr_SPEC>;

impl Cr {
    #[doc = "UARTEN"]
    #[inline(always)]
    pub fn uarten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SIREN"]
    #[inline(always)]
    pub fn siren(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "SIRLP"]
    #[inline(always)]
    pub fn sirlp(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TXE"]
    #[inline(always)]
    pub fn txe(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RXE"]
    #[inline(always)]
    pub fn rxe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DTR"]
    #[inline(always)]
    pub fn dtr(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RTS"]
    #[inline(always)]
    pub fn rts(self) -> crate::common::RegisterFieldBool<11, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RTSEN"]
    #[inline(always)]
    pub fn rtsen(self) -> crate::common::RegisterFieldBool<14, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CTSEN"]
    #[inline(always)]
    pub fn ctsen(self) -> crate::common::RegisterFieldBool<15, 1, 0, Cr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        <crate::RegValueT<Cr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifls_SPEC;
impl crate::sealed::RegSpec for Ifls_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt FIFO Level Select Register"]
pub type Ifls = crate::RegValueT<Ifls_SPEC>;

impl Ifls {
    #[doc = "TXIFLSEL"]
    #[inline(always)]
    pub fn txiflsel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Ifls_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Ifls_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "RXIFLSEL"]
    #[inline(always)]
    pub fn rxiflsel(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, Ifls_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,Ifls_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ifls {
    #[inline(always)]
    fn default() -> Ifls {
        <crate::RegValueT<Ifls_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imsc_SPEC;
impl crate::sealed::RegSpec for Imsc_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Mask set_Clear Register"]
pub type Imsc = crate::RegValueT<Imsc_SPEC>;

impl Imsc {
    #[doc = "RIMIM"]
    #[inline(always)]
    pub fn rimim(self) -> crate::common::RegisterFieldBool<0, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CTSMIM"]
    #[inline(always)]
    pub fn ctsmim(self) -> crate::common::RegisterFieldBool<1, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDMIM"]
    #[inline(always)]
    pub fn dcdmim(self) -> crate::common::RegisterFieldBool<2, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DSRMIM"]
    #[inline(always)]
    pub fn dsrmim(self) -> crate::common::RegisterFieldBool<3, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RXIM"]
    #[inline(always)]
    pub fn rxim(self) -> crate::common::RegisterFieldBool<4, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TXIM"]
    #[inline(always)]
    pub fn txim(self) -> crate::common::RegisterFieldBool<5, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "RTIM"]
    #[inline(always)]
    pub fn rtim(self) -> crate::common::RegisterFieldBool<6, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "FEIM"]
    #[inline(always)]
    pub fn feim(self) -> crate::common::RegisterFieldBool<7, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "PEIM"]
    #[inline(always)]
    pub fn peim(self) -> crate::common::RegisterFieldBool<8, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "BEIM"]
    #[inline(always)]
    pub fn beim(self) -> crate::common::RegisterFieldBool<9, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "OEIM"]
    #[inline(always)]
    pub fn oeim(self) -> crate::common::RegisterFieldBool<10, 1, 0, Imsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Imsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Imsc {
    #[inline(always)]
    fn default() -> Imsc {
        <crate::RegValueT<Imsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ris_SPEC;
impl crate::sealed::RegSpec for Ris_SPEC {
    type DataType = u32;
}

#[doc = "Raw Interrupt Status Register"]
pub type Ris = crate::RegValueT<Ris_SPEC>;

impl Ris {
    #[doc = "RIRMIS"]
    #[inline(always)]
    pub fn rirmis(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "CTSRMIS"]
    #[inline(always)]
    pub fn ctsrmis(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDRMIS"]
    #[inline(always)]
    pub fn dcdrmis(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DSRRMIS"]
    #[inline(always)]
    pub fn dsrrmis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RXRIS"]
    #[inline(always)]
    pub fn rxris(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TXRIS"]
    #[inline(always)]
    pub fn txris(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RTRIS"]
    #[inline(always)]
    pub fn rtris(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FERIS"]
    #[inline(always)]
    pub fn feris(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "PERIS"]
    #[inline(always)]
    pub fn peris(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "BERIS"]
    #[inline(always)]
    pub fn beris(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OERIS"]
    #[inline(always)]
    pub fn oeris(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ris_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ris_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ris {
    #[inline(always)]
    fn default() -> Ris {
        <crate::RegValueT<Ris_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mis_SPEC;
impl crate::sealed::RegSpec for Mis_SPEC {
    type DataType = u32;
}

#[doc = "Masked Interrupt Status Register"]
pub type Mis = crate::RegValueT<Mis_SPEC>;

impl Mis {
    #[doc = "RIMMIS"]
    #[inline(always)]
    pub fn rimmis(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "CTSMMIS"]
    #[inline(always)]
    pub fn ctsmmis(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDMMIS"]
    #[inline(always)]
    pub fn dcdmmis(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DSRMMIS"]
    #[inline(always)]
    pub fn dsrmmis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RXMIS"]
    #[inline(always)]
    pub fn rxmis(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "TXMIS"]
    #[inline(always)]
    pub fn txmis(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RTMIS"]
    #[inline(always)]
    pub fn rtmis(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FEMIS"]
    #[inline(always)]
    pub fn femis(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "PEMIS"]
    #[inline(always)]
    pub fn pemis(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "BEMIS"]
    #[inline(always)]
    pub fn bemis(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OEMIS"]
    #[inline(always)]
    pub fn oemis(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mis_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mis_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mis {
    #[inline(always)]
    fn default() -> Mis {
        <crate::RegValueT<Mis_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt Clear Register"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "RIMIC"]
    #[inline(always)]
    pub fn rimic(self) -> crate::common::RegisterFieldBool<0, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CTSMIC"]
    #[inline(always)]
    pub fn ctsmic(self) -> crate::common::RegisterFieldBool<1, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DCDMIC"]
    #[inline(always)]
    pub fn dcdmic(self) -> crate::common::RegisterFieldBool<2, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "DSRMIC"]
    #[inline(always)]
    pub fn dsrmic(self) -> crate::common::RegisterFieldBool<3, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "RXIC"]
    #[inline(always)]
    pub fn rxic(self) -> crate::common::RegisterFieldBool<4, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "TXIC"]
    #[inline(always)]
    pub fn txic(self) -> crate::common::RegisterFieldBool<5, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "RTIC"]
    #[inline(always)]
    pub fn rtic(self) -> crate::common::RegisterFieldBool<6, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "FEIC"]
    #[inline(always)]
    pub fn feic(self) -> crate::common::RegisterFieldBool<7, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "PEIC"]
    #[inline(always)]
    pub fn peic(self) -> crate::common::RegisterFieldBool<8, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "BEIC"]
    #[inline(always)]
    pub fn beic(self) -> crate::common::RegisterFieldBool<9, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "OEIC"]
    #[inline(always)]
    pub fn oeic(self) -> crate::common::RegisterFieldBool<10, 1, 0, Icr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Icr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacr_SPEC;
impl crate::sealed::RegSpec for Dmacr_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Register"]
pub type Dmacr = crate::RegValueT<Dmacr_SPEC>;

impl Dmacr {
    #[doc = "RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dmacr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dmacr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "TXDMAE"]
    #[inline(always)]
    pub fn txdmae(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dmacr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dmacr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMAONERR"]
    #[inline(always)]
    pub fn dmaonerr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dmacr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dmacr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dmacr {
    #[inline(always)]
    fn default() -> Dmacr {
        <crate::RegValueT<Dmacr_SPEC> as RegisterValue<_>>::new(0)
    }
}
