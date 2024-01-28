//! # Event system

use core::marker::PhantomData;

use crate::pac::EVSYS;

/// Extension trait to configure an `EVSYS` peripheral and all containing channels
pub trait EvsysExt {
    /// The Parts to split the `EVSYS` peripheral into
    type Parts;

    /// Splits the EVSYS block into independent channels
    fn split(self) -> Self::Parts;
}

/// `EVSYS` Register interface traits private to this module
mod private {
    pub trait EvsysRegExt {
        fn set_async_generator(&self, channel_idx: u8, generator: u8);
        fn set_sync_generator(&self, channel_idx: u8, generator: u8);

        fn set_async_user(&self, user_idx: u8, multiplexer_select: u8);
        fn set_sync_user(&self, user_idx: u8, multiplexer_select: u8);

        //FIXME: add strobes
    }

    pub trait Evsys {
        type Reg: EvsysRegExt + ?Sized;

        fn ptr(&self) -> *const Self::Reg;
    }
}

use private::EvsysRegExt;

/// Marker traits used in this module
pub mod marker {
    /// Marker trait for event systems
    pub trait Evsys: super::private::Evsys {}

    /// Marker trait for the flavor of a channel (Synchronous vs Asynchronous)
    pub trait ChannelFlavor {}

    /// Marker trait for the state of a channel
    pub trait ChannelState {}

    /// Marker trait for channel indexes
    pub trait Index {
        const X: u8;
        const UX: u8;

        #[doc(hidden)]
        fn index(&self) -> u8;
    }
}

/// Compile time defined channel index (type state)
#[derive(Debug, Default)]
pub struct U<const X: u8, const UX: u8>;

impl<const X: u8, const UX: u8> marker::Index for U<X, UX> {
    const X: u8 = X;
    const UX: u8 = UX;

    #[inline(always)]
    fn index(&self) -> u8 {
        X
    }
}

/// Asynchronous channel (type state)
#[derive(ufmt::derive::uDebug, Debug, Default)]
pub struct Async;

/// Synchronous channel (type state)
#[derive(ufmt::derive::uDebug, Debug, Default)]
pub struct Sync;

/// Unconfigured channel (type state)
#[derive(ufmt::derive::uDebug, Debug)]
pub struct Unconfigured;

/// Generator assigned to channel (type state)
#[derive(ufmt::derive::uDebug, Debug)]
pub struct GeneratorAssigned;

/// Fully configured channel where generator and user is assigned (type state)
#[derive(ufmt::derive::uDebug, Debug)]
pub struct Configured;

impl marker::ChannelFlavor for Async {}
impl marker::ChannelFlavor for Sync {}

impl marker::ChannelState for Unconfigured {}
impl marker::ChannelState for GeneratorAssigned {}
impl marker::ChannelState for Configured {}

/// Generic event channel
#[derive(Debug)]
pub struct Channel<Evsys, Flavor, Index, State> {
    pub(crate) evsys: Evsys,
    pub(crate) index: Index,
    pub(crate) _phantom: PhantomData<(Flavor, State)>,
}

impl<Evsys, Flavor, Index, State> crate::private::Sealed for Channel<Evsys, Flavor, Index, State> {}

impl<Evsys, Flavor, Index, State> Channel<Evsys, Flavor, Index, State> {
    pub (crate) fn into_state<NewState>(self) -> Channel<Evsys, Flavor, Index, NewState> {
        Channel {
            evsys: self.evsys,
            index: self.index,
            _phantom: PhantomData,
        }
    }
}


