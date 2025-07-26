#[doc = "Register `TCCR1D` reader"]
pub type R = crate::R<TCCR1D_SPEC>;
#[doc = "Register `TCCR1D` writer"]
pub type W = crate::W<TCCR1D_SPEC>;
#[doc = "Field `OC1AU` reader - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
pub type OC1AU_R = crate::BitReader;
#[doc = "Field `OC1AU` writer - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
pub type OC1AU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1AV` reader - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
pub type OC1AV_R = crate::BitReader;
#[doc = "Field `OC1AV` writer - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
pub type OC1AV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1AW` reader - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
pub type OC1AW_R = crate::BitReader;
#[doc = "Field `OC1AW` writer - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
pub type OC1AW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1AX` reader - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
pub type OC1AX_R = crate::BitReader;
#[doc = "Field `OC1AX` writer - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
pub type OC1AX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1BU` reader - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
pub type OC1BU_R = crate::BitReader;
#[doc = "Field `OC1BU` writer - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
pub type OC1BU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1BV` reader - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
pub type OC1BV_R = crate::BitReader;
#[doc = "Field `OC1BV` writer - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
pub type OC1BV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1BW` reader - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
pub type OC1BW_R = crate::BitReader;
#[doc = "Field `OC1BW` writer - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
pub type OC1BW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1BX` reader - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
pub type OC1BX_R = crate::BitReader;
#[doc = "Field `OC1BX` writer - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
pub type OC1BX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1au(&self) -> OC1AU_R {
        OC1AU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1av(&self) -> OC1AV_R {
        OC1AV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1aw(&self) -> OC1AW_R {
        OC1AW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1ax(&self) -> OC1AX_R {
        OC1AX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bu(&self) -> OC1BU_R {
        OC1BU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bv(&self) -> OC1BV_R {
        OC1BV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bw(&self) -> OC1BW_R {
        OC1BW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bx(&self) -> OC1BX_R {
        OC1BX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1au(&mut self) -> OC1AU_W<TCCR1D_SPEC> {
        OC1AU_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1av(&mut self) -> OC1AV_W<TCCR1D_SPEC> {
        OC1AV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1aw(&mut self) -> OC1AW_W<TCCR1D_SPEC> {
        OC1AW_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ax(&mut self) -> OC1AX_W<TCCR1D_SPEC> {
        OC1AX_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bu(&mut self) -> OC1BU_W<TCCR1D_SPEC> {
        OC1BU_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bv(&mut self) -> OC1BV_W<TCCR1D_SPEC> {
        OC1BV_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bw(&mut self) -> OC1BW_W<TCCR1D_SPEC> {
        OC1BW_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bx(&mut self) -> OC1BX_W<TCCR1D_SPEC> {
        OC1BX_W::new(self, 7)
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
#[doc = "Timer/Counter1 Control Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1d::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1d::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR1D_SPEC;
impl crate::RegisterSpec for TCCR1D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr1d::R`](R) reader structure"]
impl crate::Readable for TCCR1D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr1d::W`](W) writer structure"]
impl crate::Writable for TCCR1D_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1D to value 0"]
impl crate::Resettable for TCCR1D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
