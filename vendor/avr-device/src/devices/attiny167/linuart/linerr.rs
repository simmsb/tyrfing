#[doc = "Register `LINERR` reader"]
pub type R = crate::R<LINERR_SPEC>;
#[doc = "Register `LINERR` writer"]
pub type W = crate::W<LINERR_SPEC>;
#[doc = "Field `LBERR` reader - Bit Error Flag"]
pub type LBERR_R = crate::BitReader;
#[doc = "Field `LBERR` writer - Bit Error Flag"]
pub type LBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCERR` reader - Checksum Error Flag"]
pub type LCERR_R = crate::BitReader;
#[doc = "Field `LCERR` writer - Checksum Error Flag"]
pub type LCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPERR` reader - Parity Error Flag"]
pub type LPERR_R = crate::BitReader;
#[doc = "Field `LPERR` writer - Parity Error Flag"]
pub type LPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERR` reader - Synchronization Error Flag"]
pub type LSERR_R = crate::BitReader;
#[doc = "Field `LSERR` writer - Synchronization Error Flag"]
pub type LSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFERR` reader - Framing Error Flag"]
pub type LFERR_R = crate::BitReader;
#[doc = "Field `LFERR` writer - Framing Error Flag"]
pub type LFERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOVERR` reader - Overrun Error Flag"]
pub type LOVERR_R = crate::BitReader;
#[doc = "Field `LOVERR` writer - Overrun Error Flag"]
pub type LOVERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTOERR` reader - Frame Time Out Error Flag"]
pub type LTOERR_R = crate::BitReader;
#[doc = "Field `LTOERR` writer - Frame Time Out Error Flag"]
pub type LTOERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LABORT` reader - Abort Flag"]
pub type LABORT_R = crate::BitReader;
#[doc = "Field `LABORT` writer - Abort Flag"]
pub type LABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit Error Flag"]
    #[inline(always)]
    pub fn lberr(&self) -> LBERR_R {
        LBERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Checksum Error Flag"]
    #[inline(always)]
    pub fn lcerr(&self) -> LCERR_R {
        LCERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Flag"]
    #[inline(always)]
    pub fn lperr(&self) -> LPERR_R {
        LPERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Error Flag"]
    #[inline(always)]
    pub fn lserr(&self) -> LSERR_R {
        LSERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    pub fn lferr(&self) -> LFERR_R {
        LFERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn loverr(&self) -> LOVERR_R {
        LOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frame Time Out Error Flag"]
    #[inline(always)]
    pub fn ltoerr(&self) -> LTOERR_R {
        LTOERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Abort Flag"]
    #[inline(always)]
    pub fn labort(&self) -> LABORT_R {
        LABORT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lberr(&mut self) -> LBERR_W<LINERR_SPEC> {
        LBERR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Checksum Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lcerr(&mut self) -> LCERR_W<LINERR_SPEC> {
        LCERR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lperr(&mut self) -> LPERR_W<LINERR_SPEC> {
        LPERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lserr(&mut self) -> LSERR_W<LINERR_SPEC> {
        LSERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lferr(&mut self) -> LFERR_W<LINERR_SPEC> {
        LFERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn loverr(&mut self) -> LOVERR_W<LINERR_SPEC> {
        LOVERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Frame Time Out Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ltoerr(&mut self) -> LTOERR_W<LINERR_SPEC> {
        LTOERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Abort Flag"]
    #[inline(always)]
    #[must_use]
    pub fn labort(&mut self) -> LABORT_W<LINERR_SPEC> {
        LABORT_W::new(self, 7)
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
#[doc = "LIN Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINERR_SPEC;
impl crate::RegisterSpec for LINERR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linerr::R`](R) reader structure"]
impl crate::Readable for LINERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linerr::W`](W) writer structure"]
impl crate::Writable for LINERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINERR to value 0"]
impl crate::Resettable for LINERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
