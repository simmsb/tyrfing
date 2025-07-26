#[doc = "Register `OCDR` reader"]
pub type R = crate::R<OCDR_SPEC>;
#[doc = "Register `OCDR` writer"]
pub type W = crate::W<OCDR_SPEC>;
#[doc = "Field `OCDR` reader - On-Chip Debug Register Data"]
pub type OCDR_R = crate::FieldReader<OCDR_A>;
#[doc = "On-Chip Debug Register Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCDR_A {
    #[doc = "0: Refer to the debugger documentation for further information on how to use this register."]
    REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER = 0,
}
impl From<OCDR_A> for u8 {
    #[inline(always)]
    fn from(variant: OCDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCDR_A {
    type Ux = u8;
}
impl OCDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OCDR_A> {
        match self . bits { 0 => Some (OCDR_A :: REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER) , _ => None , }
    }
    #[doc = "Refer to the debugger documentation for further information on how to use this register."]
    #[inline(always)]
    pub fn is_refer_to_the_debugger_documentation_for_further_information_on_how_to_use_this_register(
        &self,
    ) -> bool {
        * self == OCDR_A :: REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER
    }
}
#[doc = "Field `OCDR` writer - On-Chip Debug Register Data"]
pub type OCDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, OCDR_A>;
impl<'a, REG> OCDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Refer to the debugger documentation for further information on how to use this register."]
    #[inline(always)]
    pub fn refer_to_the_debugger_documentation_for_further_information_on_how_to_use_this_register(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (OCDR_A :: REFER_TO_THE_DEBUGGER_DOCUMENTATION_FOR_FURTHER_INFORMATION_ON_HOW_TO_USE_THIS_REGISTER)
    }
}
impl R {
    #[doc = "Bits 0:7 - On-Chip Debug Register Data"]
    #[inline(always)]
    pub fn ocdr(&self) -> OCDR_R {
        OCDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - On-Chip Debug Register Data"]
    #[inline(always)]
    #[must_use]
    pub fn ocdr(&mut self) -> OCDR_W<OCDR_SPEC> {
        OCDR_W::new(self, 0)
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
#[doc = "On-Chip Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCDR_SPEC;
impl crate::RegisterSpec for OCDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocdr::R`](R) reader structure"]
impl crate::Readable for OCDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocdr::W`](W) writer structure"]
impl crate::Writable for OCDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCDR to value 0"]
impl crate::Resettable for OCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
