#[doc = "Register `DT1` reader"]
pub type R = crate::R<DT1_SPEC>;
#[doc = "Register `DT1` writer"]
pub type W = crate::W<DT1_SPEC>;
#[doc = "Field `DT1L` reader - No Description."]
pub type DT1L_R = crate::FieldReader;
#[doc = "Field `DT1L` writer - No Description."]
pub type DT1L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `DT1H` reader - No Description."]
pub type DT1H_R = crate::FieldReader;
#[doc = "Field `DT1H` writer - No Description."]
pub type DT1H_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn dt1l(&self) -> DT1L_R {
        DT1L_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    pub fn dt1h(&self) -> DT1H_R {
        DT1H_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dt1l(&mut self) -> DT1L_W<DT1_SPEC> {
        DT1L_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dt1h(&mut self) -> DT1H_W<DT1_SPEC> {
        DT1H_W::new(self, 4)
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
#[doc = "Timer/Counter 1 Dead Time Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT1_SPEC;
impl crate::RegisterSpec for DT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dt1::R`](R) reader structure"]
impl crate::Readable for DT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt1::W`](W) writer structure"]
impl crate::Writable for DT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT1 to value 0"]
impl crate::Resettable for DT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
