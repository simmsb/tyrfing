#[doc = "Register `CSMA_BE` reader"]
pub type R = crate::R<CSMA_BE_SPEC>;
#[doc = "Register `CSMA_BE` writer"]
pub type W = crate::W<CSMA_BE_SPEC>;
#[doc = "Field `MIN_BE` reader - Minimum Back-off Exponent"]
pub type MIN_BE_R = crate::FieldReader<MIN_BE_A>;
#[doc = "Minimum Back-off Exponent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MIN_BE_A {
    #[doc = "0: Minimum value of minimum back-off exponent."]
    MINIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT = 0,
    #[doc = "8: Maximum value of minimum back-off exponent. MIN_BE must be smaller or equal to MAX_BE."]
    MAXIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT_MIN_BE_MUST_BE_SMALLER_OR_EQUAL_TO_MAX_BE = 8,
}
impl From<MIN_BE_A> for u8 {
    #[inline(always)]
    fn from(variant: MIN_BE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MIN_BE_A {
    type Ux = u8;
}
impl MIN_BE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MIN_BE_A> {
        match self . bits { 0 => Some (MIN_BE_A :: MINIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT) , 8 => Some (MIN_BE_A :: MAXIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT_MIN_BE_MUST_BE_SMALLER_OR_EQUAL_TO_MAX_BE) , _ => None , }
    }
    #[doc = "Minimum value of minimum back-off exponent."]
    #[inline(always)]
    pub fn is_minimum_value_of_minimum_back_off_exponent(&self) -> bool {
        *self == MIN_BE_A::MINIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT
    }
    #[doc = "Maximum value of minimum back-off exponent. MIN_BE must be smaller or equal to MAX_BE."]
    #[inline(always)]
    pub fn is_maximum_value_of_minimum_back_off_exponent_min_be_must_be_smaller_or_equal_to_max_be(
        &self,
    ) -> bool {
        * self == MIN_BE_A :: MAXIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT_MIN_BE_MUST_BE_SMALLER_OR_EQUAL_TO_MAX_BE
    }
}
#[doc = "Field `MIN_BE` writer - Minimum Back-off Exponent"]
pub type MIN_BE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MIN_BE_A>;
impl<'a, REG> MIN_BE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum value of minimum back-off exponent."]
    #[inline(always)]
    pub fn minimum_value_of_minimum_back_off_exponent(self) -> &'a mut crate::W<REG> {
        self.variant(MIN_BE_A::MINIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT)
    }
    #[doc = "Maximum value of minimum back-off exponent. MIN_BE must be smaller or equal to MAX_BE."]
    #[inline(always)]
    pub fn maximum_value_of_minimum_back_off_exponent_min_be_must_be_smaller_or_equal_to_max_be(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (MIN_BE_A :: MAXIMUM_VALUE_OF_MINIMUM_BACK_OFF_EXPONENT_MIN_BE_MUST_BE_SMALLER_OR_EQUAL_TO_MAX_BE)
    }
}
#[doc = "Field `MAX_BE` reader - Maximum Back-off Exponent"]
pub type MAX_BE_R = crate::FieldReader<MAX_BE_A>;
#[doc = "Maximum Back-off Exponent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAX_BE_A {
    #[doc = "2: This value is not valid for the maximum back-off exponent."]
    THIS_VALUE_IS_NOT_VALID_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT = 2,
    #[doc = "3: Minimum, IEEE compliant value for the maximum back-off exponent."]
    MINIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT = 3,
    #[doc = "8: Maximum, IEEE compliant value for the maximum back-off exponent."]
    MAXIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT = 8,
}
impl From<MAX_BE_A> for u8 {
    #[inline(always)]
    fn from(variant: MAX_BE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAX_BE_A {
    type Ux = u8;
}
impl MAX_BE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAX_BE_A> {
        match self.bits {
            2 => Some(MAX_BE_A::THIS_VALUE_IS_NOT_VALID_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT),
            3 => Some(MAX_BE_A::MINIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT),
            8 => Some(MAX_BE_A::MAXIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT),
            _ => None,
        }
    }
    #[doc = "This value is not valid for the maximum back-off exponent."]
    #[inline(always)]
    pub fn is_this_value_is_not_valid_for_the_maximum_back_off_exponent(&self) -> bool {
        *self == MAX_BE_A::THIS_VALUE_IS_NOT_VALID_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT
    }
    #[doc = "Minimum, IEEE compliant value for the maximum back-off exponent."]
    #[inline(always)]
    pub fn is_minimum_ieee_compliant_value_for_the_maximum_back_off_exponent(&self) -> bool {
        *self == MAX_BE_A::MINIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT
    }
    #[doc = "Maximum, IEEE compliant value for the maximum back-off exponent."]
    #[inline(always)]
    pub fn is_maximum_ieee_compliant_value_for_the_maximum_back_off_exponent(&self) -> bool {
        *self == MAX_BE_A::MAXIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT
    }
}
#[doc = "Field `MAX_BE` writer - Maximum Back-off Exponent"]
pub type MAX_BE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MAX_BE_A>;
impl<'a, REG> MAX_BE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This value is not valid for the maximum back-off exponent."]
    #[inline(always)]
    pub fn this_value_is_not_valid_for_the_maximum_back_off_exponent(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(MAX_BE_A::THIS_VALUE_IS_NOT_VALID_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT)
    }
    #[doc = "Minimum, IEEE compliant value for the maximum back-off exponent."]
    #[inline(always)]
    pub fn minimum_ieee_compliant_value_for_the_maximum_back_off_exponent(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(MAX_BE_A::MINIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT)
    }
    #[doc = "Maximum, IEEE compliant value for the maximum back-off exponent."]
    #[inline(always)]
    pub fn maximum_ieee_compliant_value_for_the_maximum_back_off_exponent(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(MAX_BE_A::MAXIMUM_IEEE_COMPLIANT_VALUE_FOR_THE_MAXIMUM_BACK_OFF_EXPONENT)
    }
}
impl R {
    #[doc = "Bits 0:3 - Minimum Back-off Exponent"]
    #[inline(always)]
    pub fn min_be(&self) -> MIN_BE_R {
        MIN_BE_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Maximum Back-off Exponent"]
    #[inline(always)]
    pub fn max_be(&self) -> MAX_BE_R {
        MAX_BE_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Minimum Back-off Exponent"]
    #[inline(always)]
    #[must_use]
    pub fn min_be(&mut self) -> MIN_BE_W<CSMA_BE_SPEC> {
        MIN_BE_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Maximum Back-off Exponent"]
    #[inline(always)]
    #[must_use]
    pub fn max_be(&mut self) -> MAX_BE_W<CSMA_BE_SPEC> {
        MAX_BE_W::new(self, 4)
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
#[doc = "Transceiver CSMA-CA Back-off Exponent Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csma_be::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csma_be::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSMA_BE_SPEC;
impl crate::RegisterSpec for CSMA_BE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csma_be::R`](R) reader structure"]
impl crate::Readable for CSMA_BE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csma_be::W`](W) writer structure"]
impl crate::Writable for CSMA_BE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSMA_BE to value 0"]
impl crate::Resettable for CSMA_BE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