macro_rules! evsys {
    ({
        channels: [$(
            {
                channel: $index:literal,
                register: $register:ident,
                userindex: $userindex:literal,
                flavor: $flavor:ty,
                // generators: {
                //     $($generator:ident => $value:literal,)+
                // }
            },
        )+],
    }) => {
        paste::paste! {
            /// Event system
            #[derive(ufmt::derive::uDebug, Debug)]
            pub struct Evsys;

            impl marker::Evsys for Evsys {}

            impl private::Evsys for Evsys {
                type Reg = crate::pac::evsys::RegisterBlock;

                fn ptr(&self) -> *const Self::Reg {
                    EVSYS::ptr()
                }
            }

            impl EvsysRegExt for crate::pac::evsys::RegisterBlock {
                fn set_async_generator(&self, channel_idx: u8, generator: u8) {
                    self.asyncch(channel_idx as usize).write(|f| unsafe { f.bits(generator) });
                }
            
                fn set_sync_generator(&self, channel_idx: u8, generator: u8) {
                    self.syncch(channel_idx as usize).write(|f| unsafe { f.bits(generator) });
                }
            
                fn set_async_user(&self, user_idx: u8, multiplexer_select: u8) {
                    self.asyncuser(multiplexer_select as usize).write(|f| unsafe { f.bits(user_idx) });
                }
            
                fn set_sync_user(&self, user_idx: u8, multiplexer_select: u8) {
                    self.syncuser(multiplexer_select as usize).write(|f| unsafe { f.bits(user_idx) });
                }
            }

            $(
                #[doc = concat!("Event channel ", stringify!($index))]
                pub type [<Channel $flavor $index>] = Channel<Evsys, $flavor, U<$index, $userindex>, Unconfigured>;

                // #[doc = concat!("Event channel ", stringify!($index), " generator sources")]
                // #[allow(non_camel_case_types)]
                // #[repr(u8)]
                // pub enum [<Channel $flavor $index Generator>] {
                //     $(
                //         $generator = $value,
                //     )+
                // }
            )+

            /// EVSYS Parts
            pub struct Parts {
                $(
                    pub [<channel_ $flavor:lower $index>]: [<Channel $flavor $index>],
                )+
            }

            impl EvsysExt for EVSYS {
                type Parts = Parts;

                fn split(self) -> Self::Parts {
                    Self::Parts {
                        $(
                            [<channel_ $flavor:lower $index>]: [<Channel $flavor $index>] {
                                evsys: Evsys,
                                index: U::<$index, $userindex>::default(),
                                _phantom: PhantomData::default(),
                            },
                        )+
                    }
                }
            }
        }
    };
}

pub trait EventUser<Evsys, Flavor>
where
    Evsys: marker::Evsys,
    Flavor: marker::ChannelFlavor,
{
    const MULTIPLEXER_INDEX: u8;
}

impl<Evsys, Index> Channel<Evsys, Async, Index, GeneratorAssigned>
where
    Evsys: marker::Evsys,
    Index: marker::Index,
{
    pub fn connect_event_user<U: EventUser<Evsys, Async>>(mut self, _user: &U) -> Channel<Evsys, Async, Index, Configured> {
        self.set_multiplexer(U::MULTIPLEXER_INDEX);
        self.into_state()
    }
}

impl<Evsys, Index> Channel<Evsys, Async, Index, Configured>
where
    Evsys: marker::Evsys,
    Index: marker::Index,
{
    pub fn free_user(mut self) -> Channel<Evsys, Async, Index, GeneratorAssigned> {
        self.set_multiplexer(0);
        self.into_state()
    }
}

impl<Evsys, Index> Channel<Evsys, Async, Index, GeneratorAssigned>
where
    Evsys: marker::Evsys,
    Index: marker::Index,
{
    pub fn free_generator(mut self) -> Channel<Evsys, Async, Index, Unconfigured> {
        self.set_generator(0);
        self.into_state()
    }
}

impl<Evsys, Index> Channel<Evsys, Sync, Index, Configured>
where
    Evsys: marker::Evsys,
    Index: marker::Index,
{
    pub fn free_user(mut self) -> Channel<Evsys, Sync, Index, GeneratorAssigned> {
        self.set_multiplexer(0);
        self.into_state()
    }
}

impl<Evsys, Index> Channel<Evsys, Sync, Index, GeneratorAssigned>
where
    Evsys: marker::Evsys,
    Index: marker::Index,
{
    pub fn free_generator(mut self) -> Channel<Evsys, Sync, Index, Unconfigured> {
        self.set_generator(0);
        self.into_state()
    }
}

