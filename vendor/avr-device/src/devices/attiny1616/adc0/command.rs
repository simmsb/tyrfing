#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<COMMAND_SPEC>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<COMMAND_SPEC>;
#[doc = "Field `STCONV` reader - Start Conversion Operation"]
pub type STCONV_R = crate::BitReader;
#[doc = "Field `STCONV` writer - Start Conversion Operation"]
pub type STCONV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Conversion Operation"]
    #[inline(always)]
    pub fn stconv(&self) -> STCONV_R {
        STCONV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Operation"]
    #[inline(always)]
    #[must_use]
    pub fn stconv(&mut self) -> STCONV_W<COMMAND_SPEC> {
        STCONV_W::new(self, 0)
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
#[doc = "Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for COMMAND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
