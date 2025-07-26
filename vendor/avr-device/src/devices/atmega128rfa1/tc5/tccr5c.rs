#[doc = "Register `TCCR5C` reader"]
pub type R = crate::R<TCCR5C_SPEC>;
#[doc = "Register `TCCR5C` writer"]
pub type W = crate::W<TCCR5C_SPEC>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `FOC5C` reader - Force Output Compare for Channel C"]
pub type FOC5C_R = crate::BitReader;
#[doc = "Field `FOC5C` writer - Force Output Compare for Channel C"]
pub type FOC5C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC5B` reader - Force Output Compare for Channel B"]
pub type FOC5B_R = crate::BitReader;
#[doc = "Field `FOC5B` writer - Force Output Compare for Channel B"]
pub type FOC5B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOC5A` reader - Force Output Compare for Channel A"]
pub type FOC5A_R = crate::BitReader;
#[doc = "Field `FOC5A` writer - Force Output Compare for Channel A"]
pub type FOC5A_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Force Output Compare for Channel C"]
    #[inline(always)]
    pub fn foc5c(&self) -> FOC5C_R {
        FOC5C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    pub fn foc5b(&self) -> FOC5B_R {
        FOC5B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    pub fn foc5a(&self) -> FOC5A_R {
        FOC5A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TCCR5C_SPEC> {
        RES_W::new(self, 0)
    }
    #[doc = "Bit 5 - Force Output Compare for Channel C"]
    #[inline(always)]
    #[must_use]
    pub fn foc5c(&mut self) -> FOC5C_W<TCCR5C_SPEC> {
        FOC5C_W::new(self, 5)
    }
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn foc5b(&mut self) -> FOC5B_W<TCCR5C_SPEC> {
        FOC5B_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn foc5a(&mut self) -> FOC5A_W<TCCR5C_SPEC> {
        FOC5A_W::new(self, 7)
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
#[doc = "Timer/Counter5 Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR5C_SPEC;
impl crate::RegisterSpec for TCCR5C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr5c::R`](R) reader structure"]
impl crate::Readable for TCCR5C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr5c::W`](W) writer structure"]
impl crate::Writable for TCCR5C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR5C to value 0"]
impl crate::Resettable for TCCR5C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
