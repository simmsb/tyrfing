#[doc = "Register `BODCFG` reader"]
pub type R = crate::R<BODCFG_SPEC>;
#[doc = "Register `BODCFG` writer"]
pub type W = crate::W<BODCFG_SPEC>;
#[doc = "Field `SLEEP` reader - BOD Operation in Sleep Mode"]
pub type SLEEP_R = crate::FieldReader<SLEEP_A>;
#[doc = "BOD Operation in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLEEP_A {
    #[doc = "0: BOD disabled"]
    DISABLE = 0,
    #[doc = "1: BOD enabled in continuous mode"]
    ENABLE = 1,
    #[doc = "2: BOD enabled in sampled mode"]
    SAMPLE = 2,
}
impl From<SLEEP_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLEEP_A {
    type Ux = u8;
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SLEEP_A> {
        match self.bits {
            0 => Some(SLEEP_A::DISABLE),
            1 => Some(SLEEP_A::ENABLE),
            2 => Some(SLEEP_A::SAMPLE),
            _ => None,
        }
    }
    #[doc = "BOD disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SLEEP_A::DISABLE
    }
    #[doc = "BOD enabled in continuous mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SLEEP_A::ENABLE
    }
    #[doc = "BOD enabled in sampled mode"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == SLEEP_A::SAMPLE
    }
}
#[doc = "Field `SLEEP` writer - BOD Operation in Sleep Mode"]
pub type SLEEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SLEEP_A>;
impl<'a, REG> SLEEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BOD disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::DISABLE)
    }
    #[doc = "BOD enabled in continuous mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::ENABLE)
    }
    #[doc = "BOD enabled in sampled mode"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_A::SAMPLE)
    }
}
#[doc = "Field `ACTIVE` reader - BOD Operation in Active Mode"]
pub type ACTIVE_R = crate::FieldReader<ACTIVE_A>;
#[doc = "BOD Operation in Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIVE_A {
    #[doc = "0: BOD disabled"]
    DISABLE = 0,
    #[doc = "1: BOD enabled in continuous mode"]
    ENABLE = 1,
    #[doc = "2: BOD enabled in sampled mode"]
    SAMPLE = 2,
    #[doc = "3: BOD enabled in continuous mode. Execution is halted at wake-up until BOD is running."]
    ENABLEWAIT = 3,
}
impl From<ACTIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTIVE_A {
    type Ux = u8;
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACTIVE_A {
        match self.bits {
            0 => ACTIVE_A::DISABLE,
            1 => ACTIVE_A::ENABLE,
            2 => ACTIVE_A::SAMPLE,
            3 => ACTIVE_A::ENABLEWAIT,
            _ => unreachable!(),
        }
    }
    #[doc = "BOD disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACTIVE_A::DISABLE
    }
    #[doc = "BOD enabled in continuous mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACTIVE_A::ENABLE
    }
    #[doc = "BOD enabled in sampled mode"]
    #[inline(always)]
    pub fn is_sample(&self) -> bool {
        *self == ACTIVE_A::SAMPLE
    }
    #[doc = "BOD enabled in continuous mode. Execution is halted at wake-up until BOD is running."]
    #[inline(always)]
    pub fn is_enablewait(&self) -> bool {
        *self == ACTIVE_A::ENABLEWAIT
    }
}
#[doc = "Field `ACTIVE` writer - BOD Operation in Active Mode"]
pub type ACTIVE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ACTIVE_A>;
impl<'a, REG> ACTIVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BOD disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIVE_A::DISABLE)
    }
    #[doc = "BOD enabled in continuous mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIVE_A::ENABLE)
    }
    #[doc = "BOD enabled in sampled mode"]
    #[inline(always)]
    pub fn sample(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIVE_A::SAMPLE)
    }
    #[doc = "BOD enabled in continuous mode. Execution is halted at wake-up until BOD is running."]
    #[inline(always)]
    pub fn enablewait(self) -> &'a mut crate::W<REG> {
        self.variant(ACTIVE_A::ENABLEWAIT)
    }
}
#[doc = "Field `SAMPFREQ` reader - BOD Sample Frequency"]
pub type SAMPFREQ_R = crate::BitReader<SAMPFREQ_A>;
#[doc = "BOD Sample Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPFREQ_A {
    #[doc = "0: Sample frequency is 128 Hz"]
    _128HZ = 0,
    #[doc = "1: Sample frequency is 32 Hz"]
    _32HZ = 1,
}
impl From<SAMPFREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPFREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAMPFREQ_A {
        match self.bits {
            false => SAMPFREQ_A::_128HZ,
            true => SAMPFREQ_A::_32HZ,
        }
    }
    #[doc = "Sample frequency is 128 Hz"]
    #[inline(always)]
    pub fn is_128hz(&self) -> bool {
        *self == SAMPFREQ_A::_128HZ
    }
    #[doc = "Sample frequency is 32 Hz"]
    #[inline(always)]
    pub fn is_32hz(&self) -> bool {
        *self == SAMPFREQ_A::_32HZ
    }
}
#[doc = "Field `SAMPFREQ` writer - BOD Sample Frequency"]
pub type SAMPFREQ_W<'a, REG> = crate::BitWriter<'a, REG, SAMPFREQ_A>;
impl<'a, REG> SAMPFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample frequency is 128 Hz"]
    #[inline(always)]
    pub fn _128hz(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPFREQ_A::_128HZ)
    }
    #[doc = "Sample frequency is 32 Hz"]
    #[inline(always)]
    pub fn _32hz(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPFREQ_A::_32HZ)
    }
}
#[doc = "Field `LVL` reader - BOD Level"]
pub type LVL_R = crate::FieldReader<LVL_A>;
#[doc = "BOD Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.9V"]
    BODLEVEL0 = 0,
    #[doc = "1: 2.45V"]
    BODLEVEL1 = 1,
    #[doc = "2: 2.7V"]
    BODLEVEL2 = 2,
    #[doc = "3: 2.85V"]
    BODLEVEL3 = 3,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVL_A {
    type Ux = u8;
}
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LVL_A> {
        match self.bits {
            0 => Some(LVL_A::BODLEVEL0),
            1 => Some(LVL_A::BODLEVEL1),
            2 => Some(LVL_A::BODLEVEL2),
            3 => Some(LVL_A::BODLEVEL3),
            _ => None,
        }
    }
    #[doc = "1.9V"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "2.45V"]
    #[inline(always)]
    pub fn is_bodlevel1(&self) -> bool {
        *self == LVL_A::BODLEVEL1
    }
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "2.85V"]
    #[inline(always)]
    pub fn is_bodlevel3(&self) -> bool {
        *self == LVL_A::BODLEVEL3
    }
}
#[doc = "Field `LVL` writer - BOD Level"]
pub type LVL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LVL_A>;
impl<'a, REG> LVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.9V"]
    #[inline(always)]
    pub fn bodlevel0(self) -> &'a mut crate::W<REG> {
        self.variant(LVL_A::BODLEVEL0)
    }
    #[doc = "2.45V"]
    #[inline(always)]
    pub fn bodlevel1(self) -> &'a mut crate::W<REG> {
        self.variant(LVL_A::BODLEVEL1)
    }
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn bodlevel2(self) -> &'a mut crate::W<REG> {
        self.variant(LVL_A::BODLEVEL2)
    }
    #[doc = "2.85V"]
    #[inline(always)]
    pub fn bodlevel3(self) -> &'a mut crate::W<REG> {
        self.variant(LVL_A::BODLEVEL3)
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    pub fn sampfreq(&self) -> SAMPFREQ_R {
        SAMPFREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<BODCFG_SPEC> {
        SLEEP_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<BODCFG_SPEC> {
        ACTIVE_W::new(self, 2)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn sampfreq(&mut self) -> SAMPFREQ_W<BODCFG_SPEC> {
        SAMPFREQ_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    #[must_use]
    pub fn lvl(&mut self) -> LVL_W<BODCFG_SPEC> {
        LVL_W::new(self, 5)
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
#[doc = "BOD Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bodcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bodcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BODCFG_SPEC;
impl crate::RegisterSpec for BODCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bodcfg::R`](R) reader structure"]
impl crate::Readable for BODCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bodcfg::W`](W) writer structure"]
impl crate::Writable for BODCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BODCFG to value 0"]
impl crate::Resettable for BODCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
