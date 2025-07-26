#[doc = "Register `XMCRB` reader"]
pub type R = crate::R<XMCRB_SPEC>;
#[doc = "Register `XMCRB` writer"]
pub type W = crate::W<XMCRB_SPEC>;
#[doc = "Field `XMM` reader - External Memory High Mask"]
pub type XMM_R = crate::FieldReader<XMM_A>;
#[doc = "External Memory High Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XMM_A {
    #[doc = "0: None"]
    VAL_0X00 = 0,
    #[doc = "1: Px7"]
    VAL_0X01 = 1,
    #[doc = "2: Px7-Px6"]
    VAL_0X02 = 2,
    #[doc = "3: Px7-Px5"]
    VAL_0X03 = 3,
    #[doc = "4: Px7-Px4"]
    VAL_0X04 = 4,
    #[doc = "5: Px7-Px3"]
    VAL_0X05 = 5,
    #[doc = "6: Px7-Px2"]
    VAL_0X06 = 6,
    #[doc = "7: Full Port X"]
    VAL_0X07 = 7,
}
impl From<XMM_A> for u8 {
    #[inline(always)]
    fn from(variant: XMM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XMM_A {
    type Ux = u8;
}
impl XMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XMM_A {
        match self.bits {
            0 => XMM_A::VAL_0X00,
            1 => XMM_A::VAL_0X01,
            2 => XMM_A::VAL_0X02,
            3 => XMM_A::VAL_0X03,
            4 => XMM_A::VAL_0X04,
            5 => XMM_A::VAL_0X05,
            6 => XMM_A::VAL_0X06,
            7 => XMM_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == XMM_A::VAL_0X00
    }
    #[doc = "Px7"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == XMM_A::VAL_0X01
    }
    #[doc = "Px7-Px6"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == XMM_A::VAL_0X02
    }
    #[doc = "Px7-Px5"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == XMM_A::VAL_0X03
    }
    #[doc = "Px7-Px4"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == XMM_A::VAL_0X04
    }
    #[doc = "Px7-Px3"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == XMM_A::VAL_0X05
    }
    #[doc = "Px7-Px2"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == XMM_A::VAL_0X06
    }
    #[doc = "Full Port X"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == XMM_A::VAL_0X07
    }
}
#[doc = "Field `XMM` writer - External Memory High Mask"]
pub type XMM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, XMM_A>;
impl<'a, REG> XMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X00)
    }
    #[doc = "Px7"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X01)
    }
    #[doc = "Px7-Px6"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X02)
    }
    #[doc = "Px7-Px5"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X03)
    }
    #[doc = "Px7-Px4"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X04)
    }
    #[doc = "Px7-Px3"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X05)
    }
    #[doc = "Px7-Px2"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X06)
    }
    #[doc = "Full Port X"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut crate::W<REG> {
        self.variant(XMM_A::VAL_0X07)
    }
}
#[doc = "Field `XMBK` reader - External Memory Bus Keeper Enable"]
pub type XMBK_R = crate::BitReader;
#[doc = "Field `XMBK` writer - External Memory Bus Keeper Enable"]
pub type XMBK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    pub fn xmm(&self) -> XMM_R {
        XMM_R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    pub fn xmbk(&self) -> XMBK_R {
        XMBK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Memory High Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xmm(&mut self) -> XMM_W<XMCRB_SPEC> {
        XMM_W::new(self, 0)
    }
    #[doc = "Bit 7 - External Memory Bus Keeper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xmbk(&mut self) -> XMBK_W<XMCRB_SPEC> {
        XMBK_W::new(self, 7)
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
#[doc = "External Memory Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xmcrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xmcrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XMCRB_SPEC;
impl crate::RegisterSpec for XMCRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xmcrb::R`](R) reader structure"]
impl crate::Readable for XMCRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xmcrb::W`](W) writer structure"]
impl crate::Writable for XMCRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XMCRB to value 0"]
impl crate::Resettable for XMCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
