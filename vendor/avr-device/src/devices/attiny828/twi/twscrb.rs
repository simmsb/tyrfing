#[doc = "Register `TWSCRB` reader"]
pub type R = crate::R<TWSCRB_SPEC>;
#[doc = "Register `TWSCRB` writer"]
pub type W = crate::W<TWSCRB_SPEC>;
#[doc = "Field `TWCMD` reader - No Description."]
pub type TWCMD_R = crate::FieldReader;
#[doc = "Field `TWCMD` writer - No Description."]
pub type TWCMD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `TWAA` reader - TWI Acknowledge Action"]
pub type TWAA_R = crate::BitReader;
#[doc = "Field `TWAA` writer - TWI Acknowledge Action"]
pub type TWAA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWHNM` reader - TWI High Noise Mode"]
pub type TWHNM_R = crate::BitReader;
#[doc = "Field `TWHNM` writer - TWI High Noise Mode"]
pub type TWHNM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    pub fn twcmd(&self) -> TWCMD_R {
        TWCMD_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - TWI Acknowledge Action"]
    #[inline(always)]
    pub fn twaa(&self) -> TWAA_R {
        TWAA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TWI High Noise Mode"]
    #[inline(always)]
    pub fn twhnm(&self) -> TWHNM_R {
        TWHNM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn twcmd(&mut self) -> TWCMD_W<TWSCRB_SPEC> {
        TWCMD_W::new(self, 0)
    }
    #[doc = "Bit 2 - TWI Acknowledge Action"]
    #[inline(always)]
    #[must_use]
    pub fn twaa(&mut self) -> TWAA_W<TWSCRB_SPEC> {
        TWAA_W::new(self, 2)
    }
    #[doc = "Bit 3 - TWI High Noise Mode"]
    #[inline(always)]
    #[must_use]
    pub fn twhnm(&mut self) -> TWHNM_W<TWSCRB_SPEC> {
        TWHNM_W::new(self, 3)
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
#[doc = "TWI Slave Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twscrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twscrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWSCRB_SPEC;
impl crate::RegisterSpec for TWSCRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twscrb::R`](R) reader structure"]
impl crate::Readable for TWSCRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twscrb::W`](W) writer structure"]
impl crate::Writable for TWSCRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSCRB to value 0"]
impl crate::Resettable for TWSCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