pub trait ChannelConfigurator<F> {
    fn set_multiplexer(&mut self, multiplexer: u8);
    fn set_generator(&mut self, generator: u8);
}

impl<Evsys, Index, State> ChannelConfigurator<Async> for Channel<Evsys, Async, Index, State>
where
    Evsys: marker::Evsys,
    Index: marker::Index,
    State: marker::ChannelState,
{
    fn set_multiplexer(&mut self, multiplexer: u8) {
        unsafe { (*self.evsys.ptr()).set_async_user(Index::UX, multiplexer) }
    }

    fn set_generator(&mut self, generator: u8) {
        unsafe { (*self.evsys.ptr()).set_async_generator(self.index.index(), generator) }
    }
}

impl<Evsys, Index, State> ChannelConfigurator<Sync> for Channel<Evsys, Sync, Index, State>
where
    Evsys: marker::Evsys,
    Index: marker::Index,
    State: marker::ChannelState,
{
    fn set_multiplexer(&mut self, multiplexer: u8) {
       unsafe { (*self.evsys.ptr()).set_sync_user(Index::UX, multiplexer) }
    }

    fn set_generator(&mut self, generator: u8) {
        unsafe { (*self.evsys.ptr()).set_sync_generator(self.index.index(), generator) }
    }
}

pub trait EventGenerator<Evsys, Flavor, Index>
where
    Evsys: marker::Evsys,
    Flavor: marker::ChannelFlavor,
    Index: marker::Index,
{
    type EventSource;

    fn connect_event_generator(&mut self, channel: Channel<Evsys, Flavor, Index, Unconfigured>, source: Self::EventSource) -> Channel<Evsys, Flavor, Index, GeneratorAssigned>;
}

