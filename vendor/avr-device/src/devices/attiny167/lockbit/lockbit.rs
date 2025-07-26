#[doc = "Register `LOCKBIT` reader"]
pub type R = crate::R<LOCKBIT_SPEC>;
#[doc = "Register `LOCKBIT` writer"]
pub type W = crate::W<LOCKBIT_SPEC>;
#[doc = "Field `LB` reader - Memory Lock"]
pub type LB_R = crate::FieldReader<LB_A>;
#[doc = "Memory Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LB_A {
    #[doc = "0: Further programming and verification disabled"]
    PROG_VER_DISABLED = 0,
    #[doc = "2: Further programming disabled"]
    PROG_DISABLED = 2,
    #[doc = "3: No memory lock features enabled"]
    NO_LOCK = 3,
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
            0 => Some(LB_A::PROG_VER_DISABLED),
            2 => Some(LB_A::PROG_DISABLED),
            3 => Some(LB_A::NO_LOCK),
            _ => None,
        }
    }
    #[doc = "Further programming and verification disabled"]
    #[inline(always)]
    pub fn is_prog_ver_disabled(&self) -> bool {
        *self == LB_A::PROG_VER_DISABLED
    }
    #[doc = "Further programming disabled"]
    #[inline(always)]
    pub fn is_prog_disabled(&self) -> bool {
        *self == LB_A::PROG_DISABLED
    }
    #[doc = "No memory lock features enabled"]
    #[inline(always)]
    pub fn is_no_lock(&self) -> bool {
        *self == LB_A::NO_LOCK
    }
}
#[doc = "Field `LB` writer - Memory Lock"]
pub type LB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LB_A>;
impl<'a, REG> LB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Further programming and verification disabled"]
    #[inline(always)]
    pub fn prog_ver_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::PROG_VER_DISABLED)
    }
    #[doc = "Further programming disabled"]
    #[inline(always)]
    pub fn prog_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::PROG_DISABLED)
    }
    #[doc = "No memory lock features enabled"]
    #[inline(always)]
    pub fn no_lock(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::NO_LOCK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Lock"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Lock"]
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockbit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockbit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
