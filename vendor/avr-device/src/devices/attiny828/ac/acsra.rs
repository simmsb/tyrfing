#[doc = "Register `ACSRA` reader"]
pub type R = crate::R<ACSRA_SPEC>;
#[doc = "Register `ACSRA` writer"]
pub type W = crate::W<ACSRA_SPEC>;
#[doc = "Field `ACIS` reader - Analog Comparator Interrupt Mode Select"]
pub type ACIS_R = crate::FieldReader<ACIS_A>;
#[doc = "Analog Comparator Interrupt Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACIS_A {
    #[doc = "0: Interrupt on Toggle"]
    ON_TOGGLE = 0,
    #[doc = "2: Interrupt on Falling Edge"]
    ON_FALLING_EDGE = 2,
    #[doc = "3: Interrupt on Rising Edge"]
    ON_RISING_EDGE = 3,
}
impl From<ACIS_A> for u8 {
    #[inline(always)]
    fn from(variant: ACIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACIS_A {
    type Ux = u8;
}
impl ACIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ACIS_A> {
        match self.bits {
            0 => Some(ACIS_A::ON_TOGGLE),
            2 => Some(ACIS_A::ON_FALLING_EDGE),
            3 => Some(ACIS_A::ON_RISING_EDGE),
            _ => None,
        }
    }
    #[doc = "Interrupt on Toggle"]
    #[inline(always)]
    pub fn is_on_toggle(&self) -> bool {
        *self == ACIS_A::ON_TOGGLE
    }
    #[doc = "Interrupt on Falling Edge"]
    #[inline(always)]
    pub fn is_on_falling_edge(&self) -> bool {
        *self == ACIS_A::ON_FALLING_EDGE
    }
    #[doc = "Interrupt on Rising Edge"]
    #[inline(always)]
    pub fn is_on_rising_edge(&self) -> bool {
        *self == ACIS_A::ON_RISING_EDGE
    }
}
#[doc = "Field `ACIS` writer - Analog Comparator Interrupt Mode Select"]
pub type ACIS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ACIS_A>;
impl<'a, REG> ACIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt on Toggle"]
    #[inline(always)]
    pub fn on_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(ACIS_A::ON_TOGGLE)
    }
    #[doc = "Interrupt on Falling Edge"]
    #[inline(always)]
    pub fn on_falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ACIS_A::ON_FALLING_EDGE)
    }
    #[doc = "Interrupt on Rising Edge"]
    #[inline(always)]
    pub fn on_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ACIS_A::ON_RISING_EDGE)
    }
}
#[doc = "Field `ACIC` reader - Analog Comparator Input Capture Enable"]
pub type ACIC_R = crate::BitReader;
#[doc = "Field `ACIC` writer - Analog Comparator Input Capture Enable"]
pub type ACIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACIE` reader - Analog Comparator Interrupt Enable"]
pub type ACIE_R = crate::BitReader;
#[doc = "Field `ACIE` writer - Analog Comparator Interrupt Enable"]
pub type ACIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACI` reader - Analog Comparator Interrupt Flag"]
pub type ACI_R = crate::BitReader;
#[doc = "Field `ACI` writer - Analog Comparator Interrupt Flag"]
pub type ACI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACO` reader - Analog Compare Output"]
pub type ACO_R = crate::BitReader;
#[doc = "Field `ACPMUX2` reader - Analog Comparator Positive Input Multiplexer Bit 2"]
pub type ACPMUX2_R = crate::BitReader;
#[doc = "Field `ACPMUX2` writer - Analog Comparator Positive Input Multiplexer Bit 2"]
pub type ACPMUX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACD` reader - Analog Comparator Disable"]
pub type ACD_R = crate::BitReader;
#[doc = "Field `ACD` writer - Analog Comparator Disable"]
pub type ACD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select"]
    #[inline(always)]
    pub fn acis(&self) -> ACIS_R {
        ACIS_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    pub fn acic(&self) -> ACIC_R {
        ACIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator Interrupt Enable"]
    #[inline(always)]
    pub fn acie(&self) -> ACIE_R {
        ACIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn aci(&self) -> ACI_R {
        ACI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Compare Output"]
    #[inline(always)]
    pub fn aco(&self) -> ACO_R {
        ACO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator Positive Input Multiplexer Bit 2"]
    #[inline(always)]
    pub fn acpmux2(&self) -> ACPMUX2_R {
        ACPMUX2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator Disable"]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Comparator Interrupt Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn acis(&mut self) -> ACIS_W<ACSRA_SPEC> {
        ACIS_W::new(self, 0)
    }
    #[doc = "Bit 2 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acic(&mut self) -> ACIC_W<ACSRA_SPEC> {
        ACIC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Analog Comparator Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acie(&mut self) -> ACIE_W<ACSRA_SPEC> {
        ACIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aci(&mut self) -> ACI_W<ACSRA_SPEC> {
        ACI_W::new(self, 4)
    }
    #[doc = "Bit 6 - Analog Comparator Positive Input Multiplexer Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn acpmux2(&mut self) -> ACPMUX2_W<ACSRA_SPEC> {
        ACPMUX2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog Comparator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acd(&mut self) -> ACD_W<ACSRA_SPEC> {
        ACD_W::new(self, 7)
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
#[doc = "Analog Comparator Control And Status Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSRA_SPEC;
impl crate::RegisterSpec for ACSRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acsra::R`](R) reader structure"]
impl crate::Readable for ACSRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsra::W`](W) writer structure"]
impl crate::Writable for ACSRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSRA to value 0"]
impl crate::Resettable for ACSRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
