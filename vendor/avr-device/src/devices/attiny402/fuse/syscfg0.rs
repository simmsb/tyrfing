#[doc = "Register `SYSCFG0` reader"]
pub type R = crate::R<SYSCFG0_SPEC>;
#[doc = "Field `EESAVE` reader - EEPROM Save"]
pub type EESAVE_R = crate::BitReader;
#[doc = "Field `RSTPINCFG` reader - Reset Pin Configuration"]
pub type RSTPINCFG_R = crate::FieldReader<RSTPINCFG_A>;
#[doc = "Reset Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTPINCFG_A {
    #[doc = "0: GPIO mode"]
    GPIO = 0,
    #[doc = "1: UPDI mode"]
    UPDI = 1,
    #[doc = "2: Reset mode"]
    RST = 2,
}
impl From<RSTPINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTPINCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSTPINCFG_A {
    type Ux = u8;
}
impl RSTPINCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSTPINCFG_A> {
        match self.bits {
            0 => Some(RSTPINCFG_A::GPIO),
            1 => Some(RSTPINCFG_A::UPDI),
            2 => Some(RSTPINCFG_A::RST),
            _ => None,
        }
    }
    #[doc = "GPIO mode"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == RSTPINCFG_A::GPIO
    }
    #[doc = "UPDI mode"]
    #[inline(always)]
    pub fn is_updi(&self) -> bool {
        *self == RSTPINCFG_A::UPDI
    }
    #[doc = "Reset mode"]
    #[inline(always)]
    pub fn is_rst(&self) -> bool {
        *self == RSTPINCFG_A::RST
    }
}
#[doc = "Field `CRCSRC` reader - CRC Source"]
pub type CRCSRC_R = crate::FieldReader<CRCSRC_A>;
#[doc = "CRC Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRCSRC_A {
    #[doc = "0: The CRC is performed on the entire Flash (boot, application code and application data section)."]
    FLASH = 0,
    #[doc = "1: The CRC is performed on the boot section of Flash"]
    BOOT = 1,
    #[doc = "2: The CRC is performed on the boot and application code section of Flash"]
    BOOTAPP = 2,
    #[doc = "3: Disable CRC."]
    NOCRC = 3,
}
impl From<CRCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRCSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRCSRC_A {
    type Ux = u8;
}
impl CRCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCSRC_A {
        match self.bits {
            0 => CRCSRC_A::FLASH,
            1 => CRCSRC_A::BOOT,
            2 => CRCSRC_A::BOOTAPP,
            3 => CRCSRC_A::NOCRC,
            _ => unreachable!(),
        }
    }
    #[doc = "The CRC is performed on the entire Flash (boot, application code and application data section)."]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == CRCSRC_A::FLASH
    }
    #[doc = "The CRC is performed on the boot section of Flash"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == CRCSRC_A::BOOT
    }
    #[doc = "The CRC is performed on the boot and application code section of Flash"]
    #[inline(always)]
    pub fn is_bootapp(&self) -> bool {
        *self == CRCSRC_A::BOOTAPP
    }
    #[doc = "Disable CRC."]
    #[inline(always)]
    pub fn is_nocrc(&self) -> bool {
        *self == CRCSRC_A::NOCRC
    }
}
impl R {
    #[doc = "Bit 0 - EEPROM Save"]
    #[inline(always)]
    pub fn eesave(&self) -> EESAVE_R {
        EESAVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reset Pin Configuration"]
    #[inline(always)]
    pub fn rstpincfg(&self) -> RSTPINCFG_R {
        RSTPINCFG_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 6:7 - CRC Source"]
    #[inline(always)]
    pub fn crcsrc(&self) -> CRCSRC_R {
        CRCSRC_R::new((self.bits >> 6) & 3)
    }
}
#[doc = "System Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG0_SPEC;
impl crate::RegisterSpec for SYSCFG0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`syscfg0::R`](R) reader structure"]
impl crate::Readable for SYSCFG0_SPEC {}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for SYSCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
