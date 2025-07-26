#[doc = "Register `ADC0REF` reader"]
pub type R = crate::R<ADC0REF_SPEC>;
#[doc = "Register `ADC0REF` writer"]
pub type W = crate::W<ADC0REF_SPEC>;
#[doc = "Field `REFSEL` reader - Reference select"]
pub type REFSEL_R = crate::FieldReader<REFSEL_A>;
#[doc = "Reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal 1.024V reference"]
    _1V024 = 0,
    #[doc = "1: Internal 2.048V reference"]
    _2V048 = 1,
    #[doc = "2: Internal 4.096V reference"]
    _4V096 = 2,
    #[doc = "3: Internal 2.500V reference"]
    _2V500 = 3,
    #[doc = "5: VDD as reference"]
    VDD = 5,
    #[doc = "6: External reference on VREFA pin"]
    VREFA = 6,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFSEL_A {
    type Ux = u8;
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::_1V024),
            1 => Some(REFSEL_A::_2V048),
            2 => Some(REFSEL_A::_4V096),
            3 => Some(REFSEL_A::_2V500),
            5 => Some(REFSEL_A::VDD),
            6 => Some(REFSEL_A::VREFA),
            _ => None,
        }
    }
    #[doc = "Internal 1.024V reference"]
    #[inline(always)]
    pub fn is_1v024(&self) -> bool {
        *self == REFSEL_A::_1V024
    }
    #[doc = "Internal 2.048V reference"]
    #[inline(always)]
    pub fn is_2v048(&self) -> bool {
        *self == REFSEL_A::_2V048
    }
    #[doc = "Internal 4.096V reference"]
    #[inline(always)]
    pub fn is_4v096(&self) -> bool {
        *self == REFSEL_A::_4V096
    }
    #[doc = "Internal 2.500V reference"]
    #[inline(always)]
    pub fn is_2v500(&self) -> bool {
        *self == REFSEL_A::_2V500
    }
    #[doc = "VDD as reference"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
    #[doc = "External reference on VREFA pin"]
    #[inline(always)]
    pub fn is_vrefa(&self) -> bool {
        *self == REFSEL_A::VREFA
    }
}
#[doc = "Field `REFSEL` writer - Reference select"]
pub type REFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, REFSEL_A>;
impl<'a, REG> REFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 1.024V reference"]
    #[inline(always)]
    pub fn _1v024(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::_1V024)
    }
    #[doc = "Internal 2.048V reference"]
    #[inline(always)]
    pub fn _2v048(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::_2V048)
    }
    #[doc = "Internal 4.096V reference"]
    #[inline(always)]
    pub fn _4v096(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::_4V096)
    }
    #[doc = "Internal 2.500V reference"]
    #[inline(always)]
    pub fn _2v500(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::_2V500)
    }
    #[doc = "VDD as reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "External reference on VREFA pin"]
    #[inline(always)]
    pub fn vrefa(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::VREFA)
    }
}
#[doc = "Field `ALWAYSON` reader - Always on"]
pub type ALWAYSON_R = crate::BitReader;
#[doc = "Field `ALWAYSON` writer - Always on"]
pub type ALWAYSON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - Always on"]
    #[inline(always)]
    pub fn alwayson(&self) -> ALWAYSON_R {
        ALWAYSON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<ADC0REF_SPEC> {
        REFSEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Always on"]
    #[inline(always)]
    #[must_use]
    pub fn alwayson(&mut self) -> ALWAYSON_W<ADC0REF_SPEC> {
        ALWAYSON_W::new(self, 7)
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
#[doc = "ADC0 Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc0ref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc0ref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC0REF_SPEC;
impl crate::RegisterSpec for ADC0REF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc0ref::R`](R) reader structure"]
impl crate::Readable for ADC0REF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc0ref::W`](W) writer structure"]
impl crate::Writable for ADC0REF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0REF to value 0"]
impl crate::Resettable for ADC0REF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
