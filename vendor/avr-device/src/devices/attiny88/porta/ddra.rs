#[doc = "Register `DDRA` reader"]
pub type R = crate::R<DDRA_SPEC>;
#[doc = "Register `DDRA` writer"]
pub type W = crate::W<DDRA_SPEC>;
#[doc = "Field `PA0` reader - Pin A0"]
pub type PA0_R = crate::BitReader;
#[doc = "Field `PA0` writer - Pin A0"]
pub type PA0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA1` reader - Pin A1"]
pub type PA1_R = crate::BitReader;
#[doc = "Field `PA1` writer - Pin A1"]
pub type PA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA2` reader - Pin A2"]
pub type PA2_R = crate::BitReader;
#[doc = "Field `PA2` writer - Pin A2"]
pub type PA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA3` reader - Pin A3"]
pub type PA3_R = crate::BitReader;
#[doc = "Field `PA3` writer - Pin A3"]
pub type PA3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    pub fn pa0(&self) -> PA0_R {
        PA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    pub fn pa1(&self) -> PA1_R {
        PA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    pub fn pa2(&self) -> PA2_R {
        PA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    pub fn pa3(&self) -> PA3_R {
        PA3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    #[must_use]
    pub fn pa0(&mut self) -> PA0_W<DDRA_SPEC> {
        PA0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    #[must_use]
    pub fn pa1(&mut self) -> PA1_W<DDRA_SPEC> {
        PA1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    #[must_use]
    pub fn pa2(&mut self) -> PA2_W<DDRA_SPEC> {
        PA2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    #[must_use]
    pub fn pa3(&mut self) -> PA3_W<DDRA_SPEC> {
        PA3_W::new(self, 3)
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
#[doc = "Port A Data Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRA_SPEC;
impl crate::RegisterSpec for DDRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddra::R`](R) reader structure"]
impl crate::Readable for DDRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddra::W`](W) writer structure"]
impl crate::Writable for DDRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRA to value 0"]
impl crate::Resettable for DDRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
