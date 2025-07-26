#[doc = "Register `TWIROUTEA` reader"]
pub type R = crate::R<TWIROUTEA_SPEC>;
#[doc = "Register `TWIROUTEA` writer"]
pub type W = crate::W<TWIROUTEA_SPEC>;
#[doc = "Field `TWI0` reader - TWI0 Signals"]
pub type TWI0_R = crate::FieldReader<TWI0_A>;
#[doc = "TWI0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWI0_A {
    #[doc = "0: SDA: PA2, SCL: PA3. Dual mode: SDA: PC2, SCL: PC3."]
    DEFAULT = 0,
    #[doc = "1: SDA: PA2, SCL: PA3. Dual mode: SDA: -, SCL: -."]
    ALT1 = 1,
    #[doc = "2: SDA: PC2, SCL: PC3. Dual mode: SDA: -, SCL: -."]
    ALT2 = 2,
    #[doc = "3: SDA: PA0, SCL: PA1. Dual mode: SDA: PC2, SCL: PC3."]
    ALT3 = 3,
}
impl From<TWI0_A> for u8 {
    #[inline(always)]
    fn from(variant: TWI0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TWI0_A {
    type Ux = u8;
}
impl TWI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TWI0_A {
        match self.bits {
            0 => TWI0_A::DEFAULT,
            1 => TWI0_A::ALT1,
            2 => TWI0_A::ALT2,
            3 => TWI0_A::ALT3,
            _ => unreachable!(),
        }
    }
    #[doc = "SDA: PA2, SCL: PA3. Dual mode: SDA: PC2, SCL: PC3."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TWI0_A::DEFAULT
    }
    #[doc = "SDA: PA2, SCL: PA3. Dual mode: SDA: -, SCL: -."]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == TWI0_A::ALT1
    }
    #[doc = "SDA: PC2, SCL: PC3. Dual mode: SDA: -, SCL: -."]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == TWI0_A::ALT2
    }
    #[doc = "SDA: PA0, SCL: PA1. Dual mode: SDA: PC2, SCL: PC3."]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == TWI0_A::ALT3
    }
}
#[doc = "Field `TWI0` writer - TWI0 Signals"]
pub type TWI0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TWI0_A>;
impl<'a, REG> TWI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDA: PA2, SCL: PA3. Dual mode: SDA: PC2, SCL: PC3."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::DEFAULT)
    }
    #[doc = "SDA: PA2, SCL: PA3. Dual mode: SDA: -, SCL: -."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::ALT1)
    }
    #[doc = "SDA: PC2, SCL: PC3. Dual mode: SDA: -, SCL: -."]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::ALT2)
    }
    #[doc = "SDA: PA0, SCL: PA1. Dual mode: SDA: PC2, SCL: PC3."]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::ALT3)
    }
}
impl R {
    #[doc = "Bits 0:1 - TWI0 Signals"]
    #[inline(always)]
    pub fn twi0(&self) -> TWI0_R {
        TWI0_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - TWI0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn twi0(&mut self) -> TWI0_W<TWIROUTEA_SPEC> {
        TWI0_W::new(self, 0)
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
#[doc = "TWI route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twiroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twiroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWIROUTEA_SPEC;
impl crate::RegisterSpec for TWIROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twiroutea::R`](R) reader structure"]
impl crate::Readable for TWIROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twiroutea::W`](W) writer structure"]
impl crate::Writable for TWIROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWIROUTEA to value 0"]
impl crate::Resettable for TWIROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
