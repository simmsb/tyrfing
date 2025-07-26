#[doc = "Register `EIMSK` reader"]
pub type R = crate::R<EIMSK_SPEC>;
#[doc = "Register `EIMSK` writer"]
pub type W = crate::W<EIMSK_SPEC>;
#[doc = "Field `INT` reader - External Interrupt Request Enable"]
pub type INT_R = crate::FieldReader<INT_A>;
#[doc = "External Interrupt Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INT_A {
    #[doc = "0: All external pin interrupts are disabled."]
    ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED = 0,
    #[doc = "255: All external pin interrupts are enabled."]
    ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED = 255,
}
impl From<INT_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INT_A {
    type Ux = u8;
}
impl INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INT_A> {
        match self.bits {
            0 => Some(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED),
            255 => Some(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED),
            _ => None,
        }
    }
    #[doc = "All external pin interrupts are disabled."]
    #[inline(always)]
    pub fn is_all_external_pin_interrupts_are_disabled(&self) -> bool {
        *self == INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED
    }
    #[doc = "All external pin interrupts are enabled."]
    #[inline(always)]
    pub fn is_all_external_pin_interrupts_are_enabled(&self) -> bool {
        *self == INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED
    }
}
#[doc = "Field `INT` writer - External Interrupt Request Enable"]
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, INT_A>;
impl<'a, REG> INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All external pin interrupts are disabled."]
    #[inline(always)]
    pub fn all_external_pin_interrupts_are_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_DISABLED)
    }
    #[doc = "All external pin interrupts are enabled."]
    #[inline(always)]
    pub fn all_external_pin_interrupts_are_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(INT_A::ALL_EXTERNAL_PIN_INTERRUPTS_ARE_ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - External Interrupt Request Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Interrupt Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<EIMSK_SPEC> {
        INT_W::new(self, 0)
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
#[doc = "External Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eimsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eimsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EIMSK_SPEC;
impl crate::RegisterSpec for EIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eimsk::R`](R) reader structure"]
impl crate::Readable for EIMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eimsk::W`](W) writer structure"]
impl crate::Writable for EIMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMSK to value 0"]
impl crate::Resettable for EIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
