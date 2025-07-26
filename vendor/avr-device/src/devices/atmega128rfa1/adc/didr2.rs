#[doc = "Register `DIDR2` reader"]
pub type R = crate::R<DIDR2_SPEC>;
#[doc = "Register `DIDR2` writer"]
pub type W = crate::W<DIDR2_SPEC>;
#[doc = "Field `ADC8D` reader - Reserved Bits"]
pub type ADC8D_R = crate::BitReader;
#[doc = "Field `ADC8D` writer - Reserved Bits"]
pub type ADC8D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC9D` reader - Reserved Bits"]
pub type ADC9D_R = crate::BitReader;
#[doc = "Field `ADC9D` writer - Reserved Bits"]
pub type ADC9D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10D` reader - Reserved Bits"]
pub type ADC10D_R = crate::BitReader;
#[doc = "Field `ADC10D` writer - Reserved Bits"]
pub type ADC10D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC11D` reader - Reserved Bits"]
pub type ADC11D_R = crate::BitReader;
#[doc = "Field `ADC11D` writer - Reserved Bits"]
pub type ADC11D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12D` reader - Reserved Bits"]
pub type ADC12D_R = crate::BitReader;
#[doc = "Field `ADC12D` writer - Reserved Bits"]
pub type ADC12D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC13D` reader - Reserved Bits"]
pub type ADC13D_R = crate::BitReader;
#[doc = "Field `ADC13D` writer - Reserved Bits"]
pub type ADC13D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC14D` reader - Reserved Bits"]
pub type ADC14D_R = crate::BitReader;
#[doc = "Field `ADC14D` writer - Reserved Bits"]
pub type ADC14D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC15D` reader - Reserved Bits"]
pub type ADC15D_R = crate::BitReader;
#[doc = "Field `ADC15D` writer - Reserved Bits"]
pub type ADC15D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved Bits"]
    #[inline(always)]
    pub fn adc8d(&self) -> ADC8D_R {
        ADC8D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved Bits"]
    #[inline(always)]
    pub fn adc9d(&self) -> ADC9D_R {
        ADC9D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved Bits"]
    #[inline(always)]
    pub fn adc10d(&self) -> ADC10D_R {
        ADC10D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved Bits"]
    #[inline(always)]
    pub fn adc11d(&self) -> ADC11D_R {
        ADC11D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved Bits"]
    #[inline(always)]
    pub fn adc12d(&self) -> ADC12D_R {
        ADC12D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved Bits"]
    #[inline(always)]
    pub fn adc13d(&self) -> ADC13D_R {
        ADC13D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved Bits"]
    #[inline(always)]
    pub fn adc14d(&self) -> ADC14D_R {
        ADC14D_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved Bits"]
    #[inline(always)]
    pub fn adc15d(&self) -> ADC15D_R {
        ADC15D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc8d(&mut self) -> ADC8D_W<DIDR2_SPEC> {
        ADC8D_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc9d(&mut self) -> ADC9D_W<DIDR2_SPEC> {
        ADC9D_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc10d(&mut self) -> ADC10D_W<DIDR2_SPEC> {
        ADC10D_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc11d(&mut self) -> ADC11D_W<DIDR2_SPEC> {
        ADC11D_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc12d(&mut self) -> ADC12D_W<DIDR2_SPEC> {
        ADC12D_W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc13d(&mut self) -> ADC13D_W<DIDR2_SPEC> {
        ADC13D_W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc14d(&mut self) -> ADC14D_W<DIDR2_SPEC> {
        ADC14D_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved Bits"]
    #[inline(always)]
    #[must_use]
    pub fn adc15d(&mut self) -> ADC15D_W<DIDR2_SPEC> {
        ADC15D_W::new(self, 7)
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
#[doc = "Digital Input Disable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`didr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`didr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIDR2_SPEC;
impl crate::RegisterSpec for DIDR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`didr2::R`](R) reader structure"]
impl crate::Readable for DIDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`didr2::W`](W) writer structure"]
impl crate::Writable for DIDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIDR2 to value 0"]
impl crate::Resettable for DIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
