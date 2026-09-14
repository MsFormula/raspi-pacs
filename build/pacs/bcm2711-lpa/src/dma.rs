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
unsafe impl ::core::marker::Send for super::Dma {}
unsafe impl ::core::marker::Sync for super::Dma {}
impl super::Dma {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_0(&self) -> &'static crate::common::Reg<self::Cs0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_0(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_0(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_0(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_0(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub const fn ti_0(&self) -> &'static crate::common::Reg<self::Ti0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "DMA Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_0(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub const fn stride_0(
        &self,
    ) -> &'static crate::common::Reg<self::Stride0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stride0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_0(
        &self,
    ) -> &'static crate::common::Reg<self::Debug0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_1(&self) -> &'static crate::common::Reg<self::Cs1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_1(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_1(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_1(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_1(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub const fn ti_1(&self) -> &'static crate::common::Reg<self::Ti1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "DMA Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_1(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub const fn stride_1(
        &self,
    ) -> &'static crate::common::Reg<self::Stride1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stride1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_1(
        &self,
    ) -> &'static crate::common::Reg<self::Debug1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_2(&self) -> &'static crate::common::Reg<self::Cs2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_2(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_2(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(524usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_2(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_2(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(540usize),
            )
        }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub const fn ti_2(&self) -> &'static crate::common::Reg<self::Ti2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[doc = "DMA Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_2(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub const fn stride_2(
        &self,
    ) -> &'static crate::common::Reg<self::Stride2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stride2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_2(
        &self,
    ) -> &'static crate::common::Reg<self::Debug2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_3(&self) -> &'static crate::common::Reg<self::Cs3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(768usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_3(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(772usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_3(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(780usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_3(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(784usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_3(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(796usize),
            )
        }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub const fn ti_3(&self) -> &'static crate::common::Reg<self::Ti3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(776usize),
            )
        }
    }

    #[doc = "DMA Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_3(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(788usize),
            )
        }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub const fn stride_3(
        &self,
    ) -> &'static crate::common::Reg<self::Stride3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stride3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(792usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_3(
        &self,
    ) -> &'static crate::common::Reg<self::Debug3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(800usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_4(&self) -> &'static crate::common::Reg<self::Cs4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_4(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1028usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_4(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1036usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_4(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_4(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1052usize),
            )
        }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub const fn ti_4(&self) -> &'static crate::common::Reg<self::Ti4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1032usize),
            )
        }
    }

    #[doc = "DMA Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_4(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1044usize),
            )
        }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub const fn stride_4(
        &self,
    ) -> &'static crate::common::Reg<self::Stride4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stride4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_4(
        &self,
    ) -> &'static crate::common::Reg<self::Debug4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1056usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_5(&self) -> &'static crate::common::Reg<self::Cs5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_5(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_5(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1292usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_5(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1296usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_5(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1308usize),
            )
        }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub const fn ti_5(&self) -> &'static crate::common::Reg<self::Ti5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[doc = "DMA Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_5(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub const fn stride_5(
        &self,
    ) -> &'static crate::common::Reg<self::Stride5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stride5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1304usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_5(
        &self,
    ) -> &'static crate::common::Reg<self::Debug5_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug5_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1312usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_6(&self) -> &'static crate::common::Reg<self::Cs6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1536usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_6(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1540usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_6(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1548usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_6(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1552usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_6(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1564usize),
            )
        }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub const fn ti_6(&self) -> &'static crate::common::Reg<self::Ti6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1544usize),
            )
        }
    }

    #[doc = "DMA Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_6(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1556usize),
            )
        }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub const fn stride_6(
        &self,
    ) -> &'static crate::common::Reg<self::Stride6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stride6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1560usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_6(
        &self,
    ) -> &'static crate::common::Reg<self::Debug6_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug6_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1568usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_7(&self) -> &'static crate::common::Reg<self::Cs7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1792usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_7(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1796usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_7(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1804usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_7(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1808usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_7(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1820usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Information"]
    #[inline(always)]
    pub const fn ti_7(&self) -> &'static crate::common::Reg<self::Ti7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1800usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_7(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1812usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_7(
        &self,
    ) -> &'static crate::common::Reg<self::Debug7_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug7_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1824usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_8(&self) -> &'static crate::common::Reg<self::Cs8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2048usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_8(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2052usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_8(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2060usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_8(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2064usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_8(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2076usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Information"]
    #[inline(always)]
    pub const fn ti_8(&self) -> &'static crate::common::Reg<self::Ti8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2056usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_8(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2068usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_8(
        &self,
    ) -> &'static crate::common::Reg<self::Debug8_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug8_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2080usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_9(&self) -> &'static crate::common::Reg<self::Cs9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2304usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_9(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2308usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_9(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2316usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_9(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2320usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_9(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2332usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Information"]
    #[inline(always)]
    pub const fn ti_9(&self) -> &'static crate::common::Reg<self::Ti9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2312usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_9(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2324usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_9(
        &self,
    ) -> &'static crate::common::Reg<self::Debug9_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug9_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2336usize),
            )
        }
    }

    #[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
    #[inline(always)]
    pub const fn cs_10(&self) -> &'static crate::common::Reg<self::Cs10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2560usize),
            )
        }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub const fn conblk_ad_10(
        &self,
    ) -> &'static crate::common::Reg<self::ConblkAd10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ConblkAd10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2564usize),
            )
        }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub const fn source_ad_10(
        &self,
    ) -> &'static crate::common::Reg<self::SourceAd10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SourceAd10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2572usize),
            )
        }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub const fn dest_ad_10(
        &self,
    ) -> &'static crate::common::Reg<self::DestAd10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DestAd10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2576usize),
            )
        }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub const fn nextconbk_10(
        &self,
    ) -> &'static crate::common::Reg<self::Nextconbk10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nextconbk10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2588usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Information"]
    #[inline(always)]
    pub const fn ti_10(&self) -> &'static crate::common::Reg<self::Ti10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2568usize),
            )
        }
    }

    #[doc = "DMA Lite Transfer Length"]
    #[inline(always)]
    pub const fn txfr_len_10(
        &self,
    ) -> &'static crate::common::Reg<self::TxfrLen10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TxfrLen10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2580usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_10(
        &self,
    ) -> &'static crate::common::Reg<self::Debug10_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug10_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2592usize),
            )
        }
    }

    #[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
    #[inline(always)]
    pub const fn cs_11(&self) -> &'static crate::common::Reg<self::Cs11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2816usize),
            )
        }
    }

    #[doc = "DMA4 Control Block Address register"]
    #[inline(always)]
    pub const fn cb_11(&self) -> &'static crate::common::Reg<self::Cb11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cb11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2820usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_11(
        &self,
    ) -> &'static crate::common::Reg<self::Debug11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2828usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Information"]
    #[inline(always)]
    pub const fn ti_11(&self) -> &'static crate::common::Reg<self::Ti11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2832usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Source Address"]
    #[inline(always)]
    pub const fn src_11(&self) -> &'static crate::common::Reg<self::Src11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2836usize),
            )
        }
    }

    #[doc = "DMA4 Source Information"]
    #[inline(always)]
    pub const fn srci_11(
        &self,
    ) -> &'static crate::common::Reg<self::Srci11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srci11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2840usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Destination Address"]
    #[inline(always)]
    pub const fn dest_11(
        &self,
    ) -> &'static crate::common::Reg<self::Dest11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dest11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2844usize),
            )
        }
    }

    #[doc = "DMA4 Destination Information"]
    #[inline(always)]
    pub const fn desti_11(
        &self,
    ) -> &'static crate::common::Reg<self::Desti11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Desti11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2848usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Length"]
    #[inline(always)]
    pub const fn len_11(&self) -> &'static crate::common::Reg<self::Len11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Len11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2852usize),
            )
        }
    }

    #[doc = "DMA4 Next Control Block Address"]
    #[inline(always)]
    pub const fn next_cb_11(
        &self,
    ) -> &'static crate::common::Reg<self::NextCb11_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::NextCb11_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2856usize),
            )
        }
    }

    #[doc = "DMA4 Debug2 register"]
    #[inline(always)]
    pub const fn debug2_11(
        &self,
    ) -> &'static crate::common::Reg<self::Debug211_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Debug211_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2860usize),
            )
        }
    }

    #[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
    #[inline(always)]
    pub const fn cs_12(&self) -> &'static crate::common::Reg<self::Cs12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[doc = "DMA4 Control Block Address register"]
    #[inline(always)]
    pub const fn cb_12(&self) -> &'static crate::common::Reg<self::Cb12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cb12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3076usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_12(
        &self,
    ) -> &'static crate::common::Reg<self::Debug12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3084usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Information"]
    #[inline(always)]
    pub const fn ti_12(&self) -> &'static crate::common::Reg<self::Ti12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3088usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Source Address"]
    #[inline(always)]
    pub const fn src_12(&self) -> &'static crate::common::Reg<self::Src12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3092usize),
            )
        }
    }

    #[doc = "DMA4 Source Information"]
    #[inline(always)]
    pub const fn srci_12(
        &self,
    ) -> &'static crate::common::Reg<self::Srci12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srci12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3096usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Destination Address"]
    #[inline(always)]
    pub const fn dest_12(
        &self,
    ) -> &'static crate::common::Reg<self::Dest12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dest12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3100usize),
            )
        }
    }

    #[doc = "DMA4 Destination Information"]
    #[inline(always)]
    pub const fn desti_12(
        &self,
    ) -> &'static crate::common::Reg<self::Desti12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Desti12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3104usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Length"]
    #[inline(always)]
    pub const fn len_12(&self) -> &'static crate::common::Reg<self::Len12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Len12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3108usize),
            )
        }
    }

    #[doc = "DMA4 Next Control Block Address"]
    #[inline(always)]
    pub const fn next_cb_12(
        &self,
    ) -> &'static crate::common::Reg<self::NextCb12_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::NextCb12_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3112usize),
            )
        }
    }

    #[doc = "DMA4 Debug2 register"]
    #[inline(always)]
    pub const fn debug2_12(
        &self,
    ) -> &'static crate::common::Reg<self::Debug212_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Debug212_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3116usize),
            )
        }
    }

    #[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
    #[inline(always)]
    pub const fn cs_13(&self) -> &'static crate::common::Reg<self::Cs13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3328usize),
            )
        }
    }

    #[doc = "DMA4 Control Block Address register"]
    #[inline(always)]
    pub const fn cb_13(&self) -> &'static crate::common::Reg<self::Cb13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cb13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3332usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_13(
        &self,
    ) -> &'static crate::common::Reg<self::Debug13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3340usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Information"]
    #[inline(always)]
    pub const fn ti_13(&self) -> &'static crate::common::Reg<self::Ti13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3344usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Source Address"]
    #[inline(always)]
    pub const fn src_13(&self) -> &'static crate::common::Reg<self::Src13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3348usize),
            )
        }
    }

    #[doc = "DMA4 Source Information"]
    #[inline(always)]
    pub const fn srci_13(
        &self,
    ) -> &'static crate::common::Reg<self::Srci13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srci13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3352usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Destination Address"]
    #[inline(always)]
    pub const fn dest_13(
        &self,
    ) -> &'static crate::common::Reg<self::Dest13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dest13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3356usize),
            )
        }
    }

    #[doc = "DMA4 Destination Information"]
    #[inline(always)]
    pub const fn desti_13(
        &self,
    ) -> &'static crate::common::Reg<self::Desti13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Desti13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3360usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Length"]
    #[inline(always)]
    pub const fn len_13(&self) -> &'static crate::common::Reg<self::Len13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Len13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3364usize),
            )
        }
    }

    #[doc = "DMA4 Next Control Block Address"]
    #[inline(always)]
    pub const fn next_cb_13(
        &self,
    ) -> &'static crate::common::Reg<self::NextCb13_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::NextCb13_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3368usize),
            )
        }
    }

    #[doc = "DMA4 Debug2 register"]
    #[inline(always)]
    pub const fn debug2_13(
        &self,
    ) -> &'static crate::common::Reg<self::Debug213_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Debug213_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3372usize),
            )
        }
    }

    #[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
    #[inline(always)]
    pub const fn cs_14(&self) -> &'static crate::common::Reg<self::Cs14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cs14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3584usize),
            )
        }
    }

    #[doc = "DMA4 Control Block Address register"]
    #[inline(always)]
    pub const fn cb_14(&self) -> &'static crate::common::Reg<self::Cb14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cb14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3588usize),
            )
        }
    }

    #[doc = "DMA4 Debug register"]
    #[inline(always)]
    pub const fn debug_14(
        &self,
    ) -> &'static crate::common::Reg<self::Debug14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Debug14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3596usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Information"]
    #[inline(always)]
    pub const fn ti_14(&self) -> &'static crate::common::Reg<self::Ti14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ti14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3600usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Source Address"]
    #[inline(always)]
    pub const fn src_14(&self) -> &'static crate::common::Reg<self::Src14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Src14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3604usize),
            )
        }
    }

    #[doc = "DMA4 Source Information"]
    #[inline(always)]
    pub const fn srci_14(
        &self,
    ) -> &'static crate::common::Reg<self::Srci14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Srci14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3608usize),
            )
        }
    }

    #[doc = "Lower 32 bits of the DMA4 Destination Address"]
    #[inline(always)]
    pub const fn dest_14(
        &self,
    ) -> &'static crate::common::Reg<self::Dest14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dest14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3612usize),
            )
        }
    }

    #[doc = "DMA4 Destination Information"]
    #[inline(always)]
    pub const fn desti_14(
        &self,
    ) -> &'static crate::common::Reg<self::Desti14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Desti14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3616usize),
            )
        }
    }

    #[doc = "DMA4 Transfer Length"]
    #[inline(always)]
    pub const fn len_14(&self) -> &'static crate::common::Reg<self::Len14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Len14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3620usize),
            )
        }
    }

    #[doc = "DMA4 Next Control Block Address"]
    #[inline(always)]
    pub const fn next_cb_14(
        &self,
    ) -> &'static crate::common::Reg<self::NextCb14_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::NextCb14_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3624usize),
            )
        }
    }

    #[doc = "DMA4 Debug2 register"]
    #[inline(always)]
    pub const fn debug2_14(
        &self,
    ) -> &'static crate::common::Reg<self::Debug214_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Debug214_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(3628usize),
            )
        }
    }

    #[doc = "Interrupt status of each DMA engine"]
    #[inline(always)]
    pub const fn int_status(
        &self,
    ) -> &'static crate::common::Reg<self::IntStatus_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::IntStatus_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4064usize),
            )
        }
    }

    #[doc = "Global enable bits for each channel"]
    #[inline(always)]
    pub const fn enable(
        &self,
    ) -> &'static crate::common::Reg<self::Enable_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Enable_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4080usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs0_SPEC;
