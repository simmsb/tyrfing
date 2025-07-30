#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable Module"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable Module"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESC` reader - Prescaler"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: CLK_PER / 4"]
    DIV4 = 0,
    #[doc = "1: CLK_PER / 16"]
    DIV16 = 1,
    #[doc = "2: CLK_PER / 64"]
    DIV64 = 2,
    #[doc = "3: CLK_PER / 128"]
    DIV128 = 3,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::DIV4,
            1 => PRESC_A::DIV16,
            2 => PRESC_A::DIV64,
            3 => PRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_PER / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "CLK_PER / 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "CLK_PER / 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "CLK_PER / 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
}
#[doc = "Field `PRESC` writer - Prescaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PRESC_A>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_PER / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "CLK_PER / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "CLK_PER / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "CLK_PER / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::DIV128)
    }
}
#[doc = "Field `CLK2X` reader - Enable Double Speed"]
pub type CLK2X_R = crate::BitReader;
#[doc = "Field `CLK2X` writer - Enable Double Speed"]
pub type CLK2X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER` reader - Host Operation Enable"]
pub type MASTER_R = crate::BitReader;
#[doc = "Field `MASTER` writer - Host Operation Enable"]
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORD` reader - Data Order Setting"]
pub type DORD_R = crate::BitReader<DORD_A>;
#[doc = "Data Order Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DORD_A {
    #[doc = "0: Most significant byte first"]
    MSB_FIRST = 0,
    #[doc = "1: Least significant byte first"]
    LSB_FIRST = 1,
}
impl From<DORD_A> for bool {
    #[inline(always)]
    fn from(variant: DORD_A) -> Self {
        variant as u8 != 0
    }
}
impl DORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DORD_A {
        match self.bits {
            false => DORD_A::MSB_FIRST,
            true => DORD_A::LSB_FIRST,
        }
    }
    #[doc = "Most significant byte first"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == DORD_A::MSB_FIRST
    }
    #[doc = "Least significant byte first"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == DORD_A::LSB_FIRST
    }
}
#[doc = "Field `DORD` writer - Data Order Setting"]
pub type DORD_W<'a, REG> = crate::BitWriter<'a, REG, DORD_A>;
impl<'a, REG> DORD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Most significant byte first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(DORD_A::MSB_FIRST)
    }
    #[doc = "Least significant byte first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(DORD_A::LSB_FIRST)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Module"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 4 - Enable Double Speed"]
    #[inline(always)]
    pub fn clk2x(&self) -> CLK2X_R {
        CLK2X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host Operation Enable"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Order Setting"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Module"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CTRLA_SPEC> {
        PRESC_W::new(self, 1)
    }
    #[doc = "Bit 4 - Enable Double Speed"]
    #[inline(always)]
    #[must_use]
    pub fn clk2x(&mut self) -> CLK2X_W<CTRLA_SPEC> {
        CLK2X_W::new(self, 4)
    }
    #[doc = "Bit 5 - Host Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<CTRLA_SPEC> {
        MASTER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Data Order Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<CTRLA_SPEC> {
        DORD_W::new(self, 6)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
