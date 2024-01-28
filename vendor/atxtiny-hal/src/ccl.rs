//! # Configurable Custom Logic

use crate::pac::{
    ccl::seqctrl,
    ccl::lut::{lutctrla, lutctrlb, lutctrlc},
    CCL
};
use crate::Toggle;

use core::marker::PhantomData;

// TODO: allow config of RUNSTDBY

/// CCL Lookup table Output pin
pub trait OutputPin<LUT>: crate::private::Sealed {}

/// CCL Lookup table Input pin
pub trait InputPin<LUT, const IDX: u8>: crate::private::Sealed {}

/// Pin set for the port multiplexer
pub struct CclLutOutputPinset<LUT, Out: OutputPin<LUT>> {
    _lut: PhantomData<LUT>,
    out: Out,
}

impl<LUT, Out> CclLutOutputPinset<LUT, Out>
where
    Out: OutputPin<LUT>
{
    pub(crate) fn new(out: Out) -> Self {
        CclLutOutputPinset { _lut: PhantomData, out }
    }

    pub fn free(self) -> Out {
        self.out
    }
}



/// Extension trait to configure a `CCL` peripheral and all containing LUTs
pub trait CclExt {
    /// The Parts to split the `CCL` peripheral into
    type Parts;

    /// Splits the `CCL` block into independent LUTs
    fn split(self) -> Self::Parts;
}



/// CCL Register interface traits private to this module
mod private {
    use super::{SequencerConfig, FilterSelection, ClockSource, Input0, Input1, Input2, Toggle};

    pub trait CclRegExt {
        fn enable(&self);
        fn disable(&self);
        fn sequencer_config(&self, seq_idx: u8, config: SequencerConfig);

        fn lut_edge_detection(&self, i: u8, enable: Toggle);
        fn lut_output_enable(&self, i: u8, enable: Toggle);
        fn lut_filter_selection(&self, lut_idx: u8, filter: FilterSelection);
        fn lut_clock_source(&self, lut_idx: u8, filter: ClockSource);
        fn lut_enable(&self, lut_idx: u8, state: Toggle);
        fn lut_inputs(&self, lut_idx: u8, input0: Input0, input1: Input1, input2: Input2);
        fn lut_table(&self, lut_idx: u8, table: u8);
    }

    pub trait Ccl {
        type Reg: CclRegExt + ?Sized;

        fn ptr(&self) -> *const Self::Reg;
    }
}



/// Marker traits used in this module
pub mod marker {
    /// Marker trait for CCLs
    pub trait Ccl: super::private::Ccl {}

    /// Marker trait for unconfigured LUT
    pub trait Disabled {}

    /// Marker trait for configured and enabled LUT
    pub trait Enabled {}

    /// Marker trait for LUT index
    pub trait Index {
        #[doc(hidden)]
        fn index(&self) -> u8;
    }
}



/// Compile time defined LUT index (type state)
#[derive(ufmt::derive::uDebug, Debug)]
pub struct U<const X: u8>;

impl<const X: u8> marker::Index for U<X> {
    #[inline(always)]
    fn index(&self) -> u8 {
        X
    }
}


/// Active LUT (type state)
#[derive(ufmt::derive::uDebug, Debug)]
pub struct Active;

/// Inactive LUT (type state)
#[derive(ufmt::derive::uDebug, Debug)]
pub struct Inactive;

impl marker::Enabled for Active {}
impl marker::Disabled for Inactive {}



/// Generic LUT
#[derive(Debug)]
pub struct Lut<Ccl, Index, State> {
    pub(crate) ccl: Ccl,
    pub(crate) index: Index,
    _state: PhantomData<State>,
}

// Make all LUT peripheral trait extensions sealable.
impl<Ccl, Index, State> crate::private::Sealed for Lut<Ccl, Index, State> {}

