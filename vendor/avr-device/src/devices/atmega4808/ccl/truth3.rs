#[doc = "Register `TRUTH3` reader"]
pub type R = crate::R<TRUTH3_SPEC>;
#[doc = "Register `TRUTH3` writer"]
pub type W = crate::W<TRUTH3_SPEC>;
#[doc = "Field `TRUTH` reader - Truth Table"]
pub type TRUTH_R = crate::FieldReader;
#[doc = "Field `TRUTH` writer - Truth Table"]
pub type TRUTH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Truth Table"]
    #[inline(always)]
    pub fn truth(&self) -> TRUTH_R {
        TRUTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Truth Table"]
    #[inline(always)]
    #[must_use]
    pub fn truth(&mut self) -> TRUTH_W<TRUTH3_SPEC> {
        TRUTH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Truth 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`truth3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`truth3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRUTH3_SPEC;
impl crate::RegisterSpec for TRUTH3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`truth3::R`](R) reader structure"]
impl crate::Readable for TRUTH3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`truth3::W`](W) writer structure"]
impl crate::Writable for TRUTH3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRUTH3 to value 0"]
impl crate::Resettable for TRUTH3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
