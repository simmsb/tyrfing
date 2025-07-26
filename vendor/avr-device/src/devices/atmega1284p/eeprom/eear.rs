#[doc = "Register `EEAR` reader"]
pub type R = crate::R<EEAR_SPEC>;
#[doc = "Register `EEAR` writer"]
pub type W = crate::W<EEAR_SPEC>;
#[doc = "Field `EEAR` reader - EEPROM Address bits"]
pub type EEAR_R = crate::FieldReader<u16>;
#[doc = "Field `EEAR` writer - EEPROM Address bits"]
pub type EEAR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - EEPROM Address bits"]
    #[inline(always)]
    pub fn eear(&self) -> EEAR_R {
        EEAR_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - EEPROM Address bits"]
    #[inline(always)]
    #[must_use]
    pub fn eear(&mut self) -> EEAR_W<EEAR_SPEC> {
        EEAR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EEPROM Address Register Low Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEAR_SPEC;
impl crate::RegisterSpec for EEAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`eear::R`](R) reader structure"]
impl crate::Readable for EEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eear::W`](W) writer structure"]
impl crate::Writable for EEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEAR to value 0"]
impl crate::Resettable for EEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
