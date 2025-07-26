#[doc = "Register `ADMUXB` reader"]
pub type R = crate::R<ADMUXB_SPEC>;
#[doc = "Register `ADMUXB` writer"]
pub type W = crate::W<ADMUXB_SPEC>;
#[doc = "Field `MUX5` reader - Analog Channel Selection Bit 5"]
pub type MUX5_R = crate::BitReader;
#[doc = "Field `MUX5` writer - Analog Channel Selection Bit 5"]
pub type MUX5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFS` reader - Reference Selection Bit"]
pub type REFS_R = crate::BitReader<REFS_A>;
#[doc = "Reference Selection Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFS_A {
    #[doc = "0: Vcc used as analog reference"]
    VCC = 0,
    #[doc = "1: Internal 1.1V Voltage Reference"]
    INTERNAL = 1,
}
impl From<REFS_A> for bool {
    #[inline(always)]
    fn from(variant: REFS_A) -> Self {
        variant as u8 != 0
    }
}
impl REFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFS_A {
        match self.bits {
            false => REFS_A::VCC,
            true => REFS_A::INTERNAL,
        }
    }
    #[doc = "Vcc used as analog reference"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == REFS_A::VCC
    }
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == REFS_A::INTERNAL
    }
}
#[doc = "Field `REFS` writer - Reference Selection Bit"]
pub type REFS_W<'a, REG> = crate::BitWriter<'a, REG, REFS_A>;
impl<'a, REG> REFS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Vcc used as analog reference"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::VCC)
    }
    #[doc = "Internal 1.1V Voltage Reference"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(REFS_A::INTERNAL)
    }
}
impl R {
    #[doc = "Bit 0 - Analog Channel Selection Bit 5"]
    #[inline(always)]
    pub fn mux5(&self) -> MUX5_R {
        MUX5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Reference Selection Bit"]
    #[inline(always)]
    pub fn refs(&self) -> REFS_R {
        REFS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Channel Selection Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn mux5(&mut self) -> MUX5_W<ADMUXB_SPEC> {
        MUX5_W::new(self, 0)
    }
    #[doc = "Bit 5 - Reference Selection Bit"]
    #[inline(always)]
    #[must_use]
    pub fn refs(&mut self) -> REFS_W<ADMUXB_SPEC> {
        REFS_W::new(self, 5)
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
#[doc = "The ADC multiplexer Selection Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`admuxb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`admuxb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADMUXB_SPEC;
impl crate::RegisterSpec for ADMUXB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`admuxb::R`](R) reader structure"]
impl crate::Readable for ADMUXB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`admuxb::W`](W) writer structure"]
impl crate::Writable for ADMUXB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADMUXB to value 0"]
impl crate::Resettable for ADMUXB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
