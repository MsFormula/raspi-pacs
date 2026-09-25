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
// Generated from SVD A, with svd2pac 0.8.0 on Fri, 25 Sep 2026 21:30:44 +0000

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
#[doc = r""]
unsafe impl ::core::marker::Send for super::Dma {}
unsafe impl ::core::marker::Sync for super::Dma {}
impl super::Dma {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "DMA 0 Control And Status register"]
    #[inline(always)]
    pub fn cs_0(&self) -> &'static self::Cs0T {
        unsafe { self::Cs0T::from_ptr(self._svd2pac_as_ptr().add(0usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_0(&self) -> &'static self::ConblkAd0T {
        unsafe { self::ConblkAd0T::from_ptr(self._svd2pac_as_ptr().add(4usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_0(&self) -> &'static self::SourceAd0T {
        unsafe { self::SourceAd0T::from_ptr(self._svd2pac_as_ptr().add(12usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_0(&self) -> &'static self::DestAd0T {
        unsafe { self::DestAd0T::from_ptr(self._svd2pac_as_ptr().add(16usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_0(&self) -> &'static self::Nextconbk0T {
        unsafe { self::Nextconbk0T::from_ptr(self._svd2pac_as_ptr().add(28usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_0(&self) -> &'static self::Ti0T {
        unsafe { self::Ti0T::from_ptr(self._svd2pac_as_ptr().add(8usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_0(&self) -> &'static self::TxfrLen0T {
        unsafe { self::TxfrLen0T::from_ptr(self._svd2pac_as_ptr().add(20usize)) }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub fn stride_0(&self) -> &'static self::Stride0T {
        unsafe { self::Stride0T::from_ptr(self._svd2pac_as_ptr().add(24usize)) }
    }

    #[doc = "DMA Debug register"]
    #[inline(always)]
    pub fn debug_0(&self) -> &'static self::Debug0T {
        unsafe { self::Debug0T::from_ptr(self._svd2pac_as_ptr().add(32usize)) }
    }

    #[doc = "DMA 1 Control And Status register"]
    #[inline(always)]
    pub fn cs_1(&self) -> &'static self::Cs1T {
        unsafe { self::Cs1T::from_ptr(self._svd2pac_as_ptr().add(256usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_1(&self) -> &'static self::ConblkAd1T {
        unsafe { self::ConblkAd1T::from_ptr(self._svd2pac_as_ptr().add(260usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_1(&self) -> &'static self::SourceAd1T {
        unsafe { self::SourceAd1T::from_ptr(self._svd2pac_as_ptr().add(268usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_1(&self) -> &'static self::DestAd1T {
        unsafe { self::DestAd1T::from_ptr(self._svd2pac_as_ptr().add(272usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_1(&self) -> &'static self::Nextconbk1T {
        unsafe { self::Nextconbk1T::from_ptr(self._svd2pac_as_ptr().add(284usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_1(&self) -> &'static self::Ti1T {
        unsafe { self::Ti1T::from_ptr(self._svd2pac_as_ptr().add(264usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_1(&self) -> &'static self::TxfrLen1T {
        unsafe { self::TxfrLen1T::from_ptr(self._svd2pac_as_ptr().add(276usize)) }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub fn stride_1(&self) -> &'static self::Stride1T {
        unsafe { self::Stride1T::from_ptr(self._svd2pac_as_ptr().add(280usize)) }
    }

    #[doc = "DMA Debug register"]
    #[inline(always)]
    pub fn debug_1(&self) -> &'static self::Debug1T {
        unsafe { self::Debug1T::from_ptr(self._svd2pac_as_ptr().add(288usize)) }
    }

    #[doc = "DMA 2 Control And Status register"]
    #[inline(always)]
    pub fn cs_2(&self) -> &'static self::Cs2T {
        unsafe { self::Cs2T::from_ptr(self._svd2pac_as_ptr().add(512usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_2(&self) -> &'static self::ConblkAd2T {
        unsafe { self::ConblkAd2T::from_ptr(self._svd2pac_as_ptr().add(516usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_2(&self) -> &'static self::SourceAd2T {
        unsafe { self::SourceAd2T::from_ptr(self._svd2pac_as_ptr().add(524usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_2(&self) -> &'static self::DestAd2T {
        unsafe { self::DestAd2T::from_ptr(self._svd2pac_as_ptr().add(528usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_2(&self) -> &'static self::Nextconbk2T {
        unsafe { self::Nextconbk2T::from_ptr(self._svd2pac_as_ptr().add(540usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_2(&self) -> &'static self::Ti2T {
        unsafe { self::Ti2T::from_ptr(self._svd2pac_as_ptr().add(520usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_2(&self) -> &'static self::TxfrLen2T {
        unsafe { self::TxfrLen2T::from_ptr(self._svd2pac_as_ptr().add(532usize)) }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub fn stride_2(&self) -> &'static self::Stride2T {
        unsafe { self::Stride2T::from_ptr(self._svd2pac_as_ptr().add(536usize)) }
    }

    #[doc = "DMA Debug register"]
    #[inline(always)]
    pub fn debug_2(&self) -> &'static self::Debug2T {
        unsafe { self::Debug2T::from_ptr(self._svd2pac_as_ptr().add(544usize)) }
    }

    #[doc = "DMA 3 Control And Status register"]
    #[inline(always)]
    pub fn cs_3(&self) -> &'static self::Cs3T {
        unsafe { self::Cs3T::from_ptr(self._svd2pac_as_ptr().add(768usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_3(&self) -> &'static self::ConblkAd3T {
        unsafe { self::ConblkAd3T::from_ptr(self._svd2pac_as_ptr().add(772usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_3(&self) -> &'static self::SourceAd3T {
        unsafe { self::SourceAd3T::from_ptr(self._svd2pac_as_ptr().add(780usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_3(&self) -> &'static self::DestAd3T {
        unsafe { self::DestAd3T::from_ptr(self._svd2pac_as_ptr().add(784usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_3(&self) -> &'static self::Nextconbk3T {
        unsafe { self::Nextconbk3T::from_ptr(self._svd2pac_as_ptr().add(796usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_3(&self) -> &'static self::Ti3T {
        unsafe { self::Ti3T::from_ptr(self._svd2pac_as_ptr().add(776usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_3(&self) -> &'static self::TxfrLen3T {
        unsafe { self::TxfrLen3T::from_ptr(self._svd2pac_as_ptr().add(788usize)) }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub fn stride_3(&self) -> &'static self::Stride3T {
        unsafe { self::Stride3T::from_ptr(self._svd2pac_as_ptr().add(792usize)) }
    }

    #[doc = "DMA Debug register"]
    #[inline(always)]
    pub fn debug_3(&self) -> &'static self::Debug3T {
        unsafe { self::Debug3T::from_ptr(self._svd2pac_as_ptr().add(800usize)) }
    }

    #[doc = "DMA 4 Control And Status register"]
    #[inline(always)]
    pub fn cs_4(&self) -> &'static self::Cs4T {
        unsafe { self::Cs4T::from_ptr(self._svd2pac_as_ptr().add(1024usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_4(&self) -> &'static self::ConblkAd4T {
        unsafe { self::ConblkAd4T::from_ptr(self._svd2pac_as_ptr().add(1028usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_4(&self) -> &'static self::SourceAd4T {
        unsafe { self::SourceAd4T::from_ptr(self._svd2pac_as_ptr().add(1036usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_4(&self) -> &'static self::DestAd4T {
        unsafe { self::DestAd4T::from_ptr(self._svd2pac_as_ptr().add(1040usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_4(&self) -> &'static self::Nextconbk4T {
        unsafe { self::Nextconbk4T::from_ptr(self._svd2pac_as_ptr().add(1052usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_4(&self) -> &'static self::Ti4T {
        unsafe { self::Ti4T::from_ptr(self._svd2pac_as_ptr().add(1032usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_4(&self) -> &'static self::TxfrLen4T {
        unsafe { self::TxfrLen4T::from_ptr(self._svd2pac_as_ptr().add(1044usize)) }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub fn stride_4(&self) -> &'static self::Stride4T {
        unsafe { self::Stride4T::from_ptr(self._svd2pac_as_ptr().add(1048usize)) }
    }

    #[doc = "DMA Debug register"]
    #[inline(always)]
    pub fn debug_4(&self) -> &'static self::Debug4T {
        unsafe { self::Debug4T::from_ptr(self._svd2pac_as_ptr().add(1056usize)) }
    }

    #[doc = "DMA 5 Control And Status register"]
    #[inline(always)]
    pub fn cs_5(&self) -> &'static self::Cs5T {
        unsafe { self::Cs5T::from_ptr(self._svd2pac_as_ptr().add(1280usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_5(&self) -> &'static self::ConblkAd5T {
        unsafe { self::ConblkAd5T::from_ptr(self._svd2pac_as_ptr().add(1284usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_5(&self) -> &'static self::SourceAd5T {
        unsafe { self::SourceAd5T::from_ptr(self._svd2pac_as_ptr().add(1292usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_5(&self) -> &'static self::DestAd5T {
        unsafe { self::DestAd5T::from_ptr(self._svd2pac_as_ptr().add(1296usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_5(&self) -> &'static self::Nextconbk5T {
        unsafe { self::Nextconbk5T::from_ptr(self._svd2pac_as_ptr().add(1308usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_5(&self) -> &'static self::Ti5T {
        unsafe { self::Ti5T::from_ptr(self._svd2pac_as_ptr().add(1288usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_5(&self) -> &'static self::TxfrLen5T {
        unsafe { self::TxfrLen5T::from_ptr(self._svd2pac_as_ptr().add(1300usize)) }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub fn stride_5(&self) -> &'static self::Stride5T {
        unsafe { self::Stride5T::from_ptr(self._svd2pac_as_ptr().add(1304usize)) }
    }

    #[doc = "DMA Debug register"]
    #[inline(always)]
    pub fn debug_5(&self) -> &'static self::Debug5T {
        unsafe { self::Debug5T::from_ptr(self._svd2pac_as_ptr().add(1312usize)) }
    }

    #[doc = "DMA 6 Control And Status register"]
    #[inline(always)]
    pub fn cs_6(&self) -> &'static self::Cs6T {
        unsafe { self::Cs6T::from_ptr(self._svd2pac_as_ptr().add(1536usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_6(&self) -> &'static self::ConblkAd6T {
        unsafe { self::ConblkAd6T::from_ptr(self._svd2pac_as_ptr().add(1540usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_6(&self) -> &'static self::SourceAd6T {
        unsafe { self::SourceAd6T::from_ptr(self._svd2pac_as_ptr().add(1548usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_6(&self) -> &'static self::DestAd6T {
        unsafe { self::DestAd6T::from_ptr(self._svd2pac_as_ptr().add(1552usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_6(&self) -> &'static self::Nextconbk6T {
        unsafe { self::Nextconbk6T::from_ptr(self._svd2pac_as_ptr().add(1564usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_6(&self) -> &'static self::Ti6T {
        unsafe { self::Ti6T::from_ptr(self._svd2pac_as_ptr().add(1544usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_6(&self) -> &'static self::TxfrLen6T {
        unsafe { self::TxfrLen6T::from_ptr(self._svd2pac_as_ptr().add(1556usize)) }
    }

    #[doc = "DMA 2D Stride"]
    #[inline(always)]
    pub fn stride_6(&self) -> &'static self::Stride6T {
        unsafe { self::Stride6T::from_ptr(self._svd2pac_as_ptr().add(1560usize)) }
    }

    #[doc = "DMA Debug register"]
    #[inline(always)]
    pub fn debug_6(&self) -> &'static self::Debug6T {
        unsafe { self::Debug6T::from_ptr(self._svd2pac_as_ptr().add(1568usize)) }
    }

    #[doc = "DMA 7 Control And Status register"]
    #[inline(always)]
    pub fn cs_7(&self) -> &'static self::Cs7T {
        unsafe { self::Cs7T::from_ptr(self._svd2pac_as_ptr().add(1792usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_7(&self) -> &'static self::ConblkAd7T {
        unsafe { self::ConblkAd7T::from_ptr(self._svd2pac_as_ptr().add(1796usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_7(&self) -> &'static self::SourceAd7T {
        unsafe { self::SourceAd7T::from_ptr(self._svd2pac_as_ptr().add(1804usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_7(&self) -> &'static self::DestAd7T {
        unsafe { self::DestAd7T::from_ptr(self._svd2pac_as_ptr().add(1808usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_7(&self) -> &'static self::Nextconbk7T {
        unsafe { self::Nextconbk7T::from_ptr(self._svd2pac_as_ptr().add(1820usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_7(&self) -> &'static self::Ti7T {
        unsafe { self::Ti7T::from_ptr(self._svd2pac_as_ptr().add(1800usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_7(&self) -> &'static self::TxfrLen7T {
        unsafe { self::TxfrLen7T::from_ptr(self._svd2pac_as_ptr().add(1812usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_7(&self) -> &'static self::Debug7T {
        unsafe { self::Debug7T::from_ptr(self._svd2pac_as_ptr().add(1824usize)) }
    }

    #[doc = "DMA 8 Control And Status register"]
    #[inline(always)]
    pub fn cs_8(&self) -> &'static self::Cs8T {
        unsafe { self::Cs8T::from_ptr(self._svd2pac_as_ptr().add(2048usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_8(&self) -> &'static self::ConblkAd8T {
        unsafe { self::ConblkAd8T::from_ptr(self._svd2pac_as_ptr().add(2052usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_8(&self) -> &'static self::SourceAd8T {
        unsafe { self::SourceAd8T::from_ptr(self._svd2pac_as_ptr().add(2060usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_8(&self) -> &'static self::DestAd8T {
        unsafe { self::DestAd8T::from_ptr(self._svd2pac_as_ptr().add(2064usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_8(&self) -> &'static self::Nextconbk8T {
        unsafe { self::Nextconbk8T::from_ptr(self._svd2pac_as_ptr().add(2076usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_8(&self) -> &'static self::Ti8T {
        unsafe { self::Ti8T::from_ptr(self._svd2pac_as_ptr().add(2056usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_8(&self) -> &'static self::TxfrLen8T {
        unsafe { self::TxfrLen8T::from_ptr(self._svd2pac_as_ptr().add(2068usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_8(&self) -> &'static self::Debug8T {
        unsafe { self::Debug8T::from_ptr(self._svd2pac_as_ptr().add(2080usize)) }
    }

    #[doc = "DMA 9 Control And Status register"]
    #[inline(always)]
    pub fn cs_9(&self) -> &'static self::Cs9T {
        unsafe { self::Cs9T::from_ptr(self._svd2pac_as_ptr().add(2304usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_9(&self) -> &'static self::ConblkAd9T {
        unsafe { self::ConblkAd9T::from_ptr(self._svd2pac_as_ptr().add(2308usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_9(&self) -> &'static self::SourceAd9T {
        unsafe { self::SourceAd9T::from_ptr(self._svd2pac_as_ptr().add(2316usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_9(&self) -> &'static self::DestAd9T {
        unsafe { self::DestAd9T::from_ptr(self._svd2pac_as_ptr().add(2320usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_9(&self) -> &'static self::Nextconbk9T {
        unsafe { self::Nextconbk9T::from_ptr(self._svd2pac_as_ptr().add(2332usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_9(&self) -> &'static self::Ti9T {
        unsafe { self::Ti9T::from_ptr(self._svd2pac_as_ptr().add(2312usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_9(&self) -> &'static self::TxfrLen9T {
        unsafe { self::TxfrLen9T::from_ptr(self._svd2pac_as_ptr().add(2324usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_9(&self) -> &'static self::Debug9T {
        unsafe { self::Debug9T::from_ptr(self._svd2pac_as_ptr().add(2336usize)) }
    }

    #[doc = "DMA 10 Control And Status register"]
    #[inline(always)]
    pub fn cs_10(&self) -> &'static self::Cs10T {
        unsafe { self::Cs10T::from_ptr(self._svd2pac_as_ptr().add(2560usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_10(&self) -> &'static self::ConblkAd10T {
        unsafe { self::ConblkAd10T::from_ptr(self._svd2pac_as_ptr().add(2564usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_10(&self) -> &'static self::SourceAd10T {
        unsafe { self::SourceAd10T::from_ptr(self._svd2pac_as_ptr().add(2572usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_10(&self) -> &'static self::DestAd10T {
        unsafe { self::DestAd10T::from_ptr(self._svd2pac_as_ptr().add(2576usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_10(&self) -> &'static self::Nextconbk10T {
        unsafe { self::Nextconbk10T::from_ptr(self._svd2pac_as_ptr().add(2588usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_10(&self) -> &'static self::Ti10T {
        unsafe { self::Ti10T::from_ptr(self._svd2pac_as_ptr().add(2568usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_10(&self) -> &'static self::TxfrLen10T {
        unsafe { self::TxfrLen10T::from_ptr(self._svd2pac_as_ptr().add(2580usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_10(&self) -> &'static self::Debug10T {
        unsafe { self::Debug10T::from_ptr(self._svd2pac_as_ptr().add(2592usize)) }
    }

    #[doc = "DMA 11 Control And Status register"]
    #[inline(always)]
    pub fn cs_11(&self) -> &'static self::Cs11T {
        unsafe { self::Cs11T::from_ptr(self._svd2pac_as_ptr().add(2816usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_11(&self) -> &'static self::ConblkAd11T {
        unsafe { self::ConblkAd11T::from_ptr(self._svd2pac_as_ptr().add(2820usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_11(&self) -> &'static self::SourceAd11T {
        unsafe { self::SourceAd11T::from_ptr(self._svd2pac_as_ptr().add(2828usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_11(&self) -> &'static self::DestAd11T {
        unsafe { self::DestAd11T::from_ptr(self._svd2pac_as_ptr().add(2832usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_11(&self) -> &'static self::Nextconbk11T {
        unsafe { self::Nextconbk11T::from_ptr(self._svd2pac_as_ptr().add(2844usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_11(&self) -> &'static self::Ti11T {
        unsafe { self::Ti11T::from_ptr(self._svd2pac_as_ptr().add(2824usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_11(&self) -> &'static self::TxfrLen11T {
        unsafe { self::TxfrLen11T::from_ptr(self._svd2pac_as_ptr().add(2836usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_11(&self) -> &'static self::Debug11T {
        unsafe { self::Debug11T::from_ptr(self._svd2pac_as_ptr().add(2848usize)) }
    }

    #[doc = "DMA 12 Control And Status register"]
    #[inline(always)]
    pub fn cs_12(&self) -> &'static self::Cs12T {
        unsafe { self::Cs12T::from_ptr(self._svd2pac_as_ptr().add(3072usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_12(&self) -> &'static self::ConblkAd12T {
        unsafe { self::ConblkAd12T::from_ptr(self._svd2pac_as_ptr().add(3076usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_12(&self) -> &'static self::SourceAd12T {
        unsafe { self::SourceAd12T::from_ptr(self._svd2pac_as_ptr().add(3084usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_12(&self) -> &'static self::DestAd12T {
        unsafe { self::DestAd12T::from_ptr(self._svd2pac_as_ptr().add(3088usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_12(&self) -> &'static self::Nextconbk12T {
        unsafe { self::Nextconbk12T::from_ptr(self._svd2pac_as_ptr().add(3100usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_12(&self) -> &'static self::Ti12T {
        unsafe { self::Ti12T::from_ptr(self._svd2pac_as_ptr().add(3080usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_12(&self) -> &'static self::TxfrLen12T {
        unsafe { self::TxfrLen12T::from_ptr(self._svd2pac_as_ptr().add(3092usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_12(&self) -> &'static self::Debug12T {
        unsafe { self::Debug12T::from_ptr(self._svd2pac_as_ptr().add(3104usize)) }
    }

    #[doc = "DMA 13 Control And Status register"]
    #[inline(always)]
    pub fn cs_13(&self) -> &'static self::Cs13T {
        unsafe { self::Cs13T::from_ptr(self._svd2pac_as_ptr().add(3328usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_13(&self) -> &'static self::ConblkAd13T {
        unsafe { self::ConblkAd13T::from_ptr(self._svd2pac_as_ptr().add(3332usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_13(&self) -> &'static self::SourceAd13T {
        unsafe { self::SourceAd13T::from_ptr(self._svd2pac_as_ptr().add(3340usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_13(&self) -> &'static self::DestAd13T {
        unsafe { self::DestAd13T::from_ptr(self._svd2pac_as_ptr().add(3344usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_13(&self) -> &'static self::Nextconbk13T {
        unsafe { self::Nextconbk13T::from_ptr(self._svd2pac_as_ptr().add(3356usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_13(&self) -> &'static self::Ti13T {
        unsafe { self::Ti13T::from_ptr(self._svd2pac_as_ptr().add(3336usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_13(&self) -> &'static self::TxfrLen13T {
        unsafe { self::TxfrLen13T::from_ptr(self._svd2pac_as_ptr().add(3348usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_13(&self) -> &'static self::Debug13T {
        unsafe { self::Debug13T::from_ptr(self._svd2pac_as_ptr().add(3360usize)) }
    }

    #[doc = "DMA 14 Control And Status register"]
    #[inline(always)]
    pub fn cs_14(&self) -> &'static self::Cs14T {
        unsafe { self::Cs14T::from_ptr(self._svd2pac_as_ptr().add(3584usize)) }
    }

    #[doc = "DMA Control Block Address register"]
    #[inline(always)]
    pub fn conblk_ad_14(&self) -> &'static self::ConblkAd14T {
        unsafe { self::ConblkAd14T::from_ptr(self._svd2pac_as_ptr().add(3588usize)) }
    }

    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn source_ad_14(&self) -> &'static self::SourceAd14T {
        unsafe { self::SourceAd14T::from_ptr(self._svd2pac_as_ptr().add(3596usize)) }
    }

    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn dest_ad_14(&self) -> &'static self::DestAd14T {
        unsafe { self::DestAd14T::from_ptr(self._svd2pac_as_ptr().add(3600usize)) }
    }

    #[doc = "DMA Next Control Block Address"]
    #[inline(always)]
    pub fn nextconbk_14(&self) -> &'static self::Nextconbk14T {
        unsafe { self::Nextconbk14T::from_ptr(self._svd2pac_as_ptr().add(3612usize)) }
    }

    #[doc = "DMA Transfer Information"]
    #[inline(always)]
    pub fn ti_14(&self) -> &'static self::Ti14T {
        unsafe { self::Ti14T::from_ptr(self._svd2pac_as_ptr().add(3592usize)) }
    }

    #[doc = "DMA Transfer Length."]
    #[inline(always)]
    pub fn txfr_len_14(&self) -> &'static self::TxfrLen14T {
        unsafe { self::TxfrLen14T::from_ptr(self._svd2pac_as_ptr().add(3604usize)) }
    }

    #[doc = "DMA lite Debug register"]
    #[inline(always)]
    pub fn debug_14(&self) -> &'static self::Debug14T {
        unsafe { self::Debug14T::from_ptr(self._svd2pac_as_ptr().add(3616usize)) }
    }

    #[doc = "Interrupt status of each DMA engine"]
    #[inline(always)]
    pub fn int_status(&self) -> &'static self::IntStatusT {
        unsafe { self::IntStatusT::from_ptr(self._svd2pac_as_ptr().add(4064usize)) }
    }

    #[doc = "Global enable bits for each channel"]
    #[inline(always)]
    pub fn enable(&self) -> &'static self::EnableT {
        unsafe { self::EnableT::from_ptr(self._svd2pac_as_ptr().add(4080usize)) }
    }
}

#[doc = "DMA 0 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs0 {
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
pub struct Cs0T;
unsafe impl crate::common::AsPtr for Cs0T {}
impl crate::common::Reg<Cs0> for Cs0T {}

unsafe impl crate::common::Read<Cs0> for Cs0T {}
unsafe impl crate::common::Write<Cs0> for Cs0T {}
impl Cs0 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs0, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs0, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs0, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs0, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs0, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs0, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs0, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs0, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs0, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs0, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs0, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs0, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs0, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs0, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs0, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs0, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs0, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs0, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs0, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs0, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs0, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs0, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs0, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs0, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs0, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs0> for Cs0T {
    #[inline(always)]
    fn reset_value(&self) -> Cs0 {
        Cs0::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd0 {
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
pub struct ConblkAd0T;
unsafe impl crate::common::AsPtr for ConblkAd0T {}
impl crate::common::Reg<ConblkAd0> for ConblkAd0T {}

unsafe impl crate::common::Read<ConblkAd0> for ConblkAd0T {}
unsafe impl crate::common::Write<ConblkAd0> for ConblkAd0T {}
impl ConblkAd0 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd0, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd0,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd0> for ConblkAd0T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd0 {
        ConblkAd0::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd0 {
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
pub struct SourceAd0T;
unsafe impl crate::common::AsPtr for SourceAd0T {}
impl crate::common::Reg<SourceAd0> for SourceAd0T {}

unsafe impl crate::common::Read<SourceAd0> for SourceAd0T {}
unsafe impl crate::common::Write<SourceAd0> for SourceAd0T {}
impl SourceAd0 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd0, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd0,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd0> for SourceAd0T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd0 {
        SourceAd0::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd0 {
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
pub struct DestAd0T;
unsafe impl crate::common::AsPtr for DestAd0T {}
impl crate::common::Reg<DestAd0> for DestAd0T {}

unsafe impl crate::common::Read<DestAd0> for DestAd0T {}
unsafe impl crate::common::Write<DestAd0> for DestAd0T {}
impl DestAd0 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd0, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd0,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd0> for DestAd0T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd0 {
        DestAd0::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk0 {
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
pub struct Nextconbk0T;
unsafe impl crate::common::AsPtr for Nextconbk0T {}
impl crate::common::Reg<Nextconbk0> for Nextconbk0T {}

unsafe impl crate::common::Read<Nextconbk0> for Nextconbk0T {}
unsafe impl crate::common::Write<Nextconbk0> for Nextconbk0T {}
impl Nextconbk0 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk0, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk0,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk0> for Nextconbk0T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk0 {
        Nextconbk0::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti0 {
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
pub struct Ti0T;
unsafe impl crate::common::AsPtr for Ti0T {}
impl crate::common::Reg<Ti0> for Ti0T {}

unsafe impl crate::common::Read<Ti0> for Ti0T {}
unsafe impl crate::common::Write<Ti0> for Ti0T {}
impl Ti0 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti0, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti0, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti0, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti0, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti0, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti0, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Dont Do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ti0, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti0, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ti0> for Ti0T {
    #[inline(always)]
    fn reset_value(&self) -> Ti0 {
        Ti0::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen0 {
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
pub struct TxfrLen0T;
unsafe impl crate::common::AsPtr for TxfrLen0T {}
impl crate::common::Reg<TxfrLen0> for TxfrLen0T {}

unsafe impl crate::common::Read<TxfrLen0> for TxfrLen0T {}
unsafe impl crate::common::Write<TxfrLen0> for TxfrLen0T {}
impl TxfrLen0 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen0, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen0,common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers\n                            are performed. When in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylength(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen0, common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen0,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen0> for TxfrLen0T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen0 {
        TxfrLen0::new(0)
    }
}

#[doc = "DMA 2D Stride"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stride0 {
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
pub struct Stride0T;
unsafe impl crate::common::AsPtr for Stride0T {}
impl crate::common::Reg<Stride0> for Stride0T {}

unsafe impl crate::common::Read<Stride0> for Stride0T {}
unsafe impl crate::common::Write<Stride0> for Stride0T {}
impl Stride0 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride0, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride0,common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride0, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride0,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Stride0> for Stride0T {
    #[inline(always)]
    fn reset_value(&self) -> Stride0 {
        Stride0::new(0)
    }
}

#[doc = "DMA Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug0 {
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
pub struct Debug0T;
unsafe impl crate::common::AsPtr for Debug0T {}
impl crate::common::Reg<Debug0> for Debug0T {}

unsafe impl crate::common::Read<Debug0> for Debug0T {}
unsafe impl crate::common::Write<Debug0> for Debug0T {}
impl Debug0 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug0, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug0, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug0, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug0, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug0, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug0, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug0, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug0, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug0, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug0, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug0, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug0, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug0, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug0> for Debug0T {
    #[inline(always)]
    fn reset_value(&self) -> Debug0 {
        Debug0::new(67108864)
    }
}

#[doc = "DMA 1 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs1 {
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
pub struct Cs1T;
unsafe impl crate::common::AsPtr for Cs1T {}
impl crate::common::Reg<Cs1> for Cs1T {}

unsafe impl crate::common::Read<Cs1> for Cs1T {}
unsafe impl crate::common::Write<Cs1> for Cs1T {}
impl Cs1 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs1, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs1, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs1, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs1, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs1, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs1, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs1, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs1, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs1, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs1, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs1, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs1, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs1, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs1, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs1, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs1, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs1, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs1, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs1, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs1> for Cs1T {
    #[inline(always)]
    fn reset_value(&self) -> Cs1 {
        Cs1::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd1 {
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
pub struct ConblkAd1T;
unsafe impl crate::common::AsPtr for ConblkAd1T {}
impl crate::common::Reg<ConblkAd1> for ConblkAd1T {}

unsafe impl crate::common::Read<ConblkAd1> for ConblkAd1T {}
unsafe impl crate::common::Write<ConblkAd1> for ConblkAd1T {}
impl ConblkAd1 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd1, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd1,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd1> for ConblkAd1T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd1 {
        ConblkAd1::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd1 {
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
pub struct SourceAd1T;
unsafe impl crate::common::AsPtr for SourceAd1T {}
impl crate::common::Reg<SourceAd1> for SourceAd1T {}

unsafe impl crate::common::Read<SourceAd1> for SourceAd1T {}
unsafe impl crate::common::Write<SourceAd1> for SourceAd1T {}
impl SourceAd1 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd1, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd1,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd1> for SourceAd1T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd1 {
        SourceAd1::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd1 {
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
pub struct DestAd1T;
unsafe impl crate::common::AsPtr for DestAd1T {}
impl crate::common::Reg<DestAd1> for DestAd1T {}

unsafe impl crate::common::Read<DestAd1> for DestAd1T {}
unsafe impl crate::common::Write<DestAd1> for DestAd1T {}
impl DestAd1 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd1, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd1,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd1> for DestAd1T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd1 {
        DestAd1::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk1 {
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
pub struct Nextconbk1T;
unsafe impl crate::common::AsPtr for Nextconbk1T {}
impl crate::common::Reg<Nextconbk1> for Nextconbk1T {}

unsafe impl crate::common::Read<Nextconbk1> for Nextconbk1T {}
unsafe impl crate::common::Write<Nextconbk1> for Nextconbk1T {}
impl Nextconbk1 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk1, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk1,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk1> for Nextconbk1T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk1 {
        Nextconbk1::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti1 {
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
pub struct Ti1T;
unsafe impl crate::common::AsPtr for Ti1T {}
impl crate::common::Reg<Ti1> for Ti1T {}

unsafe impl crate::common::Read<Ti1> for Ti1T {}
unsafe impl crate::common::Write<Ti1> for Ti1T {}
impl Ti1 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti1, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti1, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti1, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti1, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Dont Do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ti1, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti1, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ti1> for Ti1T {
    #[inline(always)]
    fn reset_value(&self) -> Ti1 {
        Ti1::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen1 {
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
pub struct TxfrLen1T;
unsafe impl crate::common::AsPtr for TxfrLen1T {}
impl crate::common::Reg<TxfrLen1> for TxfrLen1T {}

unsafe impl crate::common::Read<TxfrLen1> for TxfrLen1T {}
unsafe impl crate::common::Write<TxfrLen1> for TxfrLen1T {}
impl TxfrLen1 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen1, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen1,common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers\n                            are performed. When in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylength(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen1, common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen1,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen1> for TxfrLen1T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen1 {
        TxfrLen1::new(0)
    }
}

#[doc = "DMA 2D Stride"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stride1 {
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
pub struct Stride1T;
unsafe impl crate::common::AsPtr for Stride1T {}
impl crate::common::Reg<Stride1> for Stride1T {}

unsafe impl crate::common::Read<Stride1> for Stride1T {}
unsafe impl crate::common::Write<Stride1> for Stride1T {}
impl Stride1 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride1, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride1,common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride1, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride1,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Stride1> for Stride1T {
    #[inline(always)]
    fn reset_value(&self) -> Stride1 {
        Stride1::new(0)
    }
}

#[doc = "DMA Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug1 {
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
pub struct Debug1T;
unsafe impl crate::common::AsPtr for Debug1T {}
impl crate::common::Reg<Debug1> for Debug1T {}

unsafe impl crate::common::Read<Debug1> for Debug1T {}
unsafe impl crate::common::Write<Debug1> for Debug1T {}
impl Debug1 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug1, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug1, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug1, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug1, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug1, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug1, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug1, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug1, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug1, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug1, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug1, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug1, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug1, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug1, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug1, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug1, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug1> for Debug1T {
    #[inline(always)]
    fn reset_value(&self) -> Debug1 {
        Debug1::new(67108864)
    }
}

#[doc = "DMA 2 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs2 {
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
pub struct Cs2T;
unsafe impl crate::common::AsPtr for Cs2T {}
impl crate::common::Reg<Cs2> for Cs2T {}

unsafe impl crate::common::Read<Cs2> for Cs2T {}
unsafe impl crate::common::Write<Cs2> for Cs2T {}
impl Cs2 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs2, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs2, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs2, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs2, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs2, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs2, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs2, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs2, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs2, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs2, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs2, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs2, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs2, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs2, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs2, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs2, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs2, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs2, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs2, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs2, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs2, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs2, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs2, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs2, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs2, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs2, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs2, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs2, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs2> for Cs2T {
    #[inline(always)]
    fn reset_value(&self) -> Cs2 {
        Cs2::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd2 {
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
pub struct ConblkAd2T;
unsafe impl crate::common::AsPtr for ConblkAd2T {}
impl crate::common::Reg<ConblkAd2> for ConblkAd2T {}

unsafe impl crate::common::Read<ConblkAd2> for ConblkAd2T {}
unsafe impl crate::common::Write<ConblkAd2> for ConblkAd2T {}
impl ConblkAd2 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd2, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd2,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd2> for ConblkAd2T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd2 {
        ConblkAd2::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd2 {
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
pub struct SourceAd2T;
unsafe impl crate::common::AsPtr for SourceAd2T {}
impl crate::common::Reg<SourceAd2> for SourceAd2T {}

unsafe impl crate::common::Read<SourceAd2> for SourceAd2T {}
unsafe impl crate::common::Write<SourceAd2> for SourceAd2T {}
impl SourceAd2 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd2, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd2,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd2> for SourceAd2T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd2 {
        SourceAd2::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd2 {
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
pub struct DestAd2T;
unsafe impl crate::common::AsPtr for DestAd2T {}
impl crate::common::Reg<DestAd2> for DestAd2T {}

unsafe impl crate::common::Read<DestAd2> for DestAd2T {}
unsafe impl crate::common::Write<DestAd2> for DestAd2T {}
impl DestAd2 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd2, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd2,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd2> for DestAd2T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd2 {
        DestAd2::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk2 {
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
pub struct Nextconbk2T;
unsafe impl crate::common::AsPtr for Nextconbk2T {}
impl crate::common::Reg<Nextconbk2> for Nextconbk2T {}

unsafe impl crate::common::Read<Nextconbk2> for Nextconbk2T {}
unsafe impl crate::common::Write<Nextconbk2> for Nextconbk2T {}
impl Nextconbk2 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk2, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk2,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk2> for Nextconbk2T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk2 {
        Nextconbk2::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti2 {
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
pub struct Ti2T;
unsafe impl crate::common::AsPtr for Ti2T {}
impl crate::common::Reg<Ti2> for Ti2T {}

unsafe impl crate::common::Read<Ti2> for Ti2T {}
unsafe impl crate::common::Write<Ti2> for Ti2T {}
impl Ti2 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti2, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti2, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti2, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti2, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti2, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti2, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Dont Do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ti2, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti2, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ti2> for Ti2T {
    #[inline(always)]
    fn reset_value(&self) -> Ti2 {
        Ti2::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen2 {
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
pub struct TxfrLen2T;
unsafe impl crate::common::AsPtr for TxfrLen2T {}
impl crate::common::Reg<TxfrLen2> for TxfrLen2T {}

unsafe impl crate::common::Read<TxfrLen2> for TxfrLen2T {}
unsafe impl crate::common::Write<TxfrLen2> for TxfrLen2T {}
impl TxfrLen2 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen2, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen2,common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers\n                            are performed. When in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylength(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen2, common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen2,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen2> for TxfrLen2T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen2 {
        TxfrLen2::new(0)
    }
}

#[doc = "DMA 2D Stride"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stride2 {
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
pub struct Stride2T;
unsafe impl crate::common::AsPtr for Stride2T {}
impl crate::common::Reg<Stride2> for Stride2T {}

unsafe impl crate::common::Read<Stride2> for Stride2T {}
unsafe impl crate::common::Write<Stride2> for Stride2T {}
impl Stride2 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride2, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride2,common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride2, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride2,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Stride2> for Stride2T {
    #[inline(always)]
    fn reset_value(&self) -> Stride2 {
        Stride2::new(0)
    }
}

#[doc = "DMA Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug2 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug2 {
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
pub struct Debug2T;
unsafe impl crate::common::AsPtr for Debug2T {}
impl crate::common::Reg<Debug2> for Debug2T {}

unsafe impl crate::common::Read<Debug2> for Debug2T {}
unsafe impl crate::common::Write<Debug2> for Debug2T {}
impl Debug2 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug2, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug2, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug2, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug2, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug2, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug2, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug2, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug2, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug2, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug2, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug2, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug2, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug2, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug2, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug2, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug2, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug2> for Debug2T {
    #[inline(always)]
    fn reset_value(&self) -> Debug2 {
        Debug2::new(67108864)
    }
}

#[doc = "DMA 3 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs3 {
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
pub struct Cs3T;
unsafe impl crate::common::AsPtr for Cs3T {}
impl crate::common::Reg<Cs3> for Cs3T {}

unsafe impl crate::common::Read<Cs3> for Cs3T {}
unsafe impl crate::common::Write<Cs3> for Cs3T {}
impl Cs3 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs3, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs3, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs3, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs3, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs3, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs3, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs3, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs3, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs3, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs3, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs3, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs3, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs3, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs3, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs3, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs3, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs3, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs3, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs3, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs3, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs3, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs3, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs3, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs3, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs3, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs3, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs3, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs3, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs3> for Cs3T {
    #[inline(always)]
    fn reset_value(&self) -> Cs3 {
        Cs3::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd3 {
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
pub struct ConblkAd3T;
unsafe impl crate::common::AsPtr for ConblkAd3T {}
impl crate::common::Reg<ConblkAd3> for ConblkAd3T {}

unsafe impl crate::common::Read<ConblkAd3> for ConblkAd3T {}
unsafe impl crate::common::Write<ConblkAd3> for ConblkAd3T {}
impl ConblkAd3 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd3, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd3,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd3> for ConblkAd3T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd3 {
        ConblkAd3::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd3 {
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
pub struct SourceAd3T;
unsafe impl crate::common::AsPtr for SourceAd3T {}
impl crate::common::Reg<SourceAd3> for SourceAd3T {}

unsafe impl crate::common::Read<SourceAd3> for SourceAd3T {}
unsafe impl crate::common::Write<SourceAd3> for SourceAd3T {}
impl SourceAd3 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd3, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd3,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd3> for SourceAd3T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd3 {
        SourceAd3::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd3 {
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
pub struct DestAd3T;
unsafe impl crate::common::AsPtr for DestAd3T {}
impl crate::common::Reg<DestAd3> for DestAd3T {}

unsafe impl crate::common::Read<DestAd3> for DestAd3T {}
unsafe impl crate::common::Write<DestAd3> for DestAd3T {}
impl DestAd3 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd3, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd3,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd3> for DestAd3T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd3 {
        DestAd3::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk3 {
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
pub struct Nextconbk3T;
unsafe impl crate::common::AsPtr for Nextconbk3T {}
impl crate::common::Reg<Nextconbk3> for Nextconbk3T {}

unsafe impl crate::common::Read<Nextconbk3> for Nextconbk3T {}
unsafe impl crate::common::Write<Nextconbk3> for Nextconbk3T {}
impl Nextconbk3 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk3, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk3,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk3> for Nextconbk3T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk3 {
        Nextconbk3::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti3 {
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
pub struct Ti3T;
unsafe impl crate::common::AsPtr for Ti3T {}
impl crate::common::Reg<Ti3> for Ti3T {}

unsafe impl crate::common::Read<Ti3> for Ti3T {}
unsafe impl crate::common::Write<Ti3> for Ti3T {}
impl Ti3 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti3, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti3, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti3, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti3, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti3, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti3, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Dont Do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ti3, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti3, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ti3> for Ti3T {
    #[inline(always)]
    fn reset_value(&self) -> Ti3 {
        Ti3::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen3 {
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
pub struct TxfrLen3T;
unsafe impl crate::common::AsPtr for TxfrLen3T {}
impl crate::common::Reg<TxfrLen3> for TxfrLen3T {}

unsafe impl crate::common::Read<TxfrLen3> for TxfrLen3T {}
unsafe impl crate::common::Write<TxfrLen3> for TxfrLen3T {}
impl TxfrLen3 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen3, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen3,common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers\n                            are performed. When in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylength(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen3, common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen3,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen3> for TxfrLen3T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen3 {
        TxfrLen3::new(0)
    }
}

#[doc = "DMA 2D Stride"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stride3 {
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
pub struct Stride3T;
unsafe impl crate::common::AsPtr for Stride3T {}
impl crate::common::Reg<Stride3> for Stride3T {}

unsafe impl crate::common::Read<Stride3> for Stride3T {}
unsafe impl crate::common::Write<Stride3> for Stride3T {}
impl Stride3 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride3, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride3,common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride3, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride3,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Stride3> for Stride3T {
    #[inline(always)]
    fn reset_value(&self) -> Stride3 {
        Stride3::new(0)
    }
}

#[doc = "DMA Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug3 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug3 {
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
pub struct Debug3T;
unsafe impl crate::common::AsPtr for Debug3T {}
impl crate::common::Reg<Debug3> for Debug3T {}

unsafe impl crate::common::Read<Debug3> for Debug3T {}
unsafe impl crate::common::Write<Debug3> for Debug3T {}
impl Debug3 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug3, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug3, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug3, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug3, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug3, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug3, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug3, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug3, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug3, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug3, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug3, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug3, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug3, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug3, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug3, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug3, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug3> for Debug3T {
    #[inline(always)]
    fn reset_value(&self) -> Debug3 {
        Debug3::new(67108864)
    }
}

#[doc = "DMA 4 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs4 {
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
pub struct Cs4T;
unsafe impl crate::common::AsPtr for Cs4T {}
impl crate::common::Reg<Cs4> for Cs4T {}

unsafe impl crate::common::Read<Cs4> for Cs4T {}
unsafe impl crate::common::Write<Cs4> for Cs4T {}
impl Cs4 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs4, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs4, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs4, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs4, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs4, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs4, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs4, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs4, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs4, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs4, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs4, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs4, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs4, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs4, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs4, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs4, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs4, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs4, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs4, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs4, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs4, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs4, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs4, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs4, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs4, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs4, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs4, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs4, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs4> for Cs4T {
    #[inline(always)]
    fn reset_value(&self) -> Cs4 {
        Cs4::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd4 {
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
pub struct ConblkAd4T;
unsafe impl crate::common::AsPtr for ConblkAd4T {}
impl crate::common::Reg<ConblkAd4> for ConblkAd4T {}

unsafe impl crate::common::Read<ConblkAd4> for ConblkAd4T {}
unsafe impl crate::common::Write<ConblkAd4> for ConblkAd4T {}
impl ConblkAd4 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd4, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd4,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd4> for ConblkAd4T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd4 {
        ConblkAd4::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd4 {
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
pub struct SourceAd4T;
unsafe impl crate::common::AsPtr for SourceAd4T {}
impl crate::common::Reg<SourceAd4> for SourceAd4T {}

unsafe impl crate::common::Read<SourceAd4> for SourceAd4T {}
unsafe impl crate::common::Write<SourceAd4> for SourceAd4T {}
impl SourceAd4 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd4, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd4,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd4> for SourceAd4T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd4 {
        SourceAd4::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd4 {
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
pub struct DestAd4T;
unsafe impl crate::common::AsPtr for DestAd4T {}
impl crate::common::Reg<DestAd4> for DestAd4T {}

unsafe impl crate::common::Read<DestAd4> for DestAd4T {}
unsafe impl crate::common::Write<DestAd4> for DestAd4T {}
impl DestAd4 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd4, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd4,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd4> for DestAd4T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd4 {
        DestAd4::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk4 {
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
pub struct Nextconbk4T;
unsafe impl crate::common::AsPtr for Nextconbk4T {}
impl crate::common::Reg<Nextconbk4> for Nextconbk4T {}

unsafe impl crate::common::Read<Nextconbk4> for Nextconbk4T {}
unsafe impl crate::common::Write<Nextconbk4> for Nextconbk4T {}
impl Nextconbk4 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk4, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk4,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk4> for Nextconbk4T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk4 {
        Nextconbk4::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti4 {
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
pub struct Ti4T;
unsafe impl crate::common::AsPtr for Ti4T {}
impl crate::common::Reg<Ti4> for Ti4T {}

unsafe impl crate::common::Read<Ti4> for Ti4T {}
unsafe impl crate::common::Write<Ti4> for Ti4T {}
impl Ti4 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti4, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti4, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti4, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti4, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti4, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti4, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Dont Do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ti4, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti4, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ti4> for Ti4T {
    #[inline(always)]
    fn reset_value(&self) -> Ti4 {
        Ti4::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen4 {
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
pub struct TxfrLen4T;
unsafe impl crate::common::AsPtr for TxfrLen4T {}
impl crate::common::Reg<TxfrLen4> for TxfrLen4T {}

unsafe impl crate::common::Read<TxfrLen4> for TxfrLen4T {}
unsafe impl crate::common::Write<TxfrLen4> for TxfrLen4T {}
impl TxfrLen4 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen4, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen4,common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers\n                            are performed. When in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylength(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen4, common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen4,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen4> for TxfrLen4T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen4 {
        TxfrLen4::new(0)
    }
}

#[doc = "DMA 2D Stride"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stride4 {
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
pub struct Stride4T;
unsafe impl crate::common::AsPtr for Stride4T {}
impl crate::common::Reg<Stride4> for Stride4T {}

unsafe impl crate::common::Read<Stride4> for Stride4T {}
unsafe impl crate::common::Write<Stride4> for Stride4T {}
impl Stride4 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride4, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride4,common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride4, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride4,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Stride4> for Stride4T {
    #[inline(always)]
    fn reset_value(&self) -> Stride4 {
        Stride4::new(0)
    }
}

#[doc = "DMA Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug4 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug4 {
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
pub struct Debug4T;
unsafe impl crate::common::AsPtr for Debug4T {}
impl crate::common::Reg<Debug4> for Debug4T {}

unsafe impl crate::common::Read<Debug4> for Debug4T {}
unsafe impl crate::common::Write<Debug4> for Debug4T {}
impl Debug4 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug4, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug4, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug4, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug4, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug4, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug4, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug4, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug4, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug4, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug4, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug4, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug4, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug4, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug4, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug4, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug4, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug4> for Debug4T {
    #[inline(always)]
    fn reset_value(&self) -> Debug4 {
        Debug4::new(67108864)
    }
}

#[doc = "DMA 5 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs5 {
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
pub struct Cs5T;
unsafe impl crate::common::AsPtr for Cs5T {}
impl crate::common::Reg<Cs5> for Cs5T {}

unsafe impl crate::common::Read<Cs5> for Cs5T {}
unsafe impl crate::common::Write<Cs5> for Cs5T {}
impl Cs5 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs5, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs5, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs5, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs5, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs5, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs5, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs5, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs5, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs5, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs5, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs5, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs5, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs5, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs5, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs5, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs5, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs5, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs5, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs5, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs5, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs5, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs5, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs5, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs5, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs5, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs5, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs5, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs5, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs5> for Cs5T {
    #[inline(always)]
    fn reset_value(&self) -> Cs5 {
        Cs5::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd5 {
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
pub struct ConblkAd5T;
unsafe impl crate::common::AsPtr for ConblkAd5T {}
impl crate::common::Reg<ConblkAd5> for ConblkAd5T {}

unsafe impl crate::common::Read<ConblkAd5> for ConblkAd5T {}
unsafe impl crate::common::Write<ConblkAd5> for ConblkAd5T {}
impl ConblkAd5 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd5, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd5,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd5> for ConblkAd5T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd5 {
        ConblkAd5::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd5 {
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
pub struct SourceAd5T;
unsafe impl crate::common::AsPtr for SourceAd5T {}
impl crate::common::Reg<SourceAd5> for SourceAd5T {}

unsafe impl crate::common::Read<SourceAd5> for SourceAd5T {}
unsafe impl crate::common::Write<SourceAd5> for SourceAd5T {}
impl SourceAd5 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd5, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd5,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd5> for SourceAd5T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd5 {
        SourceAd5::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd5 {
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
pub struct DestAd5T;
unsafe impl crate::common::AsPtr for DestAd5T {}
impl crate::common::Reg<DestAd5> for DestAd5T {}

unsafe impl crate::common::Read<DestAd5> for DestAd5T {}
unsafe impl crate::common::Write<DestAd5> for DestAd5T {}
impl DestAd5 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd5, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd5,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd5> for DestAd5T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd5 {
        DestAd5::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk5 {
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
pub struct Nextconbk5T;
unsafe impl crate::common::AsPtr for Nextconbk5T {}
impl crate::common::Reg<Nextconbk5> for Nextconbk5T {}

unsafe impl crate::common::Read<Nextconbk5> for Nextconbk5T {}
unsafe impl crate::common::Write<Nextconbk5> for Nextconbk5T {}
impl Nextconbk5 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk5, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk5,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk5> for Nextconbk5T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk5 {
        Nextconbk5::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti5 {
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
pub struct Ti5T;
unsafe impl crate::common::AsPtr for Ti5T {}
impl crate::common::Reg<Ti5> for Ti5T {}

unsafe impl crate::common::Read<Ti5> for Ti5T {}
unsafe impl crate::common::Write<Ti5> for Ti5T {}
impl Ti5 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti5, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti5, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti5, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti5, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti5, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti5, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Dont Do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ti5, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti5, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ti5> for Ti5T {
    #[inline(always)]
    fn reset_value(&self) -> Ti5 {
        Ti5::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen5 {
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
pub struct TxfrLen5T;
unsafe impl crate::common::AsPtr for TxfrLen5T {}
impl crate::common::Reg<TxfrLen5> for TxfrLen5T {}

unsafe impl crate::common::Read<TxfrLen5> for TxfrLen5T {}
unsafe impl crate::common::Write<TxfrLen5> for TxfrLen5T {}
impl TxfrLen5 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen5, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen5,common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers\n                            are performed. When in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylength(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen5, common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen5,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen5> for TxfrLen5T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen5 {
        TxfrLen5::new(0)
    }
}

#[doc = "DMA 2D Stride"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stride5 {
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
pub struct Stride5T;
unsafe impl crate::common::AsPtr for Stride5T {}
impl crate::common::Reg<Stride5> for Stride5T {}

unsafe impl crate::common::Read<Stride5> for Stride5T {}
unsafe impl crate::common::Write<Stride5> for Stride5T {}
impl Stride5 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride5, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride5,common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride5, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride5,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Stride5> for Stride5T {
    #[inline(always)]
    fn reset_value(&self) -> Stride5 {
        Stride5::new(0)
    }
}

#[doc = "DMA Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug5 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug5 {
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
pub struct Debug5T;
unsafe impl crate::common::AsPtr for Debug5T {}
impl crate::common::Reg<Debug5> for Debug5T {}

unsafe impl crate::common::Read<Debug5> for Debug5T {}
unsafe impl crate::common::Write<Debug5> for Debug5T {}
impl Debug5 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug5, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug5, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug5, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug5, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug5, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug5, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug5, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug5, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug5, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug5, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug5, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug5, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug5, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug5, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug5, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug5, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug5> for Debug5T {
    #[inline(always)]
    fn reset_value(&self) -> Debug5 {
        Debug5::new(67108864)
    }
}

#[doc = "DMA 6 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs6 {
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
pub struct Cs6T;
unsafe impl crate::common::AsPtr for Cs6T {}
impl crate::common::Reg<Cs6> for Cs6T {}

unsafe impl crate::common::Read<Cs6> for Cs6T {}
unsafe impl crate::common::Write<Cs6> for Cs6T {}
impl Cs6 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs6, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs6, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs6, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs6, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs6, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs6, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs6, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs6, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs6, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs6, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs6, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs6, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs6, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs6, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs6, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs6, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs6, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs6, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs6, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs6, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs6, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs6, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs6, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs6, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs6, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs6, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs6, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs6, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs6> for Cs6T {
    #[inline(always)]
    fn reset_value(&self) -> Cs6 {
        Cs6::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd6 {
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
pub struct ConblkAd6T;
unsafe impl crate::common::AsPtr for ConblkAd6T {}
impl crate::common::Reg<ConblkAd6> for ConblkAd6T {}

unsafe impl crate::common::Read<ConblkAd6> for ConblkAd6T {}
unsafe impl crate::common::Write<ConblkAd6> for ConblkAd6T {}
impl ConblkAd6 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd6, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd6,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd6> for ConblkAd6T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd6 {
        ConblkAd6::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd6 {
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
pub struct SourceAd6T;
unsafe impl crate::common::AsPtr for SourceAd6T {}
impl crate::common::Reg<SourceAd6> for SourceAd6T {}

unsafe impl crate::common::Read<SourceAd6> for SourceAd6T {}
unsafe impl crate::common::Write<SourceAd6> for SourceAd6T {}
impl SourceAd6 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd6, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd6,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd6> for SourceAd6T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd6 {
        SourceAd6::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd6 {
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
pub struct DestAd6T;
unsafe impl crate::common::AsPtr for DestAd6T {}
impl crate::common::Reg<DestAd6> for DestAd6T {}

unsafe impl crate::common::Read<DestAd6> for DestAd6T {}
unsafe impl crate::common::Write<DestAd6> for DestAd6T {}
impl DestAd6 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd6, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd6,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd6> for DestAd6T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd6 {
        DestAd6::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk6 {
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
pub struct Nextconbk6T;
unsafe impl crate::common::AsPtr for Nextconbk6T {}
impl crate::common::Reg<Nextconbk6> for Nextconbk6T {}

unsafe impl crate::common::Read<Nextconbk6> for Nextconbk6T {}
unsafe impl crate::common::Write<Nextconbk6> for Nextconbk6T {}
impl Nextconbk6 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk6, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk6,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk6> for Nextconbk6T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk6 {
        Nextconbk6::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti6 {
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
pub struct Ti6T;
unsafe impl crate::common::AsPtr for Ti6T {}
impl crate::common::Reg<Ti6> for Ti6T {}

unsafe impl crate::common::Read<Ti6> for Ti6T {}
unsafe impl crate::common::Write<Ti6> for Ti6T {}
impl Ti6 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "2D Mode"]
    #[inline(always)]
    pub fn tdmode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti6, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti6, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti6, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti6, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti6, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti6, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Dont Do wide writes as a 2 beat burst"]
    #[inline(always)]
    pub fn no_wide_bursts(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ti6, common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ti6, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Ti6> for Ti6T {
    #[inline(always)]
    fn reset_value(&self) -> Ti6 {
        Ti6::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen6 {
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
pub struct TxfrLen6T;
unsafe impl crate::common::AsPtr for TxfrLen6T {}
impl crate::common::Reg<TxfrLen6> for TxfrLen6T {}

unsafe impl crate::common::Read<TxfrLen6> for TxfrLen6T {}
unsafe impl crate::common::Write<TxfrLen6> for TxfrLen6T {}
impl TxfrLen6 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen6, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen6,common::RW>::from_register(self,0)
    }

    #[doc = "When in 2D mode, This is the Y transfer length, indicating how many xlength\n                            transfers\n                            are performed. When in normal linear mode this becomes the top bits of the XLENGTH"]
    #[inline(always)]
    pub fn ylength(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, TxfrLen6, common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,TxfrLen6,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen6> for TxfrLen6T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen6 {
        TxfrLen6::new(0)
    }
}

#[doc = "DMA 2D Stride"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stride6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Stride6 {
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
pub struct Stride6T;
unsafe impl crate::common::AsPtr for Stride6T {}
impl crate::common::Reg<Stride6> for Stride6T {}

unsafe impl crate::common::Read<Stride6> for Stride6T {}
unsafe impl crate::common::Write<Stride6> for Stride6T {}
impl Stride6 {
    #[doc = "Source Stride (2D Mode)"]
    #[inline(always)]
    pub fn s_stride(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Stride6, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Stride6,common::RW>::from_register(self,0)
    }

    #[doc = "Destination Stride (2D Mode)"]
    #[inline(always)]
    pub fn d_stride(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Stride6, common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Stride6,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Stride6> for Stride6T {
    #[inline(always)]
    fn reset_value(&self) -> Stride6 {
        Stride6::new(0)
    }
}

#[doc = "DMA Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug6 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug6 {
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
pub struct Debug6T;
unsafe impl crate::common::AsPtr for Debug6T {}
impl crate::common::Reg<Debug6> for Debug6T {}

unsafe impl crate::common::Read<Debug6> for Debug6T {}
unsafe impl crate::common::Write<Debug6> for Debug6T {}
impl Debug6 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug6, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug6, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug6, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug6, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug6, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug6, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug6, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug6, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug6, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug6, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug6, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug6, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug6, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug6, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug6, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug6, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug6> for Debug6T {
    #[inline(always)]
    fn reset_value(&self) -> Debug6 {
        Debug6::new(67108864)
    }
}

#[doc = "DMA 7 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs7 {
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
pub struct Cs7T;
unsafe impl crate::common::AsPtr for Cs7T {}
impl crate::common::Reg<Cs7> for Cs7T {}

unsafe impl crate::common::Read<Cs7> for Cs7T {}
unsafe impl crate::common::Write<Cs7> for Cs7T {}
impl Cs7 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs7, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs7, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs7, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs7, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs7, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs7, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs7, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs7, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs7, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs7, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs7, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs7, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs7, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs7, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs7, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs7, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs7, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs7, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs7, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs7, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs7, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs7, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs7, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs7, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs7, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs7, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs7, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs7, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs7> for Cs7T {
    #[inline(always)]
    fn reset_value(&self) -> Cs7 {
        Cs7::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd7 {
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
pub struct ConblkAd7T;
unsafe impl crate::common::AsPtr for ConblkAd7T {}
impl crate::common::Reg<ConblkAd7> for ConblkAd7T {}

unsafe impl crate::common::Read<ConblkAd7> for ConblkAd7T {}
unsafe impl crate::common::Write<ConblkAd7> for ConblkAd7T {}
impl ConblkAd7 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd7, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd7,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd7> for ConblkAd7T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd7 {
        ConblkAd7::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd7 {
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
pub struct SourceAd7T;
unsafe impl crate::common::AsPtr for SourceAd7T {}
impl crate::common::Reg<SourceAd7> for SourceAd7T {}

unsafe impl crate::common::Read<SourceAd7> for SourceAd7T {}
unsafe impl crate::common::Write<SourceAd7> for SourceAd7T {}
impl SourceAd7 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd7, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd7,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd7> for SourceAd7T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd7 {
        SourceAd7::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd7 {
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
pub struct DestAd7T;
unsafe impl crate::common::AsPtr for DestAd7T {}
impl crate::common::Reg<DestAd7> for DestAd7T {}

unsafe impl crate::common::Read<DestAd7> for DestAd7T {}
unsafe impl crate::common::Write<DestAd7> for DestAd7T {}
impl DestAd7 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd7, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd7,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd7> for DestAd7T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd7 {
        DestAd7::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk7 {
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
pub struct Nextconbk7T;
unsafe impl crate::common::AsPtr for Nextconbk7T {}
impl crate::common::Reg<Nextconbk7> for Nextconbk7T {}

unsafe impl crate::common::Read<Nextconbk7> for Nextconbk7T {}
unsafe impl crate::common::Write<Nextconbk7> for Nextconbk7T {}
impl Nextconbk7 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk7, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk7,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk7> for Nextconbk7T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk7 {
        Nextconbk7::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti7 {
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
pub struct Ti7T;
unsafe impl crate::common::AsPtr for Ti7T {}
impl crate::common::Reg<Ti7> for Ti7T {}

unsafe impl crate::common::Read<Ti7> for Ti7T {}
unsafe impl crate::common::Write<Ti7> for Ti7T {}
impl Ti7 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti7, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti7, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti7, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti7, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti7, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti7, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti7, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti7, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti7> for Ti7T {
    #[inline(always)]
    fn reset_value(&self) -> Ti7 {
        Ti7::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen7 {
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
pub struct TxfrLen7T;
unsafe impl crate::common::AsPtr for TxfrLen7T {}
impl crate::common::Reg<TxfrLen7> for TxfrLen7T {}

unsafe impl crate::common::Read<TxfrLen7> for TxfrLen7T {}
unsafe impl crate::common::Write<TxfrLen7> for TxfrLen7T {}
impl TxfrLen7 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen7, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen7,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen7> for TxfrLen7T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen7 {
        TxfrLen7::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug7 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug7 {
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
pub struct Debug7T;
unsafe impl crate::common::AsPtr for Debug7T {}
impl crate::common::Reg<Debug7> for Debug7T {}

unsafe impl crate::common::Read<Debug7> for Debug7T {}
unsafe impl crate::common::Write<Debug7> for Debug7T {}
impl Debug7 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug7, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug7, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug7, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug7, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug7, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug7, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug7, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug7, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug7, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug7, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug7, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug7, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug7, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug7, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug7, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug7, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug7> for Debug7T {
    #[inline(always)]
    fn reset_value(&self) -> Debug7 {
        Debug7::new(335544320)
    }
}

#[doc = "DMA 8 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs8 {
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
pub struct Cs8T;
unsafe impl crate::common::AsPtr for Cs8T {}
impl crate::common::Reg<Cs8> for Cs8T {}

unsafe impl crate::common::Read<Cs8> for Cs8T {}
unsafe impl crate::common::Write<Cs8> for Cs8T {}
impl Cs8 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs8, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs8, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs8, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs8, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs8, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs8, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs8, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs8, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs8, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs8, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs8, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs8, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs8, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs8, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs8, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs8, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs8, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs8, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs8, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs8, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs8, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs8, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs8, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs8, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs8, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs8, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs8, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs8, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs8> for Cs8T {
    #[inline(always)]
    fn reset_value(&self) -> Cs8 {
        Cs8::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd8 {
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
pub struct ConblkAd8T;
unsafe impl crate::common::AsPtr for ConblkAd8T {}
impl crate::common::Reg<ConblkAd8> for ConblkAd8T {}

unsafe impl crate::common::Read<ConblkAd8> for ConblkAd8T {}
unsafe impl crate::common::Write<ConblkAd8> for ConblkAd8T {}
impl ConblkAd8 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd8, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd8,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd8> for ConblkAd8T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd8 {
        ConblkAd8::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd8 {
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
pub struct SourceAd8T;
unsafe impl crate::common::AsPtr for SourceAd8T {}
impl crate::common::Reg<SourceAd8> for SourceAd8T {}

unsafe impl crate::common::Read<SourceAd8> for SourceAd8T {}
unsafe impl crate::common::Write<SourceAd8> for SourceAd8T {}
impl SourceAd8 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd8, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd8,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd8> for SourceAd8T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd8 {
        SourceAd8::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd8 {
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
pub struct DestAd8T;
unsafe impl crate::common::AsPtr for DestAd8T {}
impl crate::common::Reg<DestAd8> for DestAd8T {}

unsafe impl crate::common::Read<DestAd8> for DestAd8T {}
unsafe impl crate::common::Write<DestAd8> for DestAd8T {}
impl DestAd8 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd8, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd8,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd8> for DestAd8T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd8 {
        DestAd8::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk8 {
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
pub struct Nextconbk8T;
unsafe impl crate::common::AsPtr for Nextconbk8T {}
impl crate::common::Reg<Nextconbk8> for Nextconbk8T {}

unsafe impl crate::common::Read<Nextconbk8> for Nextconbk8T {}
unsafe impl crate::common::Write<Nextconbk8> for Nextconbk8T {}
impl Nextconbk8 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk8, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk8,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk8> for Nextconbk8T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk8 {
        Nextconbk8::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti8 {
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
pub struct Ti8T;
unsafe impl crate::common::AsPtr for Ti8T {}
impl crate::common::Reg<Ti8> for Ti8T {}

unsafe impl crate::common::Read<Ti8> for Ti8T {}
unsafe impl crate::common::Write<Ti8> for Ti8T {}
impl Ti8 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti8, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti8, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti8, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti8, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti8, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti8, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti8, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti8, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti8> for Ti8T {
    #[inline(always)]
    fn reset_value(&self) -> Ti8 {
        Ti8::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen8 {
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
pub struct TxfrLen8T;
unsafe impl crate::common::AsPtr for TxfrLen8T {}
impl crate::common::Reg<TxfrLen8> for TxfrLen8T {}

unsafe impl crate::common::Read<TxfrLen8> for TxfrLen8T {}
unsafe impl crate::common::Write<TxfrLen8> for TxfrLen8T {}
impl TxfrLen8 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen8, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen8,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen8> for TxfrLen8T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen8 {
        TxfrLen8::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug8 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug8 {
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
pub struct Debug8T;
unsafe impl crate::common::AsPtr for Debug8T {}
impl crate::common::Reg<Debug8> for Debug8T {}

unsafe impl crate::common::Read<Debug8> for Debug8T {}
unsafe impl crate::common::Write<Debug8> for Debug8T {}
impl Debug8 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug8, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug8, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug8, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug8, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug8, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug8, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug8, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug8, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug8, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug8, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug8, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug8, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug8, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug8, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug8, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug8, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug8> for Debug8T {
    #[inline(always)]
    fn reset_value(&self) -> Debug8 {
        Debug8::new(335544320)
    }
}

#[doc = "DMA 9 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs9 {
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
pub struct Cs9T;
unsafe impl crate::common::AsPtr for Cs9T {}
impl crate::common::Reg<Cs9> for Cs9T {}

unsafe impl crate::common::Read<Cs9> for Cs9T {}
unsafe impl crate::common::Write<Cs9> for Cs9T {}
impl Cs9 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs9, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs9, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs9, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs9, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs9, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs9, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs9, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs9, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs9, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs9, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs9, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs9, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs9, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs9, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs9, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs9, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs9, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs9, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs9, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs9, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs9, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs9, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs9, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs9, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs9, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs9, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs9, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs9, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs9> for Cs9T {
    #[inline(always)]
    fn reset_value(&self) -> Cs9 {
        Cs9::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd9 {
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
pub struct ConblkAd9T;
unsafe impl crate::common::AsPtr for ConblkAd9T {}
impl crate::common::Reg<ConblkAd9> for ConblkAd9T {}

unsafe impl crate::common::Read<ConblkAd9> for ConblkAd9T {}
unsafe impl crate::common::Write<ConblkAd9> for ConblkAd9T {}
impl ConblkAd9 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd9, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd9,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd9> for ConblkAd9T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd9 {
        ConblkAd9::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd9 {
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
pub struct SourceAd9T;
unsafe impl crate::common::AsPtr for SourceAd9T {}
impl crate::common::Reg<SourceAd9> for SourceAd9T {}

unsafe impl crate::common::Read<SourceAd9> for SourceAd9T {}
unsafe impl crate::common::Write<SourceAd9> for SourceAd9T {}
impl SourceAd9 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd9, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd9,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd9> for SourceAd9T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd9 {
        SourceAd9::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd9 {
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
pub struct DestAd9T;
unsafe impl crate::common::AsPtr for DestAd9T {}
impl crate::common::Reg<DestAd9> for DestAd9T {}

unsafe impl crate::common::Read<DestAd9> for DestAd9T {}
unsafe impl crate::common::Write<DestAd9> for DestAd9T {}
impl DestAd9 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd9, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd9,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd9> for DestAd9T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd9 {
        DestAd9::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk9 {
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
pub struct Nextconbk9T;
unsafe impl crate::common::AsPtr for Nextconbk9T {}
impl crate::common::Reg<Nextconbk9> for Nextconbk9T {}

unsafe impl crate::common::Read<Nextconbk9> for Nextconbk9T {}
unsafe impl crate::common::Write<Nextconbk9> for Nextconbk9T {}
impl Nextconbk9 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk9, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk9,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk9> for Nextconbk9T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk9 {
        Nextconbk9::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti9 {
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
pub struct Ti9T;
unsafe impl crate::common::AsPtr for Ti9T {}
impl crate::common::Reg<Ti9> for Ti9T {}

unsafe impl crate::common::Read<Ti9> for Ti9T {}
unsafe impl crate::common::Write<Ti9> for Ti9T {}
impl Ti9 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti9, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti9, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti9, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti9, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti9, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti9, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti9, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti9, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti9> for Ti9T {
    #[inline(always)]
    fn reset_value(&self) -> Ti9 {
        Ti9::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen9 {
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
pub struct TxfrLen9T;
unsafe impl crate::common::AsPtr for TxfrLen9T {}
impl crate::common::Reg<TxfrLen9> for TxfrLen9T {}

unsafe impl crate::common::Read<TxfrLen9> for TxfrLen9T {}
unsafe impl crate::common::Write<TxfrLen9> for TxfrLen9T {}
impl TxfrLen9 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen9, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen9,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen9> for TxfrLen9T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen9 {
        TxfrLen9::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug9 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug9 {
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
pub struct Debug9T;
unsafe impl crate::common::AsPtr for Debug9T {}
impl crate::common::Reg<Debug9> for Debug9T {}

unsafe impl crate::common::Read<Debug9> for Debug9T {}
unsafe impl crate::common::Write<Debug9> for Debug9T {}
impl Debug9 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug9, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug9, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug9, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug9, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug9, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug9, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug9, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug9, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug9, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug9, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug9, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug9, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(self) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug9, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug9, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug9, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug9, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug9> for Debug9T {
    #[inline(always)]
    fn reset_value(&self) -> Debug9 {
        Debug9::new(335544320)
    }
}

#[doc = "DMA 10 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs10 {
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
pub struct Cs10T;
unsafe impl crate::common::AsPtr for Cs10T {}
impl crate::common::Reg<Cs10> for Cs10T {}

unsafe impl crate::common::Read<Cs10> for Cs10T {}
unsafe impl crate::common::Write<Cs10> for Cs10T {}
impl Cs10 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs10, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs10, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs10, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs10, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs10, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs10, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs10, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs10, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs10, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs10, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs10, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs10, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs10, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs10, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs10, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs10, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs10, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs10, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs10, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs10, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs10, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs10, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs10, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs10, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs10, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs10, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs10, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs10, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs10> for Cs10T {
    #[inline(always)]
    fn reset_value(&self) -> Cs10 {
        Cs10::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd10 {
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
pub struct ConblkAd10T;
unsafe impl crate::common::AsPtr for ConblkAd10T {}
impl crate::common::Reg<ConblkAd10> for ConblkAd10T {}

unsafe impl crate::common::Read<ConblkAd10> for ConblkAd10T {}
unsafe impl crate::common::Write<ConblkAd10> for ConblkAd10T {}
impl ConblkAd10 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd10, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd10,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd10> for ConblkAd10T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd10 {
        ConblkAd10::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd10 {
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
pub struct SourceAd10T;
unsafe impl crate::common::AsPtr for SourceAd10T {}
impl crate::common::Reg<SourceAd10> for SourceAd10T {}

unsafe impl crate::common::Read<SourceAd10> for SourceAd10T {}
unsafe impl crate::common::Write<SourceAd10> for SourceAd10T {}
impl SourceAd10 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd10, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd10,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd10> for SourceAd10T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd10 {
        SourceAd10::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd10 {
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
pub struct DestAd10T;
unsafe impl crate::common::AsPtr for DestAd10T {}
impl crate::common::Reg<DestAd10> for DestAd10T {}

unsafe impl crate::common::Read<DestAd10> for DestAd10T {}
unsafe impl crate::common::Write<DestAd10> for DestAd10T {}
impl DestAd10 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd10, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd10,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd10> for DestAd10T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd10 {
        DestAd10::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk10 {
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
pub struct Nextconbk10T;
unsafe impl crate::common::AsPtr for Nextconbk10T {}
impl crate::common::Reg<Nextconbk10> for Nextconbk10T {}

unsafe impl crate::common::Read<Nextconbk10> for Nextconbk10T {}
unsafe impl crate::common::Write<Nextconbk10> for Nextconbk10T {}
impl Nextconbk10 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk10, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk10,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk10> for Nextconbk10T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk10 {
        Nextconbk10::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti10 {
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
pub struct Ti10T;
unsafe impl crate::common::AsPtr for Ti10T {}
impl crate::common::Reg<Ti10> for Ti10T {}

unsafe impl crate::common::Read<Ti10> for Ti10T {}
unsafe impl crate::common::Write<Ti10> for Ti10T {}
impl Ti10 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti10, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti10, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti10, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti10, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti10, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti10, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti10, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti10, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti10> for Ti10T {
    #[inline(always)]
    fn reset_value(&self) -> Ti10 {
        Ti10::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen10 {
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
pub struct TxfrLen10T;
unsafe impl crate::common::AsPtr for TxfrLen10T {}
impl crate::common::Reg<TxfrLen10> for TxfrLen10T {}

unsafe impl crate::common::Read<TxfrLen10> for TxfrLen10T {}
unsafe impl crate::common::Write<TxfrLen10> for TxfrLen10T {}
impl TxfrLen10 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen10, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen10,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen10> for TxfrLen10T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen10 {
        TxfrLen10::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug10 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug10 {
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
pub struct Debug10T;
unsafe impl crate::common::AsPtr for Debug10T {}
impl crate::common::Reg<Debug10> for Debug10T {}

unsafe impl crate::common::Read<Debug10> for Debug10T {}
unsafe impl crate::common::Write<Debug10> for Debug10T {}
impl Debug10 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug10, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug10, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug10, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug10, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug10, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug10, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug10, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug10, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug10, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug10, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug10, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug10, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug10, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug10, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug10, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug10, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug10> for Debug10T {
    #[inline(always)]
    fn reset_value(&self) -> Debug10 {
        Debug10::new(335544320)
    }
}

#[doc = "DMA 11 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs11 {
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
pub struct Cs11T;
unsafe impl crate::common::AsPtr for Cs11T {}
impl crate::common::Reg<Cs11> for Cs11T {}

unsafe impl crate::common::Read<Cs11> for Cs11T {}
unsafe impl crate::common::Write<Cs11> for Cs11T {}
impl Cs11 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs11, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs11, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs11, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs11, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs11, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs11, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs11, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs11, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs11, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs11, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs11, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs11, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs11, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs11, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs11, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs11, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs11, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs11, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs11, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs11, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs11, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs11, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs11, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs11, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs11, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs11, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs11, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs11, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs11> for Cs11T {
    #[inline(always)]
    fn reset_value(&self) -> Cs11 {
        Cs11::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd11 {
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
pub struct ConblkAd11T;
unsafe impl crate::common::AsPtr for ConblkAd11T {}
impl crate::common::Reg<ConblkAd11> for ConblkAd11T {}

unsafe impl crate::common::Read<ConblkAd11> for ConblkAd11T {}
unsafe impl crate::common::Write<ConblkAd11> for ConblkAd11T {}
impl ConblkAd11 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd11, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd11,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd11> for ConblkAd11T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd11 {
        ConblkAd11::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd11 {
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
pub struct SourceAd11T;
unsafe impl crate::common::AsPtr for SourceAd11T {}
impl crate::common::Reg<SourceAd11> for SourceAd11T {}

unsafe impl crate::common::Read<SourceAd11> for SourceAd11T {}
unsafe impl crate::common::Write<SourceAd11> for SourceAd11T {}
impl SourceAd11 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd11, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd11,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd11> for SourceAd11T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd11 {
        SourceAd11::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd11 {
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
pub struct DestAd11T;
unsafe impl crate::common::AsPtr for DestAd11T {}
impl crate::common::Reg<DestAd11> for DestAd11T {}

unsafe impl crate::common::Read<DestAd11> for DestAd11T {}
unsafe impl crate::common::Write<DestAd11> for DestAd11T {}
impl DestAd11 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd11, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd11,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd11> for DestAd11T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd11 {
        DestAd11::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk11 {
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
pub struct Nextconbk11T;
unsafe impl crate::common::AsPtr for Nextconbk11T {}
impl crate::common::Reg<Nextconbk11> for Nextconbk11T {}

unsafe impl crate::common::Read<Nextconbk11> for Nextconbk11T {}
unsafe impl crate::common::Write<Nextconbk11> for Nextconbk11T {}
impl Nextconbk11 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk11, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk11,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk11> for Nextconbk11T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk11 {
        Nextconbk11::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti11 {
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
pub struct Ti11T;
unsafe impl crate::common::AsPtr for Ti11T {}
impl crate::common::Reg<Ti11> for Ti11T {}

unsafe impl crate::common::Read<Ti11> for Ti11T {}
unsafe impl crate::common::Write<Ti11> for Ti11T {}
impl Ti11 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti11, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti11, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti11, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti11, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti11, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti11, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti11, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti11, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti11> for Ti11T {
    #[inline(always)]
    fn reset_value(&self) -> Ti11 {
        Ti11::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen11 {
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
pub struct TxfrLen11T;
unsafe impl crate::common::AsPtr for TxfrLen11T {}
impl crate::common::Reg<TxfrLen11> for TxfrLen11T {}

unsafe impl crate::common::Read<TxfrLen11> for TxfrLen11T {}
unsafe impl crate::common::Write<TxfrLen11> for TxfrLen11T {}
impl TxfrLen11 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen11, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen11,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen11> for TxfrLen11T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen11 {
        TxfrLen11::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug11 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug11 {
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
pub struct Debug11T;
unsafe impl crate::common::AsPtr for Debug11T {}
impl crate::common::Reg<Debug11> for Debug11T {}

unsafe impl crate::common::Read<Debug11> for Debug11T {}
unsafe impl crate::common::Write<Debug11> for Debug11T {}
impl Debug11 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug11, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug11, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug11, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug11, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug11, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug11, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug11, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug11, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug11, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug11, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug11, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug11, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug11, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug11, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug11, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug11, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug11> for Debug11T {
    #[inline(always)]
    fn reset_value(&self) -> Debug11 {
        Debug11::new(335544320)
    }
}

#[doc = "DMA 12 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs12 {
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
pub struct Cs12T;
unsafe impl crate::common::AsPtr for Cs12T {}
impl crate::common::Reg<Cs12> for Cs12T {}

unsafe impl crate::common::Read<Cs12> for Cs12T {}
unsafe impl crate::common::Write<Cs12> for Cs12T {}
impl Cs12 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs12, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs12, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs12, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs12, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs12, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs12, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs12, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs12, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs12, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs12, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs12, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs12, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs12, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs12, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs12, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs12, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs12, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs12, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs12, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs12, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs12, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs12, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs12, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs12, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs12, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs12, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs12, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs12, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs12> for Cs12T {
    #[inline(always)]
    fn reset_value(&self) -> Cs12 {
        Cs12::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd12 {
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
pub struct ConblkAd12T;
unsafe impl crate::common::AsPtr for ConblkAd12T {}
impl crate::common::Reg<ConblkAd12> for ConblkAd12T {}

unsafe impl crate::common::Read<ConblkAd12> for ConblkAd12T {}
unsafe impl crate::common::Write<ConblkAd12> for ConblkAd12T {}
impl ConblkAd12 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd12, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd12,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd12> for ConblkAd12T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd12 {
        ConblkAd12::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd12 {
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
pub struct SourceAd12T;
unsafe impl crate::common::AsPtr for SourceAd12T {}
impl crate::common::Reg<SourceAd12> for SourceAd12T {}

unsafe impl crate::common::Read<SourceAd12> for SourceAd12T {}
unsafe impl crate::common::Write<SourceAd12> for SourceAd12T {}
impl SourceAd12 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd12, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd12,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd12> for SourceAd12T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd12 {
        SourceAd12::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd12 {
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
pub struct DestAd12T;
unsafe impl crate::common::AsPtr for DestAd12T {}
impl crate::common::Reg<DestAd12> for DestAd12T {}

unsafe impl crate::common::Read<DestAd12> for DestAd12T {}
unsafe impl crate::common::Write<DestAd12> for DestAd12T {}
impl DestAd12 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd12, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd12,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd12> for DestAd12T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd12 {
        DestAd12::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk12 {
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
pub struct Nextconbk12T;
unsafe impl crate::common::AsPtr for Nextconbk12T {}
impl crate::common::Reg<Nextconbk12> for Nextconbk12T {}

unsafe impl crate::common::Read<Nextconbk12> for Nextconbk12T {}
unsafe impl crate::common::Write<Nextconbk12> for Nextconbk12T {}
impl Nextconbk12 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk12, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk12,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk12> for Nextconbk12T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk12 {
        Nextconbk12::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti12 {
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
pub struct Ti12T;
unsafe impl crate::common::AsPtr for Ti12T {}
impl crate::common::Reg<Ti12> for Ti12T {}

unsafe impl crate::common::Read<Ti12> for Ti12T {}
unsafe impl crate::common::Write<Ti12> for Ti12T {}
impl Ti12 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti12, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti12, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti12, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti12, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti12, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti12, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti12, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti12, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti12> for Ti12T {
    #[inline(always)]
    fn reset_value(&self) -> Ti12 {
        Ti12::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen12 {
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
pub struct TxfrLen12T;
unsafe impl crate::common::AsPtr for TxfrLen12T {}
impl crate::common::Reg<TxfrLen12> for TxfrLen12T {}

unsafe impl crate::common::Read<TxfrLen12> for TxfrLen12T {}
unsafe impl crate::common::Write<TxfrLen12> for TxfrLen12T {}
impl TxfrLen12 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen12, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen12,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen12> for TxfrLen12T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen12 {
        TxfrLen12::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug12 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug12 {
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
pub struct Debug12T;
unsafe impl crate::common::AsPtr for Debug12T {}
impl crate::common::Reg<Debug12> for Debug12T {}

unsafe impl crate::common::Read<Debug12> for Debug12T {}
unsafe impl crate::common::Write<Debug12> for Debug12T {}
impl Debug12 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug12, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug12, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug12, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug12, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug12, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug12, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug12, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug12, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug12, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug12, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug12, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug12, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug12, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug12, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug12, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug12, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug12> for Debug12T {
    #[inline(always)]
    fn reset_value(&self) -> Debug12 {
        Debug12::new(335544320)
    }
}

#[doc = "DMA 13 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs13 {
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
pub struct Cs13T;
unsafe impl crate::common::AsPtr for Cs13T {}
impl crate::common::Reg<Cs13> for Cs13T {}

unsafe impl crate::common::Read<Cs13> for Cs13T {}
unsafe impl crate::common::Write<Cs13> for Cs13T {}
impl Cs13 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs13, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs13, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs13, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs13, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs13, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs13, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs13, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs13, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs13, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs13, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs13, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs13, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs13, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs13, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs13, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs13, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs13, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs13, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs13, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs13, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs13, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs13, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs13, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs13, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs13, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs13, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs13, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs13, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs13> for Cs13T {
    #[inline(always)]
    fn reset_value(&self) -> Cs13 {
        Cs13::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd13 {
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
pub struct ConblkAd13T;
unsafe impl crate::common::AsPtr for ConblkAd13T {}
impl crate::common::Reg<ConblkAd13> for ConblkAd13T {}

unsafe impl crate::common::Read<ConblkAd13> for ConblkAd13T {}
unsafe impl crate::common::Write<ConblkAd13> for ConblkAd13T {}
impl ConblkAd13 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd13, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd13,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd13> for ConblkAd13T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd13 {
        ConblkAd13::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd13 {
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
pub struct SourceAd13T;
unsafe impl crate::common::AsPtr for SourceAd13T {}
impl crate::common::Reg<SourceAd13> for SourceAd13T {}

unsafe impl crate::common::Read<SourceAd13> for SourceAd13T {}
unsafe impl crate::common::Write<SourceAd13> for SourceAd13T {}
impl SourceAd13 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd13, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd13,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd13> for SourceAd13T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd13 {
        SourceAd13::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd13 {
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
pub struct DestAd13T;
unsafe impl crate::common::AsPtr for DestAd13T {}
impl crate::common::Reg<DestAd13> for DestAd13T {}

unsafe impl crate::common::Read<DestAd13> for DestAd13T {}
unsafe impl crate::common::Write<DestAd13> for DestAd13T {}
impl DestAd13 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd13, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd13,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd13> for DestAd13T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd13 {
        DestAd13::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk13 {
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
pub struct Nextconbk13T;
unsafe impl crate::common::AsPtr for Nextconbk13T {}
impl crate::common::Reg<Nextconbk13> for Nextconbk13T {}

unsafe impl crate::common::Read<Nextconbk13> for Nextconbk13T {}
unsafe impl crate::common::Write<Nextconbk13> for Nextconbk13T {}
impl Nextconbk13 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk13, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk13,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk13> for Nextconbk13T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk13 {
        Nextconbk13::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti13 {
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
pub struct Ti13T;
unsafe impl crate::common::AsPtr for Ti13T {}
impl crate::common::Reg<Ti13> for Ti13T {}

unsafe impl crate::common::Read<Ti13> for Ti13T {}
unsafe impl crate::common::Write<Ti13> for Ti13T {}
impl Ti13 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti13, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti13, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti13, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti13, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti13, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti13, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti13, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti13, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti13> for Ti13T {
    #[inline(always)]
    fn reset_value(&self) -> Ti13 {
        Ti13::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen13 {
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
pub struct TxfrLen13T;
unsafe impl crate::common::AsPtr for TxfrLen13T {}
impl crate::common::Reg<TxfrLen13> for TxfrLen13T {}

unsafe impl crate::common::Read<TxfrLen13> for TxfrLen13T {}
unsafe impl crate::common::Write<TxfrLen13> for TxfrLen13T {}
impl TxfrLen13 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen13, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen13,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen13> for TxfrLen13T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen13 {
        TxfrLen13::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug13 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug13 {
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
pub struct Debug13T;
unsafe impl crate::common::AsPtr for Debug13T {}
impl crate::common::Reg<Debug13> for Debug13T {}

unsafe impl crate::common::Read<Debug13> for Debug13T {}
unsafe impl crate::common::Write<Debug13> for Debug13T {}
impl Debug13 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug13, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug13, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug13, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug13, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug13, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug13, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug13, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug13, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug13, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug13, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug13, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug13, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug13, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug13, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug13, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug13, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug13> for Debug13T {
    #[inline(always)]
    fn reset_value(&self) -> Debug13 {
        Debug13::new(335544320)
    }
}

#[doc = "DMA 14 Control And Status register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Cs14 {
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
pub struct Cs14T;
unsafe impl crate::common::AsPtr for Cs14T {}
impl crate::common::Reg<Cs14> for Cs14T {}

unsafe impl crate::common::Read<Cs14> for Cs14T {}
unsafe impl crate::common::Write<Cs14> for Cs14T {}
impl Cs14 {
    #[doc = "Activate the DMA"]
    #[inline(always)]
    pub fn active(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cs14, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cs14, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA End Flag"]
    #[inline(always)]
    pub fn end(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cs14, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cs14, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub fn int(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cs14, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cs14, common::RW>::from_register(self, 0)
    }

    #[doc = "DREQ State"]
    #[inline(always)]
    pub fn dreq(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cs14, common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cs14, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused State"]
    #[inline(always)]
    pub fn paused(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cs14, common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cs14, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Paused by DREQ State"]
    #[inline(always)]
    pub fn dreq_stops_dma(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cs14, common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cs14, common::R>::from_register(self, 0)
    }

    #[doc = "DMA is Waiting for the Last Write to be Received"]
    #[inline(always)]
    pub fn waiting_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cs14, common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cs14, common::R>::from_register(self, 0)
    }

    #[doc = "DMA Error"]
    #[inline(always)]
    pub fn error(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cs14, common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cs14, common::R>::from_register(self, 0)
    }

    #[doc = "AXI Priority Level"]
    #[inline(always)]
    pub fn priority(self) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, Cs14, common::RW> {
        crate::common::RegisterField::<16, 0xf, 1, 0, u8, u8, Cs14, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "AXI Panic Priority Level"]
    #[inline(always)]
    pub fn panic_priority(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, Cs14, common::RW> {
        crate::common::RegisterField::<20, 0xf, 1, 0, u8, u8, Cs14, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Wait for outstanding writes"]
    #[inline(always)]
    pub fn wait_for_outstanding_writes(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Cs14, common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cs14, common::RW>::from_register(self, 0)
    }

    #[doc = "Disable debug pause signal"]
    #[inline(always)]
    pub fn disdebug(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cs14, common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cs14, common::RW>::from_register(self, 0)
    }

    #[doc = "Abort DMA"]
    #[inline(always)]
    pub fn abort(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cs14, common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cs14, common::W>::from_register(self, 0)
    }

    #[doc = "DMA Channel Reset"]
    #[inline(always)]
    pub fn reset(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cs14, common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cs14, common::W>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Cs14> for Cs14T {
    #[inline(always)]
    fn reset_value(&self) -> Cs14 {
        Cs14::new(0)
    }
}

#[doc = "DMA Control Block Address register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConblkAd14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for ConblkAd14 {
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
pub struct ConblkAd14T;
unsafe impl crate::common::AsPtr for ConblkAd14T {}
impl crate::common::Reg<ConblkAd14> for ConblkAd14T {}

unsafe impl crate::common::Read<ConblkAd14> for ConblkAd14T {}
unsafe impl crate::common::Write<ConblkAd14> for ConblkAd14T {}
impl ConblkAd14 {
    #[doc = "Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, ConblkAd14, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,ConblkAd14,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<ConblkAd14> for ConblkAd14T {
    #[inline(always)]
    fn reset_value(&self) -> ConblkAd14 {
        ConblkAd14::new(0)
    }
}

#[doc = "DMA Source Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceAd14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for SourceAd14 {
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
pub struct SourceAd14T;
unsafe impl crate::common::AsPtr for SourceAd14T {}
impl crate::common::Reg<SourceAd14> for SourceAd14T {}

unsafe impl crate::common::Read<SourceAd14> for SourceAd14T {}
unsafe impl crate::common::Write<SourceAd14> for SourceAd14T {}
impl SourceAd14 {
    #[doc = "DMA Source Address"]
    #[inline(always)]
    pub fn s_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, SourceAd14, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,SourceAd14,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<SourceAd14> for SourceAd14T {
    #[inline(always)]
    fn reset_value(&self) -> SourceAd14 {
        SourceAd14::new(0)
    }
}

#[doc = "DMA Destination Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DestAd14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for DestAd14 {
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
pub struct DestAd14T;
unsafe impl crate::common::AsPtr for DestAd14T {}
impl crate::common::Reg<DestAd14> for DestAd14T {}

unsafe impl crate::common::Read<DestAd14> for DestAd14T {}
unsafe impl crate::common::Write<DestAd14> for DestAd14T {}
impl DestAd14 {
    #[doc = "DMA Destination Address"]
    #[inline(always)]
    pub fn d_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, DestAd14, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,DestAd14,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<DestAd14> for DestAd14T {
    #[inline(always)]
    fn reset_value(&self) -> DestAd14 {
        DestAd14::new(0)
    }
}

#[doc = "DMA Next Control Block Address"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nextconbk14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Nextconbk14 {
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
pub struct Nextconbk14T;
unsafe impl crate::common::AsPtr for Nextconbk14T {}
impl crate::common::Reg<Nextconbk14> for Nextconbk14T {}

unsafe impl crate::common::Read<Nextconbk14> for Nextconbk14T {}
unsafe impl crate::common::Write<Nextconbk14> for Nextconbk14T {}
impl Nextconbk14 {
    #[doc = "Address of next CB for chained DMA operations"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Nextconbk14, common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Nextconbk14,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<Nextconbk14> for Nextconbk14T {
    #[inline(always)]
    fn reset_value(&self) -> Nextconbk14 {
        Nextconbk14::new(0)
    }
}

#[doc = "DMA Transfer Information"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ti14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Ti14 {
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
pub struct Ti14T;
unsafe impl crate::common::AsPtr for Ti14T {}
impl crate::common::Reg<Ti14> for Ti14T {}

unsafe impl crate::common::Read<Ti14> for Ti14T {}
unsafe impl crate::common::Write<Ti14> for Ti14T {}
impl Ti14 {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Wait for a Write Response"]
    #[inline(always)]
    pub fn wait_resp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Address Increment"]
    #[inline(always)]
    pub fn dest_inc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Destination Transfer Width"]
    #[inline(always)]
    pub fn dest_width(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Destination Writes with DREQ"]
    #[inline(always)]
    pub fn dest_dreq(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Writes"]
    #[inline(always)]
    pub fn dest_ignore(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Address Increment"]
    #[inline(always)]
    pub fn src_inc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Source Transfer Width"]
    #[inline(always)]
    pub fn src_width(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Control Source Reads with DREQ"]
    #[inline(always)]
    pub fn src_dreq(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Ignore Reads"]
    #[inline(always)]
    pub fn src_ignore(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ti14, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ti14, common::RW>::from_register(self, 0)
    }

    #[doc = "Burst Transfer Length"]
    #[inline(always)]
    pub fn burst_length(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Ti14, common::RW> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, u8, Ti14, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Peripheral Mapping"]
    #[inline(always)]
    pub fn permap(self) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Ti14, common::RW> {
        crate::common::RegisterField::<16, 0x1f, 1, 0, u8, u8, Ti14, common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Add Wait Cycles"]
    #[inline(always)]
    pub fn waits(self) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, u8, Ti14, common::RW> {
        crate::common::RegisterField::<21, 0x1f, 1, 0, u8, u8, Ti14, common::RW>::from_register(
            self, 0,
        )
    }
}
impl crate::common::ResetValue<Ti14> for Ti14T {
    #[inline(always)]
    fn reset_value(&self) -> Ti14 {
        Ti14::new(0)
    }
}

#[doc = "DMA Transfer Length."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxfrLen14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for TxfrLen14 {
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
pub struct TxfrLen14T;
unsafe impl crate::common::AsPtr for TxfrLen14T {}
impl crate::common::Reg<TxfrLen14> for TxfrLen14T {}

unsafe impl crate::common::Read<TxfrLen14> for TxfrLen14T {}
unsafe impl crate::common::Write<TxfrLen14> for TxfrLen14T {}
impl TxfrLen14 {
    #[doc = "Transfer Length in bytes"]
    #[inline(always)]
    pub fn xlength(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, TxfrLen14, common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,TxfrLen14,common::RW>::from_register(self,0)
    }
}
impl crate::common::ResetValue<TxfrLen14> for TxfrLen14T {
    #[inline(always)]
    fn reset_value(&self) -> TxfrLen14 {
        TxfrLen14::new(0)
    }
}

#[doc = "DMA lite Debug register"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug14 {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Debug14 {
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
pub struct Debug14T;
unsafe impl crate::common::AsPtr for Debug14T {}
impl crate::common::Reg<Debug14> for Debug14T {}

unsafe impl crate::common::Read<Debug14> for Debug14T {}
unsafe impl crate::common::Write<Debug14> for Debug14T {}
impl Debug14 {
    #[doc = "Read Last Not Set Error"]
    #[inline(always)]
    pub fn read_last_not_set_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Debug14, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Debug14, common::RW>::from_register(self, 0)
    }

    #[doc = "Fifo Error"]
    #[inline(always)]
    pub fn fifo_error(self) -> crate::common::RegisterFieldBool<1, 1, 0, Debug14, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Debug14, common::RW>::from_register(self, 0)
    }

    #[doc = "Slave Read Response Error"]
    #[inline(always)]
    pub fn read_error(self) -> crate::common::RegisterFieldBool<2, 1, 0, Debug14, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Debug14, common::RW>::from_register(self, 0)
    }

    #[doc = "DMA Outstanding Writes Counter"]
    #[inline(always)]
    pub fn outstanding_writes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, Debug14, common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, u8, Debug14, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA ID"]
    #[inline(always)]
    pub fn dma_id(self) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Debug14, common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, u8, Debug14, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA State Machine State"]
    #[inline(always)]
    pub fn dma_state(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Debug14, common::R> {
        crate::common::RegisterField::<16, 0x1ff, 1, 0, u16, u16, Debug14, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Version"]
    #[inline(always)]
    pub fn version(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, u8, Debug14, common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, u8, Debug14, common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DMA Lite"]
    #[inline(always)]
    pub fn lite(self) -> crate::common::RegisterFieldBool<28, 1, 0, Debug14, common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Debug14, common::R>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Debug14> for Debug14T {
    #[inline(always)]
    fn reset_value(&self) -> Debug14 {
        Debug14::new(335544320)
    }
}

#[doc = "Interrupt status of each DMA engine"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for IntStatus {
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
pub struct IntStatusT;
unsafe impl crate::common::AsPtr for IntStatusT {}
impl crate::common::Reg<IntStatus> for IntStatusT {}

unsafe impl crate::common::Read<IntStatus> for IntStatusT {}
unsafe impl crate::common::Write<IntStatus> for IntStatusT {}
impl IntStatus {
    #[doc = "Interrupt status of DMA engine 0"]
    #[inline(always)]
    pub fn int0(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 1"]
    #[inline(always)]
    pub fn int1(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 2"]
    #[inline(always)]
    pub fn int2(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 3"]
    #[inline(always)]
    pub fn int3(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 4"]
    #[inline(always)]
    pub fn int4(self) -> crate::common::RegisterFieldBool<4, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 5"]
    #[inline(always)]
    pub fn int5(self) -> crate::common::RegisterFieldBool<5, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 6"]
    #[inline(always)]
    pub fn int6(self) -> crate::common::RegisterFieldBool<6, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 7"]
    #[inline(always)]
    pub fn int7(self) -> crate::common::RegisterFieldBool<7, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 8"]
    #[inline(always)]
    pub fn int8(self) -> crate::common::RegisterFieldBool<8, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 9"]
    #[inline(always)]
    pub fn int9(self) -> crate::common::RegisterFieldBool<9, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 10"]
    #[inline(always)]
    pub fn int10(self) -> crate::common::RegisterFieldBool<10, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 11"]
    #[inline(always)]
    pub fn int11(self) -> crate::common::RegisterFieldBool<11, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 12"]
    #[inline(always)]
    pub fn int12(self) -> crate::common::RegisterFieldBool<12, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 13"]
    #[inline(always)]
    pub fn int13(self) -> crate::common::RegisterFieldBool<13, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 14"]
    #[inline(always)]
    pub fn int14(self) -> crate::common::RegisterFieldBool<14, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }

    #[doc = "Interrupt status of DMA engine 15"]
    #[inline(always)]
    pub fn int15(self) -> crate::common::RegisterFieldBool<15, 1, 0, IntStatus, common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, IntStatus, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<IntStatus> for IntStatusT {
    #[inline(always)]
    fn reset_value(&self) -> IntStatus {
        IntStatus::new(0)
    }
}

#[doc = "Global enable bits for each channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable {
    pub(crate) data: u32,
    pub(crate) mask: u32,
}

impl crate::common::RegisterValue for Enable {
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
pub struct EnableT;
unsafe impl crate::common::AsPtr for EnableT {}
impl crate::common::Reg<Enable> for EnableT {}

unsafe impl crate::common::Read<Enable> for EnableT {}
unsafe impl crate::common::Write<Enable> for EnableT {}
impl Enable {
    #[doc = "Enable dma engine 0"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 1"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 2"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 3"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 4"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 5"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 6"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 7"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 8"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 9"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 10"]
    #[inline(always)]
    pub fn en10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 11"]
    #[inline(always)]
    pub fn en11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 12"]
    #[inline(always)]
    pub fn en12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 13"]
    #[inline(always)]
    pub fn en13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Enable, common::RW>::from_register(self, 0)
    }

    #[doc = "Enable dma engine 14"]
    #[inline(always)]
    pub fn en14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Enable, common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Enable, common::RW>::from_register(self, 0)
    }
}
impl crate::common::ResetValue<Enable> for EnableT {
    #[inline(always)]
    fn reset_value(&self) -> Enable {
        Enable::new(32767)
    }
}
