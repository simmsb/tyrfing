#[doc = "Register `PORTC` reader"]
pub type R = crate::R<PORTC_SPEC>;
#[doc = "Register `PORTC` writer"]
pub type W = crate::W<PORTC_SPEC>;
#[doc = "Field `PC6` reader - Pin C6"]
pub type PC6_R = crate::BitReader;
#[doc = "Field `PC6` writer - Pin C6"]
pub type PC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7` reader - Pin C7"]
pub type PC7_R = crate::BitReader;
#[doc = "Field `PC7` writer - Pin C7"]
pub type PC7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> PC6_W<PORTC_SPEC> {
        PC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> PC7_W<PORTC_SPEC> {
        PC7_W::new(self, 7)
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
#[doc = "Port C Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORTC_SPEC;
impl crate::RegisterSpec for PORTC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`portc::R`](R) reader structure"]
impl crate::Readable for PORTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`portc::W`](W) writer structure"]
impl crate::Writable for PORTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTC to value 0"]
impl crate::Resettable for PORTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
