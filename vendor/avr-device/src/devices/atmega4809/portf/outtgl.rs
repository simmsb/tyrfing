#[doc = "Register `OUTTGL` reader"]
pub type R = crate::R<OUTTGL_SPEC>;
#[doc = "Register `OUTTGL` writer"]
pub type W = crate::W<OUTTGL_SPEC>;
#[doc = "Field `PF0` reader - Pin F0"]
pub type PF0_R = crate::BitReader;
#[doc = "Field `PF0` writer - Pin F0"]
pub type PF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF1` reader - Pin F1"]
pub type PF1_R = crate::BitReader;
#[doc = "Field `PF1` writer - Pin F1"]
pub type PF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF2` reader - Pin F2"]
pub type PF2_R = crate::BitReader;
#[doc = "Field `PF2` writer - Pin F2"]
pub type PF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF3` reader - Pin F3"]
pub type PF3_R = crate::BitReader;
#[doc = "Field `PF3` writer - Pin F3"]
pub type PF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF4` reader - Pin F4"]
pub type PF4_R = crate::BitReader;
#[doc = "Field `PF4` writer - Pin F4"]
pub type PF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF5` reader - Pin F5"]
pub type PF5_R = crate::BitReader;
#[doc = "Field `PF5` writer - Pin F5"]
pub type PF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF6` reader - Pin F6"]
pub type PF6_R = crate::BitReader;
#[doc = "Field `PF6` writer - Pin F6"]
pub type PF6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin F0"]
    #[inline(always)]
    pub fn pf0(&self) -> PF0_R {
        PF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin F1"]
    #[inline(always)]
    pub fn pf1(&self) -> PF1_R {
        PF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin F2"]
    #[inline(always)]
    pub fn pf2(&self) -> PF2_R {
        PF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin F3"]
    #[inline(always)]
    pub fn pf3(&self) -> PF3_R {
        PF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin F4"]
    #[inline(always)]
    pub fn pf4(&self) -> PF4_R {
        PF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin F5"]
    #[inline(always)]
    pub fn pf5(&self) -> PF5_R {
        PF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    pub fn pf6(&self) -> PF6_R {
        PF6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin F0"]
    #[inline(always)]
    #[must_use]
    pub fn pf0(&mut self) -> PF0_W<OUTTGL_SPEC> {
        PF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin F1"]
    #[inline(always)]
    #[must_use]
    pub fn pf1(&mut self) -> PF1_W<OUTTGL_SPEC> {
        PF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin F2"]
    #[inline(always)]
    #[must_use]
    pub fn pf2(&mut self) -> PF2_W<OUTTGL_SPEC> {
        PF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin F3"]
    #[inline(always)]
    #[must_use]
    pub fn pf3(&mut self) -> PF3_W<OUTTGL_SPEC> {
        PF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin F4"]
    #[inline(always)]
    #[must_use]
    pub fn pf4(&mut self) -> PF4_W<OUTTGL_SPEC> {
        PF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin F5"]
    #[inline(always)]
    #[must_use]
    pub fn pf5(&mut self) -> PF5_W<OUTTGL_SPEC> {
        PF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    #[must_use]
    pub fn pf6(&mut self) -> PF6_W<OUTTGL_SPEC> {
        PF6_W::new(self, 6)
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
#[doc = "Output Value Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outtgl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outtgl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTTGL_SPEC;
impl crate::RegisterSpec for OUTTGL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`outtgl::R`](R) reader structure"]
impl crate::Readable for OUTTGL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outtgl::W`](W) writer structure"]
impl crate::Writable for OUTTGL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTTGL to value 0"]
impl crate::Resettable for OUTTGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
