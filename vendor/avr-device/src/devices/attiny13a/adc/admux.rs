#[doc = "Register `ADMUX` reader"]
pub type R = crate::R<ADMUX_SPEC>;
#[doc = "Register `ADMUX` writer"]
pub type W = crate::W<ADMUX_SPEC>;
#[doc = "Field `MUX` reader - Analog Channel and Gain Selection Bits"]
pub type MUX_R = crate::FieldReader<MUX_A>;
#[doc = "Analog Channel and Gain Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Single-ended Input ADC0"]
    ADC0 = 0,
    #[doc = "1: Single-ended Input ADC1"]
    ADC1 = 1,
    #[doc = "2: Single-ended Input ADC2"]
    ADC2 = 2,
    #[doc = "3: Single-ended Input ADC3"]
    ADC3 = 3,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MUX_A {
    type Ux = u8;
}
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUX_A {
        match self.bits {
            0 => MUX_A::ADC0,
            1 => MUX_A::ADC1,
            2 => MUX_A::ADC2,
            3 => MUX_A::ADC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-ended Input ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == MUX_A::ADC0
    }
    #[doc = "Single-ended Input ADC1"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == MUX_A::ADC1
    }
    #[doc = "Single-ended Input ADC2"]
    #[inline(always)]
    pub fn is_adc2(&self) -> bool {
        *self == MUX_A::ADC2
    }
    #[doc = "Single-ended Input ADC3"]
    #[inline(always)]
    pub fn is_adc3(&self) -> bool {
        *self == MUX_A::ADC3
    }
}
#[doc = "Field `MUX` writer - Analog Channel and Gain Selection Bits"]
pub type MUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MUX_A>;
impl<'a, REG> MUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-ended Input ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0)
    }
    #[doc = "Single-ended Input ADC1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1)
    }
    #[doc = "Single-ended Input ADC2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2)
    }
    #[doc = "Single-ended Input ADC3"]
    #[inline(always)]
    pub fn adc3(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3)
    }
}
#[doc = "Field `ADLAR` reader - Left Adjust Result"]
pub type ADLAR_R = crate::BitReader;
#[doc = "Field `ADLAR` writer - Left Adjust Result"]
pub type ADLAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFS0` reader - Reference Selection Bit 0"]
pub type REFS0_R = crate::BitReader<REFS0_A>;
#[doc = "Reference Selection Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFS0_A {
    #[doc = "0: Vcc used as Voltage Reference"]
    VCC = 0,
    #[doc = "1: Internal Voltage Reference of 1.1V"]
    INTERNAL = 1,
}
impl From<REFS0_A> for bool {
    #[inline(always)]
    fn from(variant: REFS0_A) -> Self {
        variant as u8 != 0
    }
}
impl REFS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFS0_A {
        match self.bits {
            false => REFS0_A::VCC,
            true => REFS0_A::INTERNAL,
        }
    }
    #[doc = "Vcc used as Voltage Reference"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == REFS0_A::VCC
    }
    #[doc = "Internal Voltage Reference of 1.1V"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS0_A::INTERNAL
    }
}
#[doc = "Field `REFS0` writer - Reference Selection Bit 0"]
pub type REFS0_W<'a, REG> = crate::BitWriter<'a, REG, REFS0_A>;
impl<'a, REG> REFS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Vcc used as Voltage Reference"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut crate::W<REG> {
        self.variant(REFS0_A::VCC)
    }
    #[doc = "Internal Voltage Reference of 1.1V"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(REFS0_A::INTERNAL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 3)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
    #[inline(always)]
    pub fn adlar(&self) -> ADLAR_R {
        ADLAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reference Selection Bit 0"]
    #[inline(always)]
    pub fn refs0(&self) -> REFS0_R {
        REFS0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Channel and Gain Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<ADMUX_SPEC> {
        MUX_W::new(self, 0)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
    #[inline(always)]
    #[must_use]
    pub fn adlar(&mut self) -> ADLAR_W<ADMUX_SPEC> {
        ADLAR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Reference Selection Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn refs0(&mut self) -> REFS0_W<ADMUX_SPEC> {
        REFS0_W::new(self, 6)
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
#[doc = "The ADC multiplexer Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
