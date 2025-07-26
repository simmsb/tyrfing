#[doc = "Register `DIDR2` reader"]
pub type R = crate::R<DIDR2_SPEC>;
#[doc = "Register `DIDR2` writer"]
pub type W = crate::W<DIDR2_SPEC>;
#[doc = "Field `ADC8D` reader - ADC8 Digital input Disable"]
pub type ADC8D_R = crate::BitReader;
#[doc = "Field `ADC8D` writer - ADC8 Digital input Disable"]
pub type ADC8D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC9D` reader - ADC9 Digital input Disable"]
pub type ADC9D_R = crate::BitReader;
#[doc = "Field `ADC9D` writer - ADC9 Digital input Disable"]
pub type ADC9D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10D` reader - ADC10 Digital input Disable"]
pub type ADC10D_R = crate::BitReader;
#[doc = "Field `ADC10D` writer - ADC10 Digital input Disable"]
pub type ADC10D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC11D` reader - ADC11 Digital input Disable"]
pub type ADC11D_R = crate::BitReader;
#[doc = "Field `ADC11D` writer - ADC11 Digital input Disable"]
pub type ADC11D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12D` reader - ADC12 Digital input Disable"]
pub type ADC12D_R = crate::BitReader;
#[doc = "Field `ADC12D` writer - ADC12 Digital input Disable"]
pub type ADC12D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC13D` reader - ADC13 Digital input Disable"]
pub type ADC13D_R = crate::BitReader;
#[doc = "Field `ADC13D` writer - ADC13 Digital input Disable"]
pub type ADC13D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC8 Digital input Disable"]
    #[inline(always)]
    pub fn adc8d(&self) -> ADC8D_R {
        ADC8D_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC9 Digital input Disable"]
    #[inline(always)]
    pub fn adc9d(&self) -> ADC9D_R {
        ADC9D_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10 Digital input Disable"]
    #[inline(always)]
    pub fn adc10d(&self) -> ADC10D_R {
        ADC10D_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC11 Digital input Disable"]
    #[inline(always)]
    pub fn adc11d(&self) -> ADC11D_R {
        ADC11D_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC12 Digital input Disable"]
    #[inline(always)]
    pub fn adc12d(&self) -> ADC12D_R {
        ADC12D_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC13 Digital input Disable"]
    #[inline(always)]
    pub fn adc13d(&self) -> ADC13D_R {
        ADC13D_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC8 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc8d(&mut self) -> ADC8D_W<DIDR2_SPEC> {
        ADC8D_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC9 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc9d(&mut self) -> ADC9D_W<DIDR2_SPEC> {
        ADC9D_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC10 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc10d(&mut self) -> ADC10D_W<DIDR2_SPEC> {
        ADC10D_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC11 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc11d(&mut self) -> ADC11D_W<DIDR2_SPEC> {
        ADC11D_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC12 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12d(&mut self) -> ADC12D_W<DIDR2_SPEC> {
        ADC12D_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC13 Digital input Disable"]
    #[inline(always)]
    #[must_use]
    pub fn adc13d(&mut self) -> ADC13D_W<DIDR2_SPEC> {
        ADC13D_W::new(self, 5)
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
