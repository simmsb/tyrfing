#[doc = "Register `TCCR4C` reader"]
pub type R = crate::R<TCCR4C_SPEC>;
#[doc = "Register `TCCR4C` writer"]
pub type W = crate::W<TCCR4C_SPEC>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `FOC4C` reader - Force Output Compare for Channel C"]
pub type FOC4C_R = crate::BitReader;
#[doc = "Field `FOC4C` writer - Force Output Compare for Channel C"]
pub type FOC4C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC4B` reader - Force Output Compare for Channel B"]
pub type FOC4B_R = crate::BitReader;
#[doc = "Field `FOC4B` writer - Force Output Compare for Channel B"]
pub type FOC4B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC4A` reader - Force Output Compare for Channel A"]
pub type FOC4A_R = crate::BitReader;
#[doc = "Field `FOC4A` writer - Force Output Compare for Channel A"]
pub type FOC4A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Force Output Compare for Channel C"]
    #[inline(always)]
    pub fn foc4c(&self) -> FOC4C_R {
        FOC4C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    pub fn foc4b(&self) -> FOC4B_R {
        FOC4B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    pub fn foc4a(&self) -> FOC4A_R {
        FOC4A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TCCR4C_SPEC> {
        RES_W::new(self, 0)
    }
    #[doc = "Bit 5 - Force Output Compare for Channel C"]
    #[inline(always)]
    #[must_use]
    pub fn foc4c(&mut self) -> FOC4C_W<TCCR4C_SPEC> {
        FOC4C_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn foc4b(&mut self) -> FOC4B_W<TCCR4C_SPEC> {
        FOC4B_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn foc4a(&mut self) -> FOC4A_W<TCCR4C_SPEC> {
        FOC4A_W::new(self, 7)
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
#[doc = "Timer/Counter4 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR4C_SPEC;
impl crate::RegisterSpec for TCCR4C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr4c::R`](R) reader structure"]
impl crate::Readable for TCCR4C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr4c::W`](W) writer structure"]
impl crate::Writable for TCCR4C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR4C to value 0"]
impl crate::Resettable for TCCR4C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
