#[doc = "Register `DT4` reader"]
pub type R = crate::R<DT4_SPEC>;
#[doc = "Register `DT4` writer"]
pub type W = crate::W<DT4_SPEC>;
#[doc = "Field `DT4L` reader - Timer/Counter 4 Dead Time Value Bits"]
pub type DT4L_R = crate::FieldReader;
#[doc = "Field `DT4L` writer - Timer/Counter 4 Dead Time Value Bits"]
pub type DT4L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer/Counter 4 Dead Time Value Bits"]
    #[inline(always)]
    pub fn dt4l(&self) -> DT4L_R {
        DT4L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer/Counter 4 Dead Time Value Bits"]
    #[inline(always)]
    #[must_use]
    pub fn dt4l(&mut self) -> DT4L_W<DT4_SPEC> {
        DT4L_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer/Counter 4 Dead Time Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT4_SPEC;
impl crate::RegisterSpec for DT4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dt4::R`](R) reader structure"]
impl crate::Readable for DT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt4::W`](W) writer structure"]
impl crate::Writable for DT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT4 to value 0"]
impl crate::Resettable for DT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
