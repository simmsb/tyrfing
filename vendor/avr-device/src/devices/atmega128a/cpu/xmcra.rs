#[doc = "Register `XMCRA` reader"]
pub type R = crate::R<XMCRA_SPEC>;
#[doc = "Register `XMCRA` writer"]
pub type W = crate::W<XMCRA_SPEC>;
#[doc = "Field `SRW11` reader - Wait state select bit upper page"]
pub type SRW11_R = crate::BitReader;
#[doc = "Field `SRW11` writer - Wait state select bit upper page"]
pub type SRW11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRW0` reader - Wait state select bit lower page"]
pub type SRW0_R = crate::FieldReader<SRW0_A>;
#[doc = "Wait state select bit lower page\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRW0_A {
    #[doc = "0: No wait-states"]
    VAL_0X00 = 0,
    #[doc = "1: Wait one cycle during read/write strobe"]
    VAL_0X01 = 1,
    #[doc = "2: Wait two cycles during read/write strobe"]
    VAL_0X02 = 2,
    #[doc = "3: Wait two cycles during read/write and wait one cycle before driving out new address"]
    VAL_0X03 = 3,
}
impl From<SRW0_A> for u8 {
    #[inline(always)]
    fn from(variant: SRW0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRW0_A {
    type Ux = u8;
}
impl SRW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRW0_A {
        match self.bits {
            0 => SRW0_A::VAL_0X00,
            1 => SRW0_A::VAL_0X01,
            2 => SRW0_A::VAL_0X02,
            3 => SRW0_A::VAL_0X03,
            _ => unreachable!(),
        }
    }
    #[doc = "No wait-states"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == SRW0_A::VAL_0X00
    }
    #[doc = "Wait one cycle during read/write strobe"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == SRW0_A::VAL_0X01
    }
    #[doc = "Wait two cycles during read/write strobe"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == SRW0_A::VAL_0X02
    }
    #[doc = "Wait two cycles during read/write and wait one cycle before driving out new address"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == SRW0_A::VAL_0X03
    }
}
#[doc = "Field `SRW0` writer - Wait state select bit lower page"]
pub type SRW0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SRW0_A>;
impl<'a, REG> SRW0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No wait-states"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(SRW0_A::VAL_0X00)
    }
    #[doc = "Wait one cycle during read/write strobe"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(SRW0_A::VAL_0X01)
    }
    #[doc = "Wait two cycles during read/write strobe"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(SRW0_A::VAL_0X02)
    }
    #[doc = "Wait two cycles during read/write and wait one cycle before driving out new address"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(SRW0_A::VAL_0X03)
    }
}
#[doc = "Field `SRL` reader - Wait state page limit"]
pub type SRL_R = crate::FieldReader<SRL_A>;
#[doc = "Wait state page limit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRL_A {
    #[doc = "0: LS = N/A, US = 0x1100 - 0xFFFF"]
    VAL_0X00 = 0,
    #[doc = "1: LS = 0x1100 - 0x1FFF, US = 0x2000 - 0xFFFF"]
    VAL_0X01 = 1,
    #[doc = "2: LS = 0x1100 - 0x3FFF, US = 0x4000 - 0xFFFF"]
    VAL_0X02 = 2,
    #[doc = "3: LS = 0x1100 - 0x5FFF, US = 0x6000 - 0xFFFF"]
    VAL_0X03 = 3,
    #[doc = "4: LS = 0x1100 - 0x7FFF, US = 0x8000 - 0xFFFF"]
    VAL_0X04 = 4,
    #[doc = "5: LS = 0x1100 - 0x9FFF, US = 0xA000 - 0xFFFF"]
    VAL_0X05 = 5,
    #[doc = "6: LS = 0x1100 - 0xBFFF, US = 0xC000 - 0xFFFF"]
    VAL_0X06 = 6,
    #[doc = "7: LS = 0x1100 - 0xDFFF, US = 0xE000 - 0xFFFF"]
    VAL_0X07 = 7,
}
impl From<SRL_A> for u8 {
    #[inline(always)]
    fn from(variant: SRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRL_A {
    type Ux = u8;
}
impl SRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRL_A {
        match self.bits {
            0 => SRL_A::VAL_0X00,
            1 => SRL_A::VAL_0X01,
            2 => SRL_A::VAL_0X02,
            3 => SRL_A::VAL_0X03,
            4 => SRL_A::VAL_0X04,
            5 => SRL_A::VAL_0X05,
            6 => SRL_A::VAL_0X06,
            7 => SRL_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "LS = N/A, US = 0x1100 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == SRL_A::VAL_0X00
    }
    #[doc = "LS = 0x1100 - 0x1FFF, US = 0x2000 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == SRL_A::VAL_0X01
    }
    #[doc = "LS = 0x1100 - 0x3FFF, US = 0x4000 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == SRL_A::VAL_0X02
    }
    #[doc = "LS = 0x1100 - 0x5FFF, US = 0x6000 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == SRL_A::VAL_0X03
    }
    #[doc = "LS = 0x1100 - 0x7FFF, US = 0x8000 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == SRL_A::VAL_0X04
    }
    #[doc = "LS = 0x1100 - 0x9FFF, US = 0xA000 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == SRL_A::VAL_0X05
    }
    #[doc = "LS = 0x1100 - 0xBFFF, US = 0xC000 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == SRL_A::VAL_0X06
    }
    #[doc = "LS = 0x1100 - 0xDFFF, US = 0xE000 - 0xFFFF"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == SRL_A::VAL_0X07
    }
}
#[doc = "Field `SRL` writer - Wait state page limit"]
pub type SRL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SRL_A>;
impl<'a, REG> SRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LS = N/A, US = 0x1100 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X00)
    }
    #[doc = "LS = 0x1100 - 0x1FFF, US = 0x2000 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X01)
    }
    #[doc = "LS = 0x1100 - 0x3FFF, US = 0x4000 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X02)
    }
    #[doc = "LS = 0x1100 - 0x5FFF, US = 0x6000 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X03)
    }
    #[doc = "LS = 0x1100 - 0x7FFF, US = 0x8000 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X04)
    }
    #[doc = "LS = 0x1100 - 0x9FFF, US = 0xA000 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X05)
    }
    #[doc = "LS = 0x1100 - 0xBFFF, US = 0xC000 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X06)
    }
    #[doc = "LS = 0x1100 - 0xDFFF, US = 0xE000 - 0xFFFF"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut crate::W<REG> {
        self.variant(SRL_A::VAL_0X07)
    }
}
impl R {
    #[doc = "Bit 1 - Wait state select bit upper page"]
    #[inline(always)]
    pub fn srw11(&self) -> SRW11_R {
        SRW11_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Wait state select bit lower page"]
    #[inline(always)]
    pub fn srw0(&self) -> SRW0_R {
        SRW0_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:6 - Wait state page limit"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bit 1 - Wait state select bit upper page"]
    #[inline(always)]
    #[must_use]
    pub fn srw11(&mut self) -> SRW11_W<XMCRA_SPEC> {
        SRW11_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Wait state select bit lower page"]
    #[inline(always)]
    #[must_use]
    pub fn srw0(&mut self) -> SRW0_W<XMCRA_SPEC> {
        SRW0_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Wait state page limit"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SRL_W<XMCRA_SPEC> {
        SRL_W::new(self, 4)
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
#[doc = "External Memory Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xmcra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xmcra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XMCRA_SPEC;
impl crate::RegisterSpec for XMCRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xmcra::R`](R) reader structure"]
impl crate::Readable for XMCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xmcra::W`](W) writer structure"]
impl crate::Writable for XMCRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XMCRA to value 0"]
impl crate::Resettable for XMCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
