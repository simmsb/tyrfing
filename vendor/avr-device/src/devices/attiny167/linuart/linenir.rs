#[doc = "Register `LINENIR` reader"]
pub type R = crate::R<LINENIR_SPEC>;
#[doc = "Register `LINENIR` writer"]
pub type W = crate::W<LINENIR_SPEC>;
#[doc = "Field `LENRXOK` reader - Enable Receive Performed Interrupt"]
pub type LENRXOK_R = crate::BitReader;
#[doc = "Field `LENRXOK` writer - Enable Receive Performed Interrupt"]
pub type LENRXOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LENTXOK` reader - Enable Transmit Performed Interrupt"]
pub type LENTXOK_R = crate::BitReader;
#[doc = "Field `LENTXOK` writer - Enable Transmit Performed Interrupt"]
pub type LENTXOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LENIDOK` reader - Enable Identifier Interrupt"]
pub type LENIDOK_R = crate::BitReader;
#[doc = "Field `LENIDOK` writer - Enable Identifier Interrupt"]
pub type LENIDOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LENERR` reader - Enable Error Interrupt"]
pub type LENERR_R = crate::BitReader;
#[doc = "Field `LENERR` writer - Enable Error Interrupt"]
pub type LENERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Receive Performed Interrupt"]
    #[inline(always)]
    pub fn lenrxok(&self) -> LENRXOK_R {
        LENRXOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Performed Interrupt"]
    #[inline(always)]
    pub fn lentxok(&self) -> LENTXOK_R {
        LENTXOK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Identifier Interrupt"]
    #[inline(always)]
    pub fn lenidok(&self) -> LENIDOK_R {
        LENIDOK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt"]
    #[inline(always)]
    pub fn lenerr(&self) -> LENERR_R {
        LENERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Receive Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lenrxok(&mut self) -> LENRXOK_W<LINENIR_SPEC> {
        LENRXOK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit Performed Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lentxok(&mut self) -> LENTXOK_W<LINENIR_SPEC> {
        LENTXOK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Identifier Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lenidok(&mut self) -> LENIDOK_W<LINENIR_SPEC> {
        LENIDOK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lenerr(&mut self) -> LENERR_W<LINENIR_SPEC> {
        LENERR_W::new(self, 3)
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
#[doc = "LIN Enable Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linenir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linenir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINENIR_SPEC;
impl crate::RegisterSpec for LINENIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`linenir::R`](R) reader structure"]
impl crate::Readable for LINENIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linenir::W`](W) writer structure"]
impl crate::Writable for LINENIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINENIR to value 0"]
impl crate::Resettable for LINENIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
