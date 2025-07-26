#[doc = "Register `LOCKBIT` reader"]
pub type R = crate::R<LOCKBIT_SPEC>;
#[doc = "Register `LOCKBIT` writer"]
pub type W = crate::W<LOCKBIT_SPEC>;
#[doc = "Field `LB` reader - Lock Bits"]
pub type LB_R = crate::FieldReader<LB_A>;
#[doc = "Lock Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LB_A {
    #[doc = "58: Read and write lock"]
    RWLOCK = 58,
    #[doc = "197: No locks"]
    NOLOCK = 197,
}
impl From<LB_A> for u8 {
    #[inline(always)]
    fn from(variant: LB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LB_A {
    type Ux = u8;
}
impl LB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LB_A> {
        match self.bits {
            58 => Some(LB_A::RWLOCK),
            197 => Some(LB_A::NOLOCK),
            _ => None,
        }
    }
    #[doc = "Read and write lock"]
    #[inline(always)]
    pub fn is_rwlock(&self) -> bool {
        *self == LB_A::RWLOCK
    }
    #[doc = "No locks"]
    #[inline(always)]
    pub fn is_nolock(&self) -> bool {
        *self == LB_A::NOLOCK
    }
}
#[doc = "Field `LB` writer - Lock Bits"]
pub type LB_W<'a, REG> = crate::FieldWriter<'a, REG, 8, LB_A>;
impl<'a, REG> LB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read and write lock"]
    #[inline(always)]
    pub fn rwlock(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::RWLOCK)
    }
    #[doc = "No locks"]
    #[inline(always)]
    pub fn nolock(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::NOLOCK)
    }
}
impl R {
    #[doc = "Bits 0:7 - Lock Bits"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lock Bits"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LB_W<LOCKBIT_SPEC> {
        LB_W::new(self, 0)
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
#[doc = "Lock bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockbit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockbit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKBIT_SPEC;
impl crate::RegisterSpec for LOCKBIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lockbit::R`](R) reader structure"]
impl crate::Readable for LOCKBIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lockbit::W`](W) writer structure"]
impl crate::Writable for LOCKBIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKBIT to value 0"]
impl crate::Resettable for LOCKBIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
