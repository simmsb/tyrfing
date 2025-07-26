#[doc = "Register `TCDROUTEA` reader"]
pub type R = crate::R<TCDROUTEA_SPEC>;
#[doc = "Register `TCDROUTEA` writer"]
pub type W = crate::W<TCDROUTEA_SPEC>;
#[doc = "Field `TCD0` reader - TCD0 Signals"]
pub type TCD0_R = crate::FieldReader<TCD0_A>;
#[doc = "TCD0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCD0_A {
    #[doc = "0: WOx: PA4, PA5, PA6, PA7"]
    DEFAULT = 0,
    #[doc = "4: WOx: PA4, PA5, PD4, PD5"]
    ALT4 = 4,
}
impl From<TCD0_A> for u8 {
    #[inline(always)]
    fn from(variant: TCD0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCD0_A {
    type Ux = u8;
}
impl TCD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCD0_A> {
        match self.bits {
            0 => Some(TCD0_A::DEFAULT),
            4 => Some(TCD0_A::ALT4),
            _ => None,
        }
    }
    #[doc = "WOx: PA4, PA5, PA6, PA7"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCD0_A::DEFAULT
    }
    #[doc = "WOx: PA4, PA5, PD4, PD5"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == TCD0_A::ALT4
    }
}
#[doc = "Field `TCD0` writer - TCD0 Signals"]
pub type TCD0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TCD0_A>;
impl<'a, REG> TCD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WOx: PA4, PA5, PA6, PA7"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCD0_A::DEFAULT)
    }
    #[doc = "WOx: PA4, PA5, PD4, PD5"]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut crate::W<REG> {
        self.variant(TCD0_A::ALT4)
    }
}
impl R {
    #[doc = "Bits 0:2 - TCD0 Signals"]
    #[inline(always)]
    pub fn tcd0(&self) -> TCD0_R {
        TCD0_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - TCD0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn tcd0(&mut self) -> TCD0_W<TCDROUTEA_SPEC> {
        TCD0_W::new(self, 0)
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
#[doc = "TCD route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcdroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcdroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCDROUTEA_SPEC;
impl crate::RegisterSpec for TCDROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcdroutea::R`](R) reader structure"]
impl crate::Readable for TCDROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcdroutea::W`](W) writer structure"]
impl crate::Writable for TCDROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCDROUTEA to value 0"]
impl crate::Resettable for TCDROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
