#[doc = "Register `CCLROUTEA` reader"]
pub type R = crate::R<CCLROUTEA_SPEC>;
#[doc = "Register `CCLROUTEA` writer"]
pub type W = crate::W<CCLROUTEA_SPEC>;
#[doc = "Field `LUT0` reader - CCL LUT0"]
pub type LUT0_R = crate::BitReader;
#[doc = "Field `LUT0` writer - CCL LUT0"]
pub type LUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT1` reader - CCL LUT1"]
pub type LUT1_R = crate::BitReader;
#[doc = "Field `LUT1` writer - CCL LUT1"]
pub type LUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT2` reader - CCL LUT2"]
pub type LUT2_R = crate::BitReader;
#[doc = "Field `LUT2` writer - CCL LUT2"]
pub type LUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUT3` reader - CCL LUT3"]
pub type LUT3_R = crate::BitReader;
#[doc = "Field `LUT3` writer - CCL LUT3"]
pub type LUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCL LUT0"]
    #[inline(always)]
    pub fn lut0(&self) -> LUT0_R {
        LUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCL LUT1"]
    #[inline(always)]
    pub fn lut1(&self) -> LUT1_R {
        LUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCL LUT2"]
    #[inline(always)]
    pub fn lut2(&self) -> LUT2_R {
        LUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCL LUT3"]
    #[inline(always)]
    pub fn lut3(&self) -> LUT3_R {
        LUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCL LUT0"]
    #[inline(always)]
    #[must_use]
    pub fn lut0(&mut self) -> LUT0_W<CCLROUTEA_SPEC> {
        LUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - CCL LUT1"]
    #[inline(always)]
    #[must_use]
    pub fn lut1(&mut self) -> LUT1_W<CCLROUTEA_SPEC> {
        LUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - CCL LUT2"]
    #[inline(always)]
    #[must_use]
    pub fn lut2(&mut self) -> LUT2_W<CCLROUTEA_SPEC> {
        LUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCL LUT3"]
    #[inline(always)]
    #[must_use]
    pub fn lut3(&mut self) -> LUT3_W<CCLROUTEA_SPEC> {
        LUT3_W::new(self, 3)
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
#[doc = "Port Multiplexer CCL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cclroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cclroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCLROUTEA_SPEC;
impl crate::RegisterSpec for CCLROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cclroutea::R`](R) reader structure"]
impl crate::Readable for CCLROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cclroutea::W`](W) writer structure"]
impl crate::Writable for CCLROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCLROUTEA to value 0"]
impl crate::Resettable for CCLROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
