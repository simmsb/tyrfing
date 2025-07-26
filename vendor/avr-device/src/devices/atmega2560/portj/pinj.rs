#[doc = "Register `PINJ` reader"]
pub type R = crate::R<PINJ_SPEC>;
#[doc = "Register `PINJ` writer"]
pub type W = crate::W<PINJ_SPEC>;
#[doc = "Field `PJ0` reader - Pin J0"]
pub type PJ0_R = crate::BitReader;
#[doc = "Field `PJ0` writer - Pin J0"]
pub type PJ0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJ1` reader - Pin J1"]
pub type PJ1_R = crate::BitReader;
#[doc = "Field `PJ1` writer - Pin J1"]
pub type PJ1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJ2` reader - Pin J2"]
pub type PJ2_R = crate::BitReader;
#[doc = "Field `PJ2` writer - Pin J2"]
pub type PJ2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJ3` reader - Pin J3"]
pub type PJ3_R = crate::BitReader;
#[doc = "Field `PJ3` writer - Pin J3"]
pub type PJ3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJ4` reader - Pin J4"]
pub type PJ4_R = crate::BitReader;
#[doc = "Field `PJ4` writer - Pin J4"]
pub type PJ4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJ5` reader - Pin J5"]
pub type PJ5_R = crate::BitReader;
#[doc = "Field `PJ5` writer - Pin J5"]
pub type PJ5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJ6` reader - Pin J6"]
pub type PJ6_R = crate::BitReader;
#[doc = "Field `PJ6` writer - Pin J6"]
pub type PJ6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PJ7` reader - Pin J7"]
pub type PJ7_R = crate::BitReader;
#[doc = "Field `PJ7` writer - Pin J7"]
pub type PJ7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin J0"]
    #[inline(always)]
    pub fn pj0(&self) -> PJ0_R {
        PJ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin J1"]
    #[inline(always)]
    pub fn pj1(&self) -> PJ1_R {
        PJ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin J2"]
    #[inline(always)]
    pub fn pj2(&self) -> PJ2_R {
        PJ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin J3"]
    #[inline(always)]
    pub fn pj3(&self) -> PJ3_R {
        PJ3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin J4"]
    #[inline(always)]
    pub fn pj4(&self) -> PJ4_R {
        PJ4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin J5"]
    #[inline(always)]
    pub fn pj5(&self) -> PJ5_R {
        PJ5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin J6"]
    #[inline(always)]
    pub fn pj6(&self) -> PJ6_R {
        PJ6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin J7"]
    #[inline(always)]
    pub fn pj7(&self) -> PJ7_R {
        PJ7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin J0"]
    #[inline(always)]
    #[must_use]
    pub fn pj0(&mut self) -> PJ0_W<PINJ_SPEC> {
        PJ0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin J1"]
    #[inline(always)]
    #[must_use]
    pub fn pj1(&mut self) -> PJ1_W<PINJ_SPEC> {
        PJ1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin J2"]
    #[inline(always)]
    #[must_use]
    pub fn pj2(&mut self) -> PJ2_W<PINJ_SPEC> {
        PJ2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin J3"]
    #[inline(always)]
    #[must_use]
    pub fn pj3(&mut self) -> PJ3_W<PINJ_SPEC> {
        PJ3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin J4"]
    #[inline(always)]
    #[must_use]
    pub fn pj4(&mut self) -> PJ4_W<PINJ_SPEC> {
        PJ4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin J5"]
    #[inline(always)]
    #[must_use]
    pub fn pj5(&mut self) -> PJ5_W<PINJ_SPEC> {
        PJ5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin J6"]
    #[inline(always)]
    #[must_use]
    pub fn pj6(&mut self) -> PJ6_W<PINJ_SPEC> {
        PJ6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin J7"]
    #[inline(always)]
    #[must_use]
    pub fn pj7(&mut self) -> PJ7_W<PINJ_SPEC> {
        PJ7_W::new(self, 7)
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
#[doc = "PORT J Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINJ_SPEC;
impl crate::RegisterSpec for PINJ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pinj::R`](R) reader structure"]
impl crate::Readable for PINJ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinj::W`](W) writer structure"]
impl crate::Writable for PINJ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINJ to value 0"]
impl crate::Resettable for PINJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
