#[doc = "Register `TCCR0` reader"]
pub type R = crate::R<TCCR0_SPEC>;
#[doc = "Register `TCCR0` writer"]
pub type W = crate::W<TCCR0_SPEC>;
#[doc = "Field `CS0` reader - Clock Selects"]
pub type CS0_R = crate::FieldReader<CS0_A>;
#[doc = "Clock Selects\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CS0_A {
    #[doc = "0: No Clock Source (Stopped)"]
    VAL_0X00 = 0,
    #[doc = "1: Running, No Prescaling"]
    VAL_0X01 = 1,
    #[doc = "2: Running, CLK/8"]
    VAL_0X02 = 2,
    #[doc = "3: Running, CLK/32"]
    VAL_0X03 = 3,
    #[doc = "4: Running, CLK/64"]
    VAL_0X04 = 4,
    #[doc = "5: Running, CLK/128"]
    VAL_0X05 = 5,
    #[doc = "6: Running, CLK/256"]
    VAL_0X06 = 6,
    #[doc = "7: Running, CLK/1024"]
    VAL_0X07 = 7,
}
impl From<CS0_A> for u8 {
    #[inline(always)]
    fn from(variant: CS0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CS0_A {
    type Ux = u8;
}
impl CS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS0_A {
        match self.bits {
            0 => CS0_A::VAL_0X00,
            1 => CS0_A::VAL_0X01,
            2 => CS0_A::VAL_0X02,
            3 => CS0_A::VAL_0X03,
            4 => CS0_A::VAL_0X04,
            5 => CS0_A::VAL_0X05,
            6 => CS0_A::VAL_0X06,
            7 => CS0_A::VAL_0X07,
            _ => unreachable!(),
        }
    }
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == CS0_A::VAL_0X00
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == CS0_A::VAL_0X01
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == CS0_A::VAL_0X02
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == CS0_A::VAL_0X03
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn is_val_0x04(&self) -> bool {
        *self == CS0_A::VAL_0X04
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == CS0_A::VAL_0X05
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn is_val_0x06(&self) -> bool {
        *self == CS0_A::VAL_0X06
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn is_val_0x07(&self) -> bool {
        *self == CS0_A::VAL_0X07
    }
}
#[doc = "Field `CS0` writer - Clock Selects"]
pub type CS0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CS0_A>;
impl<'a, REG> CS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Clock Source (Stopped)"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X00)
    }
    #[doc = "Running, No Prescaling"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X01)
    }
    #[doc = "Running, CLK/8"]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X02)
    }
    #[doc = "Running, CLK/32"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X03)
    }
    #[doc = "Running, CLK/64"]
    #[inline(always)]
    pub fn val_0x04(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X04)
    }
    #[doc = "Running, CLK/128"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X05)
    }
    #[doc = "Running, CLK/256"]
    #[inline(always)]
    pub fn val_0x06(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X06)
    }
    #[doc = "Running, CLK/1024"]
    #[inline(always)]
    pub fn val_0x07(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_A::VAL_0X07)
    }
}
#[doc = "Field `WGM01` reader - Waveform Generation Mode 1"]
pub type WGM01_R = crate::BitReader;
#[doc = "Field `WGM01` writer - Waveform Generation Mode 1"]
pub type WGM01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COM0` reader - Compare Match Output Modes"]
pub type COM0_R = crate::FieldReader;
#[doc = "Field `COM0` writer - Compare Match Output Modes"]
pub type COM0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `WGM00` reader - Waveform Generation Mode 0"]
pub type WGM00_R = crate::BitReader<WGM00_A>;
#[doc = "Waveform Generation Mode 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WGM00_A {
    #[doc = "0: Normal"]
    VAL_0X00 = 0,
    #[doc = "1: CTC"]
    VAL_0X01 = 1,
}
impl From<WGM00_A> for bool {
    #[inline(always)]
    fn from(variant: WGM00_A) -> Self {
        variant as u8 != 0
    }
}
impl WGM00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM00_A {
        match self.bits {
            false => WGM00_A::VAL_0X00,
            true => WGM00_A::VAL_0X01,
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_val_0x00(&self) -> bool {
        *self == WGM00_A::VAL_0X00
    }
    #[doc = "CTC"]
    #[inline(always)]
    pub fn is_val_0x01(&self) -> bool {
        *self == WGM00_A::VAL_0X01
    }
}
#[doc = "Field `WGM00` writer - Waveform Generation Mode 0"]
pub type WGM00_W<'a, REG> = crate::BitWriter<'a, REG, WGM00_A>;
impl<'a, REG> WGM00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal"]
    #[inline(always)]
    pub fn val_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(WGM00_A::VAL_0X00)
    }
    #[doc = "CTC"]
    #[inline(always)]
    pub fn val_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(WGM00_A::VAL_0X01)
    }
}
#[doc = "Field `FOC0` reader - Force Output Compare"]
pub type FOC0_R = crate::BitReader;
#[doc = "Field `FOC0` writer - Force Output Compare"]
pub type FOC0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Clock Selects"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Waveform Generation Mode 1"]
    #[inline(always)]
    pub fn wgm01(&self) -> WGM01_R {
        WGM01_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Compare Match Output Modes"]
    #[inline(always)]
    pub fn com0(&self) -> COM0_R {
        COM0_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Waveform Generation Mode 0"]
    #[inline(always)]
    pub fn wgm00(&self) -> WGM00_R {
        WGM00_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare"]
    #[inline(always)]
    pub fn foc0(&self) -> FOC0_R {
        FOC0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selects"]
    #[inline(always)]
    #[must_use]
    pub fn cs0(&mut self) -> CS0_W<TCCR0_SPEC> {
        CS0_W::new(self, 0)
    }
    #[doc = "Bit 3 - Waveform Generation Mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn wgm01(&mut self) -> WGM01_W<TCCR0_SPEC> {
        WGM01_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Compare Match Output Modes"]
    #[inline(always)]
    #[must_use]
    pub fn com0(&mut self) -> COM0_W<TCCR0_SPEC> {
        COM0_W::new(self, 4)
    }
    #[doc = "Bit 6 - Waveform Generation Mode 0"]
    #[inline(always)]
    #[must_use]
    pub fn wgm00(&mut self) -> WGM00_W<TCCR0_SPEC> {
        WGM00_W::new(self, 6)
    }
    #[doc = "Bit 7 - Force Output Compare"]
    #[inline(always)]
    #[must_use]
    pub fn foc0(&mut self) -> FOC0_W<TCCR0_SPEC> {
        FOC0_W::new(self, 7)
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
#[doc = "Timer/Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR0_SPEC;
impl crate::RegisterSpec for TCCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr0::R`](R) reader structure"]
impl crate::Readable for TCCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr0::W`](W) writer structure"]
impl crate::Writable for TCCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0 to value 0"]
impl crate::Resettable for TCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
