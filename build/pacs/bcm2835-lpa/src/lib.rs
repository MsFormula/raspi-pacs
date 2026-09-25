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
#![no_std]
#![allow(non_camel_case_types)]
#![doc = "BCM2835 found in the Raspberry Pi 1 and Zero"]
#[doc(hidden)]
pub mod common;
#[doc(hidden)]
pub use crate::common::{
    AsPtr as _, Modify as _, NoBitfieldReg as _, Read as _, Reg as _, RegisterValue as _,
    ResetValue as _, Write as _,
};

#[cfg(feature = "emmc")]
pub mod arasan_emmc_distributor;
#[cfg(feature = "uart0")]
pub mod arm_uart_pl011;
#[cfg(feature = "aux")]
pub mod aux;
#[cfg(feature = "lic")]
pub mod bcm_lic;
#[cfg(feature = "bsc0")]
pub mod bsc0;
#[cfg(feature = "cm_pcm")]
pub mod cm_pcm;
#[cfg(feature = "dma")]
pub mod dma;
#[cfg(feature = "gpio")]
pub mod gpio;
#[cfg(feature = "pcm")]
pub mod pcm;
#[cfg(feature = "pm")]
pub mod pm;
#[cfg(feature = "pwm0")]
pub mod pwm0;
#[cfg(feature = "spi0")]
pub mod spi0;
#[cfg(feature = "spi1")]
pub mod spi1;
#[cfg(feature = "systmr")]
pub mod systmr;
#[cfg(feature = "uart1")]
pub mod uart1;
#[cfg(feature = "usb_otg_device")]
pub mod usb_otg_device;
#[cfg(feature = "usb_otg_global")]
pub mod usb_otg_global;
#[cfg(feature = "usb_otg_host")]
pub mod usb_otg_host;
#[cfg(feature = "usb_otg_pwrclk")]
pub mod usb_otg_pwrclk;
#[cfg(feature = "vcmailbox")]
pub mod vcmailbox;

#[cfg(feature = "vcmailbox")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vcmailbox {
    ptr: *mut u8,
}
#[cfg(feature = "vcmailbox")]
pub const VCMAILBOX: self::Vcmailbox = self::Vcmailbox {
    ptr: 0x2000b880u32 as _,
};

#[cfg(feature = "pm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm {
    ptr: *mut u8,
}
#[cfg(feature = "pm")]
pub const PM: self::Pm = self::Pm {
    ptr: 0x20100000u32 as _,
};

#[cfg(feature = "cm_pcm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmPcm {
    ptr: *mut u8,
}
#[cfg(feature = "cm_pcm")]
pub const CM_PCM: self::CmPcm = self::CmPcm {
    ptr: 0x20101098u32 as _,
};

#[cfg(feature = "cm_pwm")]
pub const CM_PWM: self::CmPcm = self::CmPcm {
    ptr: 0x201010a0u32 as _,
};

#[cfg(feature = "gpio")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
#[cfg(feature = "gpio")]
pub const GPIO: self::Gpio = self::Gpio {
    ptr: 0x20200000u32 as _,
};

#[cfg(feature = "systmr")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systmr {
    ptr: *mut u8,
}
#[cfg(feature = "systmr")]
pub const SYSTMR: self::Systmr = self::Systmr {
    ptr: 0x20003000u32 as _,
};

#[cfg(feature = "uart0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArmUartPl011 {
    ptr: *mut u8,
}
#[cfg(feature = "uart0")]
pub const UART0: self::ArmUartPl011 = self::ArmUartPl011 {
    ptr: 0x20201000u32 as _,
};

#[cfg(feature = "spi0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0 {
    ptr: *mut u8,
}
#[cfg(feature = "spi0")]
pub const SPI0: self::Spi0 = self::Spi0 {
    ptr: 0x20204000u32 as _,
};

#[cfg(feature = "bsc0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsc0 {
    ptr: *mut u8,
}
#[cfg(feature = "bsc0")]
pub const BSC0: self::Bsc0 = self::Bsc0 {
    ptr: 0x20205000u32 as _,
};

#[cfg(feature = "pwm0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0 {
    ptr: *mut u8,
}
#[cfg(feature = "pwm0")]
pub const PWM0: self::Pwm0 = self::Pwm0 {
    ptr: 0x2020c000u32 as _,
};

#[cfg(feature = "bsc1")]
pub const BSC1: self::Bsc0 = self::Bsc0 {
    ptr: 0x20804000u32 as _,
};

#[cfg(feature = "bsc2")]
pub const BSC2: self::Bsc0 = self::Bsc0 {
    ptr: 0x20805000u32 as _,
};

#[cfg(feature = "aux")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux {
    ptr: *mut u8,
}
#[cfg(feature = "aux")]
pub const AUX: self::Aux = self::Aux {
    ptr: 0x20215000u32 as _,
};

#[cfg(feature = "uart1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart1 {
    ptr: *mut u8,
}
#[cfg(feature = "uart1")]
pub const UART1: self::Uart1 = self::Uart1 {
    ptr: 0x20215040u32 as _,
};

#[cfg(feature = "spi1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi1 {
    ptr: *mut u8,
}
#[cfg(feature = "spi1")]
pub const SPI1: self::Spi1 = self::Spi1 {
    ptr: 0x20215080u32 as _,
};

#[cfg(feature = "spi2")]
pub const SPI2: self::Spi1 = self::Spi1 {
    ptr: 0x202150c0u32 as _,
};

#[cfg(feature = "lic")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BcmLic {
    ptr: *mut u8,
}
#[cfg(feature = "lic")]
pub const LIC: self::BcmLic = self::BcmLic {
    ptr: 0x2000b000u32 as _,
};

#[cfg(feature = "usb_otg_global")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgGlobal {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_global")]
pub const USB_OTG_GLOBAL: self::UsbOtgGlobal = self::UsbOtgGlobal {
    ptr: 0x20980000u32 as _,
};

#[cfg(feature = "usb_otg_host")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgHost {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_host")]
pub const USB_OTG_HOST: self::UsbOtgHost = self::UsbOtgHost {
    ptr: 0x20980400u32 as _,
};

#[cfg(feature = "usb_otg_device")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgDevice {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_device")]
pub const USB_OTG_DEVICE: self::UsbOtgDevice = self::UsbOtgDevice {
    ptr: 0x20980800u32 as _,
};

#[cfg(feature = "usb_otg_pwrclk")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgPwrclk {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_pwrclk")]
pub const USB_OTG_PWRCLK: self::UsbOtgPwrclk = self::UsbOtgPwrclk {
    ptr: 0x20980e00u32 as _,
};

#[cfg(feature = "emmc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArasanEmmcDistributor {
    ptr: *mut u8,
}
#[cfg(feature = "emmc")]
pub const EMMC: self::ArasanEmmcDistributor = self::ArasanEmmcDistributor {
    ptr: 0x20300000u32 as _,
};

#[cfg(feature = "dma")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
#[cfg(feature = "dma")]
pub const DMA: self::Dma = self::Dma {
    ptr: 0x20007000u32 as _,
};

#[cfg(feature = "pcm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcm {
    ptr: *mut u8,
}
#[cfg(feature = "pcm")]
pub const PCM: self::Pcm = self::Pcm {
    ptr: 0x20203000u32 as _,
};
