#[doc = "Register `TWSD` reader"]
pub type R = crate::R<TWSD_SPEC>;
#[doc = "Register `TWSD` writer"]
pub type W = crate::W<TWSD_SPEC>;
#[doc = "Field `TWSD` reader - TWI slave data bit"]
pub type TWSD_R = crate::FieldReader;
#[doc = "Field `TWSD` writer - TWI slave data bit"]
pub type TWSD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TWI slave data bit"]
    #[inline(always)]
    pub fn twsd(&self) -> TWSD_R {
        TWSD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TWI slave data bit"]
    #[inline(always)]
    #[must_use]
    pub fn twsd(&mut self) -> TWSD_W<TWSD_SPEC> {
        TWSD_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TWI Slave Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twsd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twsd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWSD_SPEC;
impl crate::RegisterSpec for TWSD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twsd::R`](R) reader structure"]
impl crate::Readable for TWSD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twsd::W`](W) writer structure"]
impl crate::Writable for TWSD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSD to value 0"]
impl crate::Resettable for TWSD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
