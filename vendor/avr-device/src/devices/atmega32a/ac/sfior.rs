#[doc = "Register `SFIOR` reader"]
pub type R = crate::R<SFIOR_SPEC>;
#[doc = "Register `SFIOR` writer"]
pub type W = crate::W<SFIOR_SPEC>;
#[doc = "Field `PSR10` reader - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
pub type PSR10_R = crate::BitReader;
#[doc = "Field `PSR10` writer - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
pub type PSR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSR2` reader - Prescaler Reset Timer/Counter2"]
pub type PSR2_R = crate::BitReader;
#[doc = "Field `PSR2` writer - Prescaler Reset Timer/Counter2"]
pub type PSR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUD` reader - Pull-up disable"]
pub type PUD_R = crate::BitReader;
#[doc = "Field `PUD` writer - Pull-up disable"]
pub type PUD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACME` reader - Analog Comparator Multiplexer Enable"]
pub type ACME_R = crate::BitReader;
#[doc = "Field `ACME` writer - Analog Comparator Multiplexer Enable"]
pub type ACME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTS` reader - ADC Auto Trigger Source"]
pub type ADTS_R = crate::FieldReader;
#[doc = "Field `ADTS` writer - ADC Auto Trigger Source"]
pub type ADTS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
    #[inline(always)]
    pub fn psr10(&self) -> PSR10_R {
        PSR10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter2"]
    #[inline(always)]
    pub fn psr2(&self) -> PSR2_R {
        PSR2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-up disable"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator Multiplexer Enable"]
    #[inline(always)]
    pub fn acme(&self) -> ACME_R {
        ACME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - ADC Auto Trigger Source"]
    #[inline(always)]
    pub fn adts(&self) -> ADTS_R {
        ADTS_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler Reset Timer/Counter1 and Timer/Counter0"]
    #[inline(always)]
    #[must_use]
    pub fn psr10(&mut self) -> PSR10_W<SFIOR_SPEC> {
        PSR10_W::new(self, 0)
    }
    #[doc = "Bit 1 - Prescaler Reset Timer/Counter2"]
    #[inline(always)]
    #[must_use]
    pub fn psr2(&mut self) -> PSR2_W<SFIOR_SPEC> {
        PSR2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pull-up disable"]
    #[inline(always)]
    #[must_use]
    pub fn pud(&mut self) -> PUD_W<SFIOR_SPEC> {
        PUD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog Comparator Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acme(&mut self) -> ACME_W<SFIOR_SPEC> {
        ACME_W::new(self, 3)
    }
    #[doc = "Bits 5:7 - ADC Auto Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn adts(&mut self) -> ADTS_W<SFIOR_SPEC> {
        ADTS_W::new(self, 5)
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
#[doc = "Special Function IO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfior::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfior::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFIOR_SPEC;
impl crate::RegisterSpec for SFIOR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfior::R`](R) reader structure"]
impl crate::Readable for SFIOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfior::W`](W) writer structure"]
impl crate::Writable for SFIOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFIOR to value 0"]
impl crate::Resettable for SFIOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
