#[doc = "Register `ADMUXB` reader"]
pub type R = crate::R<ADMUXB_SPEC>;
#[doc = "Register `ADMUXB` writer"]
pub type W = crate::W<ADMUXB_SPEC>;
#[doc = "Field `GSEL` reader - Gain Selection Bits"]
pub type GSEL_R = crate::FieldReader;
#[doc = "Field `GSEL` writer - Gain Selection Bits"]
pub type GSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: Vcc"]
    VCC = 0,
    #[doc = "1: Internal 1.1V Voltage Reference with AREF disconnected"]
    INTERNAL_1 = 1,
    #[doc = "2: Internal 2.2V Voltage Reference with AREF disconnected"]
    INTERNAL_2 = 2,
    #[doc = "3: Internal 4.096V Voltage Reference with AREF disconnected"]
    INTERNAL_4 = 3,
    #[doc = "4: AREF with internal reference off"]
    AREF = 4,
    #[doc = "5: Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    AREF_INTERNAL_1 = 5,
    #[doc = "6: Internal 2.2V Voltage Reference with external capacitor at AREF pin"]
    AREF_INTERNAL_2 = 6,
    #[doc = "7: Internal 4.096V Voltage Reference with external capacitor at AREF pin"]
    AREF_INTERNAL_4 = 7,
}
impl From<REFS_A> for u8 {
    #[inline(always)]
    fn from(variant: REFS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFS_A {
    type Ux = u8;
}
impl REFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFS_A {
        match self.bits {
            0 => REFS_A::VCC,
            1 => REFS_A::INTERNAL_1,
            2 => REFS_A::INTERNAL_2,
            3 => REFS_A::INTERNAL_4,
            4 => REFS_A::AREF,
            5 => REFS_A::AREF_INTERNAL_1,
            6 => REFS_A::AREF_INTERNAL_2,
            7 => REFS_A::AREF_INTERNAL_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Vcc"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == REFS_A::VCC
    }
    #[doc = "Internal 1.1V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn is_internal_1(&self) -> bool {
        *self == REFS_A::INTERNAL_1
    }
    #[doc = "Internal 2.2V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn is_internal_2(&self) -> bool {
        *self == REFS_A::INTERNAL_2
    }
    #[doc = "Internal 4.096V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn is_internal_4(&self) -> bool {
        *self == REFS_A::INTERNAL_4
    }
    #[doc = "AREF with internal reference off"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFS_A::AREF
    }
    #[doc = "Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn is_aref_internal_1(&self) -> bool {
        *self == REFS_A::AREF_INTERNAL_1
    }
    #[doc = "Internal 2.2V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn is_aref_internal_2(&self) -> bool {
        *self == REFS_A::AREF_INTERNAL_2
    }
    #[doc = "Internal 4.096V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn is_aref_internal_4(&self) -> bool {
        *self == REFS_A::AREF_INTERNAL_4
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, REFS_A>;
impl<'a, REG> REFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Vcc"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::VCC)
    }
    #[doc = "Internal 1.1V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn internal_1(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL_1)
    }
    #[doc = "Internal 2.2V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn internal_2(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL_2)
    }
    #[doc = "Internal 4.096V Voltage Reference with AREF disconnected"]
    #[inline(always)]
    pub fn internal_4(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL_4)
    }
    #[doc = "AREF with internal reference off"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AREF)
    }
    #[doc = "Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn aref_internal_1(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AREF_INTERNAL_1)
    }
    #[doc = "Internal 2.2V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn aref_internal_2(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AREF_INTERNAL_2)
    }
    #[doc = "Internal 4.096V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn aref_internal_4(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AREF_INTERNAL_4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Gain Selection Bits"]
    #[inline(always)]
    pub fn gsel(&self) -> GSEL_R {
        GSEL_R::new(self.bits & 3)
    }
    #[doc = "Bits 5:7 - Reference Selection Bits"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn gsel(&mut self) -> GSEL_W<ADMUXB_SPEC> {
        GSEL_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Reference Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn refs(&mut self) -> REFS_W<ADMUXB_SPEC> {
        REFS_W::new(self, 5)
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
#[doc = "The ADC multiplexer Selection Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admuxb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admuxb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADMUXB_SPEC;
impl crate::RegisterSpec for ADMUXB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`admuxb::R`](R) reader structure"]
impl crate::Readable for ADMUXB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`admuxb::W`](W) writer structure"]
impl crate::Writable for ADMUXB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUXB to value 0"]
impl crate::Resettable for ADMUXB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
