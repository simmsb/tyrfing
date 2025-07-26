#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<SINGLE_CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<SINGLE_CTRLB_SPEC>;
#[doc = "Field `WGMODE` reader - Waveform generation mode"]
pub type WGMODE_R = crate::FieldReader<WGMODE_A>;
#[doc = "Waveform generation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGMODE_A {
    #[doc = "0: Normal Mode"]
    NORMAL = 0,
    #[doc = "1: Frequency Generation Mode"]
    FRQ = 1,
    #[doc = "3: Single Slope PWM"]
    SINGLESLOPE = 3,
    #[doc = "5: Dual Slope PWM, overflow on TOP"]
    DSTOP = 5,
    #[doc = "6: Dual Slope PWM, overflow on TOP and BOTTOM"]
    DSBOTH = 6,
    #[doc = "7: Dual Slope PWM, overflow on BOTTOM"]
    DSBOTTOM = 7,
}
impl From<WGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WGMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGMODE_A {
    type Ux = u8;
}
impl WGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WGMODE_A> {
        match self.bits {
            0 => Some(WGMODE_A::NORMAL),
            1 => Some(WGMODE_A::FRQ),
            3 => Some(WGMODE_A::SINGLESLOPE),
            5 => Some(WGMODE_A::DSTOP),
            6 => Some(WGMODE_A::DSBOTH),
            7 => Some(WGMODE_A::DSBOTTOM),
            _ => None,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WGMODE_A::NORMAL
    }
    #[doc = "Frequency Generation Mode"]
    #[inline(always)]
    pub fn is_frq(&self) -> bool {
        *self == WGMODE_A::FRQ
    }
    #[doc = "Single Slope PWM"]
    #[inline(always)]
    pub fn is_singleslope(&self) -> bool {
        *self == WGMODE_A::SINGLESLOPE
    }
    #[doc = "Dual Slope PWM, overflow on TOP"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == WGMODE_A::DSTOP
    }
    #[doc = "Dual Slope PWM, overflow on TOP and BOTTOM"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == WGMODE_A::DSBOTH
    }
    #[doc = "Dual Slope PWM, overflow on BOTTOM"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == WGMODE_A::DSBOTTOM
    }
}
#[doc = "Field `WGMODE` writer - Waveform generation mode"]
pub type WGMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WGMODE_A>;
impl<'a, REG> WGMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::NORMAL)
    }
    #[doc = "Frequency Generation Mode"]
    #[inline(always)]
    pub fn frq(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::FRQ)
    }
    #[doc = "Single Slope PWM"]
    #[inline(always)]
    pub fn singleslope(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::SINGLESLOPE)
    }
    #[doc = "Dual Slope PWM, overflow on TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::DSTOP)
    }
    #[doc = "Dual Slope PWM, overflow on TOP and BOTTOM"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::DSBOTH)
    }
    #[doc = "Dual Slope PWM, overflow on BOTTOM"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::DSBOTTOM)
    }
}
#[doc = "Field `ALUPD` reader - Auto Lock Update"]
pub type ALUPD_R = crate::BitReader;
#[doc = "Field `ALUPD` writer - Auto Lock Update"]
pub type ALUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0EN` reader - Compare 0 Enable"]
pub type CMP0EN_R = crate::BitReader;
#[doc = "Field `CMP0EN` writer - Compare 0 Enable"]
pub type CMP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1EN` reader - Compare 1 Enable"]
pub type CMP1EN_R = crate::BitReader;
#[doc = "Field `CMP1EN` writer - Compare 1 Enable"]
pub type CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2EN` reader - Compare 2 Enable"]
pub type CMP2EN_R = crate::BitReader;
#[doc = "Field `CMP2EN` writer - Compare 2 Enable"]
pub type CMP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Waveform generation mode"]
    #[inline(always)]
    pub fn wgmode(&self) -> WGMODE_R {
        WGMODE_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Auto Lock Update"]
    #[inline(always)]
    pub fn alupd(&self) -> ALUPD_R {
        ALUPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare 0 Enable"]
    #[inline(always)]
    pub fn cmp0en(&self) -> CMP0EN_R {
        CMP0EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare 1 Enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare 2 Enable"]
    #[inline(always)]
    pub fn cmp2en(&self) -> CMP2EN_R {
        CMP2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Waveform generation mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgmode(&mut self) -> WGMODE_W<SINGLE_CTRLB_SPEC> {
        WGMODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Auto Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn alupd(&mut self) -> ALUPD_W<SINGLE_CTRLB_SPEC> {
        ALUPD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Compare 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0en(&mut self) -> CMP0EN_W<SINGLE_CTRLB_SPEC> {
        CMP0EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Compare 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> CMP1EN_W<SINGLE_CTRLB_SPEC> {
        CMP1EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Compare 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2en(&mut self) -> CMP2EN_W<SINGLE_CTRLB_SPEC> {
        CMP2EN_W::new(self, 6)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_CTRLB_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`single_ctrlb::R`](R) reader structure"]
impl crate::Readable for SINGLE_CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_ctrlb::W`](W) writer structure"]
impl crate::Writable for SINGLE_CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for SINGLE_CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
