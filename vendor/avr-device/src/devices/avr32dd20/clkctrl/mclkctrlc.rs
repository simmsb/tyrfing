#[doc = "Register `MCLKCTRLC` reader"]
pub type R = crate::R<MCLKCTRLC_SPEC>;
#[doc = "Register `MCLKCTRLC` writer"]
pub type W = crate::W<MCLKCTRLC_SPEC>;
#[doc = "Field `CFDEN` reader - Clock Failure Detect Enable"]
pub type CFDEN_R = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detect Enable"]
pub type CFDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDTST` writer - Clock Failure Detect Test"]
pub type CFDTST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDSRC` reader - Clock Failure Detect Source"]
pub type CFDSRC_R = crate::FieldReader<CFDSRC_A>;
#[doc = "Clock Failure Detect Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFDSRC_A {
    #[doc = "0: Main Clock"]
    CLKMAIN = 0,
    #[doc = "1: XOSCHF"]
    XOSCHF = 1,
    #[doc = "2: XOSC32K"]
    XOSC32K = 2,
}
impl From<CFDSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFDSRC_A {
    type Ux = u8;
}
impl CFDSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CFDSRC_A> {
        match self.bits {
            0 => Some(CFDSRC_A::CLKMAIN),
            1 => Some(CFDSRC_A::XOSCHF),
            2 => Some(CFDSRC_A::XOSC32K),
            _ => None,
        }
    }
    #[doc = "Main Clock"]
    #[inline(always)]
    pub fn is_clkmain(&self) -> bool {
        *self == CFDSRC_A::CLKMAIN
    }
    #[doc = "XOSCHF"]
    #[inline(always)]
    pub fn is_xoschf(&self) -> bool {
        *self == CFDSRC_A::XOSCHF
    }
    #[doc = "XOSC32K"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == CFDSRC_A::XOSC32K
    }
}
#[doc = "Field `CFDSRC` writer - Clock Failure Detect Source"]
pub type CFDSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CFDSRC_A>;
impl<'a, REG> CFDSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Clock"]
    #[inline(always)]
    pub fn clkmain(self) -> &'a mut crate::W<REG> {
        self.variant(CFDSRC_A::CLKMAIN)
    }
    #[doc = "XOSCHF"]
    #[inline(always)]
    pub fn xoschf(self) -> &'a mut crate::W<REG> {
        self.variant(CFDSRC_A::XOSCHF)
    }
    #[doc = "XOSC32K"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut crate::W<REG> {
        self.variant(CFDSRC_A::XOSC32K)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detect Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock Failure Detect Source"]
    #[inline(always)]
    pub fn cfdsrc(&self) -> CFDSRC_R {
        CFDSRC_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<MCLKCTRLC_SPEC> {
        CFDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Failure Detect Test"]
    #[inline(always)]
    #[must_use]
    pub fn cfdtst(&mut self) -> CFDTST_W<MCLKCTRLC_SPEC> {
        CFDTST_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Clock Failure Detect Source"]
    #[inline(always)]
    #[must_use]
    pub fn cfdsrc(&mut self) -> CFDSRC_W<MCLKCTRLC_SPEC> {
        CFDSRC_W::new(self, 2)
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
#[doc = "MCLK Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKCTRLC_SPEC;
impl crate::RegisterSpec for MCLKCTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclkctrlc::R`](R) reader structure"]
impl crate::Readable for MCLKCTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclkctrlc::W`](W) writer structure"]
impl crate::Writable for MCLKCTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKCTRLC to value 0"]
impl crate::Resettable for MCLKCTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
