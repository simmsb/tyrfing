#[doc = "Register `CCP` reader"]
pub type R = crate::R<CCP_SPEC>;
#[doc = "Register `CCP` writer"]
pub type W = crate::W<CCP_SPEC>;
#[doc = "Field `CCP` reader - CCP signature"]
pub type CCP_R = crate::FieldReader<CCP_A>;
#[doc = "CCP signature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCP_A {
    #[doc = "157: SPM Instruction Protection"]
    SPM = 157,
    #[doc = "216: IO Register Protection"]
    IOREG = 216,
}
impl From<CCP_A> for u8 {
    #[inline(always)]
    fn from(variant: CCP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CCP_A {
    type Ux = u8;
}
impl CCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCP_A> {
        match self.bits {
            157 => Some(CCP_A::SPM),
            216 => Some(CCP_A::IOREG),
            _ => None,
        }
    }
    #[doc = "SPM Instruction Protection"]
    #[inline(always)]
    pub fn is_spm(&self) -> bool {
        *self == CCP_A::SPM
    }
    #[doc = "IO Register Protection"]
    #[inline(always)]
    pub fn is_ioreg(&self) -> bool {
        *self == CCP_A::IOREG
    }
}
#[doc = "Field `CCP` writer - CCP signature"]
pub type CCP_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CCP_A>;
impl<'a, REG> CCP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPM Instruction Protection"]
    #[inline(always)]
    pub fn spm(self) -> &'a mut crate::W<REG> {
        self.variant(CCP_A::SPM)
    }
    #[doc = "IO Register Protection"]
    #[inline(always)]
    pub fn ioreg(self) -> &'a mut crate::W<REG> {
        self.variant(CCP_A::IOREG)
    }
}
impl R {
    #[doc = "Bits 0:7 - CCP signature"]
    #[inline(always)]
    pub fn ccp(&self) -> CCP_R {
        CCP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CCP signature"]
    #[inline(always)]
    #[must_use]
    pub fn ccp(&mut self) -> CCP_W<CCP_SPEC> {
        CCP_W::new(self, 0)
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
#[doc = "Configuration Change Protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCP_SPEC;
impl crate::RegisterSpec for CCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccp::R`](R) reader structure"]
impl crate::Readable for CCP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccp::W`](W) writer structure"]
impl crate::Writable for CCP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCP to value 0"]
impl crate::Resettable for CCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
