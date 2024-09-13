#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `WGMODE` reader - Waveform generation mode"]
pub type WGMODE_R = crate::FieldReader<WGMODE_A>;
#[doc = "Waveform generation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGMODE_A {
    #[doc = "0: One ramp mode"]
    ONERAMP = 0,
    #[doc = "1: Two ramp mode"]
    TWORAMP = 1,
    #[doc = "2: Four ramp mode"]
    FOURRAMP = 2,
    #[doc = "3: Dual slope mode"]
    DS = 3,
}
impl From<WGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WGMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGMODE_A {
    type Ux = u8;
}
impl WGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGMODE_A {
        match self.bits {
            0 => WGMODE_A::ONERAMP,
            1 => WGMODE_A::TWORAMP,
            2 => WGMODE_A::FOURRAMP,
            3 => WGMODE_A::DS,
            _ => unreachable!(),
        }
    }
    #[doc = "One ramp mode"]
    #[inline(always)]
    pub fn is_oneramp(&self) -> bool {
        *self == WGMODE_A::ONERAMP
    }
    #[doc = "Two ramp mode"]
    #[inline(always)]
    pub fn is_tworamp(&self) -> bool {
        *self == WGMODE_A::TWORAMP
    }
    #[doc = "Four ramp mode"]
    #[inline(always)]
    pub fn is_fourramp(&self) -> bool {
        *self == WGMODE_A::FOURRAMP
    }
    #[doc = "Dual slope mode"]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == WGMODE_A::DS
    }
}
#[doc = "Field `WGMODE` writer - Waveform generation mode"]
pub type WGMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGMODE_A>;
impl<'a, REG> WGMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One ramp mode"]
    #[inline(always)]
    pub fn oneramp(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::ONERAMP)
    }
    #[doc = "Two ramp mode"]
    #[inline(always)]
    pub fn tworamp(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::TWORAMP)
    }
    #[doc = "Four ramp mode"]
    #[inline(always)]
    pub fn fourramp(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::FOURRAMP)
    }
    #[doc = "Dual slope mode"]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(WGMODE_A::DS)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform generation mode"]
    #[inline(always)]
    pub fn wgmode(&self) -> WGMODE_R {
        WGMODE_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform generation mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgmode(&mut self) -> WGMODE_W<CTRLB_SPEC> {
        WGMODE_W::new(self, 0)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
