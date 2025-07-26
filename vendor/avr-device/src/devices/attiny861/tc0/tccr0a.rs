#[doc = "Register `TCCR0A` reader"]
pub type R = crate::R<TCCR0A_SPEC>;
#[doc = "Register `TCCR0A` writer"]
pub type W = crate::W<TCCR0A_SPEC>;
#[doc = "Field `WGM00` reader - Waveform Generation Mode"]
pub type WGM00_R = crate::BitReader;
#[doc = "Field `WGM00` writer - Waveform Generation Mode"]
pub type WGM00_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACIC0` reader - Analog Comparator Input Capture Enable"]
pub type ACIC0_R = crate::BitReader;
#[doc = "Field `ACIC0` writer - Analog Comparator Input Capture Enable"]
pub type ACIC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICES0` reader - Input Capture Edge Select"]
pub type ICES0_R = crate::BitReader;
#[doc = "Field `ICES0` writer - Input Capture Edge Select"]
pub type ICES0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNC0` reader - Input Capture Noice Canceler"]
pub type ICNC0_R = crate::BitReader;
#[doc = "Field `ICNC0` writer - Input Capture Noice Canceler"]
pub type ICNC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN0` reader - Input Capture Mode Enable"]
pub type ICEN0_R = crate::BitReader;
#[doc = "Field `ICEN0` writer - Input Capture Mode Enable"]
pub type ICEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCW0` reader - Timer/Counter 0 Width"]
pub type TCW0_R = crate::BitReader;
#[doc = "Field `TCW0` writer - Timer/Counter 0 Width"]
pub type TCW0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm00(&self) -> WGM00_R {
        WGM00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    pub fn acic0(&self) -> ACIC0_R {
        ACIC0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn ices0(&self) -> ICES0_R {
        ICES0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Capture Noice Canceler"]
    #[inline(always)]
    pub fn icnc0(&self) -> ICNC0_R {
        ICNC0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Capture Mode Enable"]
    #[inline(always)]
    pub fn icen0(&self) -> ICEN0_R {
        ICEN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter 0 Width"]
    #[inline(always)]
    pub fn tcw0(&self) -> TCW0_R {
        TCW0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm00(&mut self) -> WGM00_W<TCCR0A_SPEC> {
        WGM00_W::new(self, 0)
    }
    #[doc = "Bit 3 - Analog Comparator Input Capture Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acic0(&mut self) -> ACIC0_W<TCCR0A_SPEC> {
        ACIC0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Input Capture Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn ices0(&mut self) -> ICES0_W<TCCR0A_SPEC> {
        ICES0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Input Capture Noice Canceler"]
    #[inline(always)]
    #[must_use]
    pub fn icnc0(&mut self) -> ICNC0_W<TCCR0A_SPEC> {
        ICNC0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Input Capture Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen0(&mut self) -> ICEN0_W<TCCR0A_SPEC> {
        ICEN0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer/Counter 0 Width"]
    #[inline(always)]
    #[must_use]
    pub fn tcw0(&mut self) -> TCW0_W<TCCR0A_SPEC> {
        TCW0_W::new(self, 7)
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
#[doc = "Timer/Counter Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR0A_SPEC;
impl crate::RegisterSpec for TCCR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr0a::R`](R) reader structure"]
impl crate::Readable for TCCR0A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr0a::W`](W) writer structure"]
impl crate::Writable for TCCR0A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0A to value 0"]
impl crate::Resettable for TCCR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
