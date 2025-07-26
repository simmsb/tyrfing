#[doc = "Register `EVSYSROUTEA` reader"]
pub type R = crate::R<EVSYSROUTEA_SPEC>;
#[doc = "Register `EVSYSROUTEA` writer"]
pub type W = crate::W<EVSYSROUTEA_SPEC>;
#[doc = "Field `EVOUT0` reader - Event Output 0"]
pub type EVOUT0_R = crate::BitReader;
#[doc = "Field `EVOUT0` writer - Event Output 0"]
pub type EVOUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOUT1` reader - Event Output 1"]
pub type EVOUT1_R = crate::BitReader;
#[doc = "Field `EVOUT1` writer - Event Output 1"]
pub type EVOUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOUT2` reader - Event Output 2"]
pub type EVOUT2_R = crate::BitReader;
#[doc = "Field `EVOUT2` writer - Event Output 2"]
pub type EVOUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOUT3` reader - Event Output 3"]
pub type EVOUT3_R = crate::BitReader;
#[doc = "Field `EVOUT3` writer - Event Output 3"]
pub type EVOUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOUT4` reader - Event Output 4"]
pub type EVOUT4_R = crate::BitReader;
#[doc = "Field `EVOUT4` writer - Event Output 4"]
pub type EVOUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOUT5` reader - Event Output 5"]
pub type EVOUT5_R = crate::BitReader;
#[doc = "Field `EVOUT5` writer - Event Output 5"]
pub type EVOUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event Output 0"]
    #[inline(always)]
    pub fn evout0(&self) -> EVOUT0_R {
        EVOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Output 1"]
    #[inline(always)]
    pub fn evout1(&self) -> EVOUT1_R {
        EVOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Output 2"]
    #[inline(always)]
    pub fn evout2(&self) -> EVOUT2_R {
        EVOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Output 3"]
    #[inline(always)]
    pub fn evout3(&self) -> EVOUT3_R {
        EVOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Output 4"]
    #[inline(always)]
    pub fn evout4(&self) -> EVOUT4_R {
        EVOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Output 5"]
    #[inline(always)]
    pub fn evout5(&self) -> EVOUT5_R {
        EVOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn evout0(&mut self) -> EVOUT0_W<EVSYSROUTEA_SPEC> {
        EVOUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Event Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn evout1(&mut self) -> EVOUT1_W<EVSYSROUTEA_SPEC> {
        EVOUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Event Output 2"]
    #[inline(always)]
    #[must_use]
    pub fn evout2(&mut self) -> EVOUT2_W<EVSYSROUTEA_SPEC> {
        EVOUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Event Output 3"]
    #[inline(always)]
    #[must_use]
    pub fn evout3(&mut self) -> EVOUT3_W<EVSYSROUTEA_SPEC> {
        EVOUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Event Output 4"]
    #[inline(always)]
    #[must_use]
    pub fn evout4(&mut self) -> EVOUT4_W<EVSYSROUTEA_SPEC> {
        EVOUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Event Output 5"]
    #[inline(always)]
    #[must_use]
    pub fn evout5(&mut self) -> EVOUT5_W<EVSYSROUTEA_SPEC> {
        EVOUT5_W::new(self, 5)
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
#[doc = "Port Multiplexer EVSYS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evsysroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evsysroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVSYSROUTEA_SPEC;
impl crate::RegisterSpec for EVSYSROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evsysroutea::R`](R) reader structure"]
impl crate::Readable for EVSYSROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evsysroutea::W`](W) writer structure"]
impl crate::Writable for EVSYSROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSYSROUTEA to value 0"]
impl crate::Resettable for EVSYSROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
