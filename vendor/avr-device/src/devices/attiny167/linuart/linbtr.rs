#[doc = "Register `LINBTR` reader"]
pub type R = crate::R<LINBTR_SPEC>;
#[doc = "Register `LINBTR` writer"]
pub type W = crate::W<LINBTR_SPEC>;
#[doc = "Field `LBT` reader - LIN Bit Timing bits"]
pub type LBT_R = crate::FieldReader;
#[doc = "Field `LBT` writer - LIN Bit Timing bits"]
pub type LBT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `LDISR` reader - Disable Bit Timing Resynchronization"]
pub type LDISR_R = crate::BitReader;
#[doc = "Field `LDISR` writer - Disable Bit Timing Resynchronization"]
pub type LDISR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - LIN Bit Timing bits"]
    #[inline(always)]
    pub fn lbt(&self) -> LBT_R {
        LBT_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 7 - Disable Bit Timing Resynchronization"]
    #[inline(always)]
    pub fn ldisr(&self) -> LDISR_R {
        LDISR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - LIN Bit Timing bits"]
    #[inline(always)]
    #[must_use]
    pub fn lbt(&mut self) -> LBT_W<LINBTR_SPEC> {
        LBT_W::new(self, 0)
    }
    #[doc = "Bit 7 - Disable Bit Timing Resynchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ldisr(&mut self) -> LDISR_W<LINBTR_SPEC> {
        LDISR_W::new(self, 7)
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
#[doc = "LIN Bit Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linbtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linbtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINBTR_SPEC;
impl crate::RegisterSpec for LINBTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linbtr::R`](R) reader structure"]
impl crate::Readable for LINBTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linbtr::W`](W) writer structure"]
impl crate::Writable for LINBTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINBTR to value 0"]
impl crate::Resettable for LINBTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
