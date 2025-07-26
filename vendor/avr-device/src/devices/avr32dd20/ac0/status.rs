#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `CMPIF` reader - Analog Comparator Interrupt Flag"]
pub type CMPIF_R = crate::BitReader;
#[doc = "Field `CMPIF` writer - Analog Comparator Interrupt Flag"]
pub type CMPIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSTATE` reader - Analog Comparator State"]
pub type CMPSTATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn cmpif(&self) -> CMPIF_R {
        CMPIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator State"]
    #[inline(always)]
    pub fn cmpstate(&self) -> CMPSTATE_R {
        CMPSTATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpif(&mut self) -> CMPIF_W<STATUS_SPEC> {
        CMPIF_W::new(self, 0)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
