#[doc = "Register `CSMA_SEED_1` reader"]
pub type R = crate::R<CSMA_SEED_1_SPEC>;
#[doc = "Register `CSMA_SEED_1` writer"]
pub type W = crate::W<CSMA_SEED_1_SPEC>;
#[doc = "Field `CSMA_SEED_1` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_1_R = crate::FieldReader;
#[doc = "Field `CSMA_SEED_1` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `AACK_I_AM_COORD` reader - Set Personal Area Network Coordinator"]
pub type AACK_I_AM_COORD_R = crate::BitReader;
#[doc = "Field `AACK_I_AM_COORD` writer - Set Personal Area Network Coordinator"]
pub type AACK_I_AM_COORD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AACK_DIS_ACK` reader - Disable Acknowledgment Frame Transmission"]
pub type AACK_DIS_ACK_R = crate::BitReader;
#[doc = "Field `AACK_DIS_ACK` writer - Disable Acknowledgment Frame Transmission"]
pub type AACK_DIS_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AACK_SET_PD` reader - Set Frame Pending Sub-field"]
pub type AACK_SET_PD_R = crate::BitReader;
#[doc = "Field `AACK_SET_PD` writer - Set Frame Pending Sub-field"]
pub type AACK_SET_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AACK_FVN_MODE` reader - Acknowledgment Frame Filter Mode"]
pub type AACK_FVN_MODE_R = crate::FieldReader<AACK_FVN_MODE_A>;
#[doc = "Acknowledgment Frame Filter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AACK_FVN_MODE_A {
    #[doc = "0: Acknowledge frames with version number 0"]
    ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0 = 0,
    #[doc = "1: Acknowledge frames with version number 0 or 1"]
    ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1 = 1,
    #[doc = "2: Acknowledge frames with version number 0 or 1 or 2"]
    ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1_OR_2 = 2,
    #[doc = "3: Acknowledge frames independent of frame version number"]
    ACKNOWLEDGE_FRAMES_INDEPENDENT_OF_FRAME_VERSION_NUMBER = 3,
}
impl From<AACK_FVN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AACK_FVN_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AACK_FVN_MODE_A {
    type Ux = u8;
}
impl AACK_FVN_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AACK_FVN_MODE_A {
        match self.bits {
            0 => AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0,
            1 => AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1,
            2 => AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1_OR_2,
            3 => AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_INDEPENDENT_OF_FRAME_VERSION_NUMBER,
            _ => unreachable!(),
        }
    }
    #[doc = "Acknowledge frames with version number 0"]
    #[inline(always)]
    pub fn is_acknowledge_frames_with_version_number_0(&self) -> bool {
        *self == AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0
    }
    #[doc = "Acknowledge frames with version number 0 or 1"]
    #[inline(always)]
    pub fn is_acknowledge_frames_with_version_number_0_or_1(&self) -> bool {
        *self == AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1
    }
    #[doc = "Acknowledge frames with version number 0 or 1 or 2"]
    #[inline(always)]
    pub fn is_acknowledge_frames_with_version_number_0_or_1_or_2(&self) -> bool {
        *self == AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1_OR_2
    }
    #[doc = "Acknowledge frames independent of frame version number"]
    #[inline(always)]
    pub fn is_acknowledge_frames_independent_of_frame_version_number(&self) -> bool {
        *self == AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_INDEPENDENT_OF_FRAME_VERSION_NUMBER
    }
}
#[doc = "Field `AACK_FVN_MODE` writer - Acknowledgment Frame Filter Mode"]
pub type AACK_FVN_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AACK_FVN_MODE_A>;
impl<'a, REG> AACK_FVN_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Acknowledge frames with version number 0"]
    #[inline(always)]
    pub fn acknowledge_frames_with_version_number_0(self) -> &'a mut crate::W<REG> {
        self.variant(AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0)
    }
    #[doc = "Acknowledge frames with version number 0 or 1"]
    #[inline(always)]
    pub fn acknowledge_frames_with_version_number_0_or_1(self) -> &'a mut crate::W<REG> {
        self.variant(AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1)
    }
    #[doc = "Acknowledge frames with version number 0 or 1 or 2"]
    #[inline(always)]
    pub fn acknowledge_frames_with_version_number_0_or_1_or_2(self) -> &'a mut crate::W<REG> {
        self.variant(AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_WITH_VERSION_NUMBER_0_OR_1_OR_2)
    }
    #[doc = "Acknowledge frames independent of frame version number"]
    #[inline(always)]
    pub fn acknowledge_frames_independent_of_frame_version_number(self) -> &'a mut crate::W<REG> {
        self.variant(AACK_FVN_MODE_A::ACKNOWLEDGE_FRAMES_INDEPENDENT_OF_FRAME_VERSION_NUMBER)
    }
}
impl R {
    #[doc = "Bits 0:2 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_1(&self) -> CSMA_SEED_1_R {
        CSMA_SEED_1_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Set Personal Area Network Coordinator"]
    #[inline(always)]
    pub fn aack_i_am_coord(&self) -> AACK_I_AM_COORD_R {
        AACK_I_AM_COORD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable Acknowledgment Frame Transmission"]
    #[inline(always)]
    pub fn aack_dis_ack(&self) -> AACK_DIS_ACK_R {
        AACK_DIS_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set Frame Pending Sub-field"]
    #[inline(always)]
    pub fn aack_set_pd(&self) -> AACK_SET_PD_R {
        AACK_SET_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Acknowledgment Frame Filter Mode"]
    #[inline(always)]
    pub fn aack_fvn_mode(&self) -> AACK_FVN_MODE_R {
        AACK_FVN_MODE_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_1(&mut self) -> CSMA_SEED_1_W<CSMA_SEED_1_SPEC> {
        CSMA_SEED_1_W::new(self, 0)
    }
    #[doc = "Bit 3 - Set Personal Area Network Coordinator"]
    #[inline(always)]
    #[must_use]
    pub fn aack_i_am_coord(&mut self) -> AACK_I_AM_COORD_W<CSMA_SEED_1_SPEC> {
        AACK_I_AM_COORD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Disable Acknowledgment Frame Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn aack_dis_ack(&mut self) -> AACK_DIS_ACK_W<CSMA_SEED_1_SPEC> {
        AACK_DIS_ACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set Frame Pending Sub-field"]
    #[inline(always)]
    #[must_use]
    pub fn aack_set_pd(&mut self) -> AACK_SET_PD_W<CSMA_SEED_1_SPEC> {
        AACK_SET_PD_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Acknowledgment Frame Filter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aack_fvn_mode(&mut self) -> AACK_FVN_MODE_W<CSMA_SEED_1_SPEC> {
        AACK_FVN_MODE_W::new(self, 6)
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
#[doc = "Transceiver Acknowledgment Frame Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csma_seed_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csma_seed_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSMA_SEED_1_SPEC;
impl crate::RegisterSpec for CSMA_SEED_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csma_seed_1::R`](R) reader structure"]
impl crate::Readable for CSMA_SEED_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csma_seed_1::W`](W) writer structure"]
impl crate::Writable for CSMA_SEED_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSMA_SEED_1 to value 0"]
impl crate::Resettable for CSMA_SEED_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
