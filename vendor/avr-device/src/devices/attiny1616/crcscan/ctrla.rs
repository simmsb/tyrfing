#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable CRC scan"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable CRC scan"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIEN` reader - Enable NMI Trigger"]
pub type NMIEN_R = crate::BitReader;
#[doc = "Field `NMIEN` writer - Enable NMI Trigger"]
pub type NMIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Reset CRC scan"]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - Reset CRC scan"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable CRC scan"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable NMI Trigger"]
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset CRC scan"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CRC scan"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable NMI Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn nmien(&mut self) -> NMIEN_W<CTRLA_SPEC> {
        NMIEN_W::new(self, 1)
    }
    #[doc = "Bit 7 - Reset CRC scan"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CTRLA_SPEC> {
        RESET_W::new(self, 7)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
