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
// Generated from SVD A, with svd2pac 0.7.0 on Thu, 17 Sep 2026 02:44:17 +0000
#![no_std]
#![allow(non_camel_case_types)]
#![doc = "BCM2711 found in the Raspberry Pi 4"]
pub mod common;
pub use common::*;

#[cfg(feature = "emmc")]
pub mod arasan_emmc_distributor;
#[cfg(feature = "gic_cpu")]
pub mod arm_gic400_cpu;
#[cfg(feature = "gic_dist")]
pub mod arm_gic400_distributor;
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
#[cfg(feature = "pactl")]
pub mod pactl;
#[cfg(feature = "pcm")]
pub mod pcm;
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
    ptr: 0xfe00b880u32 as _,
};
#[cfg(feature = "cm_pcm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmPcm {
    ptr: *mut u8,
}
#[cfg(feature = "cm_pcm")]
pub const CM_PCM: self::CmPcm = self::CmPcm {
    ptr: 0xfe101098u32 as _,
};
#[cfg(feature = "cm_pwm")]
pub const CM_PWM: self::CmPcm = self::CmPcm {
    ptr: 0xfe1010a0u32 as _,
};
#[cfg(feature = "gpio")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
#[cfg(feature = "gpio")]
pub const GPIO: self::Gpio = self::Gpio {
    ptr: 0xfe200000u32 as _,
};
#[cfg(feature = "systmr")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systmr {
    ptr: *mut u8,
}
#[cfg(feature = "systmr")]
pub const SYSTMR: self::Systmr = self::Systmr {
    ptr: 0xfe003000u32 as _,
};
#[cfg(feature = "uart0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArmUartPl011 {
    ptr: *mut u8,
}
#[cfg(feature = "uart0")]
pub const UART0: self::ArmUartPl011 = self::ArmUartPl011 {
    ptr: 0xfe201000u32 as _,
};
#[cfg(feature = "uart2")]
pub const UART2: self::ArmUartPl011 = self::ArmUartPl011 {
    ptr: 0xfe201400u32 as _,
};
#[cfg(feature = "uart3")]
pub const UART3: self::ArmUartPl011 = self::ArmUartPl011 {
    ptr: 0xfe201600u32 as _,
};
#[cfg(feature = "uart4")]
pub const UART4: self::ArmUartPl011 = self::ArmUartPl011 {
    ptr: 0xfe201800u32 as _,
};
#[cfg(feature = "uart5")]
pub const UART5: self::ArmUartPl011 = self::ArmUartPl011 {
    ptr: 0xfe201a00u32 as _,
};
#[cfg(feature = "spi0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0 {
    ptr: *mut u8,
}
#[cfg(feature = "spi0")]
pub const SPI0: self::Spi0 = self::Spi0 {
    ptr: 0xfe204000u32 as _,
};
#[cfg(feature = "spi3")]
pub const SPI3: self::Spi0 = self::Spi0 {
    ptr: 0xfe204600u32 as _,
};
#[cfg(feature = "spi4")]
pub const SPI4: self::Spi0 = self::Spi0 {
    ptr: 0xfe204800u32 as _,
};
#[cfg(feature = "spi5")]
pub const SPI5: self::Spi0 = self::Spi0 {
    ptr: 0xfe204a00u32 as _,
};
#[cfg(feature = "spi6")]
pub const SPI6: self::Spi0 = self::Spi0 {
    ptr: 0xfe204c00u32 as _,
};
#[cfg(feature = "pactl")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pactl {
    ptr: *mut u8,
}
#[cfg(feature = "pactl")]
pub const PACTL: self::Pactl = self::Pactl {
    ptr: 0xfe204e00u32 as _,
};
#[cfg(feature = "bsc0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsc0 {
    ptr: *mut u8,
}
#[cfg(feature = "bsc0")]
pub const BSC0: self::Bsc0 = self::Bsc0 {
    ptr: 0xfe205000u32 as _,
};
#[cfg(feature = "bsc1")]
pub const BSC1: self::Bsc0 = self::Bsc0 {
    ptr: 0xfe804000u32 as _,
};
#[cfg(feature = "bsc3")]
pub const BSC3: self::Bsc0 = self::Bsc0 {
    ptr: 0xfe205600u32 as _,
};
#[cfg(feature = "bsc4")]
pub const BSC4: self::Bsc0 = self::Bsc0 {
    ptr: 0xfe205800u32 as _,
};
#[cfg(feature = "bsc5")]
pub const BSC5: self::Bsc0 = self::Bsc0 {
    ptr: 0xfe205a00u32 as _,
};
#[cfg(feature = "bsc6")]
pub const BSC6: self::Bsc0 = self::Bsc0 {
    ptr: 0xfe205c00u32 as _,
};
#[cfg(feature = "pwm0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0 {
    ptr: *mut u8,
}
#[cfg(feature = "pwm0")]
pub const PWM0: self::Pwm0 = self::Pwm0 {
    ptr: 0xfe20c000u32 as _,
};
#[cfg(feature = "pwm1")]
pub const PWM1: self::Pwm0 = self::Pwm0 {
    ptr: 0xfe20c800u32 as _,
};
#[cfg(feature = "aux")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux {
    ptr: *mut u8,
}
#[cfg(feature = "aux")]
pub const AUX: self::Aux = self::Aux {
    ptr: 0xfe215000u32 as _,
};
#[cfg(feature = "uart1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart1 {
    ptr: *mut u8,
}
#[cfg(feature = "uart1")]
pub const UART1: self::Uart1 = self::Uart1 {
    ptr: 0xfe215040u32 as _,
};
#[cfg(feature = "spi1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi1 {
    ptr: *mut u8,
}
#[cfg(feature = "spi1")]
pub const SPI1: self::Spi1 = self::Spi1 {
    ptr: 0xfe215080u32 as _,
};
#[cfg(feature = "spi2")]
pub const SPI2: self::Spi1 = self::Spi1 {
    ptr: 0xfe2150c0u32 as _,
};
#[cfg(feature = "lic")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BcmLic {
    ptr: *mut u8,
}
#[cfg(feature = "lic")]
pub const LIC: self::BcmLic = self::BcmLic {
    ptr: 0xff800000u32 as _,
};
#[cfg(feature = "gic_dist")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArmGic400Distributor {
    ptr: *mut u8,
}
#[cfg(feature = "gic_dist")]
pub const GIC_DIST: self::ArmGic400Distributor = self::ArmGic400Distributor {
    ptr: 0xff841000u32 as _,
};
#[cfg(feature = "gic_cpu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArmGic400Cpu {
    ptr: *mut u8,
}
#[cfg(feature = "gic_cpu")]
pub const GIC_CPU: self::ArmGic400Cpu = self::ArmGic400Cpu {
    ptr: 0xff842000u32 as _,
};
#[cfg(feature = "usb_otg_global")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgGlobal {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_global")]
pub const USB_OTG_GLOBAL: self::UsbOtgGlobal = self::UsbOtgGlobal {
    ptr: 0xfe980000u32 as _,
};
#[cfg(feature = "usb_otg_host")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgHost {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_host")]
pub const USB_OTG_HOST: self::UsbOtgHost = self::UsbOtgHost {
    ptr: 0xfe980400u32 as _,
};
#[cfg(feature = "usb_otg_device")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgDevice {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_device")]
pub const USB_OTG_DEVICE: self::UsbOtgDevice = self::UsbOtgDevice {
    ptr: 0xfe980800u32 as _,
};
#[cfg(feature = "usb_otg_pwrclk")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgPwrclk {
    ptr: *mut u8,
}
#[cfg(feature = "usb_otg_pwrclk")]
pub const USB_OTG_PWRCLK: self::UsbOtgPwrclk = self::UsbOtgPwrclk {
    ptr: 0xfe980e00u32 as _,
};
#[cfg(feature = "emmc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArasanEmmcDistributor {
    ptr: *mut u8,
}
#[cfg(feature = "emmc")]
pub const EMMC: self::ArasanEmmcDistributor = self::ArasanEmmcDistributor {
    ptr: 0xfe300000u32 as _,
};
#[cfg(feature = "emmc2")]
pub const EMMC2: self::ArasanEmmcDistributor = self::ArasanEmmcDistributor {
    ptr: 0xfe340000u32 as _,
};
#[cfg(feature = "dma")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
#[cfg(feature = "dma")]
pub const DMA: self::Dma = self::Dma {
    ptr: 0xfe007000u32 as _,
};
#[cfg(feature = "pcm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcm {
    ptr: *mut u8,
}
#[cfg(feature = "pcm")]
pub const PCM: self::Pcm = self::Pcm {
    ptr: 0xfe203000u32 as _,
};
