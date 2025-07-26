#[doc = "Register `TWISPIROUTEA` reader"]
pub type R = crate::R<TWISPIROUTEA_SPEC>;
#[doc = "Register `TWISPIROUTEA` writer"]
pub type W = crate::W<TWISPIROUTEA_SPEC>;
#[doc = "Field `SPI0` reader - Port Multiplexer SPI0"]
pub type SPI0_R = crate::FieldReader<SPI0_A>;
#[doc = "Port Multiplexer SPI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI0_A {
    #[doc = "0: SPI0 on PA\\[7:4\\]"]
    DEFAULT = 0,
    #[doc = "1: SPI0 on PC\\[3:0\\]"]
    ALT1 = 1,
    #[doc = "2: SPI0 on PE\\[3:0\\]"]
    ALT2 = 2,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
}
impl From<SPI0_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI0_A {
    type Ux = u8;
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI0_A {
        match self.bits {
            0 => SPI0_A::DEFAULT,
            1 => SPI0_A::ALT1,
            2 => SPI0_A::ALT2,
            3 => SPI0_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "SPI0 on PA\\[7:4\\]"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == SPI0_A::DEFAULT
    }
    #[doc = "SPI0 on PC\\[3:0\\]"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == SPI0_A::ALT1
    }
    #[doc = "SPI0 on PE\\[3:0\\]"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == SPI0_A::ALT2
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SPI0_A::NONE
    }
}
#[doc = "Field `SPI0` writer - Port Multiplexer SPI0"]
pub type SPI0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPI0_A>;
impl<'a, REG> SPI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI0 on PA\\[7:4\\]"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::DEFAULT)
    }
    #[doc = "SPI0 on PC\\[3:0\\]"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::ALT1)
    }
    #[doc = "SPI0 on PE\\[3:0\\]"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::ALT2)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SPI0_A::NONE)
    }
}
#[doc = "Field `TWI0` reader - Port Multiplexer TWI0"]
pub type TWI0_R = crate::FieldReader<TWI0_A>;
#[doc = "Port Multiplexer TWI0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWI0_A {
    #[doc = "0: SCL/SDA on PA\\[3:2\\], Slave mode on PC\\[3:2\\]
in dual TWI mode"]
    DEFAULT = 0,
    #[doc = "1: SCL/SDA on PA\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    ALT1 = 1,
    #[doc = "2: SCL/SDA on PC\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    ALT2 = 2,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
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
            3 => TWI0_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "SCL/SDA on PA\\[3:2\\], Slave mode on PC\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TWI0_A::DEFAULT
    }
    #[doc = "SCL/SDA on PA\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == TWI0_A::ALT1
    }
    #[doc = "SCL/SDA on PC\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == TWI0_A::ALT2
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TWI0_A::NONE
    }
}
#[doc = "Field `TWI0` writer - Port Multiplexer TWI0"]
pub type TWI0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TWI0_A>;
impl<'a, REG> TWI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCL/SDA on PA\\[3:2\\], Slave mode on PC\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::DEFAULT)
    }
    #[doc = "SCL/SDA on PA\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::ALT1)
    }
    #[doc = "SCL/SDA on PC\\[3:2\\], Slave mode on PF\\[3:2\\]
in dual TWI mode"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::ALT2)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(TWI0_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port Multiplexer SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Port Multiplexer TWI0"]
    #[inline(always)]
    pub fn twi0(&self) -> TWI0_R {
        TWI0_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port Multiplexer SPI0"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<TWISPIROUTEA_SPEC> {
        SPI0_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Port Multiplexer TWI0"]
    #[inline(always)]
    #[must_use]
    pub fn twi0(&mut self) -> TWI0_W<TWISPIROUTEA_SPEC> {
        TWI0_W::new(self, 4)
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
#[doc = "Port Multiplexer TWI and SPI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twispiroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twispiroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWISPIROUTEA_SPEC;
impl crate::RegisterSpec for TWISPIROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twispiroutea::R`](R) reader structure"]
impl crate::Readable for TWISPIROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twispiroutea::W`](W) writer structure"]
impl crate::Writable for TWISPIROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWISPIROUTEA to value 0"]
impl crate::Resettable for TWISPIROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
