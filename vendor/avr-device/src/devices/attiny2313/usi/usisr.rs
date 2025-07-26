#[doc = "Register `USISR` reader"]
pub type R = crate::R<USISR_SPEC>;
#[doc = "Register `USISR` writer"]
pub type W = crate::W<USISR_SPEC>;
#[doc = "Field `USICNT` reader - USI Counter Value Bits"]
pub type USICNT_R = crate::FieldReader;
#[doc = "Field `USICNT` writer - USI Counter Value Bits"]
pub type USICNT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `USIDC` reader - Data Output Collision"]
pub type USIDC_R = crate::BitReader;
#[doc = "Field `USIPF` reader - Stop Condition Flag"]
pub type USIPF_R = crate::BitReader;
#[doc = "Field `USIPF` writer - Stop Condition Flag"]
pub type USIPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USIOIF` reader - Counter Overflow Interrupt Flag"]
pub type USIOIF_R = crate::BitReader;
#[doc = "Field `USIOIF` writer - Counter Overflow Interrupt Flag"]
pub type USIOIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USISIF` reader - Start Condition Interrupt Flag"]
pub type USISIF_R = crate::BitReader;
#[doc = "Field `USISIF` writer - Start Condition Interrupt Flag"]
pub type USISIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USI Counter Value Bits"]
    #[inline(always)]
    pub fn usicnt(&self) -> USICNT_R {
        USICNT_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Data Output Collision"]
    #[inline(always)]
    pub fn usidc(&self) -> USIDC_R {
        USIDC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Condition Flag"]
    #[inline(always)]
    pub fn usipf(&self) -> USIPF_R {
        USIPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn usioif(&self) -> USIOIF_R {
        USIOIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Flag"]
    #[inline(always)]
    pub fn usisif(&self) -> USISIF_R {
        USISIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USI Counter Value Bits"]
    #[inline(always)]
    #[must_use]
    pub fn usicnt(&mut self) -> USICNT_W<USISR_SPEC> {
        USICNT_W::new(self, 0)
    }
    #[doc = "Bit 5 - Stop Condition Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usipf(&mut self) -> USIPF_W<USISR_SPEC> {
        USIPF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usioif(&mut self) -> USIOIF_W<USISR_SPEC> {
        USIOIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn usisif(&mut self) -> USISIF_W<USISR_SPEC> {
        USISIF_W::new(self, 7)
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
#[doc = "USI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USISR_SPEC;
impl crate::RegisterSpec for USISR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usisr::R`](R) reader structure"]
impl crate::Readable for USISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usisr::W`](W) writer structure"]
impl crate::Writable for USISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USISR to value 0"]
impl crate::Resettable for USISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
