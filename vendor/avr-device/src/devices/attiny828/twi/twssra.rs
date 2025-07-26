#[doc = "Register `TWSSRA` reader"]
pub type R = crate::R<TWSSRA_SPEC>;
#[doc = "Register `TWSSRA` writer"]
pub type W = crate::W<TWSSRA_SPEC>;
#[doc = "Field `TWAS` reader - TWI Address or Stop"]
pub type TWAS_R = crate::BitReader;
#[doc = "Field `TWAS` writer - TWI Address or Stop"]
pub type TWAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWDIR` reader - TWI Read/Write Direction"]
pub type TWDIR_R = crate::BitReader;
#[doc = "Field `TWDIR` writer - TWI Read/Write Direction"]
pub type TWDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWBE` reader - TWI Bus Error"]
pub type TWBE_R = crate::BitReader;
#[doc = "Field `TWBE` writer - TWI Bus Error"]
pub type TWBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWC` reader - TWI Collision"]
pub type TWC_R = crate::BitReader;
#[doc = "Field `TWC` writer - TWI Collision"]
pub type TWC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWRA` reader - TWI Receive Acknowledge"]
pub type TWRA_R = crate::BitReader;
#[doc = "Field `TWRA` writer - TWI Receive Acknowledge"]
pub type TWRA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWCH` reader - TWI Clock Hold"]
pub type TWCH_R = crate::BitReader;
#[doc = "Field `TWCH` writer - TWI Clock Hold"]
pub type TWCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWASIF` reader - TWI Address/Stop Interrupt Flag"]
pub type TWASIF_R = crate::BitReader;
#[doc = "Field `TWASIF` writer - TWI Address/Stop Interrupt Flag"]
pub type TWASIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWDIF` reader - TWI Data Interrupt Flag."]
pub type TWDIF_R = crate::BitReader;
#[doc = "Field `TWDIF` writer - TWI Data Interrupt Flag."]
pub type TWDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TWI Address or Stop"]
    #[inline(always)]
    pub fn twas(&self) -> TWAS_R {
        TWAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TWI Read/Write Direction"]
    #[inline(always)]
    pub fn twdir(&self) -> TWDIR_R {
        TWDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TWI Bus Error"]
    #[inline(always)]
    pub fn twbe(&self) -> TWBE_R {
        TWBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TWI Collision"]
    #[inline(always)]
    pub fn twc(&self) -> TWC_R {
        TWC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TWI Receive Acknowledge"]
    #[inline(always)]
    pub fn twra(&self) -> TWRA_R {
        TWRA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWI Clock Hold"]
    #[inline(always)]
    pub fn twch(&self) -> TWCH_R {
        TWCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TWI Address/Stop Interrupt Flag"]
    #[inline(always)]
    pub fn twasif(&self) -> TWASIF_R {
        TWASIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TWI Data Interrupt Flag."]
    #[inline(always)]
    pub fn twdif(&self) -> TWDIF_R {
        TWDIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI Address or Stop"]
    #[inline(always)]
    #[must_use]
    pub fn twas(&mut self) -> TWAS_W<TWSSRA_SPEC> {
        TWAS_W::new(self, 0)
    }
    #[doc = "Bit 1 - TWI Read/Write Direction"]
    #[inline(always)]
    #[must_use]
    pub fn twdir(&mut self) -> TWDIR_W<TWSSRA_SPEC> {
        TWDIR_W::new(self, 1)
    }
    #[doc = "Bit 2 - TWI Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn twbe(&mut self) -> TWBE_W<TWSSRA_SPEC> {
        TWBE_W::new(self, 2)
    }
    #[doc = "Bit 3 - TWI Collision"]
    #[inline(always)]
    #[must_use]
    pub fn twc(&mut self) -> TWC_W<TWSSRA_SPEC> {
        TWC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TWI Receive Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn twra(&mut self) -> TWRA_W<TWSSRA_SPEC> {
        TWRA_W::new(self, 4)
    }
    #[doc = "Bit 5 - TWI Clock Hold"]
    #[inline(always)]
    #[must_use]
    pub fn twch(&mut self) -> TWCH_W<TWSSRA_SPEC> {
        TWCH_W::new(self, 5)
    }
    #[doc = "Bit 6 - TWI Address/Stop Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn twasif(&mut self) -> TWASIF_W<TWSSRA_SPEC> {
        TWASIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - TWI Data Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn twdif(&mut self) -> TWDIF_W<TWSSRA_SPEC> {
        TWDIF_W::new(self, 7)
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
#[doc = "TWI Slave Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twssra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twssra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWSSRA_SPEC;
impl crate::RegisterSpec for TWSSRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twssra::R`](R) reader structure"]
impl crate::Readable for TWSSRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twssra::W`](W) writer structure"]
impl crate::Writable for TWSSRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSSRA to value 0"]
impl crate::Resettable for TWSSRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
