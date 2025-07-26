#[doc = "Register `ADMUX` reader"]
pub type R = crate::R<ADMUX_SPEC>;
#[doc = "Register `ADMUX` writer"]
pub type W = crate::W<ADMUX_SPEC>;
#[doc = "Field `MUX` reader - Analog Channel Selection Bits"]
pub type MUX_R = crate::FieldReader<MUX_A>;
#[doc = "Analog Channel Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: ADC Single Ended Input pin 0"]
    ADC0 = 0,
    #[doc = "1: ADC Single Ended Input pin 1"]
    ADC1 = 1,
    #[doc = "2: ADC Single Ended Input pin 2"]
    ADC2 = 2,
    #[doc = "3: ADC Single Ended Input pin 3"]
    ADC3 = 3,
    #[doc = "4: ADC Single Ended Input pin 4"]
    ADC4 = 4,
    #[doc = "5: ADC Single Ended Input pin 5"]
    ADC5 = 5,
    #[doc = "6: ADC Single Ended Input pin 6"]
    ADC6 = 6,
    #[doc = "7: ADC Single Ended Input pin 7"]
    ADC7 = 7,
    #[doc = "8: ADC Single Ended Input pin 8"]
    ADC8 = 8,
    #[doc = "9: ADC Single Ended Input pin 9"]
    ADC9 = 9,
    #[doc = "10: ADC Single Ended Input pin 10"]
    ADC10 = 10,
    #[doc = "11: Temperature sensor"]
    TEMPSENS = 11,
    #[doc = "12: Internal Reference (VBG)"]
    ADC_VBG = 12,
    #[doc = "13: AVcc/4"]
    ADC_AVCC_4 = 13,
    #[doc = "14: 0V (GND)"]
    ADC_GND = 14,
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
    pub const fn variant(&self) -> Option<MUX_A> {
        match self.bits {
            0 => Some(MUX_A::ADC0),
            1 => Some(MUX_A::ADC1),
            2 => Some(MUX_A::ADC2),
            3 => Some(MUX_A::ADC3),
            4 => Some(MUX_A::ADC4),
            5 => Some(MUX_A::ADC5),
            6 => Some(MUX_A::ADC6),
            7 => Some(MUX_A::ADC7),
            8 => Some(MUX_A::ADC8),
            9 => Some(MUX_A::ADC9),
            10 => Some(MUX_A::ADC10),
            11 => Some(MUX_A::TEMPSENS),
            12 => Some(MUX_A::ADC_VBG),
            13 => Some(MUX_A::ADC_AVCC_4),
            14 => Some(MUX_A::ADC_GND),
            _ => None,
        }
    }
    #[doc = "ADC Single Ended Input pin 0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == MUX_A::ADC0
    }
    #[doc = "ADC Single Ended Input pin 1"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == MUX_A::ADC1
    }
    #[doc = "ADC Single Ended Input pin 2"]
    #[inline(always)]
    pub fn is_adc2(&self) -> bool {
        *self == MUX_A::ADC2
    }
    #[doc = "ADC Single Ended Input pin 3"]
    #[inline(always)]
    pub fn is_adc3(&self) -> bool {
        *self == MUX_A::ADC3
    }
    #[doc = "ADC Single Ended Input pin 4"]
    #[inline(always)]
    pub fn is_adc4(&self) -> bool {
        *self == MUX_A::ADC4
    }
    #[doc = "ADC Single Ended Input pin 5"]
    #[inline(always)]
    pub fn is_adc5(&self) -> bool {
        *self == MUX_A::ADC5
    }
    #[doc = "ADC Single Ended Input pin 6"]
    #[inline(always)]
    pub fn is_adc6(&self) -> bool {
        *self == MUX_A::ADC6
    }
    #[doc = "ADC Single Ended Input pin 7"]
    #[inline(always)]
    pub fn is_adc7(&self) -> bool {
        *self == MUX_A::ADC7
    }
    #[doc = "ADC Single Ended Input pin 8"]
    #[inline(always)]
    pub fn is_adc8(&self) -> bool {
        *self == MUX_A::ADC8
    }
    #[doc = "ADC Single Ended Input pin 9"]
    #[inline(always)]
    pub fn is_adc9(&self) -> bool {
        *self == MUX_A::ADC9
    }
    #[doc = "ADC Single Ended Input pin 10"]
    #[inline(always)]
    pub fn is_adc10(&self) -> bool {
        *self == MUX_A::ADC10
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn is_tempsens(&self) -> bool {
        *self == MUX_A::TEMPSENS
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn is_adc_vbg(&self) -> bool {
        *self == MUX_A::ADC_VBG
    }
    #[doc = "AVcc/4"]
    #[inline(always)]
    pub fn is_adc_avcc_4(&self) -> bool {
        *self == MUX_A::ADC_AVCC_4
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn is_adc_gnd(&self) -> bool {
        *self == MUX_A::ADC_GND
    }
}
#[doc = "Field `MUX` writer - Analog Channel Selection Bits"]
pub type MUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, MUX_A>;
impl<'a, REG> MUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Single Ended Input pin 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC0)
    }
    #[doc = "ADC Single Ended Input pin 1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC1)
    }
    #[doc = "ADC Single Ended Input pin 2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC2)
    }
    #[doc = "ADC Single Ended Input pin 3"]
    #[inline(always)]
    pub fn adc3(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC3)
    }
    #[doc = "ADC Single Ended Input pin 4"]
    #[inline(always)]
    pub fn adc4(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC4)
    }
    #[doc = "ADC Single Ended Input pin 5"]
    #[inline(always)]
    pub fn adc5(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC5)
    }
    #[doc = "ADC Single Ended Input pin 6"]
    #[inline(always)]
    pub fn adc6(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC6)
    }
    #[doc = "ADC Single Ended Input pin 7"]
    #[inline(always)]
    pub fn adc7(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC7)
    }
    #[doc = "ADC Single Ended Input pin 8"]
    #[inline(always)]
    pub fn adc8(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC8)
    }
    #[doc = "ADC Single Ended Input pin 9"]
    #[inline(always)]
    pub fn adc9(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC9)
    }
    #[doc = "ADC Single Ended Input pin 10"]
    #[inline(always)]
    pub fn adc10(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC10)
    }
    #[doc = "Temperature sensor"]
    #[inline(always)]
    pub fn tempsens(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::TEMPSENS)
    }
    #[doc = "Internal Reference (VBG)"]
    #[inline(always)]
    pub fn adc_vbg(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC_VBG)
    }
    #[doc = "AVcc/4"]
    #[inline(always)]
    pub fn adc_avcc_4(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC_AVCC_4)
    }
    #[doc = "0V (GND)"]
    #[inline(always)]
    pub fn adc_gnd(self) -> &'a mut crate::W<REG> {
        self.variant(MUX_A::ADC_GND)
    }
}
#[doc = "Field `ADLAR` reader - Left Adjust Result"]
pub type ADLAR_R = crate::BitReader;
#[doc = "Field `ADLAR` writer - Left Adjust Result"]
pub type ADLAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFS` reader - Reference Selection Bits"]
pub type REFS_R = crate::FieldReader<REFS_A>;
#[doc = "Reference Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFS_A {
    #[doc = "0: AVcc or External Reference"]
    AVCC = 0,
    #[doc = "1: Internal 1.1V Voltage Reference"]
    INTERNAL_11 = 1,
    #[doc = "3: Internal 2.56V Voltage Reference"]
    INTERNAL_256 = 3,
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
            0 => Some(REFS_A::AVCC),
            1 => Some(REFS_A::INTERNAL_11),
            3 => Some(REFS_A::INTERNAL_256),
            _ => None,
        }
    }
    #[doc = "AVcc or External Reference"]
    #[inline(always)]
    pub fn is_avcc(&self) -> bool {
        *self == REFS_A::AVCC
    }
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn is_internal_11(&self) -> bool {
        *self == REFS_A::INTERNAL_11
    }
    #[doc = "Internal 2.56V Voltage Reference"]
    #[inline(always)]
    pub fn is_internal_256(&self) -> bool {
        *self == REFS_A::INTERNAL_256
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bits"]
pub type REFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REFS_A>;
impl<'a, REG> REFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVcc or External Reference"]
    #[inline(always)]
    pub fn avcc(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::AVCC)
    }
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn internal_11(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL_11)
    }
    #[doc = "Internal 2.56V Voltage Reference"]
    #[inline(always)]
    pub fn internal_256(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL_256)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog Channel Selection Bits"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Left Adjust Result"]
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
    #[doc = "Bits 0:4 - Analog Channel Selection Bits"]
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