impl<Ccl, Index, State> Lut<Ccl, Index, State> {
    fn into_state<NewState>(self) -> Lut<Ccl, Index, NewState> {
        Lut {
            ccl: self.ccl,
            index: self.index,
            _state: PhantomData,
        }
    }
}

impl<Ccl, Index> Lut<Ccl, Index, Inactive>
where
    Ccl: marker::Ccl,
    Index: marker::Index,
{
    /// Enable the LUT.
    /// 
    /// An enabled LUT cannot be reconfigured until it's disabled again using
    /// [`Lut::disable`].
    #[inline]
    pub fn enable(self) -> Lut<Ccl, Index, Active> {
        unsafe { (*self.ccl.ptr()).lut_enable(self.index.index(), Toggle::On) };
        self.into_state()
    }

    /// Configure the edge detection.
    #[inline]
    pub fn edge_detection(self, enable: Toggle) -> Self {
        unsafe { (*self.ccl.ptr()).lut_edge_detection(self.index.index(), enable) };
        self
    }

    /// Configure the output pin of the LUT.
    /// 
    /// When enabled, this overrides the pin configuration of the
    /// PORT I/O controller.
    #[inline]
    pub fn output_enable(self, enable: Toggle) -> Self {
       unsafe { (*self.ccl.ptr()).lut_output_enable(self.index.index(), enable) };
       self
    }

    /// Configure the output synchronization filter.
    #[inline]
    pub fn filter(self, filter: FilterSelection) -> Self {
       unsafe { (*self.ccl.ptr()).lut_filter_selection(self.index.index(), filter) };
       self
    }

    /// Select the clock source for the sequencer.
    #[inline]
    pub fn clock_source(self, clock_src: ClockSource) -> Self {
       unsafe { (*self.ccl.ptr()).lut_clock_source(self.index.index(), clock_src) };
       self
    }

    /// Define the two inputs into the lookup table.
    #[inline]
    pub fn inputs(self, input0: Input0, input1: Input1, input2: Input2) -> Self {
       unsafe { (*self.ccl.ptr()).lut_inputs(self.index.index(), input0, input1, input2) };
       self
    }

    /// Set the lookup table value.
    #[inline]
    pub fn table(self, lookup_table: u8) -> Self {
       unsafe { (*self.ccl.ptr()).lut_table(self.index.index(), lookup_table) };
       self
    }
}

impl<Ccl, Index> Lut<Ccl, Index, Active>
where
    Ccl: marker::Ccl,
    Index: marker::Index,
{
    /// Disable the LUT
    /// 
    /// A disabled LUT can be reconfigured again until it's enabled using
    /// [`Lut::enable`].
    #[inline]
    pub fn disable(self) -> Lut<Ccl, Index, Inactive> {
        unsafe { (*self.ccl.ptr()).lut_enable(self.index.index(), Toggle::Off) };
        self.into_state()
    }
}


use private::CclRegExt;

impl CclRegExt for crate::pac::ccl::RegisterBlock {
    #[inline(always)]
    fn enable(&self) {
        self.ctrla().write(|w| w.enable().set_bit());
    }

    #[inline(always)]
    fn disable(&self) {
        self.ctrla().write(|w| w.enable().clear_bit());
    }

    #[inline(always)]
    fn sequencer_config(&self, seq_idx: u8, config: SequencerConfig) {
        self.seqctrl(seq_idx as usize).write(|w| w.seqsel().variant(config.into()));
    }

    #[inline(always)]
    fn lut_edge_detection(&self, lut_idx: u8, enable: Toggle) {
        self.lut(lut_idx as usize).lutctrla().modify(|_, w| w.edgedet().bit(enable.into()));
    }

    #[inline(always)]
    fn lut_output_enable(&self, lut_idx: u8, enable: Toggle) {
        self.lut(lut_idx as usize).lutctrla().modify(|_, w| w.outen().bit(enable.into()));
    }

