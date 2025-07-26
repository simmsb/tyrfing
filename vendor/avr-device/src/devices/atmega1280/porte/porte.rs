#[doc = "Register `PORTE` reader"]
pub type R = crate::R<PORTE_SPEC>;
#[doc = "Register `PORTE` writer"]
pub type W = crate::W<PORTE_SPEC>;
#[doc = "Field `PE0` reader - Pin E0"]
pub type PE0_R = crate::BitReader;
#[doc = "Field `PE0` writer - Pin E0"]
pub type PE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE1` reader - Pin E1"]
pub type PE1_R = crate::BitReader;
#[doc = "Field `PE1` writer - Pin E1"]
pub type PE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE2` reader - Pin E2"]
pub type PE2_R = crate::BitReader;
#[doc = "Field `PE2` writer - Pin E2"]
pub type PE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE3` reader - Pin E3"]
pub type PE3_R = crate::BitReader;
#[doc = "Field `PE3` writer - Pin E3"]
pub type PE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE4` reader - Pin E4"]
pub type PE4_R = crate::BitReader;
#[doc = "Field `PE4` writer - Pin E4"]
pub type PE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE5` reader - Pin E5"]
pub type PE5_R = crate::BitReader;
#[doc = "Field `PE5` writer - Pin E5"]
pub type PE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE6` reader - Pin E6"]
pub type PE6_R = crate::BitReader;
#[doc = "Field `PE6` writer - Pin E6"]
pub type PE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE7` reader - Pin E7"]
pub type PE7_R = crate::BitReader;
#[doc = "Field `PE7` writer - Pin E7"]
pub type PE7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin E0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin E1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin E3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin E4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin E5"]
    #[inline(always)]
    pub fn pe5(&self) -> PE5_R {
        PE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    pub fn pe6(&self) -> PE6_R {
        PE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin E7"]
    #[inline(always)]
    pub fn pe7(&self) -> PE7_R {
        PE7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin E0"]
    #[inline(always)]
    #[must_use]
    pub fn pe0(&mut self) -> PE0_W<PORTE_SPEC> {
        PE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin E1"]
    #[inline(always)]
    #[must_use]
    pub fn pe1(&mut self) -> PE1_W<PORTE_SPEC> {
        PE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE2_W<PORTE_SPEC> {
        PE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin E3"]
    #[inline(always)]
    #[must_use]
    pub fn pe3(&mut self) -> PE3_W<PORTE_SPEC> {
        PE3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin E4"]
    #[inline(always)]
    #[must_use]
    pub fn pe4(&mut self) -> PE4_W<PORTE_SPEC> {
        PE4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin E5"]
    #[inline(always)]
    #[must_use]
    pub fn pe5(&mut self) -> PE5_W<PORTE_SPEC> {
        PE5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin E6"]
    #[inline(always)]
    #[must_use]
    pub fn pe6(&mut self) -> PE6_W<PORTE_SPEC> {
        PE6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin E7"]
    #[inline(always)]
    #[must_use]
    pub fn pe7(&mut self) -> PE7_W<PORTE_SPEC> {
        PE7_W::new(self, 7)
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
#[doc = "Data Register, Port E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`porte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORTE_SPEC;
impl crate::RegisterSpec for PORTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`porte::R`](R) reader structure"]
impl crate::Readable for PORTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`porte::W`](W) writer structure"]
impl crate::Writable for PORTE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTE to value 0"]
impl crate::Resettable for PORTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
