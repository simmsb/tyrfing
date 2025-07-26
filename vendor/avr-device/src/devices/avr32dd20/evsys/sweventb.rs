#[doc = "Register `SWEVENTB` reader"]
pub type R = crate::R<SWEVENTB_SPEC>;
#[doc = "Register `SWEVENTB` writer"]
pub type W = crate::W<SWEVENTB_SPEC>;
#[doc = "Software event on channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWEVENTB_AW {
    #[doc = "0: Software event on channel 8"]
    CH8 = 0,
    #[doc = "1: Software event on channel 9"]
    CH9 = 1,
}
impl From<SWEVENTB_AW> for u8 {
    #[inline(always)]
    fn from(variant: SWEVENTB_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWEVENTB_AW {
    type Ux = u8;
}
#[doc = "Field `SWEVENTB` writer - Software event on channel select"]
pub type SWEVENTB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SWEVENTB_AW>;
impl<'a, REG> SWEVENTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software event on channel 8"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTB_AW::CH8)
    }
    #[doc = "Software event on channel 9"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTB_AW::CH9)
    }
}
impl W {
    #[doc = "Bits 0:1 - Software event on channel select"]
    #[inline(always)]
    #[must_use]
    pub fn sweventb(&mut self) -> SWEVENTB_W<SWEVENTB_SPEC> {
        SWEVENTB_W::new(self, 0)
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
#[doc = "Software Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sweventb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sweventb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVENTB_SPEC;
impl crate::RegisterSpec for SWEVENTB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sweventb::R`](R) reader structure"]
impl crate::Readable for SWEVENTB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sweventb::W`](W) writer structure"]
impl crate::Writable for SWEVENTB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVENTB to value 0"]
impl crate::Resettable for SWEVENTB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
