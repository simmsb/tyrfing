#[doc = "Register `OSC16ERR3V` reader"]
pub type R = crate::R<OSC16ERR3V_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OSC16ERR3V_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "OSC16 error at 3V\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc16err3v::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSC16ERR3V_SPEC;
impl crate::RegisterSpec for OSC16ERR3V_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osc16err3v::R`](R) reader structure"]
impl crate::Readable for OSC16ERR3V_SPEC {}
#[doc = "`reset()` method sets OSC16ERR3V to value 0"]
impl crate::Resettable for OSC16ERR3V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
