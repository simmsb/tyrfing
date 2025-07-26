#[doc = "Register `EECR` reader"]
pub type R = crate::R<EECR_SPEC>;
#[doc = "Register `EECR` writer"]
pub type W = crate::W<EECR_SPEC>;
#[doc = "Field `EERE` reader - EEPROM Read Enable"]
pub type EERE_R = crate::BitReader;
#[doc = "Field `EERE` writer - EEPROM Read Enable"]
pub type EERE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEWE` reader - EEPROM Write Enable"]
pub type EEWE_R = crate::BitReader;
#[doc = "Field `EEWE` writer - EEPROM Write Enable"]
pub type EEWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEMWE` reader - EEPROM Master Write Enable"]
pub type EEMWE_R = crate::BitReader;
#[doc = "Field `EEMWE` writer - EEPROM Master Write Enable"]
pub type EEMWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EERIE` reader - EEPROM Ready Interrupt Enable"]
pub type EERIE_R = crate::BitReader;
#[doc = "Field `EERIE` writer - EEPROM Ready Interrupt Enable"]
pub type EERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EEPROM Read Enable"]
    #[inline(always)]
    pub fn eere(&self) -> EERE_R {
        EERE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM Write Enable"]
    #[inline(always)]
    pub fn eewe(&self) -> EEWE_R {
        EEWE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    pub fn eemwe(&self) -> EEMWE_R {
        EEMWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EEPROM Ready Interrupt Enable"]
    #[inline(always)]
    pub fn eerie(&self) -> EERIE_R {
        EERIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eere(&mut self) -> EERE_W<EECR_SPEC> {
        EERE_W::new(self, 0)
    }
    #[doc = "Bit 1 - EEPROM Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eewe(&mut self) -> EEWE_W<EECR_SPEC> {
        EEWE_W::new(self, 1)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eemwe(&mut self) -> EEMWE_W<EECR_SPEC> {
        EEMWE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EEPROM Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eerie(&mut self) -> EERIE_W<EECR_SPEC> {
        EERIE_W::new(self, 3)
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
#[doc = "EEPROM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EECR_SPEC;
impl crate::RegisterSpec for EECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eecr::R`](R) reader structure"]
impl crate::Readable for EECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eecr::W`](W) writer structure"]
impl crate::Writable for EECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EECR to value 0"]
impl crate::Resettable for EECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
