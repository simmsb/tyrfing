#[doc = "Register `AMISCR` reader"]
pub type R = crate::R<AMISCR_SPEC>;
#[doc = "Register `AMISCR` writer"]
pub type W = crate::W<AMISCR_SPEC>;
#[doc = "Field `XREFEN` reader - Internal Voltage Reference Output Enable"]
pub type XREFEN_R = crate::BitReader;
#[doc = "Field `XREFEN` writer - Internal Voltage Reference Output Enable"]
pub type XREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREFEN` reader - External Voltage Reference Input Enable"]
pub type AREFEN_R = crate::BitReader;
#[doc = "Field `AREFEN` writer - External Voltage Reference Input Enable"]
pub type AREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Internal Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn xrefen(&self) -> XREFEN_R {
        XREFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Voltage Reference Input Enable"]
    #[inline(always)]
    pub fn arefen(&self) -> AREFEN_R {
        AREFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Internal Voltage Reference Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xrefen(&mut self) -> XREFEN_W<AMISCR_SPEC> {
        XREFEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - External Voltage Reference Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arefen(&mut self) -> AREFEN_W<AMISCR_SPEC> {
        AREFEN_W::new(self, 2)
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
#[doc = "Analog Miscellaneous Control Register (Shared with CURRENT_SOURCE IO_MODULE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amiscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amiscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMISCR_SPEC;
impl crate::RegisterSpec for AMISCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`amiscr::R`](R) reader structure"]
impl crate::Readable for AMISCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amiscr::W`](W) writer structure"]
impl crate::Writable for AMISCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMISCR to value 0"]
impl crate::Resettable for AMISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
