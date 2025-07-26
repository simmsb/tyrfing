#[doc = "Register `KEY` reader"]
pub type R = crate::R<KEY_SPEC>;
#[doc = "Register `KEY` writer"]
pub type W = crate::W<KEY_SPEC>;
#[doc = "Field `KEY` reader - Lock Key"]
pub type KEY_R = crate::FieldReader<KEY_A>;
#[doc = "Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum KEY_A {
    #[doc = "1556464988: No locks"]
    NOLOCK = 1556464988,
    #[doc = "2738502307: Read and write lock"]
    RWLOCK = 2738502307,
}
impl From<KEY_A> for u32 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_A {
    type Ux = u32;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            1556464988 => Some(KEY_A::NOLOCK),
            2738502307 => Some(KEY_A::RWLOCK),
            _ => None,
        }
    }
    #[doc = "No locks"]
    #[inline(always)]
    pub fn is_nolock(&self) -> bool {
        *self == KEY_A::NOLOCK
    }
    #[doc = "Read and write lock"]
    #[inline(always)]
    pub fn is_rwlock(&self) -> bool {
        *self == KEY_A::RWLOCK
    }
}
#[doc = "Field `KEY` writer - Lock Key"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, KEY_A>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No locks"]
    #[inline(always)]
    pub fn nolock(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::NOLOCK)
    }
    #[doc = "Read and write lock"]
    #[inline(always)]
    pub fn rwlock(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::RWLOCK)
    }
}
impl R {
    #[doc = "Bits 0:31 - Lock Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KEY_SPEC> {
        KEY_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Lock Key Bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_SPEC;
impl crate::RegisterSpec for KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
