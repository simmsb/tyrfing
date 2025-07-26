#[doc = "Register `CTRLFSET` reader"]
pub type R = crate::R<SINGLE_CTRLFSET_SPEC>;
#[doc = "Register `CTRLFSET` writer"]
pub type W = crate::W<SINGLE_CTRLFSET_SPEC>;
#[doc = "Field `PERBV` reader - Period Buffer Valid"]
pub type PERBV_R = crate::BitReader;
#[doc = "Field `PERBV` writer - Period Buffer Valid"]
pub type PERBV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0BV` reader - Compare 0 Buffer Valid"]
pub type CMP0BV_R = crate::BitReader;
#[doc = "Field `CMP0BV` writer - Compare 0 Buffer Valid"]
pub type CMP0BV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1BV` reader - Compare 1 Buffer Valid"]
pub type CMP1BV_R = crate::BitReader;
#[doc = "Field `CMP1BV` writer - Compare 1 Buffer Valid"]
pub type CMP1BV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2BV` reader - Compare 2 Buffer Valid"]
pub type CMP2BV_R = crate::BitReader;
#[doc = "Field `CMP2BV` writer - Compare 2 Buffer Valid"]
pub type CMP2BV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Period Buffer Valid"]
    #[inline(always)]
    pub fn perbv(&self) -> PERBV_R {
        PERBV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 0 Buffer Valid"]
    #[inline(always)]
    pub fn cmp0bv(&self) -> CMP0BV_R {
        CMP0BV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 1 Buffer Valid"]
    #[inline(always)]
    pub fn cmp1bv(&self) -> CMP1BV_R {
        CMP1BV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 2 Buffer Valid"]
    #[inline(always)]
    pub fn cmp2bv(&self) -> CMP2BV_R {
        CMP2BV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Period Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn perbv(&mut self) -> PERBV_W<SINGLE_CTRLFSET_SPEC> {
        PERBV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 0 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0bv(&mut self) -> CMP0BV_W<SINGLE_CTRLFSET_SPEC> {
        CMP0BV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 1 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1bv(&mut self) -> CMP1BV_W<SINGLE_CTRLFSET_SPEC> {
        CMP1BV_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare 2 Buffer Valid"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2bv(&mut self) -> CMP2BV_W<SINGLE_CTRLFSET_SPEC> {
        CMP2BV_W::new(self, 3)
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
#[doc = "Control F Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrlfset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrlfset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_CTRLFSET_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLFSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`single_ctrlfset::R`](R) reader structure"]
impl crate::Readable for SINGLE_CTRLFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_ctrlfset::W`](W) writer structure"]
impl crate::Writable for SINGLE_CTRLFSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLFSET to value 0"]
impl crate::Resettable for SINGLE_CTRLFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
