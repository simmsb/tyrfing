#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CTRLC_SPEC>;
#[doc = "Field `PRESC` reader - Clock Pre-scaler"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "Clock Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: CLK_PER divided by 2"]
    DIV2 = 0,
    #[doc = "1: CLK_PER divided by 4"]
    DIV4 = 1,
    #[doc = "2: CLK_PER divided by 8"]
    DIV8 = 2,
    #[doc = "3: CLK_PER divided by 12"]
    DIV12 = 3,
    #[doc = "4: CLK_PER divided by 16"]
    DIV16 = 4,
    #[doc = "5: CLK_PER divided by 20"]
    DIV20 = 5,
    #[doc = "6: CLK_PER divided by 24"]
    DIV24 = 6,
    #[doc = "7: CLK_PER divided by 28"]
    DIV28 = 7,
    #[doc = "8: CLK_PER divided by 32"]
    DIV32 = 8,
    #[doc = "9: CLK_PER divided by 48"]
    DIV48 = 9,
    #[doc = "10: CLK_PER divided by 64"]
    DIV64 = 10,
    #[doc = "11: CLK_PER divided by 96"]
    DIV96 = 11,
    #[doc = "12: CLK_PER divided by 128"]
    DIV128 = 12,
    #[doc = "13: CLK_PER divided by 256"]
    DIV256 = 13,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::DIV2),
            1 => Some(PRESC_A::DIV4),
            2 => Some(PRESC_A::DIV8),
            3 => Some(PRESC_A::DIV12),
            4 => Some(PRESC_A::DIV16),
            5 => Some(PRESC_A::DIV20),
            6 => Some(PRESC_A::DIV24),
            7 => Some(PRESC_A::DIV28),
            8 => Some(PRESC_A::DIV32),
            9 => Some(PRESC_A::DIV48),
            10 => Some(PRESC_A::DIV64),
            11 => Some(PRESC_A::DIV96),
            12 => Some(PRESC_A::DIV128),
            13 => Some(PRESC_A::DIV256),
            _ => None,
        }
    }
    #[doc = "CLK_PER divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "CLK_PER divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "CLK_PER divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "CLK_PER divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC_A::DIV12
    }
    #[doc = "CLK_PER divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "CLK_PER divided by 20"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PRESC_A::DIV20
    }
    #[doc = "CLK_PER divided by 24"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PRESC_A::DIV24
    }
    #[doc = "CLK_PER divided by 28"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PRESC_A::DIV28
    }
    #[doc = "CLK_PER divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "CLK_PER divided by 48"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PRESC_A::DIV48
    }
    #[doc = "CLK_PER divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "CLK_PER divided by 96"]
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == PRESC_A::DIV96
    }
    #[doc = "CLK_PER divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
    #[doc = "CLK_PER divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC_A::DIV256
    }
}
#[doc = "Field `PRESC` writer - Clock Pre-scaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC_A>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_PER divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "CLK_PER divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "CLK_PER divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "CLK_PER divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV12)
    }
    #[doc = "CLK_PER divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "CLK_PER divided by 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV20)
    }
    #[doc = "CLK_PER divided by 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV24)
    }
    #[doc = "CLK_PER divided by 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV28)
    }
    #[doc = "CLK_PER divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "CLK_PER divided by 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV48)
    }
    #[doc = "CLK_PER divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "CLK_PER divided by 96"]
    #[inline(always)]
    pub fn div96(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV96)
    }
    #[doc = "CLK_PER divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV128)
    }
    #[doc = "CLK_PER divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV256)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock Pre-scaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRLC_SPEC> {
        PRESC_W::new(self, 0)
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
#[doc = "Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
