#[doc = "Register `SCIRQS` reader"]
pub type R = crate::R<SCIRQS_SPEC>;
#[doc = "Register `SCIRQS` writer"]
pub type W = crate::W<SCIRQS_SPEC>;
#[doc = "Field `IRQSCP` reader - Compare Unit 3 Compare Match IRQ"]
pub type IRQSCP_R = crate::FieldReader;
#[doc = "Field `IRQSCP` writer - Compare Unit 3 Compare Match IRQ"]
pub type IRQSCP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `IRQSOF` reader - Symbol Counter Overflow IRQ"]
pub type IRQSOF_R = crate::BitReader;
#[doc = "Field `IRQSOF` writer - Symbol Counter Overflow IRQ"]
pub type IRQSOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQSBO` reader - Backoff Slot Counter IRQ"]
pub type IRQSBO_R = crate::BitReader;
#[doc = "Field `IRQSBO` writer - Backoff Slot Counter IRQ"]
pub type IRQSBO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Compare Unit 3 Compare Match IRQ"]
    #[inline(always)]
    pub fn irqscp(&self) -> IRQSCP_R {
        IRQSCP_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ"]
    #[inline(always)]
    pub fn irqsof(&self) -> IRQSOF_R {
        IRQSOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ"]
    #[inline(always)]
    pub fn irqsbo(&self) -> IRQSBO_R {
        IRQSBO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compare Unit 3 Compare Match IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn irqscp(&mut self) -> IRQSCP_W<SCIRQS_SPEC> {
        IRQSCP_W::new(self, 0)
    }
    #[doc = "Bit 3 - Symbol Counter Overflow IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn irqsof(&mut self) -> IRQSOF_W<SCIRQS_SPEC> {
        IRQSOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Backoff Slot Counter IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn irqsbo(&mut self) -> IRQSBO_W<SCIRQS_SPEC> {
        IRQSBO_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<SCIRQS_SPEC> {
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
#[doc = "Symbol Counter Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scirqs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scirqs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCIRQS_SPEC;
impl crate::RegisterSpec for SCIRQS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scirqs::R`](R) reader structure"]
impl crate::Readable for SCIRQS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scirqs::W`](W) writer structure"]
impl crate::Writable for SCIRQS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCIRQS to value 0"]
impl crate::Resettable for SCIRQS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
