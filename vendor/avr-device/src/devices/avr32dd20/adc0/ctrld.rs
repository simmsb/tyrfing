#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<CTRLD_SPEC>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<CTRLD_SPEC>;
#[doc = "Field `SAMPDLY` reader - Sampling Delay Selection"]
pub type SAMPDLY_R = crate::FieldReader<SAMPDLY_A>;
#[doc = "Sampling Delay Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPDLY_A {
    #[doc = "0: Delay 0 CLK_ADC cycles"]
    DLY0 = 0,
    #[doc = "1: Delay 1 CLK_ADC cycles"]
    DLY1 = 1,
    #[doc = "2: Delay 2 CLK_ADC cycles"]
    DLY2 = 2,
    #[doc = "3: Delay 3 CLK_ADC cycles"]
    DLY3 = 3,
    #[doc = "4: Delay 4 CLK_ADC cycles"]
    DLY4 = 4,
    #[doc = "5: Delay 5 CLK_ADC cycles"]
    DLY5 = 5,
    #[doc = "6: Delay 6 CLK_ADC cycles"]
    DLY6 = 6,
    #[doc = "7: Delay 7 CLK_ADC cycles"]
    DLY7 = 7,
    #[doc = "8: Delay 8 CLK_ADC cycles"]
    DLY8 = 8,
    #[doc = "9: Delay 9 CLK_ADC cycles"]
    DLY9 = 9,
    #[doc = "10: Delay 10 CLK_ADC cycles"]
    DLY10 = 10,
    #[doc = "11: Delay 11 CLK_ADC cycles"]
    DLY11 = 11,
    #[doc = "12: Delay 12 CLK_ADC cycles"]
    DLY12 = 12,
    #[doc = "13: Delay 13 CLK_ADC cycles"]
    DLY13 = 13,
    #[doc = "14: Delay 14 CLK_ADC cycles"]
    DLY14 = 14,
    #[doc = "15: Delay 15 CLK_ADC cycles"]
    DLY15 = 15,
}
impl From<SAMPDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPDLY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAMPDLY_A {
    type Ux = u8;
}
impl SAMPDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAMPDLY_A {
        match self.bits {
            0 => SAMPDLY_A::DLY0,
            1 => SAMPDLY_A::DLY1,
            2 => SAMPDLY_A::DLY2,
            3 => SAMPDLY_A::DLY3,
            4 => SAMPDLY_A::DLY4,
            5 => SAMPDLY_A::DLY5,
            6 => SAMPDLY_A::DLY6,
            7 => SAMPDLY_A::DLY7,
            8 => SAMPDLY_A::DLY8,
            9 => SAMPDLY_A::DLY9,
            10 => SAMPDLY_A::DLY10,
            11 => SAMPDLY_A::DLY11,
            12 => SAMPDLY_A::DLY12,
            13 => SAMPDLY_A::DLY13,
            14 => SAMPDLY_A::DLY14,
            15 => SAMPDLY_A::DLY15,
            _ => unreachable!(),
        }
    }
    #[doc = "Delay 0 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly0(&self) -> bool {
        *self == SAMPDLY_A::DLY0
    }
    #[doc = "Delay 1 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly1(&self) -> bool {
        *self == SAMPDLY_A::DLY1
    }
    #[doc = "Delay 2 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly2(&self) -> bool {
        *self == SAMPDLY_A::DLY2
    }
    #[doc = "Delay 3 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly3(&self) -> bool {
        *self == SAMPDLY_A::DLY3
    }
    #[doc = "Delay 4 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly4(&self) -> bool {
        *self == SAMPDLY_A::DLY4
    }
    #[doc = "Delay 5 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly5(&self) -> bool {
        *self == SAMPDLY_A::DLY5
    }
    #[doc = "Delay 6 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly6(&self) -> bool {
        *self == SAMPDLY_A::DLY6
    }
    #[doc = "Delay 7 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly7(&self) -> bool {
        *self == SAMPDLY_A::DLY7
    }
    #[doc = "Delay 8 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly8(&self) -> bool {
        *self == SAMPDLY_A::DLY8
    }
    #[doc = "Delay 9 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly9(&self) -> bool {
        *self == SAMPDLY_A::DLY9
    }
    #[doc = "Delay 10 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly10(&self) -> bool {
        *self == SAMPDLY_A::DLY10
    }
    #[doc = "Delay 11 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly11(&self) -> bool {
        *self == SAMPDLY_A::DLY11
    }
    #[doc = "Delay 12 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly12(&self) -> bool {
        *self == SAMPDLY_A::DLY12
    }
    #[doc = "Delay 13 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly13(&self) -> bool {
        *self == SAMPDLY_A::DLY13
    }
    #[doc = "Delay 14 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly14(&self) -> bool {
        *self == SAMPDLY_A::DLY14
    }
    #[doc = "Delay 15 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly15(&self) -> bool {
        *self == SAMPDLY_A::DLY15
    }
}
#[doc = "Field `SAMPDLY` writer - Sampling Delay Selection"]
pub type SAMPDLY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, SAMPDLY_A>;
impl<'a, REG> SAMPDLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay 0 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly0(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY0)
    }
    #[doc = "Delay 1 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly1(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY1)
    }
    #[doc = "Delay 2 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly2(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY2)
    }
    #[doc = "Delay 3 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly3(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY3)
    }
    #[doc = "Delay 4 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly4(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY4)
    }
    #[doc = "Delay 5 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly5(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY5)
    }
    #[doc = "Delay 6 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly6(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY6)
    }
    #[doc = "Delay 7 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly7(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY7)
    }
    #[doc = "Delay 8 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly8(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY8)
    }
    #[doc = "Delay 9 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly9(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY9)
    }
    #[doc = "Delay 10 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly10(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY10)
    }
    #[doc = "Delay 11 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly11(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY11)
    }
    #[doc = "Delay 12 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly12(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY12)
    }
    #[doc = "Delay 13 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly13(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY13)
    }
    #[doc = "Delay 14 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly14(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY14)
    }
    #[doc = "Delay 15 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly15(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPDLY_A::DLY15)
    }
}
#[doc = "Field `INITDLY` reader - Initial Delay Selection"]
pub type INITDLY_R = crate::FieldReader<INITDLY_A>;
#[doc = "Initial Delay Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INITDLY_A {
    #[doc = "0: Delay 0 CLK_ADC cycles"]
    DLY0 = 0,
    #[doc = "1: Delay 16 CLK_ADC cycles"]
    DLY16 = 1,
    #[doc = "2: Delay 32 CLK_ADC cycles"]
    DLY32 = 2,
    #[doc = "3: Delay 64 CLK_ADC cycles"]
    DLY64 = 3,
    #[doc = "4: Delay 128 CLK_ADC cycles"]
    DLY128 = 4,
    #[doc = "5: Delay 256 CLK_ADC cycles"]
    DLY256 = 5,
}
impl From<INITDLY_A> for u8 {
    #[inline(always)]
    fn from(variant: INITDLY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INITDLY_A {
    type Ux = u8;
}
impl INITDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INITDLY_A> {
        match self.bits {
            0 => Some(INITDLY_A::DLY0),
            1 => Some(INITDLY_A::DLY16),
            2 => Some(INITDLY_A::DLY32),
            3 => Some(INITDLY_A::DLY64),
            4 => Some(INITDLY_A::DLY128),
            5 => Some(INITDLY_A::DLY256),
            _ => None,
        }
    }
    #[doc = "Delay 0 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly0(&self) -> bool {
        *self == INITDLY_A::DLY0
    }
    #[doc = "Delay 16 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly16(&self) -> bool {
        *self == INITDLY_A::DLY16
    }
    #[doc = "Delay 32 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly32(&self) -> bool {
        *self == INITDLY_A::DLY32
    }
    #[doc = "Delay 64 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly64(&self) -> bool {
        *self == INITDLY_A::DLY64
    }
    #[doc = "Delay 128 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly128(&self) -> bool {
        *self == INITDLY_A::DLY128
    }
    #[doc = "Delay 256 CLK_ADC cycles"]
    #[inline(always)]
    pub fn is_dly256(&self) -> bool {
        *self == INITDLY_A::DLY256
    }
}
#[doc = "Field `INITDLY` writer - Initial Delay Selection"]
pub type INITDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3, INITDLY_A>;
impl<'a, REG> INITDLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay 0 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly0(self) -> &'a mut crate::W<REG> {
        self.variant(INITDLY_A::DLY0)
    }
    #[doc = "Delay 16 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly16(self) -> &'a mut crate::W<REG> {
        self.variant(INITDLY_A::DLY16)
    }
    #[doc = "Delay 32 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly32(self) -> &'a mut crate::W<REG> {
        self.variant(INITDLY_A::DLY32)
    }
    #[doc = "Delay 64 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly64(self) -> &'a mut crate::W<REG> {
        self.variant(INITDLY_A::DLY64)
    }
    #[doc = "Delay 128 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly128(self) -> &'a mut crate::W<REG> {
        self.variant(INITDLY_A::DLY128)
    }
    #[doc = "Delay 256 CLK_ADC cycles"]
    #[inline(always)]
    pub fn dly256(self) -> &'a mut crate::W<REG> {
        self.variant(INITDLY_A::DLY256)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sampling Delay Selection"]
    #[inline(always)]
    pub fn sampdly(&self) -> SAMPDLY_R {
        SAMPDLY_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 5:7 - Initial Delay Selection"]
    #[inline(always)]
    pub fn initdly(&self) -> INITDLY_R {
        INITDLY_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sampling Delay Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sampdly(&mut self) -> SAMPDLY_W<CTRLD_SPEC> {
        SAMPDLY_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Initial Delay Selection"]
    #[inline(always)]
    #[must_use]
    pub fn initdly(&mut self) -> INITDLY_W<CTRLD_SPEC> {
        INITDLY_W::new(self, 5)
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
#[doc = "Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrld::R`](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrld::W`](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
