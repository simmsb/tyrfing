#[doc = "Register `DDRG` reader"]
pub type R = crate::R<DDRG_SPEC>;
#[doc = "Register `DDRG` writer"]
pub type W = crate::W<DDRG_SPEC>;
#[doc = "Field `PG0` reader - Pin G0"]
pub type PG0_R = crate::BitReader;
#[doc = "Field `PG0` writer - Pin G0"]
pub type PG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG1` reader - Pin G1"]
pub type PG1_R = crate::BitReader;
#[doc = "Field `PG1` writer - Pin G1"]
pub type PG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG2` reader - Pin G2"]
pub type PG2_R = crate::BitReader;
#[doc = "Field `PG2` writer - Pin G2"]
pub type PG2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG3` reader - Pin G3"]
pub type PG3_R = crate::BitReader;
#[doc = "Field `PG3` writer - Pin G3"]
pub type PG3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG4` reader - Pin G4"]
pub type PG4_R = crate::BitReader;
#[doc = "Field `PG4` writer - Pin G4"]
pub type PG4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG5` reader - Pin G5"]
pub type PG5_R = crate::BitReader;
#[doc = "Field `PG5` writer - Pin G5"]
pub type PG5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG6` reader - Pin G6"]
pub type PG6_R = crate::BitReader;
#[doc = "Field `PG6` writer - Pin G6"]
pub type PG6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG7` reader - Pin G7"]
pub type PG7_R = crate::BitReader;
#[doc = "Field `PG7` writer - Pin G7"]
pub type PG7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin G0"]
    #[inline(always)]
    pub fn pg0(&self) -> PG0_R {
        PG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin G1"]
    #[inline(always)]
    pub fn pg1(&self) -> PG1_R {
        PG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin G2"]
    #[inline(always)]
    pub fn pg2(&self) -> PG2_R {
        PG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin G3"]
    #[inline(always)]
    pub fn pg3(&self) -> PG3_R {
        PG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin G4"]
    #[inline(always)]
    pub fn pg4(&self) -> PG4_R {
        PG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin G5"]
    #[inline(always)]
    pub fn pg5(&self) -> PG5_R {
        PG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin G6"]
    #[inline(always)]
    pub fn pg6(&self) -> PG6_R {
        PG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin G7"]
    #[inline(always)]
    pub fn pg7(&self) -> PG7_R {
        PG7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin G0"]
    #[inline(always)]
    #[must_use]
    pub fn pg0(&mut self) -> PG0_W<DDRG_SPEC> {
        PG0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin G1"]
    #[inline(always)]
    #[must_use]
    pub fn pg1(&mut self) -> PG1_W<DDRG_SPEC> {
        PG1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin G2"]
    #[inline(always)]
    #[must_use]
    pub fn pg2(&mut self) -> PG2_W<DDRG_SPEC> {
        PG2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin G3"]
    #[inline(always)]
    #[must_use]
    pub fn pg3(&mut self) -> PG3_W<DDRG_SPEC> {
        PG3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin G4"]
    #[inline(always)]
    #[must_use]
    pub fn pg4(&mut self) -> PG4_W<DDRG_SPEC> {
        PG4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin G5"]
    #[inline(always)]
    #[must_use]
    pub fn pg5(&mut self) -> PG5_W<DDRG_SPEC> {
        PG5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin G6"]
    #[inline(always)]
    #[must_use]
    pub fn pg6(&mut self) -> PG6_W<DDRG_SPEC> {
        PG6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin G7"]
    #[inline(always)]
    #[must_use]
    pub fn pg7(&mut self) -> PG7_W<DDRG_SPEC> {
        PG7_W::new(self, 7)
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
#[doc = "Data Direction Register, Port G\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRG_SPEC;
impl crate::RegisterSpec for DDRG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddrg::R`](R) reader structure"]
impl crate::Readable for DDRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddrg::W`](W) writer structure"]
impl crate::Writable for DDRG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRG to value 0"]
impl crate::Resettable for DDRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
