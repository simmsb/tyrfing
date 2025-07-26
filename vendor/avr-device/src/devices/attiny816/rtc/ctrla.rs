#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `RTCEN` reader - Enable"]
pub type RTCEN_R = crate::BitReader;
#[doc = "Field `RTCEN` writer - Enable"]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALER` reader - Prescaling Factor"]
pub type PRESCALER_R = crate::FieldReader<PRESCALER_A>;
#[doc = "Prescaling Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: RTC Clock / 1"]
    DIV1 = 0,
    #[doc = "1: RTC Clock / 2"]
    DIV2 = 1,
    #[doc = "2: RTC Clock / 4"]
    DIV4 = 2,
    #[doc = "3: RTC Clock / 8"]
    DIV8 = 3,
    #[doc = "4: RTC Clock / 16"]
    DIV16 = 4,
    #[doc = "5: RTC Clock / 32"]
    DIV32 = 5,
    #[doc = "6: RTC Clock / 64"]
    DIV64 = 6,
    #[doc = "7: RTC Clock / 128"]
    DIV128 = 7,
    #[doc = "8: RTC Clock / 256"]
    DIV256 = 8,
    #[doc = "9: RTC Clock / 512"]
    DIV512 = 9,
    #[doc = "10: RTC Clock / 1024"]
    DIV1024 = 10,
    #[doc = "11: RTC Clock / 2048"]
    DIV2048 = 11,
    #[doc = "12: RTC Clock / 4096"]
    DIV4096 = 12,
    #[doc = "13: RTC Clock / 8192"]
    DIV8192 = 13,
    #[doc = "14: RTC Clock / 16384"]
    DIV16384 = 14,
    #[doc = "15: RTC Clock / 32768"]
    DIV32768 = 15,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALER_A {
    type Ux = u8;
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESCALER_A {
        match self.bits {
            0 => PRESCALER_A::DIV1,
            1 => PRESCALER_A::DIV2,
            2 => PRESCALER_A::DIV4,
            3 => PRESCALER_A::DIV8,
            4 => PRESCALER_A::DIV16,
            5 => PRESCALER_A::DIV32,
            6 => PRESCALER_A::DIV64,
            7 => PRESCALER_A::DIV128,
            8 => PRESCALER_A::DIV256,
            9 => PRESCALER_A::DIV512,
            10 => PRESCALER_A::DIV1024,
            11 => PRESCALER_A::DIV2048,
            12 => PRESCALER_A::DIV4096,
            13 => PRESCALER_A::DIV8192,
            14 => PRESCALER_A::DIV16384,
            15 => PRESCALER_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "RTC Clock / 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALER_A::DIV1
    }
    #[doc = "RTC Clock / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER_A::DIV2
    }
    #[doc = "RTC Clock / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER_A::DIV4
    }
    #[doc = "RTC Clock / 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER_A::DIV8
    }
    #[doc = "RTC Clock / 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER_A::DIV16
    }
    #[doc = "RTC Clock / 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER_A::DIV32
    }
    #[doc = "RTC Clock / 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER_A::DIV64
    }
    #[doc = "RTC Clock / 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER_A::DIV128
    }
    #[doc = "RTC Clock / 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER_A::DIV256
    }
    #[doc = "RTC Clock / 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCALER_A::DIV512
    }
    #[doc = "RTC Clock / 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALER_A::DIV1024
    }
    #[doc = "RTC Clock / 2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == PRESCALER_A::DIV2048
    }
    #[doc = "RTC Clock / 4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == PRESCALER_A::DIV4096
    }
    #[doc = "RTC Clock / 8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == PRESCALER_A::DIV8192
    }
    #[doc = "RTC Clock / 16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == PRESCALER_A::DIV16384
    }
    #[doc = "RTC Clock / 32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == PRESCALER_A::DIV32768
    }
}
#[doc = "Field `PRESCALER` writer - Prescaling Factor"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, PRESCALER_A>;
impl<'a, REG> PRESCALER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC Clock / 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV1)
    }
    #[doc = "RTC Clock / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV2)
    }
    #[doc = "RTC Clock / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV4)
    }
    #[doc = "RTC Clock / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV8)
    }
    #[doc = "RTC Clock / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV16)
    }
    #[doc = "RTC Clock / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV32)
    }
    #[doc = "RTC Clock / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV64)
    }
    #[doc = "RTC Clock / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV128)
    }
    #[doc = "RTC Clock / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV256)
    }
    #[doc = "RTC Clock / 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV512)
    }
    #[doc = "RTC Clock / 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV1024)
    }
    #[doc = "RTC Clock / 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV2048)
    }
    #[doc = "RTC Clock / 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV4096)
    }
    #[doc = "RTC Clock / 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV8192)
    }
    #[doc = "RTC Clock / 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV16384)
    }
    #[doc = "RTC Clock / 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV32768)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run In Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run In Standby"]
pub type RUNSTDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - Prescaling Factor"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits >> 3) & 0x0f)
    }
    #[doc = "Bit 7 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<CTRLA_SPEC> {
        RTCEN_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - Prescaling Factor"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CTRLA_SPEC> {
        PRESCALER_W::new(self, 3)
    }
    #[doc = "Bit 7 - Run In Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC> {
        RUNSTDBY_W::new(self, 7)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
