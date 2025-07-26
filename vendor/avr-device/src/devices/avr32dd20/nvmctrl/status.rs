#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `FBUSY` reader - Flash busy"]
pub type FBUSY_R = crate::BitReader;
#[doc = "Field `EEBUSY` reader - EEPROM busy"]
pub type EEBUSY_R = crate::BitReader;
#[doc = "Field `ERROR` reader - Write error"]
pub type ERROR_R = crate::FieldReader<ERROR_A>;
#[doc = "Write error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERROR_A {
    #[doc = "0: No Error"]
    NOERROR = 0,
    #[doc = "1: Write command not selected"]
    ILLEGALCMD = 1,
    #[doc = "2: Write to section not allowed"]
    ILLEGALSADDR = 2,
    #[doc = "3: Selecting new write command while write command already seleted"]
    DOUBLESELECT = 3,
    #[doc = "4: Starting a new programming operation before previous is completed"]
    ONGOINGPROG = 4,
}
impl From<ERROR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ERROR_A {
    type Ux = u8;
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ERROR_A> {
        match self.bits {
            0 => Some(ERROR_A::NOERROR),
            1 => Some(ERROR_A::ILLEGALCMD),
            2 => Some(ERROR_A::ILLEGALSADDR),
            3 => Some(ERROR_A::DOUBLESELECT),
            4 => Some(ERROR_A::ONGOINGPROG),
            _ => None,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == ERROR_A::NOERROR
    }
    #[doc = "Write command not selected"]
    #[inline(always)]
    pub fn is_illegalcmd(&self) -> bool {
        *self == ERROR_A::ILLEGALCMD
    }
    #[doc = "Write to section not allowed"]
    #[inline(always)]
    pub fn is_illegalsaddr(&self) -> bool {
        *self == ERROR_A::ILLEGALSADDR
    }
    #[doc = "Selecting new write command while write command already seleted"]
    #[inline(always)]
    pub fn is_doubleselect(&self) -> bool {
        *self == ERROR_A::DOUBLESELECT
    }
    #[doc = "Starting a new programming operation before previous is completed"]
    #[inline(always)]
    pub fn is_ongoingprog(&self) -> bool {
        *self == ERROR_A::ONGOINGPROG
    }
}
impl R {
    #[doc = "Bit 0 - Flash busy"]
    #[inline(always)]
    pub fn fbusy(&self) -> FBUSY_R {
        FBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM busy"]
    #[inline(always)]
    pub fn eebusy(&self) -> EEBUSY_R {
        EEBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Write error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new((self.bits >> 4) & 7)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
