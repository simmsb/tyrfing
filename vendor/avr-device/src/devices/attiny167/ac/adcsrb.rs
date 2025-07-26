#[doc = "Register `ADCSRB` reader"]
pub type R = crate::R<ADCSRB_SPEC>;
#[doc = "Register `ADCSRB` writer"]
pub type W = crate::W<ADCSRB_SPEC>;
#[doc = "Field `ACIR` reader - Analog Comparator Internal Voltage Reference Select Bits"]
pub type ACIR_R = crate::FieldReader;
#[doc = "Field `ACIR` writer - Analog Comparator Internal Voltage Reference Select Bits"]
pub type ACIR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ACME` reader - Analog Comparator Multiplexer Enable"]
pub type ACME_R = crate::BitReader;
#[doc = "Field `ACME` writer - Analog Comparator Multiplexer Enable"]
pub type ACME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - Analog Comparator Internal Voltage Reference Select Bits"]
    #[inline(always)]
    pub fn acir(&self) -> ACIR_R {
        ACIR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Analog Comparator Multiplexer Enable"]
    #[inline(always)]
    pub fn acme(&self) -> ACME_R {
        ACME_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Analog Comparator Internal Voltage Reference Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn acir(&mut self) -> ACIR_W<ADCSRB_SPEC> {
        ACIR_W::new(self, 4)
    }
    #[doc = "Bit 6 - Analog Comparator Multiplexer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acme(&mut self) -> ACME_W<ADCSRB_SPEC> {
        ACME_W::new(self, 6)
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
#[doc = "Analog Comparator &amp; ADC Control and Status Register B (Shared with AD_CONVERTER IO_MODULE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcsrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcsrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCSRB_SPEC;
impl crate::RegisterSpec for ADCSRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcsrb::R`](R) reader structure"]
impl crate::Readable for ADCSRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcsrb::W`](W) writer structure"]
impl crate::Writable for ADCSRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSRB to value 0"]
impl crate::Resettable for ADCSRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