    #[inline(always)]
    fn lut_filter_selection(&self, lut_idx: u8, filter: FilterSelection) {
        self.lut(lut_idx as usize).lutctrla().modify(|_, w| w.filtsel().variant(filter.into()));
    }

    #[inline(always)]
    fn lut_clock_source(&self, lut_idx: u8, filter: ClockSource) {
        self.lut(lut_idx as usize).lutctrla().modify(|_, w| w.clksrc().variant(filter.into()));
    }

    #[inline(always)]
    fn lut_enable(&self, lut_idx: u8, state: Toggle) {
        self.lut(lut_idx as usize).lutctrla().modify(|_, w| w.enable().variant(state.into()));
    }

    #[inline(always)]
    fn lut_inputs(&self, lut_idx: u8, input0: Input0, input1: Input1, input2: Input2) {
        self.lut(lut_idx as usize).lutctrlb().modify(|_, w| w.insel0().variant(input0.into()).insel1().variant(input1.into()));
        self.lut(lut_idx as usize).lutctrlc().modify(|_, w| w.insel2().variant(input2.into()));
    }

    #[inline(always)]
    fn lut_table(&self, lut_idx: u8, table: u8) {
        self.lut(lut_idx as usize).truth().write(|w| w.bits(table));
    }
}



/// Generic main control block for a CCL
#[derive(ufmt::derive::uDebug, Debug)]
pub struct Control<Ccl> {
    pub(crate) ccl: Ccl,
}

// Make all Control peripheral trait extensions sealable.
impl<Ccl> crate::private::Sealed for Control<Ccl> {}

impl<Ccl> Control<Ccl>
where
    Ccl: marker::Ccl,
{
    /// Enable the CCL peripheral block
    /// 
    /// NOTE: Due to an errata, the whole CCL blocks needs to be disabled
    /// completely to reconfigure even independent LUTs, otherwise registers
    /// in the LUT region are still going to be enable-protected. The AVR-DD
    /// series fixes this errata
    #[inline]
    pub fn enable(&self) {
        unsafe { (*self.ccl.ptr()).enable() };
    }

    /// Disable the CCL peripheral block
    #[inline]
    pub fn disable(&self) {
        unsafe { (*self.ccl.ptr()).disable() };
    }

    /// Set the sequencer config to connect multiple LUTs together and build
    /// feedback loops
    #[inline]
    pub fn sequencer_config(&self, seq: Sequencer, cfg: SequencerConfig) {
        unsafe { (*self.ccl.ptr()).sequencer_config(seq.into(), cfg) };
    }
}



/// The CCL itself (type state)
pub struct Ccl;

impl private::Ccl for Ccl {
    type Reg = crate::pac::ccl::RegisterBlock;

    fn ptr(&self) -> *const Self::Reg {
        CCL::ptr()
    }
}

impl marker::Ccl for Ccl {}



macro_rules! ccl {
    ({
        luts: [$(
            {
                lut: $index:literal,
            },
        )+],
    }) => {
        paste::paste! {
            $(
                #[doc = concat!("Lookup table ", stringify!([<LUT $index>]))]
                pub type [<LUT $index>] = Lut<Ccl, U<$index>, Inactive>;
            )+

            /// CCL Parts
            pub struct Parts {
                pub control: Control<Ccl>,
                $(
                    pub [<lut $index>]: [<LUT $index>],
                )+
            }

            impl CclExt for CCL {
                type Parts = Parts;

                fn split(self) -> Self::Parts {
                    Self::Parts {
                        control: Control { ccl: Ccl },
                        $(
                            [<lut $index>]: [<LUT $index>] {
                                ccl: Ccl,
                                index: U::<$index>,
                                _state: PhantomData,
                            },
                        )+
                    }
                }
            }
        }
    };
}

ccl!({
    luts: [
        { lut: 0, },
        { lut: 1, },
    ],
});

// FIXME: below structs are all device-dependent

#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sequencer {
    LUT01,
    // TODO: chip dependent
    //LUT23,
    //LUT45,
}

