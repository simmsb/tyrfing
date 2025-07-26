#[doc = "Register `TCCR4E` reader"]
pub type R = crate::R<TCCR4E_SPEC>;
#[doc = "Register `TCCR4E` writer"]
pub type W = crate::W<TCCR4E_SPEC>;
#[doc = "Field `OC4OE` reader - Output Compare Override Enable bit"]
pub type OC4OE_R = crate::FieldReader;
#[doc = "Field `OC4OE` writer - Output Compare Override Enable bit"]
pub type OC4OE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `ENHC4` reader - Enhanced Compare/PWM Mode"]
pub type ENHC4_R = crate::BitReader;
#[doc = "Field `ENHC4` writer - Enhanced Compare/PWM Mode"]
pub type ENHC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLOCK4` reader - Register Update Lock"]
pub type TLOCK4_R = crate::BitReader;
#[doc = "Field `TLOCK4` writer - Register Update Lock"]
pub type TLOCK4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Output Compare Override Enable bit"]
    #[inline(always)]
    pub fn oc4oe(&self) -> OC4OE_R {
        OC4OE_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Enhanced Compare/PWM Mode"]
    #[inline(always)]
    pub fn enhc4(&self) -> ENHC4_R {
        ENHC4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Register Update Lock"]
    #[inline(always)]
    pub fn tlock4(&self) -> TLOCK4_R {
        TLOCK4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Output Compare Override Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn oc4oe(&mut self) -> OC4OE_W<TCCR4E_SPEC> {
        OC4OE_W::new(self, 0)
    }
    #[doc = "Bit 6 - Enhanced Compare/PWM Mode"]
    #[inline(always)]
    #[must_use]
    pub fn enhc4(&mut self) -> ENHC4_W<TCCR4E_SPEC> {
        ENHC4_W::new(self, 6)
    }
    #[doc = "Bit 7 - Register Update Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tlock4(&mut self) -> TLOCK4_W<TCCR4E_SPEC> {
        TLOCK4_W::new(self, 7)
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
#[doc = "Timer/Counter 4 Control Register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR4E_SPEC;
impl crate::RegisterSpec for TCCR4E_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr4e::R`](R) reader structure"]
impl crate::Readable for TCCR4E_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr4e::W`](W) writer structure"]
impl crate::Writable for TCCR4E_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4E to value 0"]
impl crate::Resettable for TCCR4E_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