evsys!({
    channels: [
        {
            channel: 0,
            register: ASYNCCH0,
            userindex: 3,
            flavor: Async,
            // generators: {
            //     OFF             => 0x00,
            //     CCL_LUT0        => 0x01,
            //     CCL_LUT1        => 0x02,
            //     AC0_OUT         => 0x03,
            //     TCD0_CMPBCLR    => 0x04,
            //     TCD0_CMPASET    => 0x05,
            //     TCD0_CMPBSET    => 0x06,
            //     TCD0_PROGEV     => 0x07,
            //     RTC_OVF         => 0x08,
            //     RTC_CMP         => 0x09,
            //     PORTA_PIN0      => 0x0A,
            //     PORTA_PIN1      => 0x0B,
            //     PORTA_PIN2      => 0x0C,
            //     PORTA_PIN3      => 0x0D,
            //     PORTA_PIN4      => 0x0E,
            //     PORTA_PIN5      => 0x0F,
            //     PORTA_PIN6      => 0x10,
            //     PORTA_PIN7      => 0x11,
            //     UPDI            => 0x12,
            // }
        },
        {
            channel: 1,
            register: ASYNCCH1,
            userindex: 4,
            flavor: Async,
            // generators: {
            //     OFF             => 0,
            //     CCL_LUT0        => 0x01,
            //     CCL_LUT1        => 0x02,
            //     AC0_OUT         => 0x03,
            //     TCD0_CMPBCLR    => 0x04,
            //     TCD0_CMPASET    => 0x05,
            //     TCD0_CMPBSET    => 0x06,
            //     TCD0_PROGEV     => 0x07,
            //     RTC_OVF         => 0x08,
            //     RTC_CMP         => 0x09,
            //     PORTB_PIN0      => 0x0A,
            //     PORTB_PIN1      => 0x0B,
            //     PORTB_PIN2      => 0x0C,
            //     PORTB_PIN3      => 0x0D,
            //     PORTB_PIN4      => 0x0E,
            //     PORTB_PIN5      => 0x0F,
            //     PORTB_PIN6      => 0x10,
            //     PORTB_PIN7      => 0x11,
            // }
        },
        {
            channel: 2,
            register: ASYNCCH2,
            userindex: 5,
            flavor: Async,
            // generators: {
            //     OFF             => 0,
            //     CCL_LUT0        => 0x01,
            //     CCL_LUT1        => 0x02,
            //     AC0_OUT         => 0x03,
            //     TCD0_CMPBCLR    => 0x04,
            //     TCD0_CMPASET    => 0x05,
            //     TCD0_CMPBSET    => 0x06,
            //     TCD0_PROGEV     => 0x07,
            //     RTC_OVF         => 0x08,
            //     RTC_CMP         => 0x09,
            //     PORTC_PIN0      => 0x0A,
            //     PORTC_PIN1      => 0x0B,
            //     PORTC_PIN2      => 0x0C,
            //     PORTC_PIN3      => 0x0D,
            //     PORTC_PIN4      => 0x0E,
            //     PORTC_PIN5      => 0x0F,
            // }
        },
        {
            channel: 3,
            register: ASYNCCH3,
            userindex: 6,
            flavor: Async,
            // generators: {
            //     OFF             => 0,
            //     CCL_LUT0        => 0x01,
            //     CCL_LUT1        => 0x02,
            //     AC0_OUT         => 0x03,
            //     TCD0_CMPBCLR    => 0x04,
            //     TCD0_CMPASET    => 0x05,
            //     TCD0_CMPBSET    => 0x06,
            //     TCD0_PROGEV     => 0x07,
            //     RTC_OVF         => 0x08,
            //     RTC_CMP         => 0x09,
            //     PIT_DIV8192     => 0x0A,
            //     PIT_DIV4096     => 0x0B,
            //     PIT_DIV2048     => 0x0C,
            //     PIT_DIV1024     => 0x0D,
            //     PIT_DIV512      => 0x0E,
            //     PIT_DIV256      => 0x0F,
            //     PIT_DIV128      => 0x10,
            //     PIT_DIV64       => 0x11,
            // }
        },
        {
            channel: 0,
            register: SYNCCH0,
            userindex: 1,
            flavor: Sync,
            // generators: {
            //     OFF             => 0,
            //     TCB0            => 0x01,
            //     TCA0_OVF_LUNF   => 0x02,
            //     TCA0_HUNF       => 0x03,
            //     TCA0_CMP0       => 0x04,
            //     TCA0_CMP1       => 0x05,
            //     TCA0_CMP2       => 0x06,
            //     PORTC_PIN0      => 0x07,
            //     PORTC_PIN1      => 0x08,
            //     PORTC_PIN2      => 0x09,
            //     PORTC_PIN3      => 0x0A,
            //     PORTC_PIN4      => 0x0B,
            //     PORTC_PIN5      => 0x0C,
            //     PORTA_PIN0      => 0x0D,
            //     PORTA_PIN1      => 0x0E,
            //     PORTA_PIN2      => 0x0F,
            //     PORTA_PIN3      => 0x10,
            //     PORTA_PIN4      => 0x11,
            //     PORTA_PIN5      => 0x12,
            //     PORTA_PIN6      => 0x13,
            //     PORTA_PIN7      => 0x14,
            // }
        },
        {
            channel: 1,
            register: SYNCCH1,
            userindex: 2,
            flavor: Sync,
            // generators: {
            //     OFF             => 0,
            //     TCB0            => 0x01,
            //     TCA0_OVF_LUNF   => 0x02,
            //     TCA0_HUNF       => 0x03,
            //     TCA0_CMP0       => 0x04,
            //     TCA0_CMP1       => 0x05,
            //     TCA0_CMP2       => 0x06,
            //     PORTB_PIN0      => 0x08,
            //     PORTB_PIN1      => 0x09,
            //     PORTB_PIN2      => 0x0A,
            //     PORTB_PIN3      => 0x0B,
            //     PORTB_PIN4      => 0x0C,
            //     PORTB_PIN5      => 0x0D,
            //     PORTB_PIN6      => 0x0E,
            //     PORTB_PIN7      => 0x0F,
            // }
        },
    ],
});
