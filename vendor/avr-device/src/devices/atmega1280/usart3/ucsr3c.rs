#[doc = "Register `UCSR3C` reader"]
pub type R = crate::R<UCSR3C_SPEC>;
#[doc = "Register `UCSR3C` writer"]
pub type W = crate::W<UCSR3C_SPEC>;
#[doc = "Field `UCPOL3` reader - Clock Polarity"]
pub type UCPOL3_R = crate::BitReader<UCPOL3_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPOL3_A {
    #[doc = "0: Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    RISING_EDGE = 0,
    #[doc = "1: Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    FALLING_EDGE = 1,
}
impl From<UCPOL3_A> for bool {
    #[inline(always)]
    fn from(variant: UCPOL3_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPOL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCPOL3_A {
        match self.bits {
            false => UCPOL3_A::RISING_EDGE,
            true => UCPOL3_A::FALLING_EDGE,
        }
    }
    #[doc = "Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == UCPOL3_A::RISING_EDGE
    }
    #[doc = "Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == UCPOL3_A::FALLING_EDGE
    }
}
#[doc = "Field `UCPOL3` writer - Clock Polarity"]
pub type UCPOL3_W<'a, REG> = crate::BitWriter<'a, REG, UCPOL3_A>;
impl<'a, REG> UCPOL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit on Rising XCKn Edge, Receive on Falling XCKn Edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(UCPOL3_A::RISING_EDGE)
    }
    #[doc = "Transmit on Falling XCKn Edge, Receive on Rising XCKn Edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(UCPOL3_A::FALLING_EDGE)
    }
}
#[doc = "Field `UCSZ3` reader - Character Size"]
pub type UCSZ3_R = crate::FieldReader<UCSZ3_A>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSZ3_A {
    #[doc = "0: Character Size: 5 bit"]
    CHR5 = 0,
    #[doc = "1: Character Size: 6 bit"]
    CHR6 = 1,
    #[doc = "2: Character Size: 7 bit"]
    CHR7 = 2,
    #[doc = "3: Character Size: 8 bit"]
    CHR8 = 3,
}
impl From<UCSZ3_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSZ3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCSZ3_A {
    type Ux = u8;
}
impl UCSZ3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCSZ3_A {
        match self.bits {
            0 => UCSZ3_A::CHR5,
            1 => UCSZ3_A::CHR6,
            2 => UCSZ3_A::CHR7,
            3 => UCSZ3_A::CHR8,
            _ => unreachable!(),
        }
    }
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn is_chr5(&self) -> bool {
        *self == UCSZ3_A::CHR5
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn is_chr6(&self) -> bool {
        *self == UCSZ3_A::CHR6
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn is_chr7(&self) -> bool {
        *self == UCSZ3_A::CHR7
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn is_chr8(&self) -> bool {
        *self == UCSZ3_A::CHR8
    }
}
#[doc = "Field `UCSZ3` writer - Character Size"]
pub type UCSZ3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCSZ3_A>;
impl<'a, REG> UCSZ3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Character Size: 5 bit"]
    #[inline(always)]
    pub fn chr5(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ3_A::CHR5)
    }
    #[doc = "Character Size: 6 bit"]
    #[inline(always)]
    pub fn chr6(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ3_A::CHR6)
    }
    #[doc = "Character Size: 7 bit"]
    #[inline(always)]
    pub fn chr7(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ3_A::CHR7)
    }
    #[doc = "Character Size: 8 bit"]
    #[inline(always)]
    pub fn chr8(self) -> &'a mut crate::W<REG> {
        self.variant(UCSZ3_A::CHR8)
    }
}
#[doc = "Field `USBS3` reader - Stop Bit Select"]
pub type USBS3_R = crate::BitReader<USBS3_A>;
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBS3_A {
    #[doc = "0: 1-bit"]
    STOP1 = 0,
    #[doc = "1: 2-bit"]
    STOP2 = 1,
}
impl From<USBS3_A> for bool {
    #[inline(always)]
    fn from(variant: USBS3_A) -> Self {
        variant as u8 != 0
    }
}
impl USBS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBS3_A {
        match self.bits {
            false => USBS3_A::STOP1,
            true => USBS3_A::STOP2,
        }
    }
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == USBS3_A::STOP1
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == USBS3_A::STOP2
    }
}
#[doc = "Field `USBS3` writer - Stop Bit Select"]
pub type USBS3_W<'a, REG> = crate::BitWriter<'a, REG, USBS3_A>;
impl<'a, REG> USBS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1-bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(USBS3_A::STOP1)
    }
    #[doc = "2-bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(USBS3_A::STOP2)
    }
}
#[doc = "Field `UPM3` reader - Parity Mode Bits"]
pub type UPM3_R = crate::FieldReader<UPM3_A>;
#[doc = "Parity Mode Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPM3_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Enabled, Even Parity"]
    PARITY_EVEN = 2,
    #[doc = "3: Enabled, Odd Parity"]
    PARITY_ODD = 3,
}
impl From<UPM3_A> for u8 {
    #[inline(always)]
    fn from(variant: UPM3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPM3_A {
    type Ux = u8;
}
impl UPM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPM3_A> {
        match self.bits {
            0 => Some(UPM3_A::DISABLED),
            2 => Some(UPM3_A::PARITY_EVEN),
            3 => Some(UPM3_A::PARITY_ODD),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPM3_A::DISABLED
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn is_parity_even(&self) -> bool {
        *self == UPM3_A::PARITY_EVEN
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn is_parity_odd(&self) -> bool {
        *self == UPM3_A::PARITY_ODD
    }
}
#[doc = "Field `UPM3` writer - Parity Mode Bits"]
pub type UPM3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UPM3_A>;
impl<'a, REG> UPM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPM3_A::DISABLED)
    }
    #[doc = "Enabled, Even Parity"]
    #[inline(always)]
    pub fn parity_even(self) -> &'a mut crate::W<REG> {
        self.variant(UPM3_A::PARITY_EVEN)
    }
    #[doc = "Enabled, Odd Parity"]
    #[inline(always)]
    pub fn parity_odd(self) -> &'a mut crate::W<REG> {
        self.variant(UPM3_A::PARITY_ODD)
    }
}
#[doc = "Field `UMSEL3` reader - USART Mode Select"]
pub type UMSEL3_R = crate::FieldReader<UMSEL3_A>;
#[doc = "USART Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UMSEL3_A {
    #[doc = "0: Asynchronous USART"]
    USART_ASYNC = 0,
    #[doc = "1: Synchronous USART"]
    USART_SYNC = 1,
    #[doc = "3: Master SPI (MSPIM)"]
    SPI_MASTER = 3,
}
impl From<UMSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: UMSEL3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UMSEL3_A {
    type Ux = u8;
}
impl UMSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UMSEL3_A> {
        match self.bits {
            0 => Some(UMSEL3_A::USART_ASYNC),
            1 => Some(UMSEL3_A::USART_SYNC),
            3 => Some(UMSEL3_A::SPI_MASTER),
            _ => None,
        }
    }
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn is_usart_async(&self) -> bool {
        *self == UMSEL3_A::USART_ASYNC
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn is_usart_sync(&self) -> bool {
        *self == UMSEL3_A::USART_SYNC
    }
    #[doc = "Master SPI (MSPIM)"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == UMSEL3_A::SPI_MASTER
    }
}
#[doc = "Field `UMSEL3` writer - USART Mode Select"]
pub type UMSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, UMSEL3_A>;
impl<'a, REG> UMSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Asynchronous USART"]
    #[inline(always)]
    pub fn usart_async(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL3_A::USART_ASYNC)
    }
    #[doc = "Synchronous USART"]
    #[inline(always)]
    pub fn usart_sync(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL3_A::USART_SYNC)
    }
    #[doc = "Master SPI (MSPIM)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(UMSEL3_A::SPI_MASTER)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn ucpol3(&self) -> UCPOL3_R {
        UCPOL3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    pub fn ucsz3(&self) -> UCSZ3_R {
        UCSZ3_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    pub fn usbs3(&self) -> USBS3_R {
        USBS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    pub fn upm3(&self) -> UPM3_R {
        UPM3_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    pub fn umsel3(&self) -> UMSEL3_R {
        UMSEL3_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ucpol3(&mut self) -> UCPOL3_W<UCSR3C_SPEC> {
        UCPOL3_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn ucsz3(&mut self) -> UCSZ3_W<UCSR3C_SPEC> {
        UCSZ3_W::new(self, 1)
    }
    #[doc = "Bit 3 - Stop Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn usbs3(&mut self) -> USBS3_W<UCSR3C_SPEC> {
        USBS3_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Parity Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn upm3(&mut self) -> UPM3_W<UCSR3C_SPEC> {
        UPM3_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - USART Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn umsel3(&mut self) -> UMSEL3_W<UCSR3C_SPEC> {
        UMSEL3_W::new(self, 6)
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
#[doc = "USART Control and Status Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr3c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr3c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR3C_SPEC;
impl crate::RegisterSpec for UCSR3C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr3c::R`](R) reader structure"]
impl crate::Readable for UCSR3C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr3c::W`](W) writer structure"]
impl crate::Writable for UCSR3C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR3C to value 0"]
impl crate::Resettable for UCSR3C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
