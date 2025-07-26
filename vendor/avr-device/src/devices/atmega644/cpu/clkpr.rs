#[doc = "Register `CLKPR` reader"]
pub type R = crate::R<CLKPR_SPEC>;
#[doc = "Register `CLKPR` writer"]
pub type W = crate::W<CLKPR_SPEC>;
#[doc = "Field `CLKPS` reader - No Description."]
pub type CLKPS_R = crate::FieldReader<CLKPS_A>;
#[doc = "No Description.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKPS_A {
    #[doc = "0: 1"]
    VAL_0X00 = 0,
    #[doc = "1: 2"]
    VAL_0X01 = 1,
    #[doc = "2: 4"]
    VAL_0X02 = 2,
    #[doc = "3: 8"]
    VAL_0X03 = 3,
    #[doc = "4: 16"]
    VAL_0X04 = 4,
    #[doc = "5: 32"]
    VAL_0X05 = 5,
    #[doc = "6: 64"]
    VAL_0X06 = 6,
    #[doc = "7: 128"]
    VAL_0X07 = 7,
    #[doc = "8: 256"]
    VAL_0X08 = 8,
}
impl From<CLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKPS_A {
    type Ux = u8;
}
impl CLKPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKPS_A> {
        match self.bits {
            0 => Some(CLKPS_A::VAL_0X00),
            1 => Some(CLKPS_A::VAL_0X01),
            2 => Some(CLKPS_A::VAL_0X02),
            3 => Some(CLKPS_A::VAL_0X03),
            4 => Some(CLKPS_A::VAL_0X04),
            5 => Some(CLKPS_A::VAL_0X05),
            6 => Some(CLKPS_A::VAL_0X06),
            7 => Some(CLKPS_A::VAL_0X07),
            8 => Some(CLKPS_A::VAL_0X08),
            _ => None,
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CLKPS_A::VAL_0X00
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CLKPS_A::VAL_0X01
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CLKPS_A::VAL_0X02
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CLKPS_A::VAL_0X03
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CLKPS_A::VAL_0X04
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CLKPS_A::VAL_0X05
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CLKPS_A::VAL_0X06
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CLKPS_A::VAL_0X07
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_val_0x08(&self) -> bool {
        *self == CLKPS_A::VAL_0X08
    }
}
#[doc = "Field `CLKPS` writer - No Description."]
pub type CLKPS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKPS_A>;
impl<'a, REG> CLKPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X00)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X01)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X02)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X03)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X04)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X05)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X06)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X07)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn val_0x08(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::VAL_0X08)
    }
}
#[doc = "Field `CLKPCE` reader - No Description."]
pub type CLKPCE_R = crate::BitReader;
#[doc = "Field `CLKPCE` writer - No Description."]
pub type CLKPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn clkpce(&self) -> CLKPCE_R {
        CLKPCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clkps(&mut self) -> CLKPS_W<CLKPR_SPEC> {
        CLKPS_W::new(self, 0)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clkpce(&mut self) -> CLKPCE_W<CLKPR_SPEC> {
        CLKPCE_W::new(self, 7)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKPR_SPEC;
impl crate::RegisterSpec for CLKPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clkpr::R`](R) reader structure"]
impl crate::Readable for CLKPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkpr::W`](W) writer structure"]
impl crate::Writable for CLKPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKPR to value 0"]
impl crate::Resettable for CLKPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
