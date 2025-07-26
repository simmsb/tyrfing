#[doc = "Register `OCDMSTATUS` reader"]
pub type R = crate::R<OCDMSTATUS_SPEC>;
#[doc = "Register `OCDMSTATUS` writer"]
pub type W = crate::W<OCDMSTATUS_SPEC>;
#[doc = "Field `VALID` reader - OCD Message Valid"]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - OCD Message Valid"]
pub type VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OCD Message Valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OCD Message Valid"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<OCDMSTATUS_SPEC> {
        VALID_W::new(self, 0)
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
#[doc = "OCD Message Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdmstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdmstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCDMSTATUS_SPEC;
impl crate::RegisterSpec for OCDMSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocdmstatus::R`](R) reader structure"]
impl crate::Readable for OCDMSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocdmstatus::W`](W) writer structure"]
impl crate::Writable for OCDMSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDMSTATUS to value 0"]
impl crate::Resettable for OCDMSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
