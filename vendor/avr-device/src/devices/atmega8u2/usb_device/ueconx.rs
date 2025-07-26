#[doc = "Register `UECONX` reader"]
pub type R = crate::R<UECONX_SPEC>;
#[doc = "Register `UECONX` writer"]
pub type W = crate::W<UECONX_SPEC>;
#[doc = "Field `EPEN` reader - Endpoint Enable Bit"]
pub type EPEN_R = crate::BitReader;
#[doc = "Field `EPEN` writer - Endpoint Enable Bit"]
pub type EPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDT` reader - Reset Data Toggle Bit"]
pub type RSTDT_R = crate::BitReader;
#[doc = "Field `RSTDT` writer - Reset Data Toggle Bit"]
pub type RSTDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQC` reader - STALL Request Clear Handshake Bit"]
pub type STALLRQC_R = crate::BitReader;
#[doc = "Field `STALLRQC` writer - STALL Request Clear Handshake Bit"]
pub type STALLRQC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLRQ` reader - STALL Request Handshake Bit"]
pub type STALLRQ_R = crate::BitReader;
#[doc = "Field `STALLRQ` writer - STALL Request Handshake Bit"]
pub type STALLRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Endpoint Enable Bit"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Data Toggle Bit"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STALL Request Clear Handshake Bit"]
    #[inline(always)]
    pub fn stallrqc(&self) -> STALLRQC_R {
        STALLRQC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STALL Request Handshake Bit"]
    #[inline(always)]
    pub fn stallrq(&self) -> STALLRQ_R {
        STALLRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<UECONX_SPEC> {
        EPEN_W::new(self, 0)
    }
    #[doc = "Bit 3 - Reset Data Toggle Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rstdt(&mut self) -> RSTDT_W<UECONX_SPEC> {
        RSTDT_W::new(self, 3)
    }
    #[doc = "Bit 4 - STALL Request Clear Handshake Bit"]
    #[inline(always)]
    #[must_use]
    pub fn stallrqc(&mut self) -> STALLRQC_W<UECONX_SPEC> {
        STALLRQC_W::new(self, 4)
    }
    #[doc = "Bit 5 - STALL Request Handshake Bit"]
    #[inline(always)]
    #[must_use]
    pub fn stallrq(&mut self) -> STALLRQ_W<UECONX_SPEC> {
        STALLRQ_W::new(self, 5)
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
#[doc = "USB Endpoint Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueconx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueconx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UECONX_SPEC;
impl crate::RegisterSpec for UECONX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ueconx::R`](R) reader structure"]
impl crate::Readable for UECONX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ueconx::W`](W) writer structure"]
impl crate::Writable for UECONX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UECONX to value 0"]
impl crate::Resettable for UECONX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
