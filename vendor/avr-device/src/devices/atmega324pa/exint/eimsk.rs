#[doc = "Register `EIMSK` reader"]
pub type R = crate::R<EIMSK_SPEC>;
#[doc = "Register `EIMSK` writer"]
pub type W = crate::W<EIMSK_SPEC>;
#[doc = "Field `INT0` reader - External Interrupt 0 Request Enable"]
pub type INT0_R = crate::BitReader;
#[doc = "Field `INT0` writer - External Interrupt 0 Request Enable"]
pub type INT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - External Interrupt 1 Request Enable"]
pub type INT1_R = crate::BitReader;
#[doc = "Field `INT1` writer - External Interrupt 1 Request Enable"]
pub type INT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - External Interrupt 2 Request Enable"]
pub type INT2_R = crate::BitReader;
#[doc = "Field `INT2` writer - External Interrupt 2 Request Enable"]
pub type INT2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Request Enable"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Request Enable"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Request Enable"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<EIMSK_SPEC> {
        INT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<EIMSK_SPEC> {
        INT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt 2 Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<EIMSK_SPEC> {
        INT2_W::new(self, 2)
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
#[doc = "External Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eimsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eimsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EIMSK_SPEC;
impl crate::RegisterSpec for EIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eimsk::R`](R) reader structure"]
impl crate::Readable for EIMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eimsk::W`](W) writer structure"]
impl crate::Writable for EIMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMSK to value 0"]
impl crate::Resettable for EIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
