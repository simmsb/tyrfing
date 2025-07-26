#[doc = "Register `CTRLE` reader"]
pub type R = crate::R<CTRLE_SPEC>;
#[doc = "Register `CTRLE` writer"]
pub type W = crate::W<CTRLE_SPEC>;
#[doc = "Field `SYNCEOC` reader - Synchronize end of cycle strobe"]
pub type SYNCEOC_R = crate::BitReader;
#[doc = "Field `SYNCEOC` writer - Synchronize end of cycle strobe"]
pub type SYNCEOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC` reader - synchronize strobe"]
pub type SYNC_R = crate::BitReader;
#[doc = "Field `SYNC` writer - synchronize strobe"]
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART` reader - Restart strobe"]
pub type RESTART_R = crate::BitReader;
#[doc = "Field `RESTART` writer - Restart strobe"]
pub type RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAPTUREA` reader - Software Capture A Strobe"]
pub type SCAPTUREA_R = crate::BitReader;
#[doc = "Field `SCAPTUREA` writer - Software Capture A Strobe"]
pub type SCAPTUREA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAPTUREB` reader - Software Capture B Strobe"]
pub type SCAPTUREB_R = crate::BitReader;
#[doc = "Field `SCAPTUREB` writer - Software Capture B Strobe"]
pub type SCAPTUREB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISEOC` reader - Disable at end of cycle"]
pub type DISEOC_R = crate::BitReader;
#[doc = "Field `DISEOC` writer - Disable at end of cycle"]
pub type DISEOC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Synchronize end of cycle strobe"]
    #[inline(always)]
    pub fn synceoc(&self) -> SYNCEOC_R {
        SYNCEOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - synchronize strobe"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restart strobe"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Capture A Strobe"]
    #[inline(always)]
    pub fn scapturea(&self) -> SCAPTUREA_R {
        SCAPTUREA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Capture B Strobe"]
    #[inline(always)]
    pub fn scaptureb(&self) -> SCAPTUREB_R {
        SCAPTUREB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable at end of cycle"]
    #[inline(always)]
    pub fn diseoc(&self) -> DISEOC_R {
        DISEOC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronize end of cycle strobe"]
    #[inline(always)]
    #[must_use]
    pub fn synceoc(&mut self) -> SYNCEOC_W<CTRLE_SPEC> {
        SYNCEOC_W::new(self, 0)
    }
    #[doc = "Bit 1 - synchronize strobe"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<CTRLE_SPEC> {
        SYNC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Restart strobe"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<CTRLE_SPEC> {
        RESTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software Capture A Strobe"]
    #[inline(always)]
    #[must_use]
    pub fn scapturea(&mut self) -> SCAPTUREA_W<CTRLE_SPEC> {
        SCAPTUREA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software Capture B Strobe"]
    #[inline(always)]
    #[must_use]
    pub fn scaptureb(&mut self) -> SCAPTUREB_W<CTRLE_SPEC> {
        SCAPTUREB_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable at end of cycle"]
    #[inline(always)]
    #[must_use]
    pub fn diseoc(&mut self) -> DISEOC_W<CTRLE_SPEC> {
        DISEOC_W::new(self, 7)
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
#[doc = "Control E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLE_SPEC;
impl crate::RegisterSpec for CTRLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrle::R`](R) reader structure"]
impl crate::Readable for CTRLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrle::W`](W) writer structure"]
impl crate::Writable for CTRLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLE to value 0"]
impl crate::Resettable for CTRLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
