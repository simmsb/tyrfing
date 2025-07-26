#[doc = "Register `PART_NUM` reader"]
pub type R = crate::R<PART_NUM_SPEC>;
#[doc = "Register `PART_NUM` writer"]
pub type W = crate::W<PART_NUM_SPEC>;
#[doc = "Field `PART_NUM` reader - Part Number"]
pub type PART_NUM_R = crate::FieldReader<PART_NUM_A>;
#[doc = "Part Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PART_NUM_A {
    #[doc = "131: ATmega128RFA1 part number"]
    P_ATMEGA128RFA1 = 131,
}
impl From<PART_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: PART_NUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PART_NUM_A {
    type Ux = u8;
}
impl PART_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PART_NUM_A> {
        match self.bits {
            131 => Some(PART_NUM_A::P_ATMEGA128RFA1),
            _ => None,
        }
    }
    #[doc = "ATmega128RFA1 part number"]
    #[inline(always)]
    pub fn is_p_atmega128rfa1(&self) -> bool {
        *self == PART_NUM_A::P_ATMEGA128RFA1
    }
}
#[doc = "Field `PART_NUM` writer - Part Number"]
pub type PART_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PART_NUM_A>;
impl<'a, REG> PART_NUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ATmega128RFA1 part number"]
    #[inline(always)]
    pub fn p_atmega128rfa1(self) -> &'a mut crate::W<REG> {
        self.variant(PART_NUM_A::P_ATMEGA128RFA1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn part_num(&self) -> PART_NUM_R {
        PART_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    #[must_use]
    pub fn part_num(&mut self) -> PART_NUM_W<PART_NUM_SPEC> {
        PART_NUM_W::new(self, 0)
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
#[doc = "Device Identification Register (Part Number)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`part_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`part_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PART_NUM_SPEC;
impl crate::RegisterSpec for PART_NUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`part_num::R`](R) reader structure"]
impl crate::Readable for PART_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`part_num::W`](W) writer structure"]
impl crate::Writable for PART_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PART_NUM to value 0"]
impl crate::Resettable for PART_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
