#[doc = "Register `DLYVAL` reader"]
pub type R = crate::R<DLYVAL_SPEC>;
#[doc = "Register `DLYVAL` writer"]
pub type W = crate::W<DLYVAL_SPEC>;
#[doc = "Field `DLYVAL` reader - Delay value"]
pub type DLYVAL_R = crate::FieldReader;
#[doc = "Field `DLYVAL` writer - Delay value"]
pub type DLYVAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay value"]
    #[inline(always)]
    pub fn dlyval(&self) -> DLYVAL_R {
        DLYVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay value"]
    #[inline(always)]
    #[must_use]
    pub fn dlyval(&mut self) -> DLYVAL_W<DLYVAL_SPEC> {
        DLYVAL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Delay value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlyval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLYVAL_SPEC;
impl crate::RegisterSpec for DLYVAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dlyval::R`](R) reader structure"]
impl crate::Readable for DLYVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dlyval::W`](W) writer structure"]
impl crate::Writable for DLYVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLYVAL to value 0"]
impl crate::Resettable for DLYVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
