#[doc = "Register `ADMUX` reader"]
pub type R = crate::R<ADMUX_SPEC>;
#[doc = "Register `ADMUX` writer"]
pub type W = crate::W<ADMUX_SPEC>;
#[doc = "Field `MUX` reader - Analog Channel and Gain Selection Bits"]
pub type MUX_R = crate::FieldReader;
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `ADLAR` reader - ADC Left Adjust Result"]
pub type ADLAR_R = crate::BitReader;
#[doc = "Field `ADLAR` writer - ADC Left Adjust Result"]
pub type ADLAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: Aref Internal Vref turned off"]
    AREF = 0,
    #[doc = "1: AVcc with external capacitor at AREF pin"]
    AVCC = 1,
    #[doc = "3: Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    INTERNAL = 3,
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
    pub const fn variant(&self) -> Option<REFS_A> {
        match self.bits {
            0 => Some(REFS_A::AREF),
            1 => Some(REFS_A::AVCC),
            3 => Some(REFS_A::INTERNAL),
            _ => None,
        }
    }
    #[doc = "Aref Internal Vref turned off"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFS_A::AREF
    }
    #[doc = "AVcc with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        *self == REFS_A::AVCC
    }
    #[doc = "Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS_A::INTERNAL
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REFS_A>;
impl<'a, REG> REFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Aref Internal Vref turned off"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AREF)
    }
    #[doc = "AVcc with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AVCC)
    }
    #[doc = "Internal 1.1V Voltage Reference with external capacitor at AREF pin"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - ADC Left Adjust Result"]
    #[inline(always)]
    pub fn adlar(&self) -> ADLAR_R {
        ADLAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<ADMUX_SPEC> {
        MUX_W::new(self, 0)
    }
    #[doc = "Bit 5 - ADC Left Adjust Result"]
    #[inline(always)]
    #[must_use]
    pub fn adlar(&mut self) -> ADLAR_W<ADMUX_SPEC> {
        ADLAR_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reference Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn refs(&mut self) -> REFS_W<ADMUX_SPEC> {
        REFS_W::new(self, 6)
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
#[doc = "The ADC Multiplexer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADMUX_SPEC;
impl crate::RegisterSpec for ADMUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`admux::R`](R) reader structure"]
impl crate::Readable for ADMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`admux::W`](W) writer structure"]
impl crate::Writable for ADMUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUX to value 0"]
impl crate::Resettable for ADMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
