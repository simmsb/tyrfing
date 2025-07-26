#[doc = "Register `SYSCFG1` reader"]
pub type R = crate::R<SYSCFG1_SPEC>;
#[doc = "Register `SYSCFG1` writer"]
pub type W = crate::W<SYSCFG1_SPEC>;
#[doc = "Field `SUT` reader - Startup Time"]
pub type SUT_R = crate::FieldReader<SUT_A>;
#[doc = "Startup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUT_A {
    #[doc = "0: 0 ms"]
    _0MS = 0,
    #[doc = "1: 1 ms"]
    _1MS = 1,
    #[doc = "2: 2 ms"]
    _2MS = 2,
    #[doc = "3: 4 ms"]
    _4MS = 3,
    #[doc = "4: 8 ms"]
    _8MS = 4,
    #[doc = "5: 16 ms"]
    _16MS = 5,
    #[doc = "6: 32 ms"]
    _32MS = 6,
    #[doc = "7: 64 ms"]
    _64MS = 7,
}
impl From<SUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SUT_A {
    type Ux = u8;
}
impl SUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUT_A {
        match self.bits {
            0 => SUT_A::_0MS,
            1 => SUT_A::_1MS,
            2 => SUT_A::_2MS,
            3 => SUT_A::_4MS,
            4 => SUT_A::_8MS,
            5 => SUT_A::_16MS,
            6 => SUT_A::_32MS,
            7 => SUT_A::_64MS,
            _ => unreachable!(),
        }
    }
    #[doc = "0 ms"]
    #[inline(always)]
    pub fn is_0ms(&self) -> bool {
        *self == SUT_A::_0MS
    }
    #[doc = "1 ms"]
    #[inline(always)]
    pub fn is_1ms(&self) -> bool {
        *self == SUT_A::_1MS
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn is_2ms(&self) -> bool {
        *self == SUT_A::_2MS
    }
    #[doc = "4 ms"]
    #[inline(always)]
    pub fn is_4ms(&self) -> bool {
        *self == SUT_A::_4MS
    }
    #[doc = "8 ms"]
    #[inline(always)]
    pub fn is_8ms(&self) -> bool {
        *self == SUT_A::_8MS
    }
    #[doc = "16 ms"]
    #[inline(always)]
    pub fn is_16ms(&self) -> bool {
        *self == SUT_A::_16MS
    }
    #[doc = "32 ms"]
    #[inline(always)]
    pub fn is_32ms(&self) -> bool {
        *self == SUT_A::_32MS
    }
    #[doc = "64 ms"]
    #[inline(always)]
    pub fn is_64ms(&self) -> bool {
        *self == SUT_A::_64MS
    }
}
#[doc = "Field `SUT` writer - Startup Time"]
pub type SUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SUT_A>;
impl<'a, REG> SUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 ms"]
    #[inline(always)]
    pub fn _0ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_0MS)
    }
    #[doc = "1 ms"]
    #[inline(always)]
    pub fn _1ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_1MS)
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn _2ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_2MS)
    }
    #[doc = "4 ms"]
    #[inline(always)]
    pub fn _4ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_4MS)
    }
    #[doc = "8 ms"]
    #[inline(always)]
    pub fn _8ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_8MS)
    }
    #[doc = "16 ms"]
    #[inline(always)]
    pub fn _16ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_16MS)
    }
    #[doc = "32 ms"]
    #[inline(always)]
    pub fn _32ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_32MS)
    }
    #[doc = "64 ms"]
    #[inline(always)]
    pub fn _64ms(self) -> &'a mut crate::W<REG> {
        self.variant(SUT_A::_64MS)
    }
}
#[doc = "Field `MVSYSCFG` reader - MVIO System Configuration"]
pub type MVSYSCFG_R = crate::FieldReader<MVSYSCFG_A>;
#[doc = "MVIO System Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MVSYSCFG_A {
    #[doc = "1: Device used in a dual supply configuration"]
    DUAL = 1,
    #[doc = "2: Device used in a single supply configuration"]
    SINGLE = 2,
}
impl From<MVSYSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MVSYSCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MVSYSCFG_A {
    type Ux = u8;
}
impl MVSYSCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MVSYSCFG_A> {
        match self.bits {
            1 => Some(MVSYSCFG_A::DUAL),
            2 => Some(MVSYSCFG_A::SINGLE),
            _ => None,
        }
    }
    #[doc = "Device used in a dual supply configuration"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == MVSYSCFG_A::DUAL
    }
    #[doc = "Device used in a single supply configuration"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MVSYSCFG_A::SINGLE
    }
}
#[doc = "Field `MVSYSCFG` writer - MVIO System Configuration"]
pub type MVSYSCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MVSYSCFG_A>;
impl<'a, REG> MVSYSCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Device used in a dual supply configuration"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(MVSYSCFG_A::DUAL)
    }
    #[doc = "Device used in a single supply configuration"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(MVSYSCFG_A::SINGLE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    pub fn sut(&self) -> SUT_R {
        SUT_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - MVIO System Configuration"]
    #[inline(always)]
    pub fn mvsyscfg(&self) -> MVSYSCFG_R {
        MVSYSCFG_R::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    #[must_use]
    pub fn sut(&mut self) -> SUT_W<SYSCFG1_SPEC> {
        SUT_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - MVIO System Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn mvsyscfg(&mut self) -> MVSYSCFG_W<SYSCFG1_SPEC> {
        MVSYSCFG_W::new(self, 3)
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
#[doc = "System Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG1_SPEC;
impl crate::RegisterSpec for SYSCFG1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`syscfg1::R`](R) reader structure"]
impl crate::Readable for SYSCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscfg1::W`](W) writer structure"]
impl crate::Writable for SYSCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG1 to value 0"]
impl crate::Resettable for SYSCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
