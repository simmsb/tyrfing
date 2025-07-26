#[doc = "Register `DIRSET` reader"]
pub type R = crate::R<DIRSET_SPEC>;
#[doc = "Register `DIRSET` writer"]
pub type W = crate::W<DIRSET_SPEC>;
#[doc = "Field `PD0` reader - Pin D0"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Pin D0"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Pin D1"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Pin D1"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Pin D2"]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Pin D2"]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Pin D3"]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Pin D3"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Pin D4"]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - Pin D4"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Pin D5"]
pub type PD5_R = crate::BitReader;
#[doc = "Field `PD5` writer - Pin D5"]
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Pin D6"]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - Pin D6"]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Pin D7"]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - Pin D7"]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin D0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin D1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin D2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin D3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin D4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin D5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin D6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin D7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin D0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<DIRSET_SPEC> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin D1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<DIRSET_SPEC> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin D2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<DIRSET_SPEC> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin D3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<DIRSET_SPEC> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin D4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<DIRSET_SPEC> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin D5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<DIRSET_SPEC> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin D6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<DIRSET_SPEC> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin D7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<DIRSET_SPEC> {
        PD7_W::new(self, 7)
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
