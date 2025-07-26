#[doc = "Register `ACSRB` reader"]
pub type R = crate::R<ACSRB_SPEC>;
#[doc = "Register `ACSRB` writer"]
pub type W = crate::W<ACSRB_SPEC>;
#[doc = "Field `ACOE` reader - Analog Comparator Output Enable"]
pub type ACOE_R = crate::BitReader;
#[doc = "Field `ACOE` writer - Analog Comparator Output Enable"]
pub type ACOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Output Enable"]
    #[inline(always)]
    pub fn acoe(&self) -> ACOE_R {
        ACOE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acoe(&mut self) -> ACOE_W<ACSRB_SPEC> {
        ACOE_W::new(self, 0)
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
#[doc = "Analog Comparator Control And Status Register-B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSRB_SPEC;
impl crate::RegisterSpec for ACSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acsrb::R`](R) reader structure"]
impl crate::Readable for ACSRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsrb::W`](W) writer structure"]
impl crate::Writable for ACSRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSRB to value 0"]
impl crate::Resettable for ACSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
