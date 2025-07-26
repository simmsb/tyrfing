#[doc = "Register `DDRL` reader"]
pub type R = crate::R<DDRL_SPEC>;
#[doc = "Register `DDRL` writer"]
pub type W = crate::W<DDRL_SPEC>;
#[doc = "Field `PL0` reader - Pin L0"]
pub type PL0_R = crate::BitReader;
#[doc = "Field `PL0` writer - Pin L0"]
pub type PL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL1` reader - Pin L1"]
pub type PL1_R = crate::BitReader;
#[doc = "Field `PL1` writer - Pin L1"]
pub type PL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL2` reader - Pin L2"]
pub type PL2_R = crate::BitReader;
#[doc = "Field `PL2` writer - Pin L2"]
pub type PL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL3` reader - Pin L3"]
pub type PL3_R = crate::BitReader;
#[doc = "Field `PL3` writer - Pin L3"]
pub type PL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL4` reader - Pin L4"]
pub type PL4_R = crate::BitReader;
#[doc = "Field `PL4` writer - Pin L4"]
pub type PL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL5` reader - Pin L5"]
pub type PL5_R = crate::BitReader;
#[doc = "Field `PL5` writer - Pin L5"]
pub type PL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL6` reader - Pin L6"]
pub type PL6_R = crate::BitReader;
#[doc = "Field `PL6` writer - Pin L6"]
pub type PL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL7` reader - Pin L7"]
pub type PL7_R = crate::BitReader;
#[doc = "Field `PL7` writer - Pin L7"]
pub type PL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin L0"]
    #[inline(always)]
    pub fn pl0(&self) -> PL0_R {
        PL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin L1"]
    #[inline(always)]
    pub fn pl1(&self) -> PL1_R {
        PL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin L2"]
    #[inline(always)]
    pub fn pl2(&self) -> PL2_R {
        PL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin L3"]
    #[inline(always)]
    pub fn pl3(&self) -> PL3_R {
        PL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin L4"]
    #[inline(always)]
    pub fn pl4(&self) -> PL4_R {
        PL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin L5"]
    #[inline(always)]
    pub fn pl5(&self) -> PL5_R {
        PL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin L6"]
    #[inline(always)]
    pub fn pl6(&self) -> PL6_R {
        PL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin L7"]
    #[inline(always)]
    pub fn pl7(&self) -> PL7_R {
        PL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin L0"]
    #[inline(always)]
    #[must_use]
    pub fn pl0(&mut self) -> PL0_W<DDRL_SPEC> {
        PL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin L1"]
    #[inline(always)]
    #[must_use]
    pub fn pl1(&mut self) -> PL1_W<DDRL_SPEC> {
        PL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin L2"]
    #[inline(always)]
    #[must_use]
    pub fn pl2(&mut self) -> PL2_W<DDRL_SPEC> {
        PL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin L3"]
    #[inline(always)]
    #[must_use]
    pub fn pl3(&mut self) -> PL3_W<DDRL_SPEC> {
        PL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin L4"]
    #[inline(always)]
    #[must_use]
    pub fn pl4(&mut self) -> PL4_W<DDRL_SPEC> {
        PL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin L5"]
    #[inline(always)]
    #[must_use]
    pub fn pl5(&mut self) -> PL5_W<DDRL_SPEC> {
        PL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin L6"]
    #[inline(always)]
    #[must_use]
    pub fn pl6(&mut self) -> PL6_W<DDRL_SPEC> {
        PL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin L7"]
    #[inline(always)]
    #[must_use]
    pub fn pl7(&mut self) -> PL7_W<DDRL_SPEC> {
        PL7_W::new(self, 7)
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
#[doc = "PORT L Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRL_SPEC;
impl crate::RegisterSpec for DDRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddrl::R`](R) reader structure"]
impl crate::Readable for DDRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddrl::W`](W) writer structure"]
impl crate::Writable for DDRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRL to value 0"]
impl crate::Resettable for DDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
