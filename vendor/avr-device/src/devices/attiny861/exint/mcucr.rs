#[doc = "Register `MCUCR` reader"]
pub type R = crate::R<MCUCR_SPEC>;
#[doc = "Register `MCUCR` writer"]
pub type W = crate::W<MCUCR_SPEC>;
#[doc = "Field `ISC00` reader - Interrupt Sense Control 0 Bit 0"]
pub type ISC00_R = crate::BitReader<ISC00_A>;
#[doc = "Interrupt Sense Control 0 Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC00_A {
    #[doc = "0: Low Level of INTX"]
    VAL_0X00 = 0,
    #[doc = "1: Any Logical Change of INTX"]
    VAL_0X01 = 1,
}
impl From<ISC00_A> for bool {
    #[inline(always)]
    fn from(variant: ISC00_A) -> Self {
        variant as u8 != 0
    }
}
impl ISC00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISC00_A {
        match self.bits {
            false => ISC00_A::VAL_0X00,
            true => ISC00_A::VAL_0X01,
        }
    }
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == ISC00_A::VAL_0X00
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == ISC00_A::VAL_0X01
    }
}
#[doc = "Field `ISC00` writer - Interrupt Sense Control 0 Bit 0"]
pub type ISC00_W<'a, REG> = crate::BitWriter<'a, REG, ISC00_A>;
impl<'a, REG> ISC00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Level of INTX"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(ISC00_A::VAL_0X00)
    }
    #[doc = "Any Logical Change of INTX"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(ISC00_A::VAL_0X01)
    }
}
#[doc = "Field `ISC01` reader - Interrupt Sense Control 0 Bit 1"]
pub type ISC01_R = crate::BitReader;
#[doc = "Field `ISC01` writer - Interrupt Sense Control 0 Bit 1"]
pub type ISC01_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Sense Control 0 Bit 0"]
    #[inline(always)]
    pub fn isc00(&self) -> ISC00_R {
        ISC00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Sense Control 0 Bit 1"]
    #[inline(always)]
    pub fn isc01(&self) -> ISC01_R {
        ISC01_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Control 0 Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn isc00(&mut self) -> ISC00_W<MCUCR_SPEC> {
        ISC00_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Sense Control 0 Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn isc01(&mut self) -> ISC01_W<MCUCR_SPEC> {
        ISC01_W::new(self, 1)
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
#[doc = "MCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCUCR_SPEC;
impl crate::RegisterSpec for MCUCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcucr::R`](R) reader structure"]
impl crate::Readable for MCUCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcucr::W`](W) writer structure"]
impl crate::Writable for MCUCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUCR to value 0"]
impl crate::Resettable for MCUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
