#[doc = "Register `UCSR0C` reader"]
pub type R = crate::R<UCSR0C_SPEC>;
#[doc = "Register `UCSR0C` writer"]
pub type W = crate::W<UCSR0C_SPEC>;
#[doc = "Field `UCPOL0` reader - Clock Polarity"]
pub type UCPOL0_R = crate::BitReader<UCPOL0_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPOL0_A {
    #[doc = "0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    RISING_EDGE = 0,
    #[doc = "1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    FALLING_EDGE = 1,
}
impl From<UCPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: UCPOL0_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPOL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCPOL0_A {
        match self.bits {
            false => UCPOL0_A::RISING_EDGE,
            true => UCPOL0_A::FALLING_EDGE,
        }
    }
    #[doc = "Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == UCPOL0_A::RISING_EDGE
    }
    #[doc = "Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == UCPOL0_A::FALLING_EDGE
    }
}
#[doc = "Field `UCPOL0` writer - Clock Polarity"]
pub type UCPOL0_W<'a, REG> = crate::BitWriter<'a, REG, UCPOL0_A>;
impl<'a, REG> UCPOL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(UCPOL0_A::RISING_EDGE)
    }
    #[doc = "Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(UCPOL0_A::FALLING_EDGE)
    }
}
#[doc = "Field `UCSZ0` reader - Character Size"]
pub type UCSZ0_R = crate::FieldReader<UCSZ0_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSZ0_A {
    #[doc = "0: Character Size: 5 bit"]
    CHR5 = 0,
    #[doc = "1: Character Size: 6 bit"]
    CHR6 = 1,
    #[doc = "2: Character Size: 7 bit"]
    CHR7 = 2,
    #[doc = "3: Character Size: 8 bit"]
    CHR8 = 3,
}
impl From<UCSZ0_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSZ0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCSZ0_A {
    type Ux = u8;
}
impl UCSZ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCSZ0_A {
        match self.bits {
            0 => UCSZ0_A::CHR5,
            1 => UCSZ0_A::CHR6,
            2 => UCSZ0_A::CHR7,
            3 => UCSZ0_A::CHR8,
            _ => unreachable!(),
        }
    }
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn is_chr5(&self) -> bool {
        *self == UCSZ0_A::CHR5
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn is_chr6(&self) -> bool {
        *self == UCSZ0_A::CHR6
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn is_chr7(&self) -> bool {
        *self == UCSZ0_A::CHR7
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn is_chr8(&self) -> bool {
        *self == UCSZ0_A::CHR8
    }
}
#[doc = "Field `UCSZ0` writer - Character Size"]
pub type UCSZ0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCSZ0_A>;
impl<'a, REG> UCSZ0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn chr5(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ0_A::CHR5)
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn chr6(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ0_A::CHR6)
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn chr7(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ0_A::CHR7)
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn chr8(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ0_A::CHR8)
    }
}
#[doc = "Field `USBS0` reader - Stop Bit Select"]
pub type USBS0_R = crate::BitReader<USBS0_A>;
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBS0_A {
    #[doc = "0: 1-bit"]
    STOP1 = 0,
    #[doc = "1: 2-bit"]
    STOP2 = 1,
}
impl From<USBS0_A> for bool {
    #[inline(always)]
    fn from(variant: USBS0_A) -> Self {
        variant as u8 != 0
    }
}
impl USBS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBS0_A {
        match self.bits {
            false => USBS0_A::STOP1,
            true => USBS0_A::STOP2,
        }
    }
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == USBS0_A::STOP1
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == USBS0_A::STOP2
    }
}
#[doc = "Field `USBS0` writer - Stop Bit Select"]
pub type USBS0_W<'a, REG> = crate::BitWriter<'a, REG, USBS0_A>;
impl<'a, REG> USBS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(USBS0_A::STOP1)
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(USBS0_A::STOP2)
    }
}
#[doc = "Field `UPM0` reader - Parity Mode Bits"]
pub type UPM0_R = crate::FieldReader<UPM0_A>;
#[doc = "Parity Mode Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPM0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Enabled, Even Parity"]
    PARITY_EVEN = 2,
    #[doc = "3: Enabled, Odd Parity"]
    PARITY_ODD = 3,
}
impl From<UPM0_A> for u8 {
    #[inline(always)]
    fn from(variant: UPM0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPM0_A {
    type Ux = u8;
}
impl UPM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPM0_A> {
        match self.bits {
            0 => Some(UPM0_A::DISABLED),
            2 => Some(UPM0_A::PARITY_EVEN),
            3 => Some(UPM0_A::PARITY_ODD),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPM0_A::DISABLED
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn is_parity_even(&self) -> bool {
        *self == UPM0_A::PARITY_EVEN
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn is_parity_odd(&self) -> bool {
        *self == UPM0_A::PARITY_ODD
    }
}
#[doc = "Field `UPM0` writer - Parity Mode Bits"]
pub type UPM0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UPM0_A>;
impl<'a, REG> UPM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPM0_A::DISABLED)
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn parity_even(self) -> &'a mut crate::W<REG> {
        self.variant(UPM0_A::PARITY_EVEN)
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn parity_odd(self) -> &'a mut crate::W<REG> {
        self.variant(UPM0_A::PARITY_ODD)
    }
}
#[doc = "Field `UMSEL0` reader - USART Mode Select"]
pub type UMSEL0_R = crate::FieldReader<UMSEL0_A>;
#[doc = "USART Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UMSEL0_A {
    #[doc = "0: Asynchronous USART"]
    USART_ASYNC = 0,
    #[doc = "1: Synchronous USART"]
    USART_SYNC = 1,
    #[doc = "3: Master SPI (MSPIM)"]
    SPI_MASTER = 3,
}
impl From<UMSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: UMSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UMSEL0_A {
    type Ux = u8;
}
impl UMSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UMSEL0_A> {
        match self.bits {
            0 => Some(UMSEL0_A::USART_ASYNC),
            1 => Some(UMSEL0_A::USART_SYNC),
            3 => Some(UMSEL0_A::SPI_MASTER),
            _ => None,
        }
    }
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn is_usart_async(&self) -> bool {
        *self == UMSEL0_A::USART_ASYNC
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn is_usart_sync(&self) -> bool {
        *self == UMSEL0_A::USART_SYNC
    }
    #[doc = "Master SPI (MSPIM)"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == UMSEL0_A::SPI_MASTER
    }
}
#[doc = "Field `UMSEL0` writer - USART Mode Select"]
pub type UMSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UMSEL0_A>;
impl<'a, REG> UMSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn usart_async(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL0_A::USART_ASYNC)
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn usart_sync(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL0_A::USART_SYNC)
    }
    #[doc = "Master SPI (MSPIM)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL0_A::SPI_MASTER)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol0(&self) -> UCPOL0_R {
        UCPOL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    pub fn ucsz0(&self) -> UCSZ0_R {
        UCSZ0_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    pub fn usbs0(&self) -> USBS0_R {
        USBS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    pub fn upm0(&self) -> UPM0_R {
        UPM0_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    pub fn umsel0(&self) -> UMSEL0_R {
        UMSEL0_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol0(&mut self) -> UCPOL0_W<UCSR0C_SPEC> {
        UCPOL0_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz0(&mut self) -> UCSZ0_W<UCSR0C_SPEC> {
        UCSZ0_W::new(self, 1)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbs0(&mut self) -> USBS0_W<UCSR0C_SPEC> {
        USBS0_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn upm0(&mut self) -> UPM0_W<UCSR0C_SPEC> {
        UPM0_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn umsel0(&mut self) -> UMSEL0_W<UCSR0C_SPEC> {
        UMSEL0_W::new(self, 6)
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
#[doc = "USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr0c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr0c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR0C_SPEC;
impl crate::RegisterSpec for UCSR0C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr0c::R`](R) reader structure"]
impl crate::Readable for UCSR0C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr0c::W`](W) writer structure"]
impl crate::Writable for UCSR0C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR0C to value 0"]
impl crate::Resettable for UCSR0C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
