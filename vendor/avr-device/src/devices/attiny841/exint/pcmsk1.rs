#[doc = "Register `PCMSK1` reader"]
pub type R = crate::R<PCMSK1_SPEC>;
#[doc = "Register `PCMSK1` writer"]
pub type W = crate::W<PCMSK1_SPEC>;
#[doc = "Field `PCINT8` reader - Pin Change Enable Mask 1 Bit 0"]
pub type PCINT8_R = crate::BitReader;
#[doc = "Field `PCINT8` writer - Pin Change Enable Mask 1 Bit 0"]
pub type PCINT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT9` reader - Pin Change Enable Mask 1 Bit 1"]
pub type PCINT9_R = crate::BitReader;
#[doc = "Field `PCINT9` writer - Pin Change Enable Mask 1 Bit 1"]
pub type PCINT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT10` reader - Pin Change Enable Mask 1 Bit 2"]
pub type PCINT10_R = crate::BitReader;
#[doc = "Field `PCINT10` writer - Pin Change Enable Mask 1 Bit 2"]
pub type PCINT10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCINT11` reader - Pin Change Enable Mask 1 Bit 3"]
pub type PCINT11_R = crate::BitReader;
#[doc = "Field `PCINT11` writer - Pin Change Enable Mask 1 Bit 3"]
pub type PCINT11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin Change Enable Mask 1 Bit 0"]
    #[inline(always)]
    pub fn pcint8(&self) -> PCINT8_R {
        PCINT8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 1 Bit 1"]
    #[inline(always)]
    pub fn pcint9(&self) -> PCINT9_R {
        PCINT9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 1 Bit 2"]
    #[inline(always)]
    pub fn pcint10(&self) -> PCINT10_R {
        PCINT10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 1 Bit 3"]
    #[inline(always)]
    pub fn pcint11(&self) -> PCINT11_R {
        PCINT11_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Enable Mask 1 Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcint8(&mut self) -> PCINT8_W<PCMSK1_SPEC> {
        PCINT8_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin Change Enable Mask 1 Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcint9(&mut self) -> PCINT9_W<PCMSK1_SPEC> {
        PCINT9_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin Change Enable Mask 1 Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcint10(&mut self) -> PCINT10_W<PCMSK1_SPEC> {
        PCINT10_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin Change Enable Mask 1 Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcint11(&mut self) -> PCINT11_W<PCMSK1_SPEC> {
        PCINT11_W::new(self, 3)
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
#[doc = "Pin Change Enable Mask 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmsk1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmsk1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCMSK1_SPEC;
impl crate::RegisterSpec for PCMSK1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcmsk1::R`](R) reader structure"]
impl crate::Readable for PCMSK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcmsk1::W`](W) writer structure"]
impl crate::Writable for PCMSK1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCMSK1 to value 0"]
impl crate::Resettable for PCMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
