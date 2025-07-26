#[doc = "Register `UCSR2C` reader"]
pub type R = crate::R<UCSR2C_SPEC>;
#[doc = "Register `UCSR2C` writer"]
pub type W = crate::W<UCSR2C_SPEC>;
#[doc = "Field `UCPOL2` reader - Clock Polarity"]
pub type UCPOL2_R = crate::BitReader<UCPOL2_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPOL2_A {
    #[doc = "0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    RISING_EDGE = 0,
    #[doc = "1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    FALLING_EDGE = 1,
}
impl From<UCPOL2_A> for bool {
    #[inline(always)]
    fn from(variant: UCPOL2_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPOL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCPOL2_A {
        match self.bits {
            false => UCPOL2_A::RISING_EDGE,
            true => UCPOL2_A::FALLING_EDGE,
        }
    }
    #[doc = "Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == UCPOL2_A::RISING_EDGE
    }
    #[doc = "Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == UCPOL2_A::FALLING_EDGE
    }
}
#[doc = "Field `UCPOL2` writer - Clock Polarity"]
pub type UCPOL2_W<'a, REG> = crate::BitWriter<'a, REG, UCPOL2_A>;
impl<'a, REG> UCPOL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(UCPOL2_A::RISING_EDGE)
    }
    #[doc = "Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(UCPOL2_A::FALLING_EDGE)
    }
}
#[doc = "Field `UCSZ2` reader - Character Size"]
pub type UCSZ2_R = crate::FieldReader<UCSZ2_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSZ2_A {
    #[doc = "0: Character Size: 5 bit"]
    CHR5 = 0,
    #[doc = "1: Character Size: 6 bit"]
    CHR6 = 1,
    #[doc = "2: Character Size: 7 bit"]
    CHR7 = 2,
    #[doc = "3: Character Size: 8 bit"]
    CHR8 = 3,
}
impl From<UCSZ2_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSZ2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCSZ2_A {
    type Ux = u8;
}
impl UCSZ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCSZ2_A {
        match self.bits {
            0 => UCSZ2_A::CHR5,
            1 => UCSZ2_A::CHR6,
            2 => UCSZ2_A::CHR7,
            3 => UCSZ2_A::CHR8,
            _ => unreachable!(),
        }
    }
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn is_chr5(&self) -> bool {
        *self == UCSZ2_A::CHR5
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn is_chr6(&self) -> bool {
        *self == UCSZ2_A::CHR6
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn is_chr7(&self) -> bool {
        *self == UCSZ2_A::CHR7
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn is_chr8(&self) -> bool {
        *self == UCSZ2_A::CHR8
    }
}
#[doc = "Field `UCSZ2` writer - Character Size"]
pub type UCSZ2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCSZ2_A>;
impl<'a, REG> UCSZ2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn chr5(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ2_A::CHR5)
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn chr6(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ2_A::CHR6)
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn chr7(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ2_A::CHR7)
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn chr8(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ2_A::CHR8)
    }
}
#[doc = "Field `USBS2` reader - Stop Bit Select"]
pub type USBS2_R = crate::BitReader<USBS2_A>;
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBS2_A {
    #[doc = "0: 1-bit"]
    STOP1 = 0,
    #[doc = "1: 2-bit"]
    STOP2 = 1,
}
impl From<USBS2_A> for bool {
    #[inline(always)]
    fn from(variant: USBS2_A) -> Self {
        variant as u8 != 0
    }
}
impl USBS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBS2_A {
        match self.bits {
            false => USBS2_A::STOP1,
            true => USBS2_A::STOP2,
        }
    }
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == USBS2_A::STOP1
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == USBS2_A::STOP2
    }
}
#[doc = "Field `USBS2` writer - Stop Bit Select"]
pub type USBS2_W<'a, REG> = crate::BitWriter<'a, REG, USBS2_A>;
impl<'a, REG> USBS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(USBS2_A::STOP1)
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(USBS2_A::STOP2)
    }
}
#[doc = "Field `UPM2` reader - Parity Mode Bits"]
pub type UPM2_R = crate::FieldReader<UPM2_A>;
#[doc = "Parity Mode Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPM2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Enabled, Even Parity"]
    PARITY_EVEN = 2,
    #[doc = "3: Enabled, Odd Parity"]
    PARITY_ODD = 3,
}
impl From<UPM2_A> for u8 {
    #[inline(always)]
    fn from(variant: UPM2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPM2_A {
    type Ux = u8;
}
impl UPM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPM2_A> {
        match self.bits {
            0 => Some(UPM2_A::DISABLED),
            2 => Some(UPM2_A::PARITY_EVEN),
            3 => Some(UPM2_A::PARITY_ODD),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPM2_A::DISABLED
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn is_parity_even(&self) -> bool {
        *self == UPM2_A::PARITY_EVEN
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn is_parity_odd(&self) -> bool {
        *self == UPM2_A::PARITY_ODD
    }
}
#[doc = "Field `UPM2` writer - Parity Mode Bits"]
pub type UPM2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UPM2_A>;
impl<'a, REG> UPM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPM2_A::DISABLED)
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn parity_even(self) -> &'a mut crate::W<REG> {
        self.variant(UPM2_A::PARITY_EVEN)
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn parity_odd(self) -> &'a mut crate::W<REG> {
        self.variant(UPM2_A::PARITY_ODD)
    }
}
#[doc = "Field `UMSEL2` reader - USART Mode Select"]
pub type UMSEL2_R = crate::FieldReader<UMSEL2_A>;
#[doc = "USART Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UMSEL2_A {
    #[doc = "0: Asynchronous USART"]
    USART_ASYNC = 0,
    #[doc = "1: Synchronous USART"]
    USART_SYNC = 1,
    #[doc = "3: Master SPI (MSPIM)"]
    SPI_MASTER = 3,
}
impl From<UMSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: UMSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UMSEL2_A {
    type Ux = u8;
}
impl UMSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UMSEL2_A> {
        match self.bits {
            0 => Some(UMSEL2_A::USART_ASYNC),
            1 => Some(UMSEL2_A::USART_SYNC),
            3 => Some(UMSEL2_A::SPI_MASTER),
            _ => None,
        }
    }
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn is_usart_async(&self) -> bool {
        *self == UMSEL2_A::USART_ASYNC
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn is_usart_sync(&self) -> bool {
        *self == UMSEL2_A::USART_SYNC
    }
    #[doc = "Master SPI (MSPIM)"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == UMSEL2_A::SPI_MASTER
    }
}
#[doc = "Field `UMSEL2` writer - USART Mode Select"]
pub type UMSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UMSEL2_A>;
impl<'a, REG> UMSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn usart_async(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL2_A::USART_ASYNC)
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn usart_sync(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL2_A::USART_SYNC)
    }
    #[doc = "Master SPI (MSPIM)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL2_A::SPI_MASTER)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol2(&self) -> UCPOL2_R {
        UCPOL2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    pub fn ucsz2(&self) -> UCSZ2_R {
        UCSZ2_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    pub fn usbs2(&self) -> USBS2_R {
        USBS2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    pub fn upm2(&self) -> UPM2_R {
        UPM2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    pub fn umsel2(&self) -> UMSEL2_R {
        UMSEL2_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol2(&mut self) -> UCPOL2_W<UCSR2C_SPEC> {
        UCPOL2_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz2(&mut self) -> UCSZ2_W<UCSR2C_SPEC> {
        UCSZ2_W::new(self, 1)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbs2(&mut self) -> USBS2_W<UCSR2C_SPEC> {
        USBS2_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn upm2(&mut self) -> UPM2_W<UCSR2C_SPEC> {
        UPM2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn umsel2(&mut self) -> UMSEL2_W<UCSR2C_SPEC> {
        UMSEL2_W::new(self, 6)
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
#[doc = "USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR2C_SPEC;
impl crate::RegisterSpec for UCSR2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr2c::R`](R) reader structure"]
impl crate::Readable for UCSR2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr2c::W`](W) writer structure"]
impl crate::Writable for UCSR2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR2C to value 0"]
impl crate::Resettable for UCSR2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
