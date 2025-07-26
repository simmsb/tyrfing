#[doc = "Register `PORTH` reader"]
pub type R = crate::R<PORTH_SPEC>;
#[doc = "Register `PORTH` writer"]
pub type W = crate::W<PORTH_SPEC>;
#[doc = "Field `PH0` reader - Pin H0"]
pub type PH0_R = crate::BitReader;
#[doc = "Field `PH0` writer - Pin H0"]
pub type PH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH1` reader - Pin H1"]
pub type PH1_R = crate::BitReader;
#[doc = "Field `PH1` writer - Pin H1"]
pub type PH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH2` reader - Pin H2"]
pub type PH2_R = crate::BitReader;
#[doc = "Field `PH2` writer - Pin H2"]
pub type PH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH3` reader - Pin H3"]
pub type PH3_R = crate::BitReader;
#[doc = "Field `PH3` writer - Pin H3"]
pub type PH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH4` reader - Pin H4"]
pub type PH4_R = crate::BitReader;
#[doc = "Field `PH4` writer - Pin H4"]
pub type PH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH5` reader - Pin H5"]
pub type PH5_R = crate::BitReader;
#[doc = "Field `PH5` writer - Pin H5"]
pub type PH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH6` reader - Pin H6"]
pub type PH6_R = crate::BitReader;
#[doc = "Field `PH6` writer - Pin H6"]
pub type PH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH7` reader - Pin H7"]
pub type PH7_R = crate::BitReader;
#[doc = "Field `PH7` writer - Pin H7"]
pub type PH7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin H0"]
    #[inline(always)]
    pub fn ph0(&self) -> PH0_R {
        PH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin H1"]
    #[inline(always)]
    pub fn ph1(&self) -> PH1_R {
        PH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin H2"]
    #[inline(always)]
    pub fn ph2(&self) -> PH2_R {
        PH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin H3"]
    #[inline(always)]
    pub fn ph3(&self) -> PH3_R {
        PH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin H4"]
    #[inline(always)]
    pub fn ph4(&self) -> PH4_R {
        PH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin H5"]
    #[inline(always)]
    pub fn ph5(&self) -> PH5_R {
        PH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin H6"]
    #[inline(always)]
    pub fn ph6(&self) -> PH6_R {
        PH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin H7"]
    #[inline(always)]
    pub fn ph7(&self) -> PH7_R {
        PH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin H0"]
    #[inline(always)]
    #[must_use]
    pub fn ph0(&mut self) -> PH0_W<PORTH_SPEC> {
        PH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin H1"]
    #[inline(always)]
    #[must_use]
    pub fn ph1(&mut self) -> PH1_W<PORTH_SPEC> {
        PH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin H2"]
    #[inline(always)]
    #[must_use]
    pub fn ph2(&mut self) -> PH2_W<PORTH_SPEC> {
        PH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin H3"]
    #[inline(always)]
    #[must_use]
    pub fn ph3(&mut self) -> PH3_W<PORTH_SPEC> {
        PH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin H4"]
    #[inline(always)]
    #[must_use]
    pub fn ph4(&mut self) -> PH4_W<PORTH_SPEC> {
        PH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin H5"]
    #[inline(always)]
    #[must_use]
    pub fn ph5(&mut self) -> PH5_W<PORTH_SPEC> {
        PH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin H6"]
    #[inline(always)]
    #[must_use]
    pub fn ph6(&mut self) -> PH6_W<PORTH_SPEC> {
        PH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin H7"]
    #[inline(always)]
    #[must_use]
    pub fn ph7(&mut self) -> PH7_W<PORTH_SPEC> {
        PH7_W::new(self, 7)
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
#[doc = "PORT H Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`porth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORTH_SPEC;
impl crate::RegisterSpec for PORTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`porth::R`](R) reader structure"]
impl crate::Readable for PORTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`porth::W`](W) writer structure"]
impl crate::Writable for PORTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTH to value 0"]
impl crate::Resettable for PORTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
