#[doc = "Register `TWCR` reader"]
pub type R = crate::R<TWCR_SPEC>;
#[doc = "Register `TWCR` writer"]
pub type W = crate::W<TWCR_SPEC>;
#[doc = "Field `TWIE` reader - TWI Interrupt Enable"]
pub type TWIE_R = crate::BitReader;
#[doc = "Field `TWIE` writer - TWI Interrupt Enable"]
pub type TWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWEN` reader - TWI Enable Bit"]
pub type TWEN_R = crate::BitReader;
#[doc = "Field `TWEN` writer - TWI Enable Bit"]
pub type TWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWWC` reader - TWI Write Collition Flag"]
pub type TWWC_R = crate::BitReader;
#[doc = "Field `TWSTO` reader - TWI Stop Condition Bit"]
pub type TWSTO_R = crate::BitReader;
#[doc = "Field `TWSTO` writer - TWI Stop Condition Bit"]
pub type TWSTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWSTA` reader - TWI Start Condition Bit"]
pub type TWSTA_R = crate::BitReader;
#[doc = "Field `TWSTA` writer - TWI Start Condition Bit"]
pub type TWSTA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWEA` reader - TWI Enable Acknowledge Bit"]
pub type TWEA_R = crate::BitReader;
#[doc = "Field `TWEA` writer - TWI Enable Acknowledge Bit"]
pub type TWEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWINT` reader - TWI Interrupt Flag"]
pub type TWINT_R = crate::BitReader;
#[doc = "Field `TWINT` writer - TWI Interrupt Flag"]
pub type TWINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TWI Interrupt Enable"]
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TWI Enable Bit"]
    #[inline(always)]
    pub fn twen(&self) -> TWEN_R {
        TWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TWI Write Collition Flag"]
    #[inline(always)]
    pub fn twwc(&self) -> TWWC_R {
        TWWC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TWI Stop Condition Bit"]
    #[inline(always)]
    pub fn twsto(&self) -> TWSTO_R {
        TWSTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWI Start Condition Bit"]
    #[inline(always)]
    pub fn twsta(&self) -> TWSTA_R {
        TWSTA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TWI Enable Acknowledge Bit"]
    #[inline(always)]
    pub fn twea(&self) -> TWEA_R {
        TWEA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TWI Interrupt Flag"]
    #[inline(always)]
    pub fn twint(&self) -> TWINT_R {
        TWINT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twie(&mut self) -> TWIE_W<TWCR_SPEC> {
        TWIE_W::new(self, 0)
    }
    #[doc = "Bit 2 - TWI Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twen(&mut self) -> TWEN_W<TWCR_SPEC> {
        TWEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - TWI Stop Condition Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twsto(&mut self) -> TWSTO_W<TWCR_SPEC> {
        TWSTO_W::new(self, 4)
    }
    #[doc = "Bit 5 - TWI Start Condition Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twsta(&mut self) -> TWSTA_W<TWCR_SPEC> {
        TWSTA_W::new(self, 5)
    }
    #[doc = "Bit 6 - TWI Enable Acknowledge Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twea(&mut self) -> TWEA_W<TWCR_SPEC> {
        TWEA_W::new(self, 6)
    }
    #[doc = "Bit 7 - TWI Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn twint(&mut self) -> TWINT_W<TWCR_SPEC> {
        TWINT_W::new(self, 7)
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
#[doc = "TWI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWCR_SPEC;
impl crate::RegisterSpec for TWCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twcr::R`](R) reader structure"]
impl crate::Readable for TWCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twcr::W`](W) writer structure"]
impl crate::Writable for TWCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWCR to value 0"]
impl crate::Resettable for TWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
