#[doc = "Register `EIMSK` reader"]
pub type R = crate::R<EIMSK_SPEC>;
#[doc = "Register `EIMSK` writer"]
pub type W = crate::W<EIMSK_SPEC>;
#[doc = "Field `INT` reader - External Interrupt Request 2 Enable"]
pub type INT_R = crate::FieldReader;
#[doc = "Field `INT` writer - External Interrupt Request 2 Enable"]
pub type INT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - External Interrupt Request 2 Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Interrupt Request 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<EIMSK_SPEC> {
        INT_W::new(self, 0)
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
#[doc = "External Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eimsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eimsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EIMSK_SPEC;
impl crate::RegisterSpec for EIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eimsk::R`](R) reader structure"]
impl crate::Readable for EIMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eimsk::W`](W) writer structure"]
impl crate::Writable for EIMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMSK to value 0"]
impl crate::Resettable for EIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
