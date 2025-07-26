#[doc = "Register `DIRSET` reader"]
pub type R = crate::R<DIRSET_SPEC>;
#[doc = "Register `DIRSET` writer"]
pub type W = crate::W<DIRSET_SPEC>;
#[doc = "Field `PB0` reader - Pin B0"]
pub type PB0_R = crate::BitReader;
#[doc = "Field `PB0` writer - Pin B0"]
pub type PB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB1` reader - Pin B1"]
pub type PB1_R = crate::BitReader;
#[doc = "Field `PB1` writer - Pin B1"]
pub type PB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB2` reader - Pin B2"]
pub type PB2_R = crate::BitReader;
#[doc = "Field `PB2` writer - Pin B2"]
pub type PB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB3` reader - Pin B3"]
pub type PB3_R = crate::BitReader;
#[doc = "Field `PB3` writer - Pin B3"]
pub type PB3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    pub fn pb0(&self) -> PB0_R {
        PB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    pub fn pb1(&self) -> PB1_R {
        PB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    pub fn pb2(&self) -> PB2_R {
        PB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    pub fn pb3(&self) -> PB3_R {
        PB3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    #[must_use]
    pub fn pb0(&mut self) -> PB0_W<DIRSET_SPEC> {
        PB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    #[must_use]
    pub fn pb1(&mut self) -> PB1_W<DIRSET_SPEC> {
        PB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    #[must_use]
    pub fn pb2(&mut self) -> PB2_W<DIRSET_SPEC> {
        PB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    #[must_use]
    pub fn pb3(&mut self) -> PB3_W<DIRSET_SPEC> {
        PB3_W::new(self, 3)
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
#[doc = "Data Direction Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRSET_SPEC;
impl crate::RegisterSpec for DIRSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dirset::R`](R) reader structure"]
impl crate::Readable for DIRSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dirset::W`](W) writer structure"]
impl crate::Writable for DIRSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRSET to value 0"]
impl crate::Resettable for DIRSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
