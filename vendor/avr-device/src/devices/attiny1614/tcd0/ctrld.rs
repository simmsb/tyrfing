#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<CTRLD_SPEC>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<CTRLD_SPEC>;
#[doc = "Field `CMPAVAL` reader - Compare A value"]
pub type CMPAVAL_R = crate::FieldReader;
#[doc = "Field `CMPAVAL` writer - Compare A value"]
pub type CMPAVAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `CMPBVAL` reader - Compare B value"]
pub type CMPBVAL_R = crate::FieldReader;
#[doc = "Field `CMPBVAL` writer - Compare B value"]
pub type CMPBVAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Compare A value"]
    #[inline(always)]
    pub fn cmpaval(&self) -> CMPAVAL_R {
        CMPAVAL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Compare B value"]
    #[inline(always)]
    pub fn cmpbval(&self) -> CMPBVAL_R {
        CMPBVAL_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compare A value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpaval(&mut self) -> CMPAVAL_W<CTRLD_SPEC> {
        CMPAVAL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Compare B value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpbval(&mut self) -> CMPBVAL_W<CTRLD_SPEC> {
        CMPBVAL_W::new(self, 4)
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
#[doc = "Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrld::R`](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrld::W`](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
