#[doc = "Register `PINK` reader"]
pub type R = crate::R<PINK_SPEC>;
#[doc = "Register `PINK` writer"]
pub type W = crate::W<PINK_SPEC>;
#[doc = "Field `PK0` reader - Pin K0"]
pub type PK0_R = crate::BitReader;
#[doc = "Field `PK0` writer - Pin K0"]
pub type PK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK1` reader - Pin K1"]
pub type PK1_R = crate::BitReader;
#[doc = "Field `PK1` writer - Pin K1"]
pub type PK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK2` reader - Pin K2"]
pub type PK2_R = crate::BitReader;
#[doc = "Field `PK2` writer - Pin K2"]
pub type PK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK3` reader - Pin K3"]
pub type PK3_R = crate::BitReader;
#[doc = "Field `PK3` writer - Pin K3"]
pub type PK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK4` reader - Pin K4"]
pub type PK4_R = crate::BitReader;
#[doc = "Field `PK4` writer - Pin K4"]
pub type PK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK5` reader - Pin K5"]
pub type PK5_R = crate::BitReader;
#[doc = "Field `PK5` writer - Pin K5"]
pub type PK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK6` reader - Pin K6"]
pub type PK6_R = crate::BitReader;
#[doc = "Field `PK6` writer - Pin K6"]
pub type PK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PK7` reader - Pin K7"]
pub type PK7_R = crate::BitReader;
#[doc = "Field `PK7` writer - Pin K7"]
pub type PK7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin K0"]
    #[inline(always)]
    pub fn pk0(&self) -> PK0_R {
        PK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin K1"]
    #[inline(always)]
    pub fn pk1(&self) -> PK1_R {
        PK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin K2"]
    #[inline(always)]
    pub fn pk2(&self) -> PK2_R {
        PK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin K3"]
    #[inline(always)]
    pub fn pk3(&self) -> PK3_R {
        PK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin K4"]
    #[inline(always)]
    pub fn pk4(&self) -> PK4_R {
        PK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin K5"]
    #[inline(always)]
    pub fn pk5(&self) -> PK5_R {
        PK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin K6"]
    #[inline(always)]
    pub fn pk6(&self) -> PK6_R {
        PK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin K7"]
    #[inline(always)]
    pub fn pk7(&self) -> PK7_R {
        PK7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin K0"]
    #[inline(always)]
    #[must_use]
    pub fn pk0(&mut self) -> PK0_W<PINK_SPEC> {
        PK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin K1"]
    #[inline(always)]
    #[must_use]
    pub fn pk1(&mut self) -> PK1_W<PINK_SPEC> {
        PK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin K2"]
    #[inline(always)]
    #[must_use]
    pub fn pk2(&mut self) -> PK2_W<PINK_SPEC> {
        PK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin K3"]
    #[inline(always)]
    #[must_use]
    pub fn pk3(&mut self) -> PK3_W<PINK_SPEC> {
        PK3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin K4"]
    #[inline(always)]
    #[must_use]
    pub fn pk4(&mut self) -> PK4_W<PINK_SPEC> {
        PK4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin K5"]
    #[inline(always)]
    #[must_use]
    pub fn pk5(&mut self) -> PK5_W<PINK_SPEC> {
        PK5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin K6"]
    #[inline(always)]
    #[must_use]
    pub fn pk6(&mut self) -> PK6_W<PINK_SPEC> {
        PK6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin K7"]
    #[inline(always)]
    #[must_use]
    pub fn pk7(&mut self) -> PK7_W<PINK_SPEC> {
        PK7_W::new(self, 7)
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
#[doc = "PORT K Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pink::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pink::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINK_SPEC;
impl crate::RegisterSpec for PINK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pink::R`](R) reader structure"]
impl crate::Readable for PINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pink::W`](W) writer structure"]
impl crate::Writable for PINK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINK to value 0"]
impl crate::Resettable for PINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