impl From<Sequencer> for u8 {
    fn from(value: Sequencer) -> Self {
        match value {
            Sequencer::LUT01 => 0,
            //Sequencer::LUT23 => 1,
            //Sequencer::LUT45 => 2,
        }
    }
}

#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequencerConfig {
    Disable,
    DFlipFlop,
    JKFlipFlop,
    DLatch,
    RSLatch
}

impl From<SequencerConfig> for seqctrl::SEQSEL_A {
    fn from(value: SequencerConfig) -> Self {
        match value {
            SequencerConfig::Disable => seqctrl::SEQSEL_A::DISABLE,
            SequencerConfig::DFlipFlop => seqctrl::SEQSEL_A::DFF,
            SequencerConfig::JKFlipFlop => seqctrl::SEQSEL_A::JK,
            SequencerConfig::DLatch => seqctrl::SEQSEL_A::LATCH,
            SequencerConfig::RSLatch => seqctrl::SEQSEL_A::RS,
        }
    }
}

#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterSelection {
    Disable,
    SynchronizerEnabled,
    FilterEnabled,
}

impl From<FilterSelection> for lutctrla::FILTSEL_A {
    fn from(value: FilterSelection) -> Self {
        match value {
            FilterSelection::Disable => lutctrla::FILTSEL_A::DISABLE,
            FilterSelection::SynchronizerEnabled => lutctrla::FILTSEL_A::SYNCH,
            FilterSelection::FilterEnabled => lutctrla::FILTSEL_A::FILTER,
        }
    }
}

#[derive(ufmt::derive::uDebug, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSource {
    PeripheralClock,
    Input2,
}

impl From<ClockSource> for bool {
    fn from(value: ClockSource) -> Self {
        match value {
            ClockSource::PeripheralClock => false,
            ClockSource::Input2 => true,
        }
    }
}

pub enum Input0 {
    Masked,
    Feedback,
    Link,
    Event01,
    Event23,
    IoPin,
    Ac0Out,
    Tcb0Wo,
    Tca0Wo0,
    Tcd0Woa,
    Usart0Xck,
    Spi0Sck,
}

impl From<Input0> for lutctrlb::INSEL0_A {
    fn from(input: Input0) -> Self {
        match input {
            Input0::Masked => lutctrlb::INSEL0_A::MASK,
            Input0::Feedback => lutctrlb::INSEL0_A::FEEDBACK,
            Input0::Link => lutctrlb::INSEL0_A::LINK,
            Input0::Event01 => lutctrlb::INSEL0_A::EVENT0,
            Input0::Event23 => lutctrlb::INSEL0_A::EVENT1,
            Input0::IoPin => lutctrlb::INSEL0_A::IO,
            Input0::Ac0Out => lutctrlb::INSEL0_A::AC0,
            Input0::Tcb0Wo => lutctrlb::INSEL0_A::TCB0,
            Input0::Tca0Wo0 => lutctrlb::INSEL0_A::TCA0,
            Input0::Tcd0Woa => lutctrlb::INSEL0_A::TCD0,
            Input0::Usart0Xck => lutctrlb::INSEL0_A::USART0,
            Input0::Spi0Sck => lutctrlb::INSEL0_A::SPI0,
        }
    }
}

pub enum Input1 {
    Masked,
    Feedback,
    Link,
    Event01,
    Event23,
    IoPin,
    Ac0Out,
    Tcb0Wo,
    Tca0Wo1,
    Tcd0Wob,
    Usart0Txd,
    Spi0Mosi,
}

