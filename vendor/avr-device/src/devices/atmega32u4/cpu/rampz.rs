#[doc = "Register `RAMPZ` reader"]
pub type R = crate::R<RAMPZ_SPEC>;
#[doc = "Register `RAMPZ` writer"]
pub type W = crate::W<RAMPZ_SPEC>;
#[doc = "Field `RAMPZ` reader - Extended Z-Pointer Value"]
pub type RAMPZ_R = crate::FieldReader<RAMPZ_A>;
#[doc = "Extended Z-Pointer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPZ_A {
    #[doc = "0: Default value of Z-pointer MSB's."]
    VAL_0 = 0,
}
impl From<RAMPZ_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMPZ_A {
    type Ux = u8;
}
impl RAMPZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RAMPZ_A> {
        match self.bits {
            0 => Some(RAMPZ_A::VAL_0),
            _ => None,
        }
    }
    #[doc = "Default value of Z-pointer MSB's."]
    #[inline(always)]
    pub fn is_val_0(&self) -> bool {
        *self == RAMPZ_A::VAL_0
    }
}
#[doc = "Field `RAMPZ` writer - Extended Z-Pointer Value"]
pub type RAMPZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RAMPZ_A>;
impl<'a, REG> RAMPZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default value of Z-pointer MSB's."]
    #[inline(always)]
    pub fn val_0(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPZ_A::VAL_0)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Extended Z-Pointer Value"]
    #[inline(always)]
    pub fn rampz(&self) -> RAMPZ_R {
        RAMPZ_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Extended Z-Pointer Value"]
    #[inline(always)]
    #[must_use]
    pub fn rampz(&mut self) -> RAMPZ_W<RAMPZ_SPEC> {
        RAMPZ_W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<RAMPZ_SPEC> {
        RES_W::new(self, 2)
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
#[doc = "Extended Z-pointer Register for ELPM/SPM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rampz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rampz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAMPZ_SPEC;
impl crate::RegisterSpec for RAMPZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rampz::R`](R) reader structure"]
impl crate::Readable for RAMPZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rampz::W`](W) writer structure"]
impl crate::Writable for RAMPZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMPZ to value 0"]
impl crate::Resettable for RAMPZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
