#[doc = "Register `LINSEL` reader"]
pub type R = crate::R<LINSEL_SPEC>;
#[doc = "Register `LINSEL` writer"]
pub type W = crate::W<LINSEL_SPEC>;
#[doc = "Field `LINDX` reader - FIFO LIN Data Buffer Index bits"]
pub type LINDX_R = crate::FieldReader;
#[doc = "Field `LINDX` writer - FIFO LIN Data Buffer Index bits"]
pub type LINDX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `LAINC` reader - Auto Increment of Data Buffer Index (Active Low)"]
pub type LAINC_R = crate::BitReader;
#[doc = "Field `LAINC` writer - Auto Increment of Data Buffer Index (Active Low)"]
pub type LAINC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FIFO LIN Data Buffer Index bits"]
    #[inline(always)]
    pub fn lindx(&self) -> LINDX_R {
        LINDX_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Auto Increment of Data Buffer Index (Active Low)"]
    #[inline(always)]
    pub fn lainc(&self) -> LAINC_R {
        LAINC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO LIN Data Buffer Index bits"]
    #[inline(always)]
    #[must_use]
    pub fn lindx(&mut self) -> LINDX_W<LINSEL_SPEC> {
        LINDX_W::new(self, 0)
    }
    #[doc = "Bit 3 - Auto Increment of Data Buffer Index (Active Low)"]
    #[inline(always)]
    #[must_use]
    pub fn lainc(&mut self) -> LAINC_W<LINSEL_SPEC> {
        LAINC_W::new(self, 3)
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
#[doc = "LIN Data Buffer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINSEL_SPEC;
impl crate::RegisterSpec for LINSEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linsel::R`](R) reader structure"]
impl crate::Readable for LINSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linsel::W`](W) writer structure"]
impl crate::Writable for LINSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINSEL to value 0"]
impl crate::Resettable for LINSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
