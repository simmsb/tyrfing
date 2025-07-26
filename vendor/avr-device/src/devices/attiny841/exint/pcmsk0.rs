#[doc = "Register `PCMSK0` reader"]
pub type R = crate::R<PCMSK0_SPEC>;
#[doc = "Register `PCMSK0` writer"]
pub type W = crate::W<PCMSK0_SPEC>;
#[doc = "Field `PCINT0` reader - Pin Change Enable Mask 0 Bit 0"]
pub type PCINT0_R = crate::BitReader;
#[doc = "Field `PCINT0` writer - Pin Change Enable Mask 0 Bit 0"]
pub type PCINT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT1` reader - Pin Change Enable Mask 0 Bit 1"]
pub type PCINT1_R = crate::BitReader;
#[doc = "Field `PCINT1` writer - Pin Change Enable Mask 0 Bit 1"]
pub type PCINT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT2` reader - Pin Change Enable Mask 0 Bit 2"]
pub type PCINT2_R = crate::BitReader;
#[doc = "Field `PCINT2` writer - Pin Change Enable Mask 0 Bit 2"]
pub type PCINT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT3` reader - Pin Change Enable Mask 0 Bit 3"]
pub type PCINT3_R = crate::BitReader;
#[doc = "Field `PCINT3` writer - Pin Change Enable Mask 0 Bit 3"]
pub type PCINT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT4` reader - Pin Change Enable Mask 0 Bit 4"]
pub type PCINT4_R = crate::BitReader;
#[doc = "Field `PCINT4` writer - Pin Change Enable Mask 0 Bit 4"]
pub type PCINT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT5` reader - Pin Change Enable Mask 0 Bit 5"]
pub type PCINT5_R = crate::BitReader;
#[doc = "Field `PCINT5` writer - Pin Change Enable Mask 0 Bit 5"]
pub type PCINT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT6` reader - Pin Change Enable Mask 0 Bit 6"]
pub type PCINT6_R = crate::BitReader;
#[doc = "Field `PCINT6` writer - Pin Change Enable Mask 0 Bit 6"]
pub type PCINT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT7` reader - Pin Change Enable Mask 0 Bit 7"]
pub type PCINT7_R = crate::BitReader;
#[doc = "Field `PCINT7` writer - Pin Change Enable Mask 0 Bit 7"]
pub type PCINT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin Change Enable Mask 0 Bit 0"]
    #[inline(always)]
    pub fn pcint0(&self) -> PCINT0_R {
        PCINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 0 Bit 1"]
    #[inline(always)]
    pub fn pcint1(&self) -> PCINT1_R {
        PCINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 0 Bit 2"]
    #[inline(always)]
    pub fn pcint2(&self) -> PCINT2_R {
        PCINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 0 Bit 3"]
    #[inline(always)]
    pub fn pcint3(&self) -> PCINT3_R {
        PCINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin Change Enable Mask 0 Bit 4"]
    #[inline(always)]
    pub fn pcint4(&self) -> PCINT4_R {
        PCINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin Change Enable Mask 0 Bit 5"]
    #[inline(always)]
    pub fn pcint5(&self) -> PCINT5_R {
        PCINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin Change Enable Mask 0 Bit 6"]
    #[inline(always)]
    pub fn pcint6(&self) -> PCINT6_R {
        PCINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin Change Enable Mask 0 Bit 7"]
    #[inline(always)]
    pub fn pcint7(&self) -> PCINT7_R {
        PCINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Enable Mask 0 Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcint0(&mut self) -> PCINT0_W<PCMSK0_SPEC> {
        PCINT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 0 Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcint1(&mut self) -> PCINT1_W<PCMSK0_SPEC> {
        PCINT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 0 Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcint2(&mut self) -> PCINT2_W<PCMSK0_SPEC> {
        PCINT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 0 Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcint3(&mut self) -> PCINT3_W<PCMSK0_SPEC> {
        PCINT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin Change Enable Mask 0 Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pcint4(&mut self) -> PCINT4_W<PCMSK0_SPEC> {
        PCINT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin Change Enable Mask 0 Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pcint5(&mut self) -> PCINT5_W<PCMSK0_SPEC> {
        PCINT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin Change Enable Mask 0 Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pcint6(&mut self) -> PCINT6_W<PCMSK0_SPEC> {
        PCINT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin Change Enable Mask 0 Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pcint7(&mut self) -> PCINT7_W<PCMSK0_SPEC> {
        PCINT7_W::new(self, 7)
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
#[doc = "Pin Change Enable Mask 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCMSK0_SPEC;
impl crate::RegisterSpec for PCMSK0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcmsk0::R`](R) reader structure"]
impl crate::Readable for PCMSK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcmsk0::W`](W) writer structure"]
impl crate::Writable for PCMSK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK0 to value 0"]
impl crate::Resettable for PCMSK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
