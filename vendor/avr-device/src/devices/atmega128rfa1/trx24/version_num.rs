#[doc = "Register `VERSION_NUM` reader"]
pub type R = crate::R<VERSION_NUM_SPEC>;
#[doc = "Register `VERSION_NUM` writer"]
pub type W = crate::W<VERSION_NUM_SPEC>;
#[doc = "Field `VERSION_NUM` reader - Version Number"]
pub type VERSION_NUM_R = crate::FieldReader<VERSION_NUM_A>;
#[doc = "Version Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VERSION_NUM_A {
    #[doc = "2: Revision A"]
    REV_A = 2,
    #[doc = "3: Revision B"]
    REV_B = 3,
}
impl From<VERSION_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: VERSION_NUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VERSION_NUM_A {
    type Ux = u8;
}
impl VERSION_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VERSION_NUM_A> {
        match self.bits {
            2 => Some(VERSION_NUM_A::REV_A),
            3 => Some(VERSION_NUM_A::REV_B),
            _ => None,
        }
    }
    #[doc = "Revision A"]
    #[inline(always)]
    pub fn is_rev_a(&self) -> bool {
        *self == VERSION_NUM_A::REV_A
    }
    #[doc = "Revision B"]
    #[inline(always)]
    pub fn is_rev_b(&self) -> bool {
        *self == VERSION_NUM_A::REV_B
    }
}
#[doc = "Field `VERSION_NUM` writer - Version Number"]
pub type VERSION_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8, VERSION_NUM_A>;
impl<'a, REG> VERSION_NUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Revision A"]
    #[inline(always)]
    pub fn rev_a(self) -> &'a mut crate::W<REG> {
        self.variant(VERSION_NUM_A::REV_A)
    }
    #[doc = "Revision B"]
    #[inline(always)]
    pub fn rev_b(self) -> &'a mut crate::W<REG> {
        self.variant(VERSION_NUM_A::REV_B)
    }
}
impl R {
    #[doc = "Bits 0:7 - Version Number"]
    #[inline(always)]
    pub fn version_num(&self) -> VERSION_NUM_R {
        VERSION_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Version Number"]
    #[inline(always)]
    #[must_use]
    pub fn version_num(&mut self) -> VERSION_NUM_W<VERSION_NUM_SPEC> {
        VERSION_NUM_W::new(self, 0)
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
#[doc = "Device Identification Register (Version Number)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_NUM_SPEC;
impl crate::RegisterSpec for VERSION_NUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`version_num::R`](R) reader structure"]
impl crate::Readable for VERSION_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`version_num::W`](W) writer structure"]
impl crate::Writable for VERSION_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VERSION_NUM to value 0"]
impl crate::Resettable for VERSION_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
