#[doc = "Register `CLKSEL0` reader"]
pub type R = crate::R<CLKSEL0_SPEC>;
#[doc = "Register `CLKSEL0` writer"]
pub type W = crate::W<CLKSEL0_SPEC>;
#[doc = "Field `CLKS` reader - No Description."]
pub type CLKS_R = crate::BitReader;
#[doc = "Field `CLKS` writer - No Description."]
pub type CLKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTE` reader - No Description."]
pub type EXTE_R = crate::BitReader;
#[doc = "Field `EXTE` writer - No Description."]
pub type EXTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCE` reader - No Description."]
pub type RCE_R = crate::BitReader;
#[doc = "Field `RCE` writer - No Description."]
pub type RCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXSUT` reader - No Description."]
pub type EXSUT_R = crate::FieldReader;
#[doc = "Field `EXSUT` writer - No Description."]
pub type EXSUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `RCSUT` reader - No Description."]
pub type RCSUT_R = crate::FieldReader;
#[doc = "Field `RCSUT` writer - No Description."]
pub type RCSUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    pub fn exte(&self) -> EXTE_R {
        EXTE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    pub fn rce(&self) -> RCE_R {
        RCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - No Description."]
    #[inline(always)]
    pub fn exsut(&self) -> EXSUT_R {
        EXSUT_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    pub fn rcsut(&self) -> RCSUT_R {
        RCSUT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> CLKS_W<CLKSEL0_SPEC> {
        CLKS_W::new(self, 0)
    }
    #[doc = "Bit 2 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn exte(&mut self) -> EXTE_W<CLKSEL0_SPEC> {
        EXTE_W::new(self, 2)
    }
    #[doc = "Bit 3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rce(&mut self) -> RCE_W<CLKSEL0_SPEC> {
        RCE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn exsut(&mut self) -> EXSUT_W<CLKSEL0_SPEC> {
        EXSUT_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rcsut(&mut self) -> RCSUT_W<CLKSEL0_SPEC> {
        RCSUT_W::new(self, 6)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSEL0_SPEC;
impl crate::RegisterSpec for CLKSEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clksel0::R`](R) reader structure"]
impl crate::Readable for CLKSEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksel0::W`](W) writer structure"]
impl crate::Writable for CLKSEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL0 to value 0"]
impl crate::Resettable for CLKSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
