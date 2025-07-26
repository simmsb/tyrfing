#[doc = "Register `NEMCR` reader"]
pub type R = crate::R<NEMCR_SPEC>;
#[doc = "Register `NEMCR` writer"]
pub type W = crate::W<NEMCR_SPEC>;
#[doc = "Field `AEAM` reader - Address for Extended Address Mode of Extra Rows"]
pub type AEAM_R = crate::FieldReader<AEAM_A>;
#[doc = "Address for Extended Address Mode of Extra Rows\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AEAM_A {
    #[doc = "0: Factory Row"]
    FACTORY_ROW = 0,
    #[doc = "1: User Row 1"]
    USER_ROW_1 = 1,
    #[doc = "2: User Row 2"]
    USER_ROW_2 = 2,
    #[doc = "3: User Row 3"]
    USER_ROW_3 = 3,
}
impl From<AEAM_A> for u8 {
    #[inline(always)]
    fn from(variant: AEAM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AEAM_A {
    type Ux = u8;
}
impl AEAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AEAM_A {
        match self.bits {
            0 => AEAM_A::FACTORY_ROW,
            1 => AEAM_A::USER_ROW_1,
            2 => AEAM_A::USER_ROW_2,
            3 => AEAM_A::USER_ROW_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Factory Row"]
    #[inline(always)]
    pub fn is_factory_row(&self) -> bool {
        *self == AEAM_A::FACTORY_ROW
    }
    #[doc = "User Row 1"]
    #[inline(always)]
    pub fn is_user_row_1(&self) -> bool {
        *self == AEAM_A::USER_ROW_1
    }
    #[doc = "User Row 2"]
    #[inline(always)]
    pub fn is_user_row_2(&self) -> bool {
        *self == AEAM_A::USER_ROW_2
    }
    #[doc = "User Row 3"]
    #[inline(always)]
    pub fn is_user_row_3(&self) -> bool {
        *self == AEAM_A::USER_ROW_3
    }
}
#[doc = "Field `AEAM` writer - Address for Extended Address Mode of Extra Rows"]
pub type AEAM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AEAM_A>;
impl<'a, REG> AEAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Factory Row"]
    #[inline(always)]
    pub fn factory_row(self) -> &'a mut crate::W<REG> {
        self.variant(AEAM_A::FACTORY_ROW)
    }
    #[doc = "User Row 1"]
    #[inline(always)]
    pub fn user_row_1(self) -> &'a mut crate::W<REG> {
        self.variant(AEAM_A::USER_ROW_1)
    }
    #[doc = "User Row 2"]
    #[inline(always)]
    pub fn user_row_2(self) -> &'a mut crate::W<REG> {
        self.variant(AEAM_A::USER_ROW_2)
    }
    #[doc = "User Row 3"]
    #[inline(always)]
    pub fn user_row_3(self) -> &'a mut crate::W<REG> {
        self.variant(AEAM_A::USER_ROW_3)
    }
}
#[doc = "Field `ENEAM` reader - Enable Extended Address Mode for Extra Rows"]
pub type ENEAM_R = crate::BitReader;
#[doc = "Field `ENEAM` writer - Enable Extended Address Mode for Extra Rows"]
pub type ENEAM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - Address for Extended Address Mode of Extra Rows"]
    #[inline(always)]
    pub fn aeam(&self) -> AEAM_R {
        AEAM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Enable Extended Address Mode for Extra Rows"]
    #[inline(always)]
    pub fn eneam(&self) -> ENEAM_R {
        ENEAM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Address for Extended Address Mode of Extra Rows"]
    #[inline(always)]
    #[must_use]
    pub fn aeam(&mut self) -> AEAM_W<NEMCR_SPEC> {
        AEAM_W::new(self, 4)
    }
    #[doc = "Bit 6 - Enable Extended Address Mode for Extra Rows"]
    #[inline(always)]
    #[must_use]
    pub fn eneam(&mut self) -> ENEAM_W<NEMCR_SPEC> {
        ENEAM_W::new(self, 6)
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
#[doc = "Flash Extended-Mode Control-Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nemcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nemcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NEMCR_SPEC;
impl crate::RegisterSpec for NEMCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`nemcr::R`](R) reader structure"]
impl crate::Readable for NEMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nemcr::W`](W) writer structure"]
impl crate::Writable for NEMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NEMCR to value 0"]
impl crate::Resettable for NEMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
