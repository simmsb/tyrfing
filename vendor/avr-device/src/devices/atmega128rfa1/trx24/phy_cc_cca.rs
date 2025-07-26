#[doc = "Register `PHY_CC_CCA` reader"]
pub type R = crate::R<PHY_CC_CCA_SPEC>;
#[doc = "Register `PHY_CC_CCA` writer"]
pub type W = crate::W<PHY_CC_CCA_SPEC>;
#[doc = "Field `CHANNEL` reader - RX/TX Channel Selection"]
pub type CHANNEL_R = crate::FieldReader<CHANNEL_A>;
#[doc = "RX/TX Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNEL_A {
    #[doc = "11: 2405 MHz"]
    F_2405MHZ = 11,
    #[doc = "12: 2410 MHz"]
    F_2410MHZ = 12,
    #[doc = "13: 2415 MHz"]
    F_2415MHZ = 13,
    #[doc = "14: 2420 MHz"]
    F_2420MHZ = 14,
    #[doc = "15: 2425 MHz"]
    F_2425MHZ = 15,
    #[doc = "16: 2430 MHz"]
    F_2430MHZ = 16,
    #[doc = "17: 2435 MHz"]
    F_2435MHZ = 17,
    #[doc = "18: 2440 MHz"]
    F_2440MHZ = 18,
    #[doc = "19: 2445 MHz"]
    F_2445MHZ = 19,
    #[doc = "20: 2450 MHz"]
    F_2450MHZ = 20,
    #[doc = "21: 2455 MHz"]
    F_2455MHZ = 21,
    #[doc = "22: 2460 MHz"]
    F_2460MHZ = 22,
    #[doc = "23: 2465 MHz"]
    F_2465MHZ = 23,
    #[doc = "24: 2470 MHz"]
    F_2470MHZ = 24,
    #[doc = "25: 2475 MHz"]
    F_2475MHZ = 25,
    #[doc = "26: 2480 MHz"]
    F_2480MHZ = 26,
}
impl From<CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHANNEL_A {
    type Ux = u8;
}
impl CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHANNEL_A> {
        match self.bits {
            11 => Some(CHANNEL_A::F_2405MHZ),
            12 => Some(CHANNEL_A::F_2410MHZ),
            13 => Some(CHANNEL_A::F_2415MHZ),
            14 => Some(CHANNEL_A::F_2420MHZ),
            15 => Some(CHANNEL_A::F_2425MHZ),
            16 => Some(CHANNEL_A::F_2430MHZ),
            17 => Some(CHANNEL_A::F_2435MHZ),
            18 => Some(CHANNEL_A::F_2440MHZ),
            19 => Some(CHANNEL_A::F_2445MHZ),
            20 => Some(CHANNEL_A::F_2450MHZ),
            21 => Some(CHANNEL_A::F_2455MHZ),
            22 => Some(CHANNEL_A::F_2460MHZ),
            23 => Some(CHANNEL_A::F_2465MHZ),
            24 => Some(CHANNEL_A::F_2470MHZ),
            25 => Some(CHANNEL_A::F_2475MHZ),
            26 => Some(CHANNEL_A::F_2480MHZ),
            _ => None,
        }
    }
    #[doc = "2405 MHz"]
    #[inline(always)]
    pub fn is_f_2405mhz(&self) -> bool {
        *self == CHANNEL_A::F_2405MHZ
    }
    #[doc = "2410 MHz"]
    #[inline(always)]
    pub fn is_f_2410mhz(&self) -> bool {
        *self == CHANNEL_A::F_2410MHZ
    }
    #[doc = "2415 MHz"]
    #[inline(always)]
    pub fn is_f_2415mhz(&self) -> bool {
        *self == CHANNEL_A::F_2415MHZ
    }
    #[doc = "2420 MHz"]
    #[inline(always)]
    pub fn is_f_2420mhz(&self) -> bool {
        *self == CHANNEL_A::F_2420MHZ
    }
    #[doc = "2425 MHz"]
    #[inline(always)]
    pub fn is_f_2425mhz(&self) -> bool {
        *self == CHANNEL_A::F_2425MHZ
    }
    #[doc = "2430 MHz"]
    #[inline(always)]
    pub fn is_f_2430mhz(&self) -> bool {
        *self == CHANNEL_A::F_2430MHZ
    }
    #[doc = "2435 MHz"]
    #[inline(always)]
    pub fn is_f_2435mhz(&self) -> bool {
        *self == CHANNEL_A::F_2435MHZ
    }
    #[doc = "2440 MHz"]
    #[inline(always)]
    pub fn is_f_2440mhz(&self) -> bool {
        *self == CHANNEL_A::F_2440MHZ
    }
    #[doc = "2445 MHz"]
    #[inline(always)]
    pub fn is_f_2445mhz(&self) -> bool {
        *self == CHANNEL_A::F_2445MHZ
    }
    #[doc = "2450 MHz"]
    #[inline(always)]
    pub fn is_f_2450mhz(&self) -> bool {
        *self == CHANNEL_A::F_2450MHZ
    }
    #[doc = "2455 MHz"]
    #[inline(always)]
    pub fn is_f_2455mhz(&self) -> bool {
        *self == CHANNEL_A::F_2455MHZ
    }
    #[doc = "2460 MHz"]
    #[inline(always)]
    pub fn is_f_2460mhz(&self) -> bool {
        *self == CHANNEL_A::F_2460MHZ
    }
    #[doc = "2465 MHz"]
    #[inline(always)]
    pub fn is_f_2465mhz(&self) -> bool {
        *self == CHANNEL_A::F_2465MHZ
    }
    #[doc = "2470 MHz"]
    #[inline(always)]
    pub fn is_f_2470mhz(&self) -> bool {
        *self == CHANNEL_A::F_2470MHZ
    }
    #[doc = "2475 MHz"]
    #[inline(always)]
    pub fn is_f_2475mhz(&self) -> bool {
        *self == CHANNEL_A::F_2475MHZ
    }
    #[doc = "2480 MHz"]
    #[inline(always)]
    pub fn is_f_2480mhz(&self) -> bool {
        *self == CHANNEL_A::F_2480MHZ
    }
}
#[doc = "Field `CHANNEL` writer - RX/TX Channel Selection"]
pub type CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, CHANNEL_A>;
impl<'a, REG> CHANNEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2405 MHz"]
    #[inline(always)]
    pub fn f_2405mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2405MHZ)
    }
    #[doc = "2410 MHz"]
    #[inline(always)]
    pub fn f_2410mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2410MHZ)
    }
    #[doc = "2415 MHz"]
    #[inline(always)]
    pub fn f_2415mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2415MHZ)
    }
    #[doc = "2420 MHz"]
    #[inline(always)]
    pub fn f_2420mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2420MHZ)
    }
    #[doc = "2425 MHz"]
    #[inline(always)]
    pub fn f_2425mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2425MHZ)
    }
    #[doc = "2430 MHz"]
    #[inline(always)]
    pub fn f_2430mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2430MHZ)
    }
    #[doc = "2435 MHz"]
    #[inline(always)]
    pub fn f_2435mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2435MHZ)
    }
    #[doc = "2440 MHz"]
    #[inline(always)]
    pub fn f_2440mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2440MHZ)
    }
    #[doc = "2445 MHz"]
    #[inline(always)]
    pub fn f_2445mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2445MHZ)
    }
    #[doc = "2450 MHz"]
    #[inline(always)]
    pub fn f_2450mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2450MHZ)
    }
    #[doc = "2455 MHz"]
    #[inline(always)]
    pub fn f_2455mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2455MHZ)
    }
    #[doc = "2460 MHz"]
    #[inline(always)]
    pub fn f_2460mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2460MHZ)
    }
    #[doc = "2465 MHz"]
    #[inline(always)]
    pub fn f_2465mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2465MHZ)
    }
    #[doc = "2470 MHz"]
    #[inline(always)]
    pub fn f_2470mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2470MHZ)
    }
    #[doc = "2475 MHz"]
    #[inline(always)]
    pub fn f_2475mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2475MHZ)
    }
    #[doc = "2480 MHz"]
    #[inline(always)]
    pub fn f_2480mhz(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::F_2480MHZ)
    }
}
#[doc = "Field `CCA_MODE` reader - Select CCA Measurement Mode"]
pub type CCA_MODE_R = crate::FieldReader<CCA_MODE_A>;
#[doc = "Select CCA Measurement Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCA_MODE_A {
    #[doc = "0: Mode 3a, Carrier sense OR energy above threshold"]
    CCA_CS_OR_ED = 0,
    #[doc = "1: Mode 1, Energy above threshold"]
    CCA_ED = 1,
    #[doc = "2: Mode 2, Carrier sense only"]
    CCA_CS = 2,
    #[doc = "3: Mode 3b, Carrier sense AND energy above threshold"]
    CCA_CS_AND_ED = 3,
}
impl From<CCA_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCA_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCA_MODE_A {
    type Ux = u8;
}
impl CCA_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCA_MODE_A {
        match self.bits {
            0 => CCA_MODE_A::CCA_CS_OR_ED,
            1 => CCA_MODE_A::CCA_ED,
            2 => CCA_MODE_A::CCA_CS,
            3 => CCA_MODE_A::CCA_CS_AND_ED,
            _ => unreachable!(),
        }
    }
    #[doc = "Mode 3a, Carrier sense OR energy above threshold"]
    #[inline(always)]
    pub fn is_cca_cs_or_ed(&self) -> bool {
        *self == CCA_MODE_A::CCA_CS_OR_ED
    }
    #[doc = "Mode 1, Energy above threshold"]
    #[inline(always)]
    pub fn is_cca_ed(&self) -> bool {
        *self == CCA_MODE_A::CCA_ED
    }
    #[doc = "Mode 2, Carrier sense only"]
    #[inline(always)]
    pub fn is_cca_cs(&self) -> bool {
        *self == CCA_MODE_A::CCA_CS
    }
    #[doc = "Mode 3b, Carrier sense AND energy above threshold"]
    #[inline(always)]
    pub fn is_cca_cs_and_ed(&self) -> bool {
        *self == CCA_MODE_A::CCA_CS_AND_ED
    }
}
#[doc = "Field `CCA_MODE` writer - Select CCA Measurement Mode"]
pub type CCA_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CCA_MODE_A>;
impl<'a, REG> CCA_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 3a, Carrier sense OR energy above threshold"]
    #[inline(always)]
    pub fn cca_cs_or_ed(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_MODE_A::CCA_CS_OR_ED)
    }
    #[doc = "Mode 1, Energy above threshold"]
    #[inline(always)]
    pub fn cca_ed(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_MODE_A::CCA_ED)
    }
    #[doc = "Mode 2, Carrier sense only"]
    #[inline(always)]
    pub fn cca_cs(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_MODE_A::CCA_CS)
    }
    #[doc = "Mode 3b, Carrier sense AND energy above threshold"]
    #[inline(always)]
    pub fn cca_cs_and_ed(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_MODE_A::CCA_CS_AND_ED)
    }
}
#[doc = "Field `CCA_REQUEST` reader - Manual CCA Measurement Request"]
pub type CCA_REQUEST_R = crate::BitReader;
#[doc = "Field `CCA_REQUEST` writer - Manual CCA Measurement Request"]
pub type CCA_REQUEST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - RX/TX Channel Selection"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:6 - Select CCA Measurement Mode"]
    #[inline(always)]
    pub fn cca_mode(&self) -> CCA_MODE_R {
        CCA_MODE_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - Manual CCA Measurement Request"]
    #[inline(always)]
    pub fn cca_request(&self) -> CCA_REQUEST_R {
        CCA_REQUEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - RX/TX Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<PHY_CC_CCA_SPEC> {
        CHANNEL_W::new(self, 0)
    }
    #[doc = "Bits 5:6 - Select CCA Measurement Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cca_mode(&mut self) -> CCA_MODE_W<PHY_CC_CCA_SPEC> {
        CCA_MODE_W::new(self, 5)
    }
    #[doc = "Bit 7 - Manual CCA Measurement Request"]
    #[inline(always)]
    #[must_use]
    pub fn cca_request(&mut self) -> CCA_REQUEST_W<PHY_CC_CCA_SPEC> {
        CCA_REQUEST_W::new(self, 7)
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
#[doc = "Transceiver Clear Channel Assessment (CCA) Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_cc_cca::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_cc_cca::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_CC_CCA_SPEC;
impl crate::RegisterSpec for PHY_CC_CCA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_cc_cca::R`](R) reader structure"]
impl crate::Readable for PHY_CC_CCA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_cc_cca::W`](W) writer structure"]
impl crate::Writable for PHY_CC_CCA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_CC_CCA to value 0"]
impl crate::Resettable for PHY_CC_CCA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
