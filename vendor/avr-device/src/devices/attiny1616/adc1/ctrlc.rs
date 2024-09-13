#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CTRLC_SPEC>;
#[doc = "Field `PRESC` reader - Clock Pre-scaler"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "Clock Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: CLK_PER divided by 2"]
    DIV2 = 0,
    #[doc = "1: CLK_PER divided by 4"]
    DIV4 = 1,
    #[doc = "2: CLK_PER divided by 8"]
    DIV8 = 2,
    #[doc = "3: CLK_PER divided by 16"]
    DIV16 = 3,
    #[doc = "4: CLK_PER divided by 32"]
    DIV32 = 4,
    #[doc = "5: CLK_PER divided by 64"]
    DIV64 = 5,
    #[doc = "6: CLK_PER divided by 128"]
    DIV128 = 6,
    #[doc = "7: CLK_PER divided by 256"]
    DIV256 = 7,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::DIV2,
            1 => PRESC_A::DIV4,
            2 => PRESC_A::DIV8,
            3 => PRESC_A::DIV16,
            4 => PRESC_A::DIV32,
            5 => PRESC_A::DIV64,
            6 => PRESC_A::DIV128,
            7 => PRESC_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_PER divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "CLK_PER divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "CLK_PER divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "CLK_PER divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "CLK_PER divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "CLK_PER divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "CLK_PER divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
    #[doc = "CLK_PER divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC_A::DIV256
    }
}
#[doc = "Field `PRESC` writer - Clock Pre-scaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PRESC_A>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_PER divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "CLK_PER divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "CLK_PER divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "CLK_PER divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "CLK_PER divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "CLK_PER divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "CLK_PER divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV128)
    }
    #[doc = "CLK_PER divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV256)
    }
}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type REFSEL_R = crate::FieldReader<REFSEL_A>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Internal reference"]
    INTREF = 0,
    #[doc = "1: VDD"]
    VDDREF = 1,
    #[doc = "2: External reference"]
    VREFA = 2,
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
            0 => Some(REFSEL_A::INTREF),
            1 => Some(REFSEL_A::VDDREF),
            2 => Some(REFSEL_A::VREFA),
            _ => None,
        }
    }
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == REFSEL_A::INTREF
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn is_vddref(&self) -> bool {
        *self == REFSEL_A::VDDREF
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn is_vrefa(&self) -> bool {
        *self == REFSEL_A::VREFA
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type REFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, REFSEL_A>;
impl<'a, REG> REFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::INTREF)
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vddref(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::VDDREF)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn vrefa(self) -> &'a mut crate::W<REG> {
        self.variant(REFSEL_A::VREFA)
    }
}
#[doc = "Field `SAMPCAP` reader - Sample Capacitance Selection"]
pub type SAMPCAP_R = crate::BitReader;
#[doc = "Field `SAMPCAP` writer - Sample Capacitance Selection"]
pub type SAMPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Pre-scaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:5 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Sample Capacitance Selection"]
    #[inline(always)]
    pub fn sampcap(&self) -> SAMPCAP_R {
        SAMPCAP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRLC_SPEC> {
        PRESC_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<CTRLC_SPEC> {
        REFSEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - Sample Capacitance Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sampcap(&mut self) -> SAMPCAP_W<CTRLC_SPEC> {
        SAMPCAP_W::new(self, 6)
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
#[doc = "Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLC_SPEC;
impl crate::RegisterSpec for CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
