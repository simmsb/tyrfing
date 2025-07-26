#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `LVL` reader - Bod level"]
pub type LVL_R = crate::FieldReader<LVL_A>;
#[doc = "Bod level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.8 V"]
    BODLEVEL0 = 0,
    #[doc = "1: 2.1 V"]
    BODLEVEL1 = 1,
    #[doc = "2: 2.6 V"]
    BODLEVEL2 = 2,
    #[doc = "3: 2.9 V"]
    BODLEVEL3 = 3,
    #[doc = "4: 3.3 V"]
    BODLEVEL4 = 4,
    #[doc = "5: 3.7 V"]
    BODLEVEL5 = 5,
    #[doc = "6: 4.0 V"]
    BODLEVEL6 = 6,
    #[doc = "7: 4.2 V"]
    BODLEVEL7 = 7,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LVL_A {
    type Ux = u8;
}
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LVL_A {
        match self.bits {
            0 => LVL_A::BODLEVEL0,
            1 => LVL_A::BODLEVEL1,
            2 => LVL_A::BODLEVEL2,
            3 => LVL_A::BODLEVEL3,
            4 => LVL_A::BODLEVEL4,
            5 => LVL_A::BODLEVEL5,
            6 => LVL_A::BODLEVEL6,
            7 => LVL_A::BODLEVEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn is_bodlevel1(&self) -> bool {
        *self == LVL_A::BODLEVEL1
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn is_bodlevel3(&self) -> bool {
        *self == LVL_A::BODLEVEL3
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn is_bodlevel4(&self) -> bool {
        *self == LVL_A::BODLEVEL4
    }
    #[doc = "3.7 V"]
    #[inline(always)]
    pub fn is_bodlevel5(&self) -> bool {
        *self == LVL_A::BODLEVEL5
    }
    #[doc = "4.0 V"]
    #[inline(always)]
    pub fn is_bodlevel6(&self) -> bool {
        *self == LVL_A::BODLEVEL6
    }
    #[doc = "4.2 V"]
    #[inline(always)]
    pub fn is_bodlevel7(&self) -> bool {
        *self == LVL_A::BODLEVEL7
    }
}
impl R {
    #[doc = "Bits 0:2 - Bod level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(self.bits & 7)
    }
}
impl W {
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
