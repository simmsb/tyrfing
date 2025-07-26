#[doc = "Register `PORTE` reader"]
pub type R = crate::R<PORTE_SPEC>;
#[doc = "Register `PORTE` writer"]
pub type W = crate::W<PORTE_SPEC>;
#[doc = "Field `PE2` reader - Pin E2"]
pub type PE2_R = crate::BitReader;
#[doc = "Field `PE2` writer - Pin E2"]
pub type PE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE6` reader - Pin E6"]
pub type PE6_R = crate::BitReader;
#[doc = "Field `PE6` writer - Pin E6"]
pub type PE6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    pub fn pe6(&self) -> PE6_R {
        PE6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE2_W<PORTE_SPEC> {
        PE2_W::new(self, 2)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    #[must_use]
    pub fn pe6(&mut self) -> PE6_W<PORTE_SPEC> {
        PE6_W::new(self, 6)
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
#[doc = "Data Register, Port E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`porte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORTE_SPEC;
impl crate::RegisterSpec for PORTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`porte::R`](R) reader structure"]
impl crate::Readable for PORTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`porte::W`](W) writer structure"]
impl crate::Writable for PORTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTE to value 0"]
impl crate::Resettable for PORTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
