#[doc = "Register `LINSIR` reader"]
pub type R = crate::R<LINSIR_SPEC>;
#[doc = "Register `LINSIR` writer"]
pub type W = crate::W<LINSIR_SPEC>;
#[doc = "Field `LRXOK` reader - Receive Performed Interrupt"]
pub type LRXOK_R = crate::BitReader;
#[doc = "Field `LRXOK` writer - Receive Performed Interrupt"]
pub type LRXOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTXOK` reader - Transmit Performed Interrupt"]
pub type LTXOK_R = crate::BitReader;
#[doc = "Field `LTXOK` writer - Transmit Performed Interrupt"]
pub type LTXOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIDOK` reader - Identifier Interrupt"]
pub type LIDOK_R = crate::BitReader;
#[doc = "Field `LIDOK` writer - Identifier Interrupt"]
pub type LIDOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LERR` reader - Error Interrupt"]
pub type LERR_R = crate::BitReader;
#[doc = "Field `LERR` writer - Error Interrupt"]
pub type LERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBUSY` reader - Busy Signal"]
pub type LBUSY_R = crate::BitReader;
#[doc = "Field `LBUSY` writer - Busy Signal"]
pub type LBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIDST` reader - Identifier Status bits"]
pub type LIDST_R = crate::FieldReader;
#[doc = "Field `LIDST` writer - Identifier Status bits"]
pub type LIDST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Receive Performed Interrupt"]
    #[inline(always)]
    pub fn lrxok(&self) -> LRXOK_R {
        LRXOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Performed Interrupt"]
    #[inline(always)]
    pub fn ltxok(&self) -> LTXOK_R {
        LTXOK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier Interrupt"]
    #[inline(always)]
    pub fn lidok(&self) -> LIDOK_R {
        LIDOK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt"]
    #[inline(always)]
    pub fn lerr(&self) -> LERR_R {
        LERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy Signal"]
    #[inline(always)]
    pub fn lbusy(&self) -> LBUSY_R {
        LBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Identifier Status bits"]
    #[inline(always)]
    pub fn lidst(&self) -> LIDST_R {
        LIDST_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lrxok(&mut self) -> LRXOK_W<LINSIR_SPEC> {
        LRXOK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ltxok(&mut self) -> LTXOK_W<LINSIR_SPEC> {
        LTXOK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Identifier Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lidok(&mut self) -> LIDOK_W<LINSIR_SPEC> {
        LIDOK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lerr(&mut self) -> LERR_W<LINSIR_SPEC> {
        LERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Busy Signal"]
    #[inline(always)]
    #[must_use]
    pub fn lbusy(&mut self) -> LBUSY_W<LINSIR_SPEC> {
        LBUSY_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Identifier Status bits"]
    #[inline(always)]
    #[must_use]
    pub fn lidst(&mut self) -> LIDST_W<LINSIR_SPEC> {
        LIDST_W::new(self, 5)
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
#[doc = "LIN Status and Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linsir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linsir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINSIR_SPEC;
impl crate::RegisterSpec for LINSIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linsir::R`](R) reader structure"]
impl crate::Readable for LINSIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linsir::W`](W) writer structure"]
impl crate::Writable for LINSIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINSIR to value 0"]
impl crate::Resettable for LINSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
