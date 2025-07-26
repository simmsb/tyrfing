#[doc = "Register `EXTENDED` reader"]
pub type R = crate::R<EXTENDED_SPEC>;
#[doc = "Register `EXTENDED` writer"]
pub type W = crate::W<EXTENDED_SPEC>;
#[doc = "Field `BOOTRST` reader - Boot Reset vector Enabled"]
pub type BOOTRST_R = crate::BitReader;
#[doc = "Field `BOOTRST` writer - Boot Reset vector Enabled"]
pub type BOOTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTSZ` reader - Select boot size"]
pub type BOOTSZ_R = crate::FieldReader<BOOTSZ_A>;
#[doc = "Select boot size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOTSZ_A {
    #[doc = "0: Boot Flash size=1024 words Boot address=$0C00"]
    _1024W_0C00 = 0,
    #[doc = "1: Boot Flash size=512 words Boot address=$0E00"]
    _512W_0E00 = 1,
    #[doc = "2: Boot Flash size=256 words Boot address=$0F00"]
    _256W_0F00 = 2,
    #[doc = "3: Boot Flash size=128 words Boot address=$0F80"]
    _128W_0F80 = 3,
}
impl From<BOOTSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTSZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOTSZ_A {
    type Ux = u8;
}
impl BOOTSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOTSZ_A {
        match self.bits {
            0 => BOOTSZ_A::_1024W_0C00,
            1 => BOOTSZ_A::_512W_0E00,
            2 => BOOTSZ_A::_256W_0F00,
            3 => BOOTSZ_A::_128W_0F80,
            _ => unreachable!(),
        }
    }
    #[doc = "Boot Flash size=1024 words Boot address=$0C00"]
    #[inline(always)]
    pub fn is_1024w_0c00(&self) -> bool {
        *self == BOOTSZ_A::_1024W_0C00
    }
    #[doc = "Boot Flash size=512 words Boot address=$0E00"]
    #[inline(always)]
    pub fn is_512w_0e00(&self) -> bool {
        *self == BOOTSZ_A::_512W_0E00
    }
    #[doc = "Boot Flash size=256 words Boot address=$0F00"]
    #[inline(always)]
    pub fn is_256w_0f00(&self) -> bool {
        *self == BOOTSZ_A::_256W_0F00
    }
    #[doc = "Boot Flash size=128 words Boot address=$0F80"]
    #[inline(always)]
    pub fn is_128w_0f80(&self) -> bool {
        *self == BOOTSZ_A::_128W_0F80
    }
}
#[doc = "Field `BOOTSZ` writer - Select boot size"]
pub type BOOTSZ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BOOTSZ_A>;
impl<'a, REG> BOOTSZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Boot Flash size=1024 words Boot address=$0C00"]
    #[inline(always)]
    pub fn _1024w_0c00(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_1024W_0C00)
    }
    #[doc = "Boot Flash size=512 words Boot address=$0E00"]
    #[inline(always)]
    pub fn _512w_0e00(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_512W_0E00)
    }
    #[doc = "Boot Flash size=256 words Boot address=$0F00"]
    #[inline(always)]
    pub fn _256w_0f00(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_256W_0F00)
    }
    #[doc = "Boot Flash size=128 words Boot address=$0F80"]
    #[inline(always)]
    pub fn _128w_0f80(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_128W_0F80)
    }
}
#[doc = "Field `BODACT` reader - BOD mode of operation when the device is active or idle"]
pub type BODACT_R = crate::FieldReader<BODACT_A>;
#[doc = "BOD mode of operation when the device is active or idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODACT_A {
    #[doc = "1: Sampled"]
    BOD_SAMPLED = 1,
    #[doc = "2: Enabled"]
    BOD_ENABLED = 2,
    #[doc = "3: Disabled"]
    BOD_DISABLED = 3,
}
impl From<BODACT_A> for u8 {
    #[inline(always)]
    fn from(variant: BODACT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BODACT_A {
    type Ux = u8;
}
impl BODACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BODACT_A> {
        match self.bits {
            1 => Some(BODACT_A::BOD_SAMPLED),
            2 => Some(BODACT_A::BOD_ENABLED),
            3 => Some(BODACT_A::BOD_DISABLED),
            _ => None,
        }
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn is_bod_sampled(&self) -> bool {
        *self == BODACT_A::BOD_SAMPLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bod_enabled(&self) -> bool {
        *self == BODACT_A::BOD_ENABLED
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_bod_disabled(&self) -> bool {
        *self == BODACT_A::BOD_DISABLED
    }
}
#[doc = "Field `BODACT` writer - BOD mode of operation when the device is active or idle"]
pub type BODACT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BODACT_A>;
impl<'a, REG> BODACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn bod_sampled(self) -> &'a mut crate::W<REG> {
        self.variant(BODACT_A::BOD_SAMPLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bod_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODACT_A::BOD_ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn bod_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODACT_A::BOD_DISABLED)
    }
}
#[doc = "Field `BODPD` reader - BOD mode of operation when the device is in sleep mode"]
pub type BODPD_R = crate::FieldReader<BODPD_A>;
#[doc = "BOD mode of operation when the device is in sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODPD_A {
    #[doc = "1: Sampled"]
    BOD_SAMPLED = 1,
    #[doc = "2: Enabled"]
    BOD_ENABLED = 2,
    #[doc = "3: Disabled"]
    BOD_DISABLED = 3,
}
impl From<BODPD_A> for u8 {
    #[inline(always)]
    fn from(variant: BODPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BODPD_A {
    type Ux = u8;
}
impl BODPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BODPD_A> {
        match self.bits {
            1 => Some(BODPD_A::BOD_SAMPLED),
            2 => Some(BODPD_A::BOD_ENABLED),
            3 => Some(BODPD_A::BOD_DISABLED),
            _ => None,
        }
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn is_bod_sampled(&self) -> bool {
        *self == BODPD_A::BOD_SAMPLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_bod_enabled(&self) -> bool {
        *self == BODPD_A::BOD_ENABLED
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_bod_disabled(&self) -> bool {
        *self == BODPD_A::BOD_DISABLED
    }
}
#[doc = "Field `BODPD` writer - BOD mode of operation when the device is in sleep mode"]
pub type BODPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BODPD_A>;
impl<'a, REG> BODPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn bod_sampled(self) -> &'a mut crate::W<REG> {
        self.variant(BODPD_A::BOD_SAMPLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bod_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODPD_A::BOD_ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn bod_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODPD_A::BOD_DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    pub fn bootrst(&self) -> BOOTRST_R {
        BOOTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Select boot size"]
    #[inline(always)]
    pub fn bootsz(&self) -> BOOTSZ_R {
        BOOTSZ_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 4:5 - BOD mode of operation when the device is active or idle"]
    #[inline(always)]
    pub fn bodact(&self) -> BODACT_R {
        BODACT_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - BOD mode of operation when the device is in sleep mode"]
    #[inline(always)]
    pub fn bodpd(&self) -> BODPD_R {
        BODPD_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bootrst(&mut self) -> BOOTRST_W<EXTENDED_SPEC> {
        BOOTRST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Select boot size"]
    #[inline(always)]
    #[must_use]
    pub fn bootsz(&mut self) -> BOOTSZ_W<EXTENDED_SPEC> {
        BOOTSZ_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - BOD mode of operation when the device is active or idle"]
    #[inline(always)]
    #[must_use]
    pub fn bodact(&mut self) -> BODACT_W<EXTENDED_SPEC> {
        BODACT_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - BOD mode of operation when the device is in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn bodpd(&mut self) -> BODPD_W<EXTENDED_SPEC> {
        BODPD_W::new(self, 6)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extended::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extended::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTENDED_SPEC;
impl crate::RegisterSpec for EXTENDED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`extended::R`](R) reader structure"]
impl crate::Readable for EXTENDED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extended::W`](W) writer structure"]
impl crate::Writable for EXTENDED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTENDED to value 0"]
impl crate::Resettable for EXTENDED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
