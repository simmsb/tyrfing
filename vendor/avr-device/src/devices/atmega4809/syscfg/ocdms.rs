#[doc = "Register `OCDMS` reader"]
pub type R = crate::R<OCDMS_SPEC>;
#[doc = "Register `OCDMS` writer"]
pub type W = crate::W<OCDMS_SPEC>;
#[doc = "Field `OCDMR` reader - OCD Message Read"]
pub type OCDMR_R = crate::BitReader;
#[doc = "Field `OCDMR` writer - OCD Message Read"]
pub type OCDMR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OCD Message Read"]
    #[inline(always)]
    pub fn ocdmr(&self) -> OCDMR_R {
        OCDMR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OCD Message Read"]
    #[inline(always)]
    #[must_use]
    pub fn ocdmr(&mut self) -> OCDMR_W<OCDMS_SPEC> {
        OCDMR_W::new(self, 0)
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
#[doc = "OCD Message Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCDMS_SPEC;
impl crate::RegisterSpec for OCDMS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocdms::R`](R) reader structure"]
impl crate::Readable for OCDMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocdms::W`](W) writer structure"]
impl crate::Writable for OCDMS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDMS to value 0"]
impl crate::Resettable for OCDMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
