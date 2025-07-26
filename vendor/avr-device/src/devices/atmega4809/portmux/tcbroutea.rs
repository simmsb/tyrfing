#[doc = "Register `TCBROUTEA` reader"]
pub type R = crate::R<TCBROUTEA_SPEC>;
#[doc = "Register `TCBROUTEA` writer"]
pub type W = crate::W<TCBROUTEA_SPEC>;
#[doc = "Field `TCB0` reader - Port Multiplexer TCB0"]
pub type TCB0_R = crate::BitReader;
#[doc = "Field `TCB0` writer - Port Multiplexer TCB0"]
pub type TCB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCB1` reader - Port Multiplexer TCB1"]
pub type TCB1_R = crate::BitReader;
#[doc = "Field `TCB1` writer - Port Multiplexer TCB1"]
pub type TCB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCB2` reader - Port Multiplexer TCB2"]
pub type TCB2_R = crate::BitReader;
#[doc = "Field `TCB2` writer - Port Multiplexer TCB2"]
pub type TCB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCB3` reader - Port Multiplexer TCB3"]
pub type TCB3_R = crate::BitReader;
#[doc = "Field `TCB3` writer - Port Multiplexer TCB3"]
pub type TCB3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    pub fn tcb0(&self) -> TCB0_R {
        TCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    pub fn tcb1(&self) -> TCB1_R {
        TCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Multiplexer TCB2"]
    #[inline(always)]
    pub fn tcb2(&self) -> TCB2_R {
        TCB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Multiplexer TCB3"]
    #[inline(always)]
    pub fn tcb3(&self) -> TCB3_R {
        TCB3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    #[must_use]
    pub fn tcb0(&mut self) -> TCB0_W<TCBROUTEA_SPEC> {
        TCB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    #[must_use]
    pub fn tcb1(&mut self) -> TCB1_W<TCBROUTEA_SPEC> {
        TCB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Multiplexer TCB2"]
    #[inline(always)]
    #[must_use]
    pub fn tcb2(&mut self) -> TCB2_W<TCBROUTEA_SPEC> {
        TCB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Multiplexer TCB3"]
    #[inline(always)]
    #[must_use]
    pub fn tcb3(&mut self) -> TCB3_W<TCBROUTEA_SPEC> {
        TCB3_W::new(self, 3)
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
#[doc = "Port Multiplexer TCB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcbroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcbroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCBROUTEA_SPEC;
impl crate::RegisterSpec for TCBROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcbroutea::R`](R) reader structure"]
impl crate::Readable for TCBROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcbroutea::W`](W) writer structure"]
impl crate::Writable for TCBROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCBROUTEA to value 0"]
impl crate::Resettable for TCBROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
