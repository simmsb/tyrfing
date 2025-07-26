#[doc = "Register `SFD_VALUE` reader"]
pub type R = crate::R<SFD_VALUE_SPEC>;
#[doc = "Register `SFD_VALUE` writer"]
pub type W = crate::W<SFD_VALUE_SPEC>;
#[doc = "Field `SFD_VALUE` reader - Start of Frame Delimiter Value"]
pub type SFD_VALUE_R = crate::FieldReader<SFD_VALUE_A>;
#[doc = "Start of Frame Delimiter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFD_VALUE_A {
    #[doc = "167: IEEE 802.15.4 compliant value of the SFD"]
    IEEE_SFD = 167,
}
impl From<SFD_VALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: SFD_VALUE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SFD_VALUE_A {
    type Ux = u8;
}
impl SFD_VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SFD_VALUE_A> {
        match self.bits {
            167 => Some(SFD_VALUE_A::IEEE_SFD),
            _ => None,
        }
    }
    #[doc = "IEEE 802.15.4 compliant value of the SFD"]
    #[inline(always)]
    pub fn is_ieee_sfd(&self) -> bool {
        *self == SFD_VALUE_A::IEEE_SFD
    }
}
#[doc = "Field `SFD_VALUE` writer - Start of Frame Delimiter Value"]
pub type SFD_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SFD_VALUE_A>;
impl<'a, REG> SFD_VALUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IEEE 802.15.4 compliant value of the SFD"]
    #[inline(always)]
    pub fn ieee_sfd(self) -> &'a mut crate::W<REG> {
        self.variant(SFD_VALUE_A::IEEE_SFD)
    }
}
impl R {
    #[doc = "Bits 0:7 - Start of Frame Delimiter Value"]
    #[inline(always)]
    pub fn sfd_value(&self) -> SFD_VALUE_R {
        SFD_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start of Frame Delimiter Value"]
    #[inline(always)]
    #[must_use]
    pub fn sfd_value(&mut self) -> SFD_VALUE_W<SFD_VALUE_SPEC> {
        SFD_VALUE_W::new(self, 0)
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
#[doc = "Start of Frame Delimiter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfd_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfd_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFD_VALUE_SPEC;
impl crate::RegisterSpec for SFD_VALUE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfd_value::R`](R) reader structure"]
impl crate::Readable for SFD_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfd_value::W`](W) writer structure"]
impl crate::Writable for SFD_VALUE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFD_VALUE to value 0"]
impl crate::Resettable for SFD_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
