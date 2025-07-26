#[doc = "Register `PCMSK3` reader"]
pub type R = crate::R<PCMSK3_SPEC>;
#[doc = "Register `PCMSK3` writer"]
pub type W = crate::W<PCMSK3_SPEC>;
#[doc = "Field `PCINT` reader - Pin Change Enable Masks"]
pub type PCINT_R = crate::FieldReader;
#[doc = "Field `PCINT` writer - Pin Change Enable Masks"]
pub type PCINT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Pin Change Enable Masks"]
    #[inline(always)]
    pub fn pcint(&self) -> PCINT_R {
        PCINT_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin Change Enable Masks"]
    #[inline(always)]
    #[must_use]
    pub fn pcint(&mut self) -> PCINT_W<PCMSK3_SPEC> {
        PCINT_W::new(self, 0)
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
#[doc = "Pin Change Mask Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCMSK3_SPEC;
impl crate::RegisterSpec for PCMSK3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcmsk3::R`](R) reader structure"]
impl crate::Readable for PCMSK3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcmsk3::W`](W) writer structure"]
impl crate::Writable for PCMSK3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK3 to value 0"]
impl crate::Resettable for PCMSK3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
