#[doc = "Register `SSTATUS` reader"]
pub type R = crate::R<SSTATUS_SPEC>;
#[doc = "Register `SSTATUS` writer"]
pub type W = crate::W<SSTATUS_SPEC>;
#[doc = "Field `AP` reader - Address or Stop"]
pub type AP_R = crate::BitReader<AP_A>;
#[doc = "Address or Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AP_A {
    #[doc = "0: A Stop condition generated the interrupt on APIF flag"]
    STOP = 0,
    #[doc = "1: Address detection generated the interrupt on APIF flag"]
    ADR = 1,
}
impl From<AP_A> for bool {
    #[inline(always)]
    fn from(variant: AP_A) -> Self {
        variant as u8 != 0
    }
}
impl AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AP_A {
        match self.bits {
            false => AP_A::STOP,
            true => AP_A::ADR,
        }
    }
    #[doc = "A Stop condition generated the interrupt on APIF flag"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == AP_A::STOP
    }
    #[doc = "Address detection generated the interrupt on APIF flag"]
    #[inline(always)]
    pub fn is_adr(&self) -> bool {
        *self == AP_A::ADR
    }
}
#[doc = "Field `DIR` reader - Read/Write Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLL` reader - Collision"]
pub type COLL_R = crate::BitReader;
#[doc = "Field `COLL` writer - Collision"]
pub type COLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXACK` reader - Received Acknowledge"]
pub type RXACK_R = crate::BitReader;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type CLKHOLD_R = crate::BitReader;
#[doc = "Field `APIF` reader - Address or Stop Interrupt Flag"]
pub type APIF_R = crate::BitReader;
#[doc = "Field `DIF` reader - Data Interrupt Flag"]
pub type DIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Address or Stop"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read/Write Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Collision"]
    #[inline(always)]
    pub fn coll(&self) -> COLL_R {
        COLL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received Acknowledge"]
    #[inline(always)]
    pub fn rxack(&self) -> RXACK_R {
        RXACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address or Stop Interrupt Flag"]
    #[inline(always)]
    pub fn apif(&self) -> APIF_R {
        APIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Interrupt Flag"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<SSTATUS_SPEC> {
        BUSERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Collision"]
    #[inline(always)]
    #[must_use]
    pub fn coll(&mut self) -> COLL_W<SSTATUS_SPEC> {
        COLL_W::new(self, 3)
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
#[doc = "Client Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSTATUS_SPEC;
impl crate::RegisterSpec for SSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sstatus::R`](R) reader structure"]
impl crate::Readable for SSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sstatus::W`](W) writer structure"]
impl crate::Writable for SSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSTATUS to value 0"]
impl crate::Resettable for SSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
