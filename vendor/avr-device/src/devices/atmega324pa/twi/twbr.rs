#[doc = "Register `TWBR` reader"]
pub type R = crate::R<TWBR_SPEC>;
#[doc = "Register `TWBR` writer"]
pub type W = crate::W<TWBR_SPEC>;
#[doc = "Field `TWBR` reader - TWI bit rate bits"]
pub type TWBR_R = crate::FieldReader;
#[doc = "Field `TWBR` writer - TWI bit rate bits"]
pub type TWBR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TWI bit rate bits"]
    #[inline(always)]
    pub fn twbr(&self) -> TWBR_R {
        TWBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TWI bit rate bits"]
    #[inline(always)]
    #[must_use]
    pub fn twbr(&mut self) -> TWBR_W<TWBR_SPEC> {
        TWBR_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TWI Bit Rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWBR_SPEC;
impl crate::RegisterSpec for TWBR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twbr::R`](R) reader structure"]
impl crate::Readable for TWBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twbr::W`](W) writer structure"]
impl crate::Writable for TWBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWBR to value 0"]
impl crate::Resettable for TWBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
