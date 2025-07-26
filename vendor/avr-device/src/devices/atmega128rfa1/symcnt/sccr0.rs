#[doc = "Register `SCCR0` reader"]
pub type R = crate::R<SCCR0_SPEC>;
#[doc = "Register `SCCR0` writer"]
pub type W = crate::W<SCCR0_SPEC>;
#[doc = "Field `SCCMP` reader - Symbol Counter Compare Unit 3 Mode select"]
pub type SCCMP_R = crate::FieldReader;
#[doc = "Field `SCCMP` writer - Symbol Counter Compare Unit 3 Mode select"]
pub type SCCMP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `SCTSE` reader - Symbol Counter Automatic Timestamping enable"]
pub type SCTSE_R = crate::BitReader;
#[doc = "Field `SCTSE` writer - Symbol Counter Automatic Timestamping enable"]
pub type SCTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCCKSEL` reader - Symbol Counter Clock Source select"]
pub type SCCKSEL_R = crate::BitReader;
#[doc = "Field `SCCKSEL` writer - Symbol Counter Clock Source select"]
pub type SCCKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCEN` reader - Symbol Counter enable"]
pub type SCEN_R = crate::BitReader;
#[doc = "Field `SCEN` writer - Symbol Counter enable"]
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMBTS` reader - Manual Beacon Timestamp"]
pub type SCMBTS_R = crate::BitReader;
#[doc = "Field `SCMBTS` writer - Manual Beacon Timestamp"]
pub type SCMBTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRES` reader - Symbol Counter Synchronization"]
pub type SCRES_R = crate::BitReader;
#[doc = "Field `SCRES` writer - Symbol Counter Synchronization"]
pub type SCRES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Symbol Counter Compare Unit 3 Mode select"]
    #[inline(always)]
    pub fn sccmp(&self) -> SCCMP_R {
        SCCMP_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Symbol Counter Automatic Timestamping enable"]
    #[inline(always)]
    pub fn sctse(&self) -> SCTSE_R {
        SCTSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Symbol Counter Clock Source select"]
    #[inline(always)]
    pub fn sccksel(&self) -> SCCKSEL_R {
        SCCKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Symbol Counter enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Manual Beacon Timestamp"]
    #[inline(always)]
    pub fn scmbts(&self) -> SCMBTS_R {
        SCMBTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Symbol Counter Synchronization"]
    #[inline(always)]
    pub fn scres(&self) -> SCRES_R {
        SCRES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Symbol Counter Compare Unit 3 Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sccmp(&mut self) -> SCCMP_W<SCCR0_SPEC> {
        SCCMP_W::new(self, 0)
    }
    #[doc = "Bit 3 - Symbol Counter Automatic Timestamping enable"]
    #[inline(always)]
    #[must_use]
    pub fn sctse(&mut self) -> SCTSE_W<SCCR0_SPEC> {
        SCTSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Symbol Counter Clock Source select"]
    #[inline(always)]
    #[must_use]
    pub fn sccksel(&mut self) -> SCCKSEL_W<SCCR0_SPEC> {
        SCCKSEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Symbol Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<SCCR0_SPEC> {
        SCEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Manual Beacon Timestamp"]
    #[inline(always)]
    #[must_use]
    pub fn scmbts(&mut self) -> SCMBTS_W<SCCR0_SPEC> {
        SCMBTS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Symbol Counter Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn scres(&mut self) -> SCRES_W<SCCR0_SPEC> {
        SCRES_W::new(self, 7)
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
#[doc = "Symbol Counter Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCR0_SPEC;
impl crate::RegisterSpec for SCCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sccr0::R`](R) reader structure"]
impl crate::Readable for SCCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccr0::W`](W) writer structure"]
impl crate::Writable for SCCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCR0 to value 0"]
impl crate::Resettable for SCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
