#[doc = "Register `SADDR` reader"]
pub type R = crate::R<SADDR_SPEC>;
#[doc = "Register `SADDR` writer"]
pub type W = crate::W<SADDR_SPEC>;
#[doc = "Field `GCE` reader - General Call Recognition Enable Bit"]
pub type GCE_R = crate::BitReader;
#[doc = "Field `GCE` writer - General Call Recognition Enable Bit"]
pub type GCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Client Address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Client Address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - General Call Recognition Enable Bit"]
    #[inline(always)]
    pub fn gce(&self) -> GCE_R {
        GCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Client Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Recognition Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn gce(&mut self) -> GCE_W<SADDR_SPEC> {
        GCE_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Client Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SADDR_SPEC> {
        ADDR_W::new(self, 1)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Client Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDR_SPEC;
impl crate::RegisterSpec for SADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`saddr::R`](R) reader structure"]
impl crate::Readable for SADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddr::W`](W) writer structure"]
impl crate::Writable for SADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDR to value 0"]
impl crate::Resettable for SADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
