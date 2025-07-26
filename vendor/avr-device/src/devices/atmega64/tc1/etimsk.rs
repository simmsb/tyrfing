#[doc = "Register `ETIMSK` reader"]
pub type R = crate::R<ETIMSK_SPEC>;
#[doc = "Register `ETIMSK` writer"]
pub type W = crate::W<ETIMSK_SPEC>;
#[doc = "Field `OCIE1C` reader - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
pub type OCIE1C_R = crate::BitReader;
#[doc = "Field `OCIE1C` writer - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
pub type OCIE1C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
    #[inline(always)]
    pub fn ocie1c(&self) -> OCIE1C_R {
        OCIE1C_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter 1, Output Compare Match C Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ocie1c(&mut self) -> OCIE1C_W<ETIMSK_SPEC> {
        OCIE1C_W::new(self, 0)
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
#[doc = "Extended Timer/Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etimsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etimsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETIMSK_SPEC;
impl crate::RegisterSpec for ETIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`etimsk::R`](R) reader structure"]
impl crate::Readable for ETIMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etimsk::W`](W) writer structure"]
impl crate::Writable for ETIMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETIMSK to value 0"]
impl crate::Resettable for ETIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