impl crate::sealed::RegSpec for Cs0_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs0 = crate::RegValueT<Cs0_SPEC>;

impl Cs0 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs0 {
    #[inline(always)]
    fn default() -> Cs0 {
        <crate::RegValueT<Cs0_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd0_SPEC;
impl crate::sealed::RegSpec for ConblkAd0_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd0 = crate::RegValueT<ConblkAd0_SPEC>;

impl ConblkAd0 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd0 {
    #[inline(always)]
    fn default() -> ConblkAd0 {
        <crate::RegValueT<ConblkAd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd0_SPEC;
impl crate::sealed::RegSpec for SourceAd0_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd0 = crate::RegValueT<SourceAd0_SPEC>;

impl SourceAd0 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd0 {
    #[inline(always)]
    fn default() -> SourceAd0 {
        <crate::RegValueT<SourceAd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd0_SPEC;
impl crate::sealed::RegSpec for DestAd0_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd0 = crate::RegValueT<DestAd0_SPEC>;

impl DestAd0 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd0 {
    #[inline(always)]
    fn default() -> DestAd0 {
        <crate::RegValueT<DestAd0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk0_SPEC;
impl crate::sealed::RegSpec for Nextconbk0_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk0 = crate::RegValueT<Nextconbk0_SPEC>;

impl Nextconbk0 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk0 {
    #[inline(always)]
    fn default() -> Nextconbk0 {
        <crate::RegValueT<Nextconbk0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti0_SPEC;
impl crate::sealed::RegSpec for Ti0_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Information"]
pub type Ti0 = crate::RegValueT<Ti0_SPEC>;

impl Ti0 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dont do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ti0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ti0 {
    #[inline(always)]
    fn default() -> Ti0 {
        <crate::RegValueT<Ti0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen0_SPEC;
impl crate::sealed::RegSpec for TxfrLen0_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Length"]
pub type TxfrLen0 = crate::RegValueT<TxfrLen0_SPEC>;

impl TxfrLen0 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                                transfers are performed, when in normal linear mode this becomes the top bits of the\n                                XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen0 {
    #[inline(always)]
    fn default() -> TxfrLen0 {
        <crate::RegValueT<TxfrLen0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride0_SPEC;
impl crate::sealed::RegSpec for Stride0_SPEC {
    type DataType = u32;
}

#[doc = "DMA 2D Stride"]
pub type Stride0 = crate::RegValueT<Stride0_SPEC>;

impl Stride0 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stride0 {
    #[inline(always)]
    fn default() -> Stride0 {
        <crate::RegValueT<Stride0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0_SPEC;
impl crate::sealed::RegSpec for Debug0_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug0 = crate::RegValueT<Debug0_SPEC>;

impl Debug0 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug0_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug0_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug0 {
    #[inline(always)]
    fn default() -> Debug0 {
        <crate::RegValueT<Debug0_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs1_SPEC;
impl crate::sealed::RegSpec for Cs1_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs1 = crate::RegValueT<Cs1_SPEC>;

impl Cs1 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs1 {
    #[inline(always)]
    fn default() -> Cs1 {
        <crate::RegValueT<Cs1_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd1_SPEC;
impl crate::sealed::RegSpec for ConblkAd1_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd1 = crate::RegValueT<ConblkAd1_SPEC>;

impl ConblkAd1 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd1 {
    #[inline(always)]
    fn default() -> ConblkAd1 {
        <crate::RegValueT<ConblkAd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd1_SPEC;
impl crate::sealed::RegSpec for SourceAd1_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd1 = crate::RegValueT<SourceAd1_SPEC>;

impl SourceAd1 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd1 {
    #[inline(always)]
    fn default() -> SourceAd1 {
        <crate::RegValueT<SourceAd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd1_SPEC;
impl crate::sealed::RegSpec for DestAd1_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd1 = crate::RegValueT<DestAd1_SPEC>;

impl DestAd1 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd1 {
    #[inline(always)]
    fn default() -> DestAd1 {
        <crate::RegValueT<DestAd1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk1_SPEC;
impl crate::sealed::RegSpec for Nextconbk1_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk1 = crate::RegValueT<Nextconbk1_SPEC>;

impl Nextconbk1 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk1 {
    #[inline(always)]
    fn default() -> Nextconbk1 {
        <crate::RegValueT<Nextconbk1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti1_SPEC;
impl crate::sealed::RegSpec for Ti1_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Information"]
pub type Ti1 = crate::RegValueT<Ti1_SPEC>;

impl Ti1 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dont do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ti1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ti1 {
    #[inline(always)]
    fn default() -> Ti1 {
        <crate::RegValueT<Ti1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen1_SPEC;
impl crate::sealed::RegSpec for TxfrLen1_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Length"]
pub type TxfrLen1 = crate::RegValueT<TxfrLen1_SPEC>;

impl TxfrLen1 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                                transfers are performed, when in normal linear mode this becomes the top bits of the\n                                XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen1 {
    #[inline(always)]
    fn default() -> TxfrLen1 {
        <crate::RegValueT<TxfrLen1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride1_SPEC;
impl crate::sealed::RegSpec for Stride1_SPEC {
    type DataType = u32;
}

#[doc = "DMA 2D Stride"]
pub type Stride1 = crate::RegValueT<Stride1_SPEC>;

impl Stride1 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stride1 {
    #[inline(always)]
    fn default() -> Stride1 {
        <crate::RegValueT<Stride1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1_SPEC;
impl crate::sealed::RegSpec for Debug1_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug1 = crate::RegValueT<Debug1_SPEC>;

impl Debug1 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug1_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug1_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug1 {
    #[inline(always)]
    fn default() -> Debug1 {
        <crate::RegValueT<Debug1_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs2_SPEC;
impl crate::sealed::RegSpec for Cs2_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs2 = crate::RegValueT<Cs2_SPEC>;

impl Cs2 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs2 {
    #[inline(always)]
    fn default() -> Cs2 {
        <crate::RegValueT<Cs2_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd2_SPEC;
impl crate::sealed::RegSpec for ConblkAd2_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd2 = crate::RegValueT<ConblkAd2_SPEC>;

impl ConblkAd2 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd2 {
    #[inline(always)]
    fn default() -> ConblkAd2 {
        <crate::RegValueT<ConblkAd2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd2_SPEC;
impl crate::sealed::RegSpec for SourceAd2_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd2 = crate::RegValueT<SourceAd2_SPEC>;

impl SourceAd2 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd2 {
    #[inline(always)]
    fn default() -> SourceAd2 {
        <crate::RegValueT<SourceAd2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd2_SPEC;
impl crate::sealed::RegSpec for DestAd2_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd2 = crate::RegValueT<DestAd2_SPEC>;

impl DestAd2 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd2 {
    #[inline(always)]
    fn default() -> DestAd2 {
        <crate::RegValueT<DestAd2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk2_SPEC;
impl crate::sealed::RegSpec for Nextconbk2_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk2 = crate::RegValueT<Nextconbk2_SPEC>;

impl Nextconbk2 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk2 {
    #[inline(always)]
    fn default() -> Nextconbk2 {
        <crate::RegValueT<Nextconbk2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti2_SPEC;
impl crate::sealed::RegSpec for Ti2_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Information"]
pub type Ti2 = crate::RegValueT<Ti2_SPEC>;

impl Ti2 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dont do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ti2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ti2 {
    #[inline(always)]
    fn default() -> Ti2 {
        <crate::RegValueT<Ti2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen2_SPEC;
impl crate::sealed::RegSpec for TxfrLen2_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Length"]
pub type TxfrLen2 = crate::RegValueT<TxfrLen2_SPEC>;

impl TxfrLen2 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                                transfers are performed, when in normal linear mode this becomes the top bits of the\n                                XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen2 {
    #[inline(always)]
    fn default() -> TxfrLen2 {
        <crate::RegValueT<TxfrLen2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride2_SPEC;
impl crate::sealed::RegSpec for Stride2_SPEC {
    type DataType = u32;
}

#[doc = "DMA 2D Stride"]
pub type Stride2 = crate::RegValueT<Stride2_SPEC>;

impl Stride2 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stride2 {
    #[inline(always)]
    fn default() -> Stride2 {
        <crate::RegValueT<Stride2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug2_SPEC;
impl crate::sealed::RegSpec for Debug2_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug2 = crate::RegValueT<Debug2_SPEC>;

impl Debug2 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug2_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug2_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug2 {
    #[inline(always)]
    fn default() -> Debug2 {
        <crate::RegValueT<Debug2_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs3_SPEC;
impl crate::sealed::RegSpec for Cs3_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs3 = crate::RegValueT<Cs3_SPEC>;

impl Cs3 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs3 {
    #[inline(always)]
    fn default() -> Cs3 {
        <crate::RegValueT<Cs3_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd3_SPEC;
impl crate::sealed::RegSpec for ConblkAd3_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd3 = crate::RegValueT<ConblkAd3_SPEC>;

impl ConblkAd3 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd3 {
    #[inline(always)]
    fn default() -> ConblkAd3 {
        <crate::RegValueT<ConblkAd3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd3_SPEC;
impl crate::sealed::RegSpec for SourceAd3_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd3 = crate::RegValueT<SourceAd3_SPEC>;

impl SourceAd3 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd3 {
    #[inline(always)]
    fn default() -> SourceAd3 {
        <crate::RegValueT<SourceAd3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd3_SPEC;
impl crate::sealed::RegSpec for DestAd3_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd3 = crate::RegValueT<DestAd3_SPEC>;

impl DestAd3 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd3 {
    #[inline(always)]
    fn default() -> DestAd3 {
        <crate::RegValueT<DestAd3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk3_SPEC;
impl crate::sealed::RegSpec for Nextconbk3_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk3 = crate::RegValueT<Nextconbk3_SPEC>;

impl Nextconbk3 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk3 {
    #[inline(always)]
    fn default() -> Nextconbk3 {
        <crate::RegValueT<Nextconbk3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti3_SPEC;
impl crate::sealed::RegSpec for Ti3_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Information"]
pub type Ti3 = crate::RegValueT<Ti3_SPEC>;

impl Ti3 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dont do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ti3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ti3 {
    #[inline(always)]
    fn default() -> Ti3 {
        <crate::RegValueT<Ti3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen3_SPEC;
impl crate::sealed::RegSpec for TxfrLen3_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Length"]
pub type TxfrLen3 = crate::RegValueT<TxfrLen3_SPEC>;

impl TxfrLen3 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                                transfers are performed, when in normal linear mode this becomes the top bits of the\n                                XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen3 {
    #[inline(always)]
    fn default() -> TxfrLen3 {
        <crate::RegValueT<TxfrLen3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride3_SPEC;
impl crate::sealed::RegSpec for Stride3_SPEC {
    type DataType = u32;
}

#[doc = "DMA 2D Stride"]
pub type Stride3 = crate::RegValueT<Stride3_SPEC>;

impl Stride3 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stride3 {
    #[inline(always)]
    fn default() -> Stride3 {
        <crate::RegValueT<Stride3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug3_SPEC;
impl crate::sealed::RegSpec for Debug3_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug3 = crate::RegValueT<Debug3_SPEC>;

impl Debug3 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug3_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug3_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug3_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug3_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug3 {
    #[inline(always)]
    fn default() -> Debug3 {
        <crate::RegValueT<Debug3_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs4_SPEC;
impl crate::sealed::RegSpec for Cs4_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs4 = crate::RegValueT<Cs4_SPEC>;

impl Cs4 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs4 {
    #[inline(always)]
    fn default() -> Cs4 {
        <crate::RegValueT<Cs4_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd4_SPEC;
impl crate::sealed::RegSpec for ConblkAd4_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd4 = crate::RegValueT<ConblkAd4_SPEC>;

impl ConblkAd4 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd4 {
    #[inline(always)]
    fn default() -> ConblkAd4 {
        <crate::RegValueT<ConblkAd4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd4_SPEC;
impl crate::sealed::RegSpec for SourceAd4_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd4 = crate::RegValueT<SourceAd4_SPEC>;

impl SourceAd4 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd4 {
    #[inline(always)]
    fn default() -> SourceAd4 {
        <crate::RegValueT<SourceAd4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd4_SPEC;
impl crate::sealed::RegSpec for DestAd4_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd4 = crate::RegValueT<DestAd4_SPEC>;

impl DestAd4 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd4 {
    #[inline(always)]
    fn default() -> DestAd4 {
        <crate::RegValueT<DestAd4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk4_SPEC;
impl crate::sealed::RegSpec for Nextconbk4_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk4 = crate::RegValueT<Nextconbk4_SPEC>;

impl Nextconbk4 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk4 {
    #[inline(always)]
    fn default() -> Nextconbk4 {
        <crate::RegValueT<Nextconbk4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti4_SPEC;
impl crate::sealed::RegSpec for Ti4_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Information"]
pub type Ti4 = crate::RegValueT<Ti4_SPEC>;

impl Ti4 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dont do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ti4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ti4 {
    #[inline(always)]
    fn default() -> Ti4 {
        <crate::RegValueT<Ti4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen4_SPEC;
impl crate::sealed::RegSpec for TxfrLen4_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Length"]
pub type TxfrLen4 = crate::RegValueT<TxfrLen4_SPEC>;

impl TxfrLen4 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                                transfers are performed, when in normal linear mode this becomes the top bits of the\n                                XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen4 {
    #[inline(always)]
    fn default() -> TxfrLen4 {
        <crate::RegValueT<TxfrLen4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride4_SPEC;
impl crate::sealed::RegSpec for Stride4_SPEC {
    type DataType = u32;
}

#[doc = "DMA 2D Stride"]
pub type Stride4 = crate::RegValueT<Stride4_SPEC>;

impl Stride4 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stride4 {
    #[inline(always)]
    fn default() -> Stride4 {
        <crate::RegValueT<Stride4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug4_SPEC;
impl crate::sealed::RegSpec for Debug4_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug4 = crate::RegValueT<Debug4_SPEC>;

impl Debug4 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug4_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug4_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug4_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug4_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug4_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug4 {
    #[inline(always)]
    fn default() -> Debug4 {
        <crate::RegValueT<Debug4_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs5_SPEC;
impl crate::sealed::RegSpec for Cs5_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs5 = crate::RegValueT<Cs5_SPEC>;

impl Cs5 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs5 {
    #[inline(always)]
    fn default() -> Cs5 {
        <crate::RegValueT<Cs5_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd5_SPEC;
impl crate::sealed::RegSpec for ConblkAd5_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd5 = crate::RegValueT<ConblkAd5_SPEC>;

impl ConblkAd5 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd5 {
    #[inline(always)]
    fn default() -> ConblkAd5 {
        <crate::RegValueT<ConblkAd5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd5_SPEC;
impl crate::sealed::RegSpec for SourceAd5_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd5 = crate::RegValueT<SourceAd5_SPEC>;

impl SourceAd5 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd5 {
    #[inline(always)]
    fn default() -> SourceAd5 {
        <crate::RegValueT<SourceAd5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd5_SPEC;
impl crate::sealed::RegSpec for DestAd5_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd5 = crate::RegValueT<DestAd5_SPEC>;

impl DestAd5 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd5 {
    #[inline(always)]
    fn default() -> DestAd5 {
        <crate::RegValueT<DestAd5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk5_SPEC;
impl crate::sealed::RegSpec for Nextconbk5_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk5 = crate::RegValueT<Nextconbk5_SPEC>;

impl Nextconbk5 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk5 {
    #[inline(always)]
    fn default() -> Nextconbk5 {
        <crate::RegValueT<Nextconbk5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti5_SPEC;
impl crate::sealed::RegSpec for Ti5_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Information"]
pub type Ti5 = crate::RegValueT<Ti5_SPEC>;

impl Ti5 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dont do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ti5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ti5 {
    #[inline(always)]
    fn default() -> Ti5 {
        <crate::RegValueT<Ti5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen5_SPEC;
impl crate::sealed::RegSpec for TxfrLen5_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Length"]
pub type TxfrLen5 = crate::RegValueT<TxfrLen5_SPEC>;

impl TxfrLen5 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                                transfers are performed, when in normal linear mode this becomes the top bits of the\n                                XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen5 {
    #[inline(always)]
    fn default() -> TxfrLen5 {
        <crate::RegValueT<TxfrLen5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride5_SPEC;
impl crate::sealed::RegSpec for Stride5_SPEC {
    type DataType = u32;
}

#[doc = "DMA 2D Stride"]
pub type Stride5 = crate::RegValueT<Stride5_SPEC>;

impl Stride5 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride5_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stride5 {
    #[inline(always)]
    fn default() -> Stride5 {
        <crate::RegValueT<Stride5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug5_SPEC;
impl crate::sealed::RegSpec for Debug5_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug5 = crate::RegValueT<Debug5_SPEC>;

impl Debug5 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug5_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug5_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug5_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug5_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug5_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug5 {
    #[inline(always)]
    fn default() -> Debug5 {
        <crate::RegValueT<Debug5_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs6_SPEC;
impl crate::sealed::RegSpec for Cs6_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs6 = crate::RegValueT<Cs6_SPEC>;

impl Cs6 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs6_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs6_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs6_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs6_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs6_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs6 {
    #[inline(always)]
    fn default() -> Cs6 {
        <crate::RegValueT<Cs6_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd6_SPEC;
impl crate::sealed::RegSpec for ConblkAd6_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd6 = crate::RegValueT<ConblkAd6_SPEC>;

impl ConblkAd6 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd6 {
    #[inline(always)]
    fn default() -> ConblkAd6 {
        <crate::RegValueT<ConblkAd6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd6_SPEC;
impl crate::sealed::RegSpec for SourceAd6_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd6 = crate::RegValueT<SourceAd6_SPEC>;

impl SourceAd6 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd6 {
    #[inline(always)]
    fn default() -> SourceAd6 {
        <crate::RegValueT<SourceAd6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd6_SPEC;
impl crate::sealed::RegSpec for DestAd6_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd6 = crate::RegValueT<DestAd6_SPEC>;

impl DestAd6 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd6 {
    #[inline(always)]
    fn default() -> DestAd6 {
        <crate::RegValueT<DestAd6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk6_SPEC;
impl crate::sealed::RegSpec for Nextconbk6_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk6 = crate::RegValueT<Nextconbk6_SPEC>;

impl Nextconbk6 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk6 {
    #[inline(always)]
    fn default() -> Nextconbk6 {
        <crate::RegValueT<Nextconbk6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti6_SPEC;
impl crate::sealed::RegSpec for Ti6_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Information"]
pub type Ti6 = crate::RegValueT<Ti6_SPEC>;

impl Ti6 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Dont do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ti6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ti6 {
    #[inline(always)]
    fn default() -> Ti6 {
        <crate::RegValueT<Ti6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen6_SPEC;
impl crate::sealed::RegSpec for TxfrLen6_SPEC {
    type DataType = u32;
}

#[doc = "DMA Transfer Length"]
pub type TxfrLen6 = crate::RegValueT<TxfrLen6_SPEC>;

impl TxfrLen6 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                                transfers are performed, when in normal linear mode this becomes the top bits of the\n                                XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen6 {
    #[inline(always)]
    fn default() -> TxfrLen6 {
        <crate::RegValueT<TxfrLen6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride6_SPEC;
impl crate::sealed::RegSpec for Stride6_SPEC {
    type DataType = u32;
}

#[doc = "DMA 2D Stride"]
pub type Stride6 = crate::RegValueT<Stride6_SPEC>;

impl Stride6 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride6_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Stride6 {
    #[inline(always)]
    fn default() -> Stride6 {
        <crate::RegValueT<Stride6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug6_SPEC;
impl crate::sealed::RegSpec for Debug6_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug6 = crate::RegValueT<Debug6_SPEC>;

impl Debug6 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug6_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug6_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug6_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug6_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug6_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug6_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug6 {
    #[inline(always)]
    fn default() -> Debug6 {
        <crate::RegValueT<Debug6_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs7_SPEC;
impl crate::sealed::RegSpec for Cs7_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs7 = crate::RegValueT<Cs7_SPEC>;

impl Cs7 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs7_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs7_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs7_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs7_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs7_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs7 {
    #[inline(always)]
    fn default() -> Cs7 {
        <crate::RegValueT<Cs7_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd7_SPEC;
impl crate::sealed::RegSpec for ConblkAd7_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd7 = crate::RegValueT<ConblkAd7_SPEC>;

impl ConblkAd7 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd7 {
    #[inline(always)]
    fn default() -> ConblkAd7 {
        <crate::RegValueT<ConblkAd7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd7_SPEC;
impl crate::sealed::RegSpec for SourceAd7_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd7 = crate::RegValueT<SourceAd7_SPEC>;

impl SourceAd7 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd7 {
    #[inline(always)]
    fn default() -> SourceAd7 {
        <crate::RegValueT<SourceAd7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd7_SPEC;
impl crate::sealed::RegSpec for DestAd7_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd7 = crate::RegValueT<DestAd7_SPEC>;

impl DestAd7 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd7 {
    #[inline(always)]
    fn default() -> DestAd7 {
        <crate::RegValueT<DestAd7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk7_SPEC;
impl crate::sealed::RegSpec for Nextconbk7_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk7 = crate::RegValueT<Nextconbk7_SPEC>;

impl Nextconbk7 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk7 {
    #[inline(always)]
    fn default() -> Nextconbk7 {
        <crate::RegValueT<Nextconbk7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti7_SPEC;
impl crate::sealed::RegSpec for Ti7_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Information"]
pub type Ti7 = crate::RegValueT<Ti7_SPEC>;

impl Ti7 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Ti7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti7_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti7 {
    #[inline(always)]
    fn default() -> Ti7 {
        <crate::RegValueT<Ti7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen7_SPEC;
impl crate::sealed::RegSpec for TxfrLen7_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Length"]
pub type TxfrLen7 = crate::RegValueT<TxfrLen7_SPEC>;

impl TxfrLen7 {
    #[doc = "Transfer Length"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen7 {
    #[inline(always)]
    fn default() -> TxfrLen7 {
        <crate::RegValueT<TxfrLen7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug7_SPEC;
impl crate::sealed::RegSpec for Debug7_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug7 = crate::RegValueT<Debug7_SPEC>;

impl Debug7 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug7_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug7_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug7_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug7_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug7_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug7_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug7_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug7_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug7 {
    #[inline(always)]
    fn default() -> Debug7 {
        <crate::RegValueT<Debug7_SPEC> as RegisterValue<_>>::new(335544320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs8_SPEC;
impl crate::sealed::RegSpec for Cs8_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs8 = crate::RegValueT<Cs8_SPEC>;

impl Cs8 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs8_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs8_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs8_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs8_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs8_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs8 {
    #[inline(always)]
    fn default() -> Cs8 {
        <crate::RegValueT<Cs8_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd8_SPEC;
impl crate::sealed::RegSpec for ConblkAd8_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd8 = crate::RegValueT<ConblkAd8_SPEC>;

impl ConblkAd8 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd8 {
    #[inline(always)]
    fn default() -> ConblkAd8 {
        <crate::RegValueT<ConblkAd8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd8_SPEC;
impl crate::sealed::RegSpec for SourceAd8_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd8 = crate::RegValueT<SourceAd8_SPEC>;

impl SourceAd8 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd8 {
    #[inline(always)]
    fn default() -> SourceAd8 {
        <crate::RegValueT<SourceAd8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd8_SPEC;
impl crate::sealed::RegSpec for DestAd8_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd8 = crate::RegValueT<DestAd8_SPEC>;

impl DestAd8 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd8 {
    #[inline(always)]
    fn default() -> DestAd8 {
        <crate::RegValueT<DestAd8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk8_SPEC;
impl crate::sealed::RegSpec for Nextconbk8_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk8 = crate::RegValueT<Nextconbk8_SPEC>;

impl Nextconbk8 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk8 {
    #[inline(always)]
    fn default() -> Nextconbk8 {
        <crate::RegValueT<Nextconbk8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti8_SPEC;
impl crate::sealed::RegSpec for Ti8_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Information"]
pub type Ti8 = crate::RegValueT<Ti8_SPEC>;

impl Ti8 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Ti8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti8_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti8 {
    #[inline(always)]
    fn default() -> Ti8 {
        <crate::RegValueT<Ti8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen8_SPEC;
impl crate::sealed::RegSpec for TxfrLen8_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Length"]
pub type TxfrLen8 = crate::RegValueT<TxfrLen8_SPEC>;

impl TxfrLen8 {
    #[doc = "Transfer Length"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen8_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen8 {
    #[inline(always)]
    fn default() -> TxfrLen8 {
        <crate::RegValueT<TxfrLen8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug8_SPEC;
impl crate::sealed::RegSpec for Debug8_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug8 = crate::RegValueT<Debug8_SPEC>;

impl Debug8 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug8_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug8_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug8_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug8_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug8_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug8_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug8_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug8 {
    #[inline(always)]
    fn default() -> Debug8 {
        <crate::RegValueT<Debug8_SPEC> as RegisterValue<_>>::new(335544320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs9_SPEC;
impl crate::sealed::RegSpec for Cs9_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs9 = crate::RegValueT<Cs9_SPEC>;

impl Cs9 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs9_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs9_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs9_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs9_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs9_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs9 {
    #[inline(always)]
    fn default() -> Cs9 {
        <crate::RegValueT<Cs9_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd9_SPEC;
impl crate::sealed::RegSpec for ConblkAd9_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd9 = crate::RegValueT<ConblkAd9_SPEC>;

impl ConblkAd9 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd9 {
    #[inline(always)]
    fn default() -> ConblkAd9 {
        <crate::RegValueT<ConblkAd9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd9_SPEC;
impl crate::sealed::RegSpec for SourceAd9_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd9 = crate::RegValueT<SourceAd9_SPEC>;

impl SourceAd9 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd9 {
    #[inline(always)]
    fn default() -> SourceAd9 {
        <crate::RegValueT<SourceAd9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd9_SPEC;
impl crate::sealed::RegSpec for DestAd9_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd9 = crate::RegValueT<DestAd9_SPEC>;

impl DestAd9 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd9_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DestAd9 {
    #[inline(always)]
    fn default() -> DestAd9 {
        <crate::RegValueT<DestAd9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk9_SPEC;
impl crate::sealed::RegSpec for Nextconbk9_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk9 = crate::RegValueT<Nextconbk9_SPEC>;

impl Nextconbk9 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk9_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk9_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk9 {
    #[inline(always)]
    fn default() -> Nextconbk9 {
        <crate::RegValueT<Nextconbk9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti9_SPEC;
impl crate::sealed::RegSpec for Ti9_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Information"]
pub type Ti9 = crate::RegValueT<Ti9_SPEC>;

impl Ti9 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Ti9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti9_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti9 {
    #[inline(always)]
    fn default() -> Ti9 {
        <crate::RegValueT<Ti9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen9_SPEC;
impl crate::sealed::RegSpec for TxfrLen9_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Length"]
pub type TxfrLen9 = crate::RegValueT<TxfrLen9_SPEC>;

impl TxfrLen9 {
    #[doc = "Transfer Length"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen9_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen9 {
    #[inline(always)]
    fn default() -> TxfrLen9 {
        <crate::RegValueT<TxfrLen9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug9_SPEC;
impl crate::sealed::RegSpec for Debug9_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug9 = crate::RegValueT<Debug9_SPEC>;

impl Debug9 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug9_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug9_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug9_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug9_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug9_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug9_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug9_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug9_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug9 {
    #[inline(always)]
    fn default() -> Debug9 {
        <crate::RegValueT<Debug9_SPEC> as RegisterValue<_>>::new(335544320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs10_SPEC;
impl crate::sealed::RegSpec for Cs10_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control and Status register contains the main control and status bits for this DMA\n                    channel"]
pub type Cs10 = crate::RegValueT<Cs10_SPEC>;

impl Cs10 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs10_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs10_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs10_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs10_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs10_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs10 {
    #[inline(always)]
    fn default() -> Cs10 {
        <crate::RegValueT<Cs10_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd10_SPEC;
impl crate::sealed::RegSpec for ConblkAd10_SPEC {
    type DataType = u32;
}

#[doc = "DMA Control Block Address register"]
pub type ConblkAd10 = crate::RegValueT<ConblkAd10_SPEC>;

impl ConblkAd10 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ConblkAd10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ConblkAd10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ConblkAd10 {
    #[inline(always)]
    fn default() -> ConblkAd10 {
        <crate::RegValueT<ConblkAd10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd10_SPEC;
impl crate::sealed::RegSpec for SourceAd10_SPEC {
    type DataType = u32;
}

#[doc = "DMA Source Address"]
pub type SourceAd10 = crate::RegValueT<SourceAd10_SPEC>;

impl SourceAd10 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        SourceAd10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            SourceAd10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SourceAd10 {
    #[inline(always)]
    fn default() -> SourceAd10 {
        <crate::RegValueT<SourceAd10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd10_SPEC;
impl crate::sealed::RegSpec for DestAd10_SPEC {
    type DataType = u32;
}

#[doc = "DMA Destination Address"]
pub type DestAd10 = crate::RegValueT<DestAd10_SPEC>;

impl DestAd10 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DestAd10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DestAd10 {
    #[inline(always)]
    fn default() -> DestAd10 {
        <crate::RegValueT<DestAd10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk10_SPEC;
impl crate::sealed::RegSpec for Nextconbk10_SPEC {
    type DataType = u32;
}

#[doc = "DMA Next Control Block Address"]
pub type Nextconbk10 = crate::RegValueT<Nextconbk10_SPEC>;

impl Nextconbk10 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        Nextconbk10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            Nextconbk10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nextconbk10 {
    #[inline(always)]
    fn default() -> Nextconbk10 {
        <crate::RegValueT<Nextconbk10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti10_SPEC;
impl crate::sealed::RegSpec for Ti10_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Information"]
pub type Ti10 = crate::RegValueT<Ti10_SPEC>;

impl Ti10 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Ti10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Ti10_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8,u8,Ti10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti10 {
    #[inline(always)]
    fn default() -> Ti10 {
        <crate::RegValueT<Ti10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen10_SPEC;
impl crate::sealed::RegSpec for TxfrLen10_SPEC {
    type DataType = u32;
}

#[doc = "DMA Lite Transfer Length"]
pub type TxfrLen10 = crate::RegValueT<TxfrLen10_SPEC>;

impl TxfrLen10 {
    #[doc = "Transfer Length"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TxfrLen10 {
    #[inline(always)]
    fn default() -> TxfrLen10 {
        <crate::RegValueT<TxfrLen10_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug10_SPEC;
impl crate::sealed::RegSpec for Debug10_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug10 = crate::RegValueT<Debug10_SPEC>;

impl Debug10 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug10_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,Debug10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug10_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Debug10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug10_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug10_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x7,1,0,u8,u8,Debug10_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Debug10_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug10_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Debug10 {
    #[inline(always)]
    fn default() -> Debug10 {
        <crate::RegValueT<Debug10_SPEC> as RegisterValue<_>>::new(335544320)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs11_SPEC;
impl crate::sealed::RegSpec for Cs11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
pub type Cs11 = crate::RegValueT<Cs11_SPEC>;

impl Cs11 {
    #[doc = "Activate the DMA4"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Read Paused State"]
    #[inline(always)]
    pub fn rd_paused(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Write Paused State"]
    #[inline(always)]
    pub fn wr_paused(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA4 is Waiting for all the Write Response to be returned"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI QOS Level"]
    #[inline(always)]
    pub fn qos(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic QOS Level"]
    #[inline(always)]
    pub fn panic_qos(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the DMA4 is BUSY"]
    #[inline(always)]
    pub fn dma_busy(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that there are outstanding AXI transfers"]
    #[inline(always)]
    pub fn outstanding_transactions(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Cs11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Cs11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA4"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing a 1 to this bit will cleanly halt the current DMA4 transfer"]
    #[inline(always)]
    pub fn halt(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs11 {
    #[inline(always)]
    fn default() -> Cs11 {
        <crate::RegValueT<Cs11_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cb11_SPEC;
impl crate::sealed::RegSpec for Cb11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control Block Address register"]
pub type Cb11 = crate::RegValueT<Cb11_SPEC>;

impl Cb11 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cb11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cb11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cb11 {
    #[inline(always)]
    fn default() -> Cb11 {
        <crate::RegValueT<Cb11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug11_SPEC;
impl crate::sealed::RegSpec for Debug11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug11 = crate::RegValueT<Debug11_SPEC>;

impl Debug11 {
    #[doc = "Slave Write Response Error"]
    #[inline(always)]
    pub fn write_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error During Control Block Read"]
    #[inline(always)]
    pub fn read_cb_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Debug11_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Generate an interrupt if an error is detected"]
    #[inline(always)]
    pub fn int_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Debug11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Debug11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to HALT if it detects an error"]
    #[inline(always)]
    pub fn halt_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Debug11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Debug11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to ABORT if it detects an error"]
    #[inline(always)]
    pub fn abort_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Debug11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Debug11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable the clock gating logic"]
    #[inline(always)]
    pub fn disable_clk_gate(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Debug11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Debug11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read State Machine State"]
    #[inline(always)]
    pub fn r_state(
        self,
    ) -> crate::common::RegisterField<14, 0xf, 1, 0, u8, u8, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0xf,1,0,u8,u8,Debug11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Write State Machine State"]
    #[inline(always)]
    pub fn w_state(
        self,
    ) -> crate::common::RegisterField<18, 0xf, 1, 0, u8, u8, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0xf,1,0,u8,u8,Debug11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Reset"]
    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Debug11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Debug11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ID"]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Debug11_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Debug11_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Debug11_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug11 {
    #[inline(always)]
    fn default() -> Debug11 {
        <crate::RegValueT<Debug11_SPEC> as RegisterValue<_>>::new(268436480)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti11_SPEC;
impl crate::sealed::RegSpec for Ti11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Information"]
pub type Ti11 = crate::RegValueT<Ti11_SPEC>;

impl Ti11 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Perform a 2D transfer instead of a normal linear transfer"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Read Response"]
    #[inline(always)]
    pub fn wait_rd_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<9, 0x1f, 1, 0, u8, u8, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1f,1,0,u8,u8,Ti11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn s_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn d_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read Wait Cycles"]
    #[inline(always)]
    pub fn s_waits(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ti11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Wait Cycles"]
    #[inline(always)]
    pub fn d_waits(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ti11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ti11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti11 {
    #[inline(always)]
    fn default() -> Ti11 {
        <crate::RegValueT<Ti11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src11_SPEC;
impl crate::sealed::RegSpec for Src11_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Source Address"]
pub type Src11 = crate::RegValueT<Src11_SPEC>;

impl Src11 {
    #[doc = "Lower bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Src11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Src11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Src11 {
    #[inline(always)]
    fn default() -> Src11 {
        <crate::RegValueT<Src11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srci11_SPEC;
impl crate::sealed::RegSpec for Srci11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Source Information"]
pub type Srci11 = crate::RegValueT<Srci11_SPEC>;

impl Srci11 {
    #[doc = "High Bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Srci11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Srci11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Srci11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Srci11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Source Address"]
    #[inline(always)]
    pub fn inc(self) -> crate::common::RegisterFieldBool<12, 1, 0, Srci11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Srci11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Srci11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Srci11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Srci11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Srci11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Srci11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Srci11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srci11 {
    #[inline(always)]
    fn default() -> Srci11 {
        <crate::RegValueT<Srci11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dest11_SPEC;
impl crate::sealed::RegSpec for Dest11_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Destination Address"]
pub type Dest11 = crate::RegValueT<Dest11_SPEC>;

impl Dest11 {
    #[doc = "Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dest11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dest11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dest11 {
    #[inline(always)]
    fn default() -> Dest11 {
        <crate::RegValueT<Dest11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Desti11_SPEC;
impl crate::sealed::RegSpec for Desti11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Destination Information"]
pub type Desti11 = crate::RegValueT<Desti11_SPEC>;

impl Desti11 {
    #[doc = "High Bits of the Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Desti11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Desti11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Desti11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Desti11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Destination Address"]
    #[inline(always)]
    pub fn inc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Desti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Desti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Desti11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Desti11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Desti11_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Desti11_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Desti11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Desti11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Desti11 {
    #[inline(always)]
    fn default() -> Desti11 {
        <crate::RegValueT<Desti11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Len11_SPEC;
impl crate::sealed::RegSpec for Len11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Length"]
pub type Len11 = crate::RegValueT<Len11_SPEC>;

impl Len11 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Len11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Len11_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers are performed, when in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, Len11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,Len11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Len11 {
    #[inline(always)]
    fn default() -> Len11 {
        <crate::RegValueT<Len11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NextCb11_SPEC;
impl crate::sealed::RegSpec for NextCb11_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Next Control Block Address"]
pub type NextCb11 = crate::RegValueT<NextCb11_SPEC>;

impl NextCb11 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, NextCb11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            NextCb11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for NextCb11 {
    #[inline(always)]
    fn default() -> NextCb11 {
        <crate::RegValueT<NextCb11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug211_SPEC;
impl crate::sealed::RegSpec for Debug211_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug2 register"]
pub type Debug211 = crate::RegValueT<Debug211_SPEC>;

impl Debug211 {
    #[doc = "Outstanding Write Response Count"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Debug211_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Debug211_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Outstanding read Words Count"]
    #[inline(always)]
    pub fn outstanding_reads(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug211_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug211_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug211 {
    #[inline(always)]
    fn default() -> Debug211 {
        <crate::RegValueT<Debug211_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs12_SPEC;
impl crate::sealed::RegSpec for Cs12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
pub type Cs12 = crate::RegValueT<Cs12_SPEC>;

impl Cs12 {
    #[doc = "Activate the DMA4"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Read Paused State"]
    #[inline(always)]
    pub fn rd_paused(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Write Paused State"]
    #[inline(always)]
    pub fn wr_paused(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA4 is Waiting for all the Write Response to be returned"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI QOS Level"]
    #[inline(always)]
    pub fn qos(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic QOS Level"]
    #[inline(always)]
    pub fn panic_qos(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the DMA4 is BUSY"]
    #[inline(always)]
    pub fn dma_busy(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that there are outstanding AXI transfers"]
    #[inline(always)]
    pub fn outstanding_transactions(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Cs12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Cs12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA4"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing a 1 to this bit will cleanly halt the current DMA4 transfer"]
    #[inline(always)]
    pub fn halt(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs12 {
    #[inline(always)]
    fn default() -> Cs12 {
        <crate::RegValueT<Cs12_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cb12_SPEC;
impl crate::sealed::RegSpec for Cb12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control Block Address register"]
pub type Cb12 = crate::RegValueT<Cb12_SPEC>;

impl Cb12 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cb12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cb12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cb12 {
    #[inline(always)]
    fn default() -> Cb12 {
        <crate::RegValueT<Cb12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug12_SPEC;
impl crate::sealed::RegSpec for Debug12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug12 = crate::RegValueT<Debug12_SPEC>;

impl Debug12 {
    #[doc = "Slave Write Response Error"]
    #[inline(always)]
    pub fn write_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error During Control Block Read"]
    #[inline(always)]
    pub fn read_cb_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Debug12_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Generate an interrupt if an error is detected"]
    #[inline(always)]
    pub fn int_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Debug12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Debug12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to HALT if it detects an error"]
    #[inline(always)]
    pub fn halt_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Debug12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Debug12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to ABORT if it detects an error"]
    #[inline(always)]
    pub fn abort_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Debug12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Debug12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable the clock gating logic"]
    #[inline(always)]
    pub fn disable_clk_gate(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Debug12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Debug12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read State Machine State"]
    #[inline(always)]
    pub fn r_state(
        self,
    ) -> crate::common::RegisterField<14, 0xf, 1, 0, u8, u8, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0xf,1,0,u8,u8,Debug12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Write State Machine State"]
    #[inline(always)]
    pub fn w_state(
        self,
    ) -> crate::common::RegisterField<18, 0xf, 1, 0, u8, u8, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0xf,1,0,u8,u8,Debug12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Reset"]
    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Debug12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Debug12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ID"]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Debug12_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Debug12_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Debug12_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug12 {
    #[inline(always)]
    fn default() -> Debug12 {
        <crate::RegValueT<Debug12_SPEC> as RegisterValue<_>>::new(268436480)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti12_SPEC;
impl crate::sealed::RegSpec for Ti12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Information"]
pub type Ti12 = crate::RegValueT<Ti12_SPEC>;

impl Ti12 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Perform a 2D transfer instead of a normal linear transfer"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Read Response"]
    #[inline(always)]
    pub fn wait_rd_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<9, 0x1f, 1, 0, u8, u8, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1f,1,0,u8,u8,Ti12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn s_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn d_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read Wait Cycles"]
    #[inline(always)]
    pub fn s_waits(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ti12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Wait Cycles"]
    #[inline(always)]
    pub fn d_waits(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ti12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ti12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti12 {
    #[inline(always)]
    fn default() -> Ti12 {
        <crate::RegValueT<Ti12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src12_SPEC;
impl crate::sealed::RegSpec for Src12_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Source Address"]
pub type Src12 = crate::RegValueT<Src12_SPEC>;

impl Src12 {
    #[doc = "Lower bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Src12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Src12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Src12 {
    #[inline(always)]
    fn default() -> Src12 {
        <crate::RegValueT<Src12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srci12_SPEC;
impl crate::sealed::RegSpec for Srci12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Source Information"]
pub type Srci12 = crate::RegValueT<Srci12_SPEC>;

impl Srci12 {
    #[doc = "High Bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Srci12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Srci12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Srci12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Srci12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Source Address"]
    #[inline(always)]
    pub fn inc(self) -> crate::common::RegisterFieldBool<12, 1, 0, Srci12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Srci12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Srci12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Srci12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Srci12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Srci12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Srci12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Srci12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srci12 {
    #[inline(always)]
    fn default() -> Srci12 {
        <crate::RegValueT<Srci12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dest12_SPEC;
impl crate::sealed::RegSpec for Dest12_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Destination Address"]
pub type Dest12 = crate::RegValueT<Dest12_SPEC>;

impl Dest12 {
    #[doc = "Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dest12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dest12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dest12 {
    #[inline(always)]
    fn default() -> Dest12 {
        <crate::RegValueT<Dest12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Desti12_SPEC;
impl crate::sealed::RegSpec for Desti12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Destination Information"]
pub type Desti12 = crate::RegValueT<Desti12_SPEC>;

impl Desti12 {
    #[doc = "High Bits of the Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Desti12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Desti12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Desti12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Desti12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Destination Address"]
    #[inline(always)]
    pub fn inc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Desti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Desti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Desti12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Desti12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Desti12_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Desti12_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Desti12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Desti12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Desti12 {
    #[inline(always)]
    fn default() -> Desti12 {
        <crate::RegValueT<Desti12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Len12_SPEC;
impl crate::sealed::RegSpec for Len12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Length"]
pub type Len12 = crate::RegValueT<Len12_SPEC>;

impl Len12 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Len12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Len12_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers are performed, when in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, Len12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,Len12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Len12 {
    #[inline(always)]
    fn default() -> Len12 {
        <crate::RegValueT<Len12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NextCb12_SPEC;
impl crate::sealed::RegSpec for NextCb12_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Next Control Block Address"]
pub type NextCb12 = crate::RegValueT<NextCb12_SPEC>;

impl NextCb12 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, NextCb12_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            NextCb12_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for NextCb12 {
    #[inline(always)]
    fn default() -> NextCb12 {
        <crate::RegValueT<NextCb12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug212_SPEC;
impl crate::sealed::RegSpec for Debug212_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug2 register"]
pub type Debug212 = crate::RegValueT<Debug212_SPEC>;

impl Debug212 {
    #[doc = "Outstanding Write Response Count"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Debug212_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Debug212_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Outstanding read Words Count"]
    #[inline(always)]
    pub fn outstanding_reads(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug212_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug212_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug212 {
    #[inline(always)]
    fn default() -> Debug212 {
        <crate::RegValueT<Debug212_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs13_SPEC;
impl crate::sealed::RegSpec for Cs13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
pub type Cs13 = crate::RegValueT<Cs13_SPEC>;

impl Cs13 {
    #[doc = "Activate the DMA4"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Read Paused State"]
    #[inline(always)]
    pub fn rd_paused(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Write Paused State"]
    #[inline(always)]
    pub fn wr_paused(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA4 is Waiting for all the Write Response to be returned"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI QOS Level"]
    #[inline(always)]
    pub fn qos(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic QOS Level"]
    #[inline(always)]
    pub fn panic_qos(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the DMA4 is BUSY"]
    #[inline(always)]
    pub fn dma_busy(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that there are outstanding AXI transfers"]
    #[inline(always)]
    pub fn outstanding_transactions(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Cs13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Cs13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA4"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing a 1 to this bit will cleanly halt the current DMA4 transfer"]
    #[inline(always)]
    pub fn halt(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs13 {
    #[inline(always)]
    fn default() -> Cs13 {
        <crate::RegValueT<Cs13_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cb13_SPEC;
impl crate::sealed::RegSpec for Cb13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control Block Address register"]
pub type Cb13 = crate::RegValueT<Cb13_SPEC>;

impl Cb13 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cb13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cb13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cb13 {
    #[inline(always)]
    fn default() -> Cb13 {
        <crate::RegValueT<Cb13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug13_SPEC;
impl crate::sealed::RegSpec for Debug13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug13 = crate::RegValueT<Debug13_SPEC>;

impl Debug13 {
    #[doc = "Slave Write Response Error"]
    #[inline(always)]
    pub fn write_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error During Control Block Read"]
    #[inline(always)]
    pub fn read_cb_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Debug13_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Generate an interrupt if an error is detected"]
    #[inline(always)]
    pub fn int_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Debug13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Debug13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to HALT if it detects an error"]
    #[inline(always)]
    pub fn halt_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Debug13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Debug13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to ABORT if it detects an error"]
    #[inline(always)]
    pub fn abort_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Debug13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Debug13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable the clock gating logic"]
    #[inline(always)]
    pub fn disable_clk_gate(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Debug13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Debug13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read State Machine State"]
    #[inline(always)]
    pub fn r_state(
        self,
    ) -> crate::common::RegisterField<14, 0xf, 1, 0, u8, u8, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0xf,1,0,u8,u8,Debug13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Write State Machine State"]
    #[inline(always)]
    pub fn w_state(
        self,
    ) -> crate::common::RegisterField<18, 0xf, 1, 0, u8, u8, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0xf,1,0,u8,u8,Debug13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Reset"]
    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Debug13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Debug13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ID"]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Debug13_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Debug13_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Debug13_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug13 {
    #[inline(always)]
    fn default() -> Debug13 {
        <crate::RegValueT<Debug13_SPEC> as RegisterValue<_>>::new(268436480)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti13_SPEC;
impl crate::sealed::RegSpec for Ti13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Information"]
pub type Ti13 = crate::RegValueT<Ti13_SPEC>;

impl Ti13 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Perform a 2D transfer instead of a normal linear transfer"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Read Response"]
    #[inline(always)]
    pub fn wait_rd_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<9, 0x1f, 1, 0, u8, u8, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1f,1,0,u8,u8,Ti13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn s_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn d_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read Wait Cycles"]
    #[inline(always)]
    pub fn s_waits(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ti13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Wait Cycles"]
    #[inline(always)]
    pub fn d_waits(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ti13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ti13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti13 {
    #[inline(always)]
    fn default() -> Ti13 {
        <crate::RegValueT<Ti13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src13_SPEC;
impl crate::sealed::RegSpec for Src13_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Source Address"]
pub type Src13 = crate::RegValueT<Src13_SPEC>;

impl Src13 {
    #[doc = "Lower bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Src13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Src13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Src13 {
    #[inline(always)]
    fn default() -> Src13 {
        <crate::RegValueT<Src13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srci13_SPEC;
impl crate::sealed::RegSpec for Srci13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Source Information"]
pub type Srci13 = crate::RegValueT<Srci13_SPEC>;

impl Srci13 {
    #[doc = "High Bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Srci13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Srci13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Srci13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Srci13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Source Address"]
    #[inline(always)]
    pub fn inc(self) -> crate::common::RegisterFieldBool<12, 1, 0, Srci13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Srci13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Srci13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Srci13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Srci13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Srci13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Srci13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Srci13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srci13 {
    #[inline(always)]
    fn default() -> Srci13 {
        <crate::RegValueT<Srci13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dest13_SPEC;
impl crate::sealed::RegSpec for Dest13_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Destination Address"]
pub type Dest13 = crate::RegValueT<Dest13_SPEC>;

impl Dest13 {
    #[doc = "Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dest13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dest13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dest13 {
    #[inline(always)]
    fn default() -> Dest13 {
        <crate::RegValueT<Dest13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Desti13_SPEC;
impl crate::sealed::RegSpec for Desti13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Destination Information"]
pub type Desti13 = crate::RegValueT<Desti13_SPEC>;

impl Desti13 {
    #[doc = "High Bits of the Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Desti13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Desti13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Desti13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Desti13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Destination Address"]
    #[inline(always)]
    pub fn inc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Desti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Desti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Desti13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Desti13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Desti13_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Desti13_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Desti13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Desti13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Desti13 {
    #[inline(always)]
    fn default() -> Desti13 {
        <crate::RegValueT<Desti13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Len13_SPEC;
impl crate::sealed::RegSpec for Len13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Length"]
pub type Len13 = crate::RegValueT<Len13_SPEC>;

impl Len13 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Len13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Len13_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers are performed, when in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, Len13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,Len13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Len13 {
    #[inline(always)]
    fn default() -> Len13 {
        <crate::RegValueT<Len13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NextCb13_SPEC;
impl crate::sealed::RegSpec for NextCb13_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Next Control Block Address"]
pub type NextCb13 = crate::RegValueT<NextCb13_SPEC>;

impl NextCb13 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, NextCb13_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            NextCb13_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for NextCb13 {
    #[inline(always)]
    fn default() -> NextCb13 {
        <crate::RegValueT<NextCb13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug213_SPEC;
impl crate::sealed::RegSpec for Debug213_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug2 register"]
pub type Debug213 = crate::RegValueT<Debug213_SPEC>;

impl Debug213 {
    #[doc = "Outstanding Write Response Count"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Debug213_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Debug213_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Outstanding read Words Count"]
    #[inline(always)]
    pub fn outstanding_reads(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug213_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug213_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug213 {
    #[inline(always)]
    fn default() -> Debug213 {
        <crate::RegValueT<Debug213_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs14_SPEC;
impl crate::sealed::RegSpec for Cs14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control and Status register contains the main control and status bits for this DMA4\n                channel"]
pub type Cs14 = crate::RegValueT<Cs14_SPEC>;

impl Cs14 {
    #[doc = "Activate the DMA4"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Read Paused State"]
    #[inline(always)]
    pub fn rd_paused(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Write Paused State"]
    #[inline(always)]
    pub fn wr_paused(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "The DMA4 is Waiting for all the Write Response to be returned"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA4 Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<10, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI QOS Level"]
    #[inline(always)]
    pub fn qos(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,Cs14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "AXI Panic QOS Level"]
    #[inline(always)]
    pub fn panic_qos(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,Cs14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Indicates the DMA4 is BUSY"]
    #[inline(always)]
    pub fn dma_busy(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Indicates that there are outstanding AXI transfers"]
    #[inline(always)]
    pub fn outstanding_transactions(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Cs14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Cs14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable Debug Pause Signal"]
    #[inline(always)]
    pub fn disdebug(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Abort DMA4"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Writing a 1 to this bit will cleanly halt the current DMA4 transfer"]
    #[inline(always)]
    pub fn halt(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cs14 {
    #[inline(always)]
    fn default() -> Cs14 {
        <crate::RegValueT<Cs14_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cb14_SPEC;
impl crate::sealed::RegSpec for Cb14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Control Block Address register"]
pub type Cb14 = crate::RegValueT<Cb14_SPEC>;

impl Cb14 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cb14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cb14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cb14 {
    #[inline(always)]
    fn default() -> Cb14 {
        <crate::RegValueT<Cb14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug14_SPEC;
impl crate::sealed::RegSpec for Debug14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug register"]
pub type Debug14 = crate::RegValueT<Debug14_SPEC>;

impl Debug14 {
    #[doc = "Slave Write Response Error"]
    #[inline(always)]
    pub fn write_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Slave Read Response Error During Control Block Read"]
    #[inline(always)]
    pub fn read_cb_error(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Debug14_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Generate an interrupt if an error is detected"]
    #[inline(always)]
    pub fn int_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Debug14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Debug14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to HALT if it detects an error"]
    #[inline(always)]
    pub fn halt_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Debug14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Debug14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Instruct the DMA4 to ABORT if it detects an error"]
    #[inline(always)]
    pub fn abort_on_error(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Debug14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Debug14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Disable the clock gating logic"]
    #[inline(always)]
    pub fn disable_clk_gate(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Debug14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Debug14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read State Machine State"]
    #[inline(always)]
    pub fn r_state(
        self,
    ) -> crate::common::RegisterField<14, 0xf, 1, 0, u8, u8, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0xf,1,0,u8,u8,Debug14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Write State Machine State"]
    #[inline(always)]
    pub fn w_state(
        self,
    ) -> crate::common::RegisterField<18, 0xf, 1, 0, u8, u8, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0xf,1,0,u8,u8,Debug14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Reset"]
    #[inline(always)]
    pub fn reset(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Debug14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Debug14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ID"]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Debug14_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Debug14_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Debug14_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug14 {
    #[inline(always)]
    fn default() -> Debug14 {
        <crate::RegValueT<Debug14_SPEC> as RegisterValue<_>>::new(268436480)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti14_SPEC;
impl crate::sealed::RegSpec for Ti14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Information"]
pub type Ti14 = crate::RegValueT<Ti14_SPEC>;

impl Ti14 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Perform a 2D transfer instead of a normal linear transfer"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for a Read Response"]
    #[inline(always)]
    pub fn wait_rd_resp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(
        self,
    ) -> crate::common::RegisterField<9, 0x1f, 1, 0, u8, u8, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1f,1,0,u8,u8,Ti14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn s_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn d_dreq(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Read Wait Cycles"]
    #[inline(always)]
    pub fn s_waits(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Ti14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write Wait Cycles"]
    #[inline(always)]
    pub fn d_waits(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ti14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ti14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ti14 {
    #[inline(always)]
    fn default() -> Ti14 {
        <crate::RegValueT<Ti14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src14_SPEC;
impl crate::sealed::RegSpec for Src14_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Source Address"]
pub type Src14 = crate::RegValueT<Src14_SPEC>;

impl Src14 {
    #[doc = "Lower bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Src14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Src14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Src14 {
    #[inline(always)]
    fn default() -> Src14 {
        <crate::RegValueT<Src14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srci14_SPEC;
impl crate::sealed::RegSpec for Srci14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Source Information"]
pub type Srci14 = crate::RegValueT<Srci14_SPEC>;

impl Srci14 {
    #[doc = "High Bits of the Source Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Srci14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Srci14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Srci14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Srci14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Source Address"]
    #[inline(always)]
    pub fn inc(self) -> crate::common::RegisterFieldBool<12, 1, 0, Srci14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Srci14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Srci14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Srci14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Srci14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Srci14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Source Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Srci14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Srci14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srci14 {
    #[inline(always)]
    fn default() -> Srci14 {
        <crate::RegValueT<Srci14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dest14_SPEC;
impl crate::sealed::RegSpec for Dest14_SPEC {
    type DataType = u32;
}

#[doc = "Lower 32 bits of the DMA4 Destination Address"]
pub type Dest14 = crate::RegValueT<Dest14_SPEC>;

impl Dest14 {
    #[doc = "Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Dest14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Dest14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dest14 {
    #[inline(always)]
    fn default() -> Dest14 {
        <crate::RegValueT<Dest14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Desti14_SPEC;
impl crate::sealed::RegSpec for Desti14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Destination Information"]
pub type Desti14 = crate::RegValueT<Desti14_SPEC>;

impl Desti14 {
    #[doc = "High Bits of the Destination Address"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Desti14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Desti14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Desti14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Desti14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Increment the Destination Address"]
    #[inline(always)]
    pub fn inc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Desti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Desti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn size(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, Desti14_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,Desti14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn ignore(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Desti14_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Desti14_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Destination Stride"]
    #[inline(always)]
    pub fn stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Desti14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Desti14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Desti14 {
    #[inline(always)]
    fn default() -> Desti14 {
        <crate::RegValueT<Desti14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Len14_SPEC;
impl crate::sealed::RegSpec for Len14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Transfer Length"]
pub type Len14 = crate::RegValueT<Len14_SPEC>;

impl Len14 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlenth(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Len14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Len14_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers are performed, when in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylenth(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, Len14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,Len14_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Len14 {
    #[inline(always)]
    fn default() -> Len14 {
        <crate::RegValueT<Len14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NextCb14_SPEC;
impl crate::sealed::RegSpec for NextCb14_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Next Control Block Address"]
pub type NextCb14 = crate::RegValueT<NextCb14_SPEC>;

impl NextCb14 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, NextCb14_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            NextCb14_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for NextCb14 {
    #[inline(always)]
    fn default() -> NextCb14 {
        <crate::RegValueT<NextCb14_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug214_SPEC;
impl crate::sealed::RegSpec for Debug214_SPEC {
    type DataType = u32;
}

#[doc = "DMA4 Debug2 register"]
pub type Debug214 = crate::RegValueT<Debug214_SPEC>;

impl Debug214 {
    #[doc = "Outstanding Write Response Count"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Debug214_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Debug214_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Outstanding read Words Count"]
    #[inline(always)]
    pub fn outstanding_reads(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug214_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Debug214_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Debug214 {
    #[inline(always)]
    fn default() -> Debug214 {
        <crate::RegValueT<Debug214_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus_SPEC;
impl crate::sealed::RegSpec for IntStatus_SPEC {
    type DataType = u32;
}

#[doc = "Interrupt status of each DMA engine"]
pub type IntStatus = crate::RegValueT<IntStatus_SPEC>;

impl IntStatus {
    #[doc = "Interrupt status of DMA engine 0"]
    #[inline(always)]
    pub fn int0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 1"]
    #[inline(always)]
    pub fn int1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 2"]
    #[inline(always)]
    pub fn int2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 3"]
    #[inline(always)]
    pub fn int3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 4"]
    #[inline(always)]
    pub fn int4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 5"]
    #[inline(always)]
    pub fn int5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 6"]
    #[inline(always)]
    pub fn int6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 7"]
    #[inline(always)]
    pub fn int7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 8"]
    #[inline(always)]
    pub fn int8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 9"]
    #[inline(always)]
    pub fn int9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of DMA engine 10"]
    #[inline(always)]
    pub fn int10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,IntStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Interrupt status of DMA engine 11"]
    #[inline(always)]
    pub fn int11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,IntStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Interrupt status of DMA engine 12"]
    #[inline(always)]
    pub fn int12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,IntStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Interrupt status of DMA engine 13"]
    #[inline(always)]
    pub fn int13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,IntStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Interrupt status of DMA engine 14"]
    #[inline(always)]
    pub fn int14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,IntStatus_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Interrupt status of DMA engine 15"]
    #[inline(always)]
    pub fn int15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,IntStatus_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        <crate::RegValueT<IntStatus_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable_SPEC;
impl crate::sealed::RegSpec for Enable_SPEC {
    type DataType = u32;
}

#[doc = "Global enable bits for each channel"]
pub type Enable = crate::RegValueT<Enable_SPEC>;

impl Enable {
    #[doc = "Enable dma engine 0"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 1"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 2"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 3"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 4"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 5"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 6"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 7"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 8"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 9"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 10"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 11"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 12"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 13"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Enable dma engine 14"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Enable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Set the 1G SDRAM ram page that the 30-bit DMA engines (DMA0-6) will\n                                access\n                                when\n                                addressing the 1G uncached range C000_0000->ffff_ffff"]
    #[inline(always)]
    pub fn page(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Enable_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set the 1G SDRAM ram page that the 30-bit DMA llite engines (DMA7-10)\n                                will\n                                access\n                                when\n                                addressing the 1G uncached range C000_0000->ffff_ffff"]
    #[inline(always)]
    pub fn pagelite(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Enable_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Enable_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        <crate::RegValueT<Enable_SPEC> as RegisterValue<_>>::new(32767)
    }
}
