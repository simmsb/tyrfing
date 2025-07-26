#[doc = "Register `TWSCRA` reader"]
pub type R = crate::R<TWSCRA_SPEC>;
#[doc = "Register `TWSCRA` writer"]
pub type W = crate::W<TWSCRA_SPEC>;
#[doc = "Field `TWSME` reader - TWI Smart Mode Enable"]
pub type TWSME_R = crate::BitReader;
#[doc = "Field `TWSME` writer - TWI Smart Mode Enable"]
pub type TWSME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWPME` reader - TWI Promiscuous Mode Enable"]
pub type TWPME_R = crate::BitReader;
#[doc = "Field `TWPME` writer - TWI Promiscuous Mode Enable"]
pub type TWPME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWSIE` reader - TWI Stop Interrupt Enable"]
pub type TWSIE_R = crate::BitReader;
#[doc = "Field `TWSIE` writer - TWI Stop Interrupt Enable"]
pub type TWSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWEN` reader - Two-Wire Interface Enable"]
pub type TWEN_R = crate::BitReader;
#[doc = "Field `TWEN` writer - Two-Wire Interface Enable"]
pub type TWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWASIE` reader - TWI Address/Stop Interrupt Enable"]
pub type TWASIE_R = crate::BitReader;
#[doc = "Field `TWASIE` writer - TWI Address/Stop Interrupt Enable"]
pub type TWASIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWDIE` reader - TWI Data Interrupt Enable"]
pub type TWDIE_R = crate::BitReader;
#[doc = "Field `TWDIE` writer - TWI Data Interrupt Enable"]
pub type TWDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWSHE` reader - TWI SDA Hold Time Enable"]
pub type TWSHE_R = crate::BitReader;
#[doc = "Field `TWSHE` writer - TWI SDA Hold Time Enable"]
pub type TWSHE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TWI Smart Mode Enable"]
    #[inline(always)]
    pub fn twsme(&self) -> TWSME_R {
        TWSME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TWI Promiscuous Mode Enable"]
    #[inline(always)]
    pub fn twpme(&self) -> TWPME_R {
        TWPME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TWI Stop Interrupt Enable"]
    #[inline(always)]
    pub fn twsie(&self) -> TWSIE_R {
        TWSIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Two-Wire Interface Enable"]
    #[inline(always)]
    pub fn twen(&self) -> TWEN_R {
        TWEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TWI Address/Stop Interrupt Enable"]
    #[inline(always)]
    pub fn twasie(&self) -> TWASIE_R {
        TWASIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWI Data Interrupt Enable"]
    #[inline(always)]
    pub fn twdie(&self) -> TWDIE_R {
        TWDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TWI SDA Hold Time Enable"]
    #[inline(always)]
    pub fn twshe(&self) -> TWSHE_R {
        TWSHE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI Smart Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twsme(&mut self) -> TWSME_W<TWSCRA_SPEC> {
        TWSME_W::new(self, 0)
    }
    #[doc = "Bit 1 - TWI Promiscuous Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twpme(&mut self) -> TWPME_W<TWSCRA_SPEC> {
        TWPME_W::new(self, 1)
    }
    #[doc = "Bit 2 - TWI Stop Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twsie(&mut self) -> TWSIE_W<TWSCRA_SPEC> {
        TWSIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Two-Wire Interface Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twen(&mut self) -> TWEN_W<TWSCRA_SPEC> {
        TWEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TWI Address/Stop Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twasie(&mut self) -> TWASIE_W<TWSCRA_SPEC> {
        TWASIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - TWI Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twdie(&mut self) -> TWDIE_W<TWSCRA_SPEC> {
        TWDIE_W::new(self, 5)
    }
    #[doc = "Bit 7 - TWI SDA Hold Time Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twshe(&mut self) -> TWSHE_W<TWSCRA_SPEC> {
        TWSHE_W::new(self, 7)
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
#[doc = "TWI Slave Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twscra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twscra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWSCRA_SPEC;
impl crate::RegisterSpec for TWSCRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twscra::R`](R) reader structure"]
impl crate::Readable for TWSCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twscra::W`](W) writer structure"]
impl crate::Writable for TWSCRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSCRA to value 0"]
impl crate::Resettable for TWSCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
