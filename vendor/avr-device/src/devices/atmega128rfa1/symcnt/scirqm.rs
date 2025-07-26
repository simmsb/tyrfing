#[doc = "Register `SCIRQM` reader"]
pub type R = crate::R<SCIRQM_SPEC>;
#[doc = "Register `SCIRQM` writer"]
pub type W = crate::W<SCIRQM_SPEC>;
#[doc = "Field `IRQMCP` reader - Symbol Counter Compare Match 3 IRQ enable"]
pub type IRQMCP_R = crate::FieldReader;
#[doc = "Field `IRQMCP` writer - Symbol Counter Compare Match 3 IRQ enable"]
pub type IRQMCP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `IRQMOF` reader - Symbol Counter Overflow IRQ enable"]
pub type IRQMOF_R = crate::BitReader;
#[doc = "Field `IRQMOF` writer - Symbol Counter Overflow IRQ enable"]
pub type IRQMOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQMBO` reader - Backoff Slot Counter IRQ enable"]
pub type IRQMBO_R = crate::BitReader;
#[doc = "Field `IRQMBO` writer - Backoff Slot Counter IRQ enable"]
pub type IRQMBO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Symbol Counter Compare Match 3 IRQ enable"]
    #[inline(always)]
    pub fn irqmcp(&self) -> IRQMCP_R {
        IRQMCP_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ enable"]
    #[inline(always)]
    pub fn irqmof(&self) -> IRQMOF_R {
        IRQMOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ enable"]
    #[inline(always)]
    pub fn irqmbo(&self) -> IRQMBO_R {
        IRQMBO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Symbol Counter Compare Match 3 IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqmcp(&mut self) -> IRQMCP_W<SCIRQM_SPEC> {
        IRQMCP_W::new(self, 0)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqmof(&mut self) -> IRQMOF_W<SCIRQM_SPEC> {
        IRQMOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ enable"]
    #[inline(always)]
    #[must_use]
    pub fn irqmbo(&mut self) -> IRQMBO_W<SCIRQM_SPEC> {
        IRQMBO_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<SCIRQM_SPEC> {
        RES_W::new(self, 5)
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
#[doc = "Symbol Counter Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scirqm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scirqm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCIRQM_SPEC;
impl crate::RegisterSpec for SCIRQM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scirqm::R`](R) reader structure"]
impl crate::Readable for SCIRQM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scirqm::W`](W) writer structure"]
impl crate::Writable for SCIRQM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCIRQM to value 0"]
impl crate::Resettable for SCIRQM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
