#[doc = "Register `WDTCSR` reader"]
pub type R = crate::R<WDTCSR_SPEC>;
#[doc = "Register `WDTCSR` writer"]
pub type W = crate::W<WDTCSR_SPEC>;
#[doc = "Field `WDP` reader - Watchdog Timer Prescaler Bits"]
pub type WDP_R = crate::FieldReader<WDP_A>;
#[doc = "Watchdog Timer Prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDP_A {
    #[doc = "0: Oscillator Cycles 2K"]
    OSCILLATOR_CYCLES_2K = 0,
    #[doc = "1: Oscillator Cycles 4K"]
    OSCILLATOR_CYCLES_4K = 1,
    #[doc = "2: Oscillator Cycles 8K"]
    OSCILLATOR_CYCLES_8K = 2,
    #[doc = "3: Oscillator Cycles 16K"]
    OSCILLATOR_CYCLES_16K = 3,
    #[doc = "4: Oscillator Cycles 32K"]
    OSCILLATOR_CYCLES_32K = 4,
    #[doc = "5: Oscillator Cycles 64K"]
    OSCILLATOR_CYCLES_64K = 5,
    #[doc = "6: Oscillator Cycles 128K"]
    OSCILLATOR_CYCLES_128K = 6,
    #[doc = "7: Oscillator Cycles 256K"]
    OSCILLATOR_CYCLES_256K = 7,
}
impl From<WDP_A> for u8 {
    #[inline(always)]
    fn from(variant: WDP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDP_A {
    type Ux = u8;
}
impl WDP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDP_A> {
        match self.bits {
            0 => Some(WDP_A::OSCILLATOR_CYCLES_2K),
            1 => Some(WDP_A::OSCILLATOR_CYCLES_4K),
            2 => Some(WDP_A::OSCILLATOR_CYCLES_8K),
            3 => Some(WDP_A::OSCILLATOR_CYCLES_16K),
            4 => Some(WDP_A::OSCILLATOR_CYCLES_32K),
            5 => Some(WDP_A::OSCILLATOR_CYCLES_64K),
            6 => Some(WDP_A::OSCILLATOR_CYCLES_128K),
            7 => Some(WDP_A::OSCILLATOR_CYCLES_256K),
            _ => None,
        }
    }
    #[doc = "Oscillator Cycles 2K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_2k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_2K
    }
    #[doc = "Oscillator Cycles 4K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_4k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_4K
    }
    #[doc = "Oscillator Cycles 8K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_8k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_8K
    }
    #[doc = "Oscillator Cycles 16K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_16k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_16K
    }
    #[doc = "Oscillator Cycles 32K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_32k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_32K
    }
    #[doc = "Oscillator Cycles 64K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_64k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_64K
    }
    #[doc = "Oscillator Cycles 128K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_128k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_128K
    }
    #[doc = "Oscillator Cycles 256K"]
    #[inline(always)]
    pub fn is_oscillator_cycles_256k(&self) -> bool {
        *self == WDP_A::OSCILLATOR_CYCLES_256K
    }
}
#[doc = "Field `WDP` writer - Watchdog Timer Prescaler Bits"]
pub type WDP_W<'a, REG> = crate::FieldWriter<'a, REG, 6, WDP_A>;
impl<'a, REG> WDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Oscillator Cycles 2K"]
    #[inline(always)]
    pub fn oscillator_cycles_2k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_2K)
    }
    #[doc = "Oscillator Cycles 4K"]
    #[inline(always)]
    pub fn oscillator_cycles_4k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_4K)
    }
    #[doc = "Oscillator Cycles 8K"]
    #[inline(always)]
    pub fn oscillator_cycles_8k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_8K)
    }
    #[doc = "Oscillator Cycles 16K"]
    #[inline(always)]
    pub fn oscillator_cycles_16k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_16K)
    }
    #[doc = "Oscillator Cycles 32K"]
    #[inline(always)]
    pub fn oscillator_cycles_32k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_32K)
    }
    #[doc = "Oscillator Cycles 64K"]
    #[inline(always)]
    pub fn oscillator_cycles_64k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_64K)
    }
    #[doc = "Oscillator Cycles 128K"]
    #[inline(always)]
    pub fn oscillator_cycles_128k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_128K)
    }
    #[doc = "Oscillator Cycles 256K"]
    #[inline(always)]
    pub fn oscillator_cycles_256k(self) -> &'a mut crate::W<REG> {
        self.variant(WDP_A::OSCILLATOR_CYCLES_256K)
    }
}
#[doc = "Field `WDE` reader - Watch Dog Enable"]
pub type WDE_R = crate::BitReader;
#[doc = "Field `WDE` writer - Watch Dog Enable"]
pub type WDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDCE` reader - Watchdog Change Enable"]
pub type WDCE_R = crate::BitReader;
#[doc = "Field `WDCE` writer - Watchdog Change Enable"]
pub type WDCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIE` reader - Watchdog Timeout Interrupt Enable"]
pub type WDIE_R = crate::BitReader;
#[doc = "Field `WDIE` writer - Watchdog Timeout Interrupt Enable"]
pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIF` reader - Watchdog Timeout Interrupt Flag"]
pub type WDIF_R = crate::BitReader;
#[doc = "Field `WDIF` writer - Watchdog Timeout Interrupt Flag"]
pub type WDIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Watchdog Timer Prescaler Bits"]
    #[inline(always)]
    pub fn wdp(&self) -> WDP_R {
        WDP_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 3 - Watch Dog Enable"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Change Enable"]
    #[inline(always)]
    pub fn wdce(&self) -> WDCE_R {
        WDCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Watchdog Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn wdif(&self) -> WDIF_R {
        WDIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Watchdog Timer Prescaler Bits"]
    #[inline(always)]
    #[must_use]
    pub fn wdp(&mut self) -> WDP_W<WDTCSR_SPEC> {
        WDP_W::new(self, 0)
    }
    #[doc = "Bit 3 - Watch Dog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WDE_W<WDTCSR_SPEC> {
        WDE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Watchdog Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdce(&mut self) -> WDCE_W<WDTCSR_SPEC> {
        WDCE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WDIE_W<WDTCSR_SPEC> {
        WDIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Watchdog Timeout Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdif(&mut self) -> WDIF_W<WDTCSR_SPEC> {
        WDIF_W::new(self, 7)
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
#[doc = "Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCSR_SPEC;
impl crate::RegisterSpec for WDTCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdtcsr::R`](R) reader structure"]
impl crate::Readable for WDTCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtcsr::W`](W) writer structure"]
impl crate::Writable for WDTCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCSR to value 0"]
impl crate::Resettable for WDTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
