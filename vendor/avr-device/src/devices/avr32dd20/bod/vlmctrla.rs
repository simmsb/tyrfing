#[doc = "Register `VLMCTRLA` reader"]
pub type R = crate::R<VLMCTRLA_SPEC>;
#[doc = "Register `VLMCTRLA` writer"]
pub type W = crate::W<VLMCTRLA_SPEC>;
#[doc = "Field `VLMLVL` reader - voltage level monitor level"]
pub type VLMLVL_R = crate::FieldReader<VLMLVL_A>;
#[doc = "voltage level monitor level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VLMLVL_A {
    #[doc = "0: VLM Disabled"]
    OFF = 0,
    #[doc = "1: VLM threshold 5% above BOD level"]
    _5ABOVE = 1,
    #[doc = "2: VLM threshold 15% above BOD level"]
    _15ABOVE = 2,
    #[doc = "3: VLM threshold 25% above BOD level"]
    _25ABOVE = 3,
}
impl From<VLMLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: VLMLVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VLMLVL_A {
    type Ux = u8;
}
impl VLMLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLMLVL_A {
        match self.bits {
            0 => VLMLVL_A::OFF,
            1 => VLMLVL_A::_5ABOVE,
            2 => VLMLVL_A::_15ABOVE,
            3 => VLMLVL_A::_25ABOVE,
            _ => unreachable!(),
        }
    }
    #[doc = "VLM Disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == VLMLVL_A::OFF
    }
    #[doc = "VLM threshold 5% above BOD level"]
    #[inline(always)]
    pub fn is_5above(&self) -> bool {
        *self == VLMLVL_A::_5ABOVE
    }
    #[doc = "VLM threshold 15% above BOD level"]
    #[inline(always)]
    pub fn is_15above(&self) -> bool {
        *self == VLMLVL_A::_15ABOVE
    }
    #[doc = "VLM threshold 25% above BOD level"]
    #[inline(always)]
    pub fn is_25above(&self) -> bool {
        *self == VLMLVL_A::_25ABOVE
    }
}
#[doc = "Field `VLMLVL` writer - voltage level monitor level"]
pub type VLMLVL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VLMLVL_A>;
impl<'a, REG> VLMLVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VLM Disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(VLMLVL_A::OFF)
    }
    #[doc = "VLM threshold 5% above BOD level"]
    #[inline(always)]
    pub fn _5above(self) -> &'a mut crate::W<REG> {
        self.variant(VLMLVL_A::_5ABOVE)
    }
    #[doc = "VLM threshold 15% above BOD level"]
    #[inline(always)]
    pub fn _15above(self) -> &'a mut crate::W<REG> {
        self.variant(VLMLVL_A::_15ABOVE)
    }
    #[doc = "VLM threshold 25% above BOD level"]
    #[inline(always)]
    pub fn _25above(self) -> &'a mut crate::W<REG> {
        self.variant(VLMLVL_A::_25ABOVE)
    }
}
impl R {
    #[doc = "Bits 0:1 - voltage level monitor level"]
    #[inline(always)]
    pub fn vlmlvl(&self) -> VLMLVL_R {
        VLMLVL_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - voltage level monitor level"]
    #[inline(always)]
    #[must_use]
    pub fn vlmlvl(&mut self) -> VLMLVL_W<VLMCTRLA_SPEC> {
        VLMLVL_W::new(self, 0)
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
#[doc = "Voltage level monitor Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlmctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlmctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VLMCTRLA_SPEC;
impl crate::RegisterSpec for VLMCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vlmctrla::R`](R) reader structure"]
impl crate::Readable for VLMCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vlmctrla::W`](W) writer structure"]
impl crate::Writable for VLMCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLMCTRLA to value 0"]
impl crate::Resettable for VLMCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
