#[doc = "Register `OUT` reader"]
pub type R = crate::R<OUT_SPEC>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OUT_SPEC>;
#[doc = "Field `PF6` reader - Pin F6"]
pub type PF6_R = crate::BitReader;
#[doc = "Field `PF6` writer - Pin F6"]
pub type PF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF7` reader - Pin F7"]
pub type PF7_R = crate::BitReader;
#[doc = "Field `PF7` writer - Pin F7"]
pub type PF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    pub fn pf6(&self) -> PF6_R {
        PF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin F7"]
    #[inline(always)]
    pub fn pf7(&self) -> PF7_R {
        PF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    #[must_use]
    pub fn pf6(&mut self) -> PF6_W<OUT_SPEC> {
        PF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin F7"]
    #[inline(always)]
    #[must_use]
    pub fn pf7(&mut self) -> PF7_W<OUT_SPEC> {
        PF7_W::new(self, 7)
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
#[doc = "Output Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
