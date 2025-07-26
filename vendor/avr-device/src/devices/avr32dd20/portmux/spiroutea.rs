#[doc = "Register `SPIROUTEA` reader"]
pub type R = crate::R<SPIROUTEA_SPEC>;
#[doc = "Register `SPIROUTEA` writer"]
pub type W = crate::W<SPIROUTEA_SPEC>;
#[doc = "Field `SPI0` reader - SPI0 Signals"]
pub type SPI0_R = crate::FieldReader<SPI0_A>;
#[doc = "SPI0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI0_A {
    #[doc = "0: MOSI: PA4, MISO: PA5, SCK: PA6, SS: PA7"]
    DEFAULT = 0,
    #[doc = "3: MOSI: PA0, MISO: PA1, SCK: -, SS: PC1"]
    ALT3 = 3,
    #[doc = "4: MOSI: PD4, MISO: PD5, SCK: PD6, SS: PD7"]
    ALT4 = 4,
    #[doc = "5: MOSI: -, MISO: PC1, SCK: PC2, SS: PC3"]
    ALT5 = 5,
    #[doc = "6: MOSI: PC1, MISO: PC2, SCK: PC3, SS: PF7"]
    ALT6 = 6,
    #[doc = "7: Not connected to any pins. SS set to 1."]
    NONE = 7,
}
impl From<SPI0_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI0_A {
    type Ux = u8;
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI0_A> {
        match self.bits {
            0 => Some(SPI0_A::DEFAULT),
            3 => Some(SPI0_A::ALT3),
            4 => Some(SPI0_A::ALT4),
            5 => Some(SPI0_A::ALT5),
            6 => Some(SPI0_A::ALT6),
            7 => Some(SPI0_A::NONE),
            _ => None,
        }
    }
    #[doc = "MOSI: PA4, MISO: PA5, SCK: PA6, SS: PA7"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == SPI0_A::DEFAULT
    }
    #[doc = "MOSI: PA0, MISO: PA1, SCK: -, SS: PC1"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == SPI0_A::ALT3
    }
    #[doc = "MOSI: PD4, MISO: PD5, SCK: PD6, SS: PD7"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == SPI0_A::ALT4
    }
    #[doc = "MOSI: -, MISO: PC1, SCK: PC2, SS: PC3"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == SPI0_A::ALT5
    }
    #[doc = "MOSI: PC1, MISO: PC2, SCK: PC3, SS: PF7"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == SPI0_A::ALT6
    }
    #[doc = "Not connected to any pins. SS set to 1."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SPI0_A::NONE
    }
}
#[doc = "Field `SPI0` writer - SPI0 Signals"]
pub type SPI0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI0_A>;
impl<'a, REG> SPI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MOSI: PA4, MISO: PA5, SCK: PA6, SS: PA7"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::DEFAULT)
    }
    #[doc = "MOSI: PA0, MISO: PA1, SCK: -, SS: PC1"]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::ALT3)
    }
    #[doc = "MOSI: PD4, MISO: PD5, SCK: PD6, SS: PD7"]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::ALT4)
    }
    #[doc = "MOSI: -, MISO: PC1, SCK: PC2, SS: PC3"]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::ALT5)
    }
    #[doc = "MOSI: PC1, MISO: PC2, SCK: PC3, SS: PF7"]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::ALT6)
    }
    #[doc = "Not connected to any pins. SS set to 1."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI0 Signals"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<SPIROUTEA_SPEC> {
        SPI0_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spiroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spiroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPIROUTEA_SPEC;
impl crate::RegisterSpec for SPIROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spiroutea::R`](R) reader structure"]
impl crate::Readable for SPIROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spiroutea::W`](W) writer structure"]
impl crate::Writable for SPIROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPIROUTEA to value 0"]
impl crate::Resettable for SPIROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
