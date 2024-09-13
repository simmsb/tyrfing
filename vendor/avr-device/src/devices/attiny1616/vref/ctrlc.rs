#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CTRLC_SPEC>;
#[doc = "Field `DAC1REFSEL` reader - DAC1/AC1 reference select"]
pub type DAC1REFSEL_R = crate::FieldReader<DAC1REFSEL_A>;
#[doc = "DAC1/AC1 reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC1REFSEL_A {
    #[doc = "0: Voltage reference at 0.55V"]
    _0V55 = 0,
    #[doc = "1: Voltage reference at 1.1V"]
    _1V1 = 1,
    #[doc = "2: Voltage reference at 2.5V"]
    _2V5 = 2,
    #[doc = "3: Voltage reference at 4.34V"]
    _4V34 = 3,
    #[doc = "4: Voltage reference at 1.5V"]
    _1V5 = 4,
}
impl From<DAC1REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC1REFSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC1REFSEL_A {
    type Ux = u8;
}
impl DAC1REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC1REFSEL_A> {
        match self.bits {
            0 => Some(DAC1REFSEL_A::_0V55),
            1 => Some(DAC1REFSEL_A::_1V1),
            2 => Some(DAC1REFSEL_A::_2V5),
            3 => Some(DAC1REFSEL_A::_4V34),
            4 => Some(DAC1REFSEL_A::_1V5),
            _ => None,
        }
    }
    #[doc = "Voltage reference at 0.55V"]
    #[inline(always)]
    pub fn is_0v55(&self) -> bool {
        *self == DAC1REFSEL_A::_0V55
    }
    #[doc = "Voltage reference at 1.1V"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        *self == DAC1REFSEL_A::_1V1
    }
    #[doc = "Voltage reference at 2.5V"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == DAC1REFSEL_A::_2V5
    }
    #[doc = "Voltage reference at 4.34V"]
    #[inline(always)]
    pub fn is_4v34(&self) -> bool {
        *self == DAC1REFSEL_A::_4V34
    }
    #[doc = "Voltage reference at 1.5V"]
    #[inline(always)]
    pub fn is_1v5(&self) -> bool {
        *self == DAC1REFSEL_A::_1V5
    }
}
#[doc = "Field `DAC1REFSEL` writer - DAC1/AC1 reference select"]
pub type DAC1REFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DAC1REFSEL_A>;
impl<'a, REG> DAC1REFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Voltage reference at 0.55V"]
    #[inline(always)]
    pub fn _0v55(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1REFSEL_A::_0V55)
    }
    #[doc = "Voltage reference at 1.1V"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1REFSEL_A::_1V1)
    }
    #[doc = "Voltage reference at 2.5V"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1REFSEL_A::_2V5)
    }
    #[doc = "Voltage reference at 4.34V"]
    #[inline(always)]
    pub fn _4v34(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1REFSEL_A::_4V34)
    }
    #[doc = "Voltage reference at 1.5V"]
    #[inline(always)]
    pub fn _1v5(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1REFSEL_A::_1V5)
    }
}
#[doc = "Field `ADC1REFSEL` reader - ADC1 reference select"]
pub type ADC1REFSEL_R = crate::FieldReader<ADC1REFSEL_A>;
#[doc = "ADC1 reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC1REFSEL_A {
    #[doc = "0: Voltage reference at 0.55V"]
    _0V55 = 0,
    #[doc = "1: Voltage reference at 1.1V"]
    _1V1 = 1,
    #[doc = "2: Voltage reference at 2.5V"]
    _2V5 = 2,
    #[doc = "3: Voltage reference at 4.34V"]
    _4V34 = 3,
    #[doc = "4: Voltage reference at 1.5V"]
    _1V5 = 4,
}
impl From<ADC1REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1REFSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC1REFSEL_A {
    type Ux = u8;
}
impl ADC1REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC1REFSEL_A> {
        match self.bits {
            0 => Some(ADC1REFSEL_A::_0V55),
            1 => Some(ADC1REFSEL_A::_1V1),
            2 => Some(ADC1REFSEL_A::_2V5),
            3 => Some(ADC1REFSEL_A::_4V34),
            4 => Some(ADC1REFSEL_A::_1V5),
            _ => None,
        }
    }
    #[doc = "Voltage reference at 0.55V"]
    #[inline(always)]
    pub fn is_0v55(&self) -> bool {
        *self == ADC1REFSEL_A::_0V55
    }
    #[doc = "Voltage reference at 1.1V"]
    #[inline(always)]
    pub fn is_1v1(&self) -> bool {
        *self == ADC1REFSEL_A::_1V1
    }
    #[doc = "Voltage reference at 2.5V"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == ADC1REFSEL_A::_2V5
    }
    #[doc = "Voltage reference at 4.34V"]
    #[inline(always)]
    pub fn is_4v34(&self) -> bool {
        *self == ADC1REFSEL_A::_4V34
    }
    #[doc = "Voltage reference at 1.5V"]
    #[inline(always)]
    pub fn is_1v5(&self) -> bool {
        *self == ADC1REFSEL_A::_1V5
    }
}
#[doc = "Field `ADC1REFSEL` writer - ADC1 reference select"]
pub type ADC1REFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADC1REFSEL_A>;
impl<'a, REG> ADC1REFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Voltage reference at 0.55V"]
    #[inline(always)]
    pub fn _0v55(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1REFSEL_A::_0V55)
    }
    #[doc = "Voltage reference at 1.1V"]
    #[inline(always)]
    pub fn _1v1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1REFSEL_A::_1V1)
    }
    #[doc = "Voltage reference at 2.5V"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1REFSEL_A::_2V5)
    }
    #[doc = "Voltage reference at 4.34V"]
    #[inline(always)]
    pub fn _4v34(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1REFSEL_A::_4V34)
    }
    #[doc = "Voltage reference at 1.5V"]
    #[inline(always)]
    pub fn _1v5(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1REFSEL_A::_1V5)
    }
}
impl R {
    #[doc = "Bits 0:2 - DAC1/AC1 reference select"]
    #[inline(always)]
    pub fn dac1refsel(&self) -> DAC1REFSEL_R {
        DAC1REFSEL_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - ADC1 reference select"]
    #[inline(always)]
    pub fn adc1refsel(&self) -> ADC1REFSEL_R {
        ADC1REFSEL_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC1/AC1 reference select"]
    #[inline(always)]
    #[must_use]
    pub fn dac1refsel(&mut self) -> DAC1REFSEL_W<CTRLC_SPEC> {
        DAC1REFSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ADC1 reference select"]
    #[inline(always)]
    #[must_use]
    pub fn adc1refsel(&mut self) -> ADC1REFSEL_W<CTRLC_SPEC> {
        ADC1REFSEL_W::new(self, 4)
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
