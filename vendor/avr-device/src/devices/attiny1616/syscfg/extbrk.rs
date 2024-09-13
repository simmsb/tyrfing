#[doc = "Register `EXTBRK` reader"]
pub type R = crate::R<EXTBRK_SPEC>;
#[doc = "Register `EXTBRK` writer"]
pub type W = crate::W<EXTBRK_SPEC>;
#[doc = "Field `ENEXTBRK` reader - External break enable"]
pub type ENEXTBRK_R = crate::BitReader;
#[doc = "Field `ENEXTBRK` writer - External break enable"]
pub type ENEXTBRK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External break enable"]
    #[inline(always)]
    pub fn enextbrk(&self) -> ENEXTBRK_R {
        ENEXTBRK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External break enable"]
    #[inline(always)]
    #[must_use]
    pub fn enextbrk(&mut self) -> ENEXTBRK_W<EXTBRK_SPEC> {
        ENEXTBRK_W::new(self, 0)
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
#[doc = "External Break\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extbrk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extbrk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTBRK_SPEC;
impl crate::RegisterSpec for EXTBRK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`extbrk::R`](R) reader structure"]
impl crate::Readable for EXTBRK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extbrk::W`](W) writer structure"]
impl crate::Writable for EXTBRK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTBRK to value 0"]
impl crate::Resettable for EXTBRK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
