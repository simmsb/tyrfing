#[doc = "Register `SCTRLA` reader"]
pub type R = crate::R<SCTRLA_SPEC>;
#[doc = "Register `SCTRLA` writer"]
pub type W = crate::W<SCTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable TWI Slave"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable TWI Slave"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SMEN_R = crate::BitReader;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMEN` reader - Promiscuous Mode Enable"]
pub type PMEN_R = crate::BitReader;
#[doc = "Field `PMEN` writer - Promiscuous Mode Enable"]
pub type PMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIEN` reader - Stop Interrupt Enable"]
pub type PIEN_R = crate::BitReader;
#[doc = "Field `PIEN` writer - Stop Interrupt Enable"]
pub type PIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APIEN` reader - Address/Stop Interrupt Enable"]
pub type APIEN_R = crate::BitReader;
#[doc = "Field `APIEN` writer - Address/Stop Interrupt Enable"]
pub type APIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIEN` reader - Data Interrupt Enable"]
pub type DIEN_R = crate::BitReader;
#[doc = "Field `DIEN` writer - Data Interrupt Enable"]
pub type DIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable TWI Slave"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Promiscuous Mode Enable"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn pien(&self) -> PIEN_R {
        PIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address/Stop Interrupt Enable"]
    #[inline(always)]
    pub fn apien(&self) -> APIEN_R {
        APIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Interrupt Enable"]
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable TWI Slave"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<SCTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<SCTRLA_SPEC> {
        SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Promiscuous Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmen(&mut self) -> PMEN_W<SCTRLA_SPEC> {
        PMEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Stop Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pien(&mut self) -> PIEN_W<SCTRLA_SPEC> {
        PIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Address/Stop Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn apien(&mut self) -> APIEN_W<SCTRLA_SPEC> {
        APIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dien(&mut self) -> DIEN_W<SCTRLA_SPEC> {
        DIEN_W::new(self, 7)
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
#[doc = "Slave Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTRLA_SPEC;
impl crate::RegisterSpec for SCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sctrla::R`](R) reader structure"]
impl crate::Readable for SCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctrla::W`](W) writer structure"]
impl crate::Writable for SCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTRLA to value 0"]
impl crate::Resettable for SCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
