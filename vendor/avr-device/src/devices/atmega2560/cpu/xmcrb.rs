#[doc = "Register `XMCRB` reader"]
pub type R = crate::R<XMCRB_SPEC>;
#[doc = "Register `XMCRB` writer"]
pub type W = crate::W<XMCRB_SPEC>;
#[doc = "Field `XMM` reader - External Memory High Mask"]
pub type XMM_R = crate::FieldReader;
#[doc = "Field `XMM` writer - External Memory High Mask"]
pub type XMM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `XMBK` reader - External Memory Bus Keeper Enable"]
pub type XMBK_R = crate::BitReader;
#[doc = "Field `XMBK` writer - External Memory Bus Keeper Enable"]
pub type XMBK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    pub fn xmm(&self) -> XMM_R {
        XMM_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    pub fn xmbk(&self) -> XMBK_R {
        XMBK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xmm(&mut self) -> XMM_W<XMCRB_SPEC> {
        XMM_W::new(self, 0)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmbk(&mut self) -> XMBK_W<XMCRB_SPEC> {
        XMBK_W::new(self, 7)
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
#[doc = "External Memory Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xmcrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xmcrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XMCRB_SPEC;
impl crate::RegisterSpec for XMCRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xmcrb::R`](R) reader structure"]
impl crate::Readable for XMCRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xmcrb::W`](W) writer structure"]
impl crate::Writable for XMCRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XMCRB to value 0"]
impl crate::Resettable for XMCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
