#[doc = "Register `CLKPR` reader"]
pub type R = crate::R<CLKPR_SPEC>;
#[doc = "Register `CLKPR` writer"]
pub type W = crate::W<CLKPR_SPEC>;
#[doc = "Field `CLKPS` reader - No Description."]
pub type CLKPS_R = crate::FieldReader;
#[doc = "Field `CLKPS` writer - No Description."]
pub type CLKPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `CLKPCE` reader - No Description."]
pub type CLKPCE_R = crate::BitReader;
#[doc = "Field `CLKPCE` writer - No Description."]
pub type CLKPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn clkpce(&self) -> CLKPCE_R {
        CLKPCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clkps(&mut self) -> CLKPS_W<CLKPR_SPEC> {
        CLKPS_W::new(self, 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clkpce(&mut self) -> CLKPCE_W<CLKPR_SPEC> {
        CLKPCE_W::new(self, 7)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKPR_SPEC;
impl crate::RegisterSpec for CLKPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clkpr::R`](R) reader structure"]
impl crate::Readable for CLKPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkpr::W`](W) writer structure"]
impl crate::Writable for CLKPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKPR to value 0"]
impl crate::Resettable for CLKPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
