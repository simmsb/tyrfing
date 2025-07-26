#[doc = "Register `SCCR1` reader"]
pub type R = crate::R<SCCR1_SPEC>;
#[doc = "Register `SCCR1` writer"]
pub type W = crate::W<SCCR1_SPEC>;
#[doc = "Field `SCENBO` reader - Backoff Slot Counter enable"]
pub type SCENBO_R = crate::BitReader;
#[doc = "Field `SCENBO` writer - Backoff Slot Counter enable"]
pub type SCENBO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Backoff Slot Counter enable"]
    #[inline(always)]
    pub fn scenbo(&self) -> SCENBO_R {
        SCENBO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - Backoff Slot Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn scenbo(&mut self) -> SCENBO_W<SCCR1_SPEC> {
        SCENBO_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<SCCR1_SPEC> {
        RES_W::new(self, 1)
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
#[doc = "Symbol Counter Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCR1_SPEC;
impl crate::RegisterSpec for SCCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sccr1::R`](R) reader structure"]
impl crate::Readable for SCCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccr1::W`](W) writer structure"]
impl crate::Writable for SCCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCR1 to value 0"]
impl crate::Resettable for SCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
