#[doc = "Register `ACSR` reader"]
pub type R = crate::R<ACSR_SPEC>;
#[doc = "Register `ACSR` writer"]
pub type W = crate::W<ACSR_SPEC>;
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
#[doc = "Field `ACBG` reader - Analog Comparator Bandgap Select"]
pub type ACBG_R = crate::BitReader;
#[doc = "Field `ACBG` writer - Analog Comparator Bandgap Select"]
pub type ACBG_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - Analog Comparator Bandgap Select"]
    #[inline(always)]
    pub fn acbg(&self) -> ACBG_R {
        ACBG_R::new(((self.bits >> 6) & 1) != 0)
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
    pub fn acis(&mut self) -> ACIS_W<ACSR_SPEC> {
        ACIS_W::new(self, 0)
    }
    #[doc = "Bit 3 - Analog Comparator Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acie(&mut self) -> ACIE_W<ACSR_SPEC> {
        ACIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Analog Comparator Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aci(&mut self) -> ACI_W<ACSR_SPEC> {
        ACI_W::new(self, 4)
    }
    #[doc = "Bit 6 - Analog Comparator Bandgap Select"]
    #[inline(always)]
    #[must_use]
    pub fn acbg(&mut self) -> ACBG_W<ACSR_SPEC> {
        ACBG_W::new(self, 6)
    }
    #[doc = "Bit 7 - Analog Comparator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acd(&mut self) -> ACD_W<ACSR_SPEC> {
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
#[doc = "Analog Comparator Control And Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACSR_SPEC;
impl crate::RegisterSpec for ACSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acsr::R`](R) reader structure"]
impl crate::Readable for ACSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acsr::W`](W) writer structure"]
impl crate::Writable for ACSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACSR to value 0"]
impl crate::Resettable for ACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