impl From<Input1> for lutctrlb::INSEL1_A {
    fn from(input: Input1) -> Self {
        match input {
            Input1::Masked => lutctrlb::INSEL1_A::MASK,
            Input1::Feedback => lutctrlb::INSEL1_A::FEEDBACK,
            Input1::Link => lutctrlb::INSEL1_A::LINK,
            Input1::Event01 => lutctrlb::INSEL1_A::EVENT0,
            Input1::Event23 => lutctrlb::INSEL1_A::EVENT1,
            Input1::IoPin => lutctrlb::INSEL1_A::IO,
            Input1::Ac0Out => lutctrlb::INSEL1_A::AC0,
            Input1::Tcb0Wo => lutctrlb::INSEL1_A::TCB0,
            Input1::Tca0Wo1 => lutctrlb::INSEL1_A::TCA0,
            Input1::Tcd0Wob => lutctrlb::INSEL1_A::TCD0,
            Input1::Usart0Txd => lutctrlb::INSEL1_A::USART0,
            Input1::Spi0Mosi => lutctrlb::INSEL1_A::SPI0,
        }
    }
}

pub enum Input2 {
    Masked,
    Feedback,
    Link,
    Event0,
    Event1,
    IoPin,
    Ac0Out,
    Tcb0Wo,
    Tca0Wo2,
    Tcd0Woa,
    Spi0Miso,
}

impl From<Input2> for lutctrlc::INSEL2_A {
    fn from(input: Input2) -> Self {
        match input {
            Input2::Masked => lutctrlc::INSEL2_A::MASK,
            Input2::Feedback => lutctrlc::INSEL2_A::FEEDBACK,
            Input2::Link => lutctrlc::INSEL2_A::LINK,
            Input2::Event0 => lutctrlc::INSEL2_A::EVENT0,
            Input2::Event1 => lutctrlc::INSEL2_A::EVENT1,
            Input2::IoPin => lutctrlc::INSEL2_A::IO,
            Input2::Ac0Out => lutctrlc::INSEL2_A::AC0,
            Input2::Tcb0Wo => lutctrlc::INSEL2_A::TCB0,
            Input2::Tca0Wo2 => lutctrlc::INSEL2_A::TCA0,
            Input2::Tcd0Woa => lutctrlc::INSEL2_A::TCD0,
            Input2::Spi0Miso => lutctrlc::INSEL2_A::SPI0,
        }
    }
}



// TODO: I didn't manage yet to add pins to the LUT state so far
// TODO: macros
use crate::gpio::{Output, Stateless, Input};

impl OutputPin<LUT0> for crate::gpio::porta::PA4<Output<Stateless>> {}
impl OutputPin<LUT0> for crate::gpio::portb::PB4<Output<Stateless>> {}

impl OutputPin<LUT1> for crate::gpio::porta::PA7<Output<Stateless>> {}
impl OutputPin<LUT1> for crate::gpio::portc::PC1<Output<Stateless>> {}

impl InputPin<LUT0, 0> for crate::gpio::porta::PA0<Input> {}
impl InputPin<LUT0, 1> for crate::gpio::porta::PA1<Input> {}
impl InputPin<LUT0, 2> for crate::gpio::porta::PA2<Input> {}

impl InputPin<LUT1, 0> for crate::gpio::portc::PC3<Input> {}
impl InputPin<LUT1, 1> for crate::gpio::portc::PC4<Input> {}
impl InputPin<LUT1, 2> for crate::gpio::portc::PC5<Input> {}


use crate::evsys::ChannelConfigurator;
use crate::evsys::{EventGenerator, Channel, Unconfigured, GeneratorAssigned};

impl<Evsys, Index, CCL, State, const X: u8> EventGenerator<Evsys, crate::evsys::Async, Index> for Lut<CCL, U<X>, State>
where
    Evsys: crate::evsys::marker::Evsys,
    Index: crate::evsys::marker::Index,
{
    type EventSource = ();

    fn connect_event_generator(&mut self, mut channel: Channel<Evsys, crate::evsys::Async, Index, Unconfigured>, _source: ()) -> Channel<Evsys, crate::evsys::Async, Index, GeneratorAssigned> {
        channel.set_generator(0x01 + X);
        channel.into_state()
    }
}
