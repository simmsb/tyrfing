#[doc = "Register `DT1A` reader"]
pub type R = crate::R<DT1A_SPEC>;
#[doc = "Register `DT1A` writer"]
pub type W = crate::W<DT1A_SPEC>;
#[doc = "Field `DTVL` reader - No Description."]
pub type DTVL_R = crate::FieldReader;
#[doc = "Field `DTVL` writer - No Description."]
pub type DTVL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `DTVH` reader - No Description."]
pub type DTVH_R = crate::FieldReader;
#[doc = "Field `DTVH` writer - No Description."]
pub type DTVH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn dtvl(&self) -> DTVL_R {
        DTVL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    pub fn dtvh(&self) -> DTVH_R {
        DTVH_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dtvl(&mut self) -> DTVL_W<DT1A_SPEC> {
        DTVL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dtvh(&mut self) -> DTVH_W<DT1A_SPEC> {
        DTVH_W::new(self, 4)
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
#[doc = "Dead Time Value Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT1A_SPEC;
impl crate::RegisterSpec for DT1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dt1a::R`](R) reader structure"]
impl crate::Readable for DT1A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt1a::W`](W) writer structure"]
impl crate::Writable for DT1A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT1A to value 0"]
impl crate::Resettable for DT1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
