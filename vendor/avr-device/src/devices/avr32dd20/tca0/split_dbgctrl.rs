#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<SPLIT_DBGCTRL_SPEC>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<SPLIT_DBGCTRL_SPEC>;
#[doc = "Field `DBGRUN` reader - Debug Run"]
pub type DBGRUN_R = crate::BitReader;
#[doc = "Field `DBGRUN` writer - Debug Run"]
pub type DBGRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Run"]
    #[inline(always)]
    pub fn dbgrun(&self) -> DBGRUN_R {
        DBGRUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Run"]
    #[inline(always)]
    #[must_use]
    pub fn dbgrun(&mut self) -> DBGRUN_W<SPLIT_DBGCTRL_SPEC> {
        DBGRUN_W::new(self, 0)
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
#[doc = "Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_dbgctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_dbgctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPLIT_DBGCTRL_SPEC;
impl crate::RegisterSpec for SPLIT_DBGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`split_dbgctrl::R`](R) reader structure"]
impl crate::Readable for SPLIT_DBGCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`split_dbgctrl::W`](W) writer structure"]
impl crate::Writable for SPLIT_DBGCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for SPLIT_DBGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
