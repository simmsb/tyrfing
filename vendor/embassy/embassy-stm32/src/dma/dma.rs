use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::sync::atomic::{fence, AtomicUsize, Ordering};
use core::task::{Context, Poll, Waker};

use embassy_hal_internal::{into_ref, Peripheral, PeripheralRef};
use embassy_sync::waitqueue::AtomicWaker;

use super::ringbuffer::{DmaCtrl, OverrunError, ReadableDmaRingBuffer, WritableDmaRingBuffer};
use super::word::{Word, WordSize};
use super::Dir;
use crate::_generated::DMA_CHANNEL_COUNT;
use crate::interrupt::typelevel::Interrupt;
use crate::interrupt::Priority;
use crate::pac::dma::{regs, vals};
use crate::{interrupt, pac};

/// DMA transfer options.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct TransferOptions {
    /// Peripheral burst transfer configuration
    pub pburst: Burst,
    /// Memory burst transfer configuration
    pub mburst: Burst,
    /// Flow control configuration
    pub flow_ctrl: FlowControl,
    /// FIFO threshold for DMA FIFO mode. If none, direct mode is used.
    pub fifo_threshold: Option<FifoThreshold>,
    /// Enable circular DMA
    ///
    /// Note:
    /// If you enable circular mode manually, you may want to build and `.await` the `Transfer` in a separate task.
    /// Since DMA in circular mode need manually stop, `.await` in current task would block the task forever.
    pub circular: bool,
    /// Enable half transfer interrupt
    pub half_transfer_ir: bool,
    /// Enable transfer complete interrupt
    pub complete_transfer_ir: bool,
}

impl Default for TransferOptions {
    fn default() -> Self {
        Self {
            pburst: Burst::Single,
            mburst: Burst::Single,
            flow_ctrl: FlowControl::Dma,
            fifo_threshold: None,
            circular: false,
            half_transfer_ir: false,
            complete_transfer_ir: true,
        }
    }
}

impl From<WordSize> for vals::Size {
    fn from(raw: WordSize) -> Self {
        match raw {
            WordSize::OneByte => Self::BITS8,
            WordSize::TwoBytes => Self::BITS16,
            WordSize::FourBytes => Self::BITS32,
        }
    }
}

impl From<Dir> for vals::Dir {
    fn from(raw: Dir) -> Self {
        match raw {
            Dir::MemoryToPeripheral => Self::MEMORYTOPERIPHERAL,
            Dir::PeripheralToMemory => Self::PERIPHERALTOMEMORY,
        }
    }
}

/// DMA transfer burst setting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Burst {
    /// Single transfer
    Single,
    /// Incremental burst of 4 beats
    Incr4,
    /// Incremental burst of 8 beats
    Incr8,
    /// Incremental burst of 16 beats
    Incr16,
}

impl From<Burst> for vals::Burst {
    fn from(burst: Burst) -> Self {
        match burst {
            Burst::Single => vals::Burst::SINGLE,
            Burst::Incr4 => vals::Burst::INCR4,
            Burst::Incr8 => vals::Burst::INCR8,
            Burst::Incr16 => vals::Burst::INCR16,
        }
    }
}

/// DMA flow control setting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlowControl {
    /// Flow control by DMA
    Dma,
    /// Flow control by peripheral
    Peripheral,
}

impl From<FlowControl> for vals::Pfctrl {
    fn from(flow: FlowControl) -> Self {
        match flow {
            FlowControl::Dma => vals::Pfctrl::DMA,
            FlowControl::Peripheral => vals::Pfctrl::PERIPHERAL,
        }
    }
}

/// DMA FIFO threshold.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoThreshold {
    /// 1/4 full FIFO
    Quarter,
    /// 1/2 full FIFO
    Half,
    /// 3/4 full FIFO
    ThreeQuarters,
    /// Full FIFO
    Full,
}

impl From<FifoThreshold> for vals::Fth {
    fn from(value: FifoThreshold) -> Self {
        match value {
            FifoThreshold::Quarter => vals::Fth::QUARTER,
            FifoThreshold::Half => vals::Fth::HALF,
            FifoThreshold::ThreeQuarters => vals::Fth::THREEQUARTERS,
            FifoThreshold::Full => vals::Fth::FULL,
        }
    }
}

struct State {
    ch_wakers: [AtomicWaker; DMA_CHANNEL_COUNT],
    complete_count: [AtomicUsize; DMA_CHANNEL_COUNT],
}

impl State {
    const fn new() -> Self {
        const ZERO: AtomicUsize = AtomicUsize::new(0);
        const AW: AtomicWaker = AtomicWaker::new();
        Self {
            ch_wakers: [AW; DMA_CHANNEL_COUNT],
            complete_count: [ZERO; DMA_CHANNEL_COUNT],
        }
    }
}

static STATE: State = State::new();

/// safety: must be called only once
pub(crate) unsafe fn init(cs: critical_section::CriticalSection, irq_priority: Priority) {
    foreach_interrupt! {
        ($peri:ident, dma, $block:ident, $signal_name:ident, $irq:ident) => {
            interrupt::typelevel::$irq::set_priority_with_cs(cs, irq_priority);
            interrupt::typelevel::$irq::enable();
        };
    }
    crate::_generated::init_dma();
}

foreach_dma_channel! {
    ($channel_peri:ident, $dma_peri:ident, dma, $channel_num:expr, $index:expr, $dmamux:tt) => {
        impl sealed::Channel for crate::peripherals::$channel_peri {
            fn regs(&self) -> pac::dma::Dma {
                pac::$dma_peri
            }
            fn num(&self) -> usize {
                $channel_num
            }
            fn index(&self) -> usize {
                $index
            }
            fn on_irq() {
                unsafe { on_irq_inner(pac::$dma_peri, $channel_num, $index) }
            }
        }

        impl Channel for crate::peripherals::$channel_peri {}
    };
}

/// Safety: Must be called with a matching set of parameters for a valid dma channel
pub(crate) unsafe fn on_irq_inner(dma: pac::dma::Dma, channel_num: usize, index: usize) {
    let cr = dma.st(channel_num).cr();
    let isr = dma.isr(channel_num / 4).read();

    if isr.teif(channel_num % 4) {
        panic!("DMA: error on DMA@{:08x} channel {}", dma.as_ptr() as u32, channel_num);
    }

    if isr.htif(channel_num % 4) && cr.read().htie() {
        // Acknowledge half transfer complete interrupt
        dma.ifcr(channel_num / 4).write(|w| w.set_htif(channel_num % 4, true));
    } else if isr.tcif(channel_num % 4) && cr.read().tcie() {
        // Acknowledge  transfer complete interrupt
        dma.ifcr(channel_num / 4).write(|w| w.set_tcif(channel_num % 4, true));
        STATE.complete_count[index].fetch_add(1, Ordering::Release);
    } else {
        return;
    }

    STATE.ch_wakers[index].wake();
}

/// DMA request type alias. (also known as DMA channel number in some chips)
#[cfg(any(dma_v2, dmamux))]
pub type Request = u8;
/// DMA request type alias. (also known as DMA channel number in some chips)
#[cfg(not(any(dma_v2, dmamux)))]
pub type Request = ();

/// DMA channel.
#[cfg(dmamux)]
pub trait Channel: sealed::Channel + Peripheral<P = Self> + 'static + super::dmamux::MuxChannel {}
/// DMA channel.
#[cfg(not(dmamux))]
pub trait Channel: sealed::Channel + Peripheral<P = Self> + 'static {}

pub(crate) mod sealed {
    use super::*;

    pub trait Channel {
        fn regs(&self) -> pac::dma::Dma;
        fn num(&self) -> usize;
        fn index(&self) -> usize;
        fn on_irq();
    }
}

/// DMA transfer.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Transfer<'a, C: Channel> {
    channel: PeripheralRef<'a, C>,
}

impl<'a, C: Channel> Transfer<'a, C> {
    /// Create a new read DMA transfer (peripheral to memory).
    pub unsafe fn new_read<W: Word>(
        channel: impl Peripheral<P = C> + 'a,
        request: Request,
        peri_addr: *mut W,
        buf: &'a mut [W],
        options: TransferOptions,
    ) -> Self {
        Self::new_read_raw(channel, request, peri_addr, buf, options)
    }

    /// Create a new read DMA transfer (peripheral to memory), using raw pointers.
    pub unsafe fn new_read_raw<W: Word>(
        channel: impl Peripheral<P = C> + 'a,
        request: Request,
        peri_addr: *mut W,
        buf: *mut [W],
        options: TransferOptions,
    ) -> Self {
        into_ref!(channel);

        let (ptr, len) = super::slice_ptr_parts_mut(buf);
        assert!(len > 0 && len <= 0xFFFF);

        Self::new_inner(
            channel,
            request,
            Dir::PeripheralToMemory,
            peri_addr as *const u32,
            ptr as *mut u32,
            len,
            true,
            W::size(),
            options,
        )
    }

    /// Create a new write DMA transfer (memory to peripheral).
    pub unsafe fn new_write<W: Word>(
        channel: impl Peripheral<P = C> + 'a,
        request: Request,
        buf: &'a [W],
        peri_addr: *mut W,
        options: TransferOptions,
    ) -> Self {
        Self::new_write_raw(channel, request, buf, peri_addr, options)
    }

    /// Create a new write DMA transfer (memory to peripheral), using raw pointers.
    pub unsafe fn new_write_raw<W: Word>(
        channel: impl Peripheral<P = C> + 'a,
        request: Request,
        buf: *const [W],
        peri_addr: *mut W,
        options: TransferOptions,
    ) -> Self {
        into_ref!(channel);

        let (ptr, len) = super::slice_ptr_parts(buf);
        assert!(len > 0 && len <= 0xFFFF);

        Self::new_inner(
            channel,
            request,
            Dir::MemoryToPeripheral,
            peri_addr as *const u32,
            ptr as *mut u32,
            len,
            true,
            W::size(),
            options,
        )
    }

    /// Create a new write DMA transfer (memory to peripheral), writing the same value repeatedly.
    pub unsafe fn new_write_repeated<W: Word>(
        channel: impl Peripheral<P = C> + 'a,
        request: Request,
        repeated: &'a W,
        count: usize,
        peri_addr: *mut W,
        options: TransferOptions,
    ) -> Self {
        into_ref!(channel);

        Self::new_inner(
            channel,
            request,
            Dir::MemoryToPeripheral,
            peri_addr as *const u32,
            repeated as *const W as *mut u32,
            count,
            false,
            W::size(),
            options,
        )
    }

    unsafe fn new_inner(
        channel: PeripheralRef<'a, C>,
        _request: Request,
        dir: Dir,
        peri_addr: *const u32,
        mem_addr: *mut u32,
        mem_len: usize,
        incr_mem: bool,
        data_size: WordSize,
        options: TransferOptions,
    ) -> Self {
        let ch = channel.regs().st(channel.num());

        // "Preceding reads and writes cannot be moved past subsequent writes."
        fence(Ordering::SeqCst);

        let mut this = Self { channel };
        this.clear_irqs();

        #[cfg(dmamux)]
        super::dmamux::configure_dmamux(&mut *this.channel, _request);

        ch.par().write_value(peri_addr as u32);
        ch.m0ar().write_value(mem_addr as u32);
        ch.ndtr().write_value(regs::Ndtr(mem_len as _));
        ch.fcr().write(|w| {
            if let Some(fth) = options.fifo_threshold {
                // FIFO mode
                w.set_dmdis(vals::Dmdis::DISABLED);
                w.set_fth(fth.into());
            } else {
                // Direct mode
                w.set_dmdis(vals::Dmdis::ENABLED);
            }
        });
        ch.cr().write(|w| {
            w.set_dir(dir.into());
            w.set_msize(data_size.into());
            w.set_psize(data_size.into());
            w.set_pl(vals::Pl::VERYHIGH);
            w.set_minc(incr_mem);
            w.set_pinc(false);
            w.set_teie(true);
            w.set_tcie(options.complete_transfer_ir);
            w.set_circ(options.circular);
            if options.circular {
                debug!("Setting circular mode");
            }
            #[cfg(dma_v1)]
            w.set_trbuff(true);

            #[cfg(dma_v2)]
            w.set_chsel(_request);

            w.set_pburst(options.pburst.into());
            w.set_mburst(options.mburst.into());
            w.set_pfctrl(options.flow_ctrl.into());

            w.set_en(true);
        });

        this
    }

    fn clear_irqs(&mut self) {
        let isrn = self.channel.num() / 4;
        let isrbit = self.channel.num() % 4;

        self.channel.regs().ifcr(isrn).write(|w| {
            w.set_tcif(isrbit, true);
            w.set_teif(isrbit, true);
        });
    }

    /// Request the transfer to stop.
    ///
    /// This doesn't immediately stop the transfer, you have to wait until [`is_running`](Self::is_running) returns false.
    pub fn request_stop(&mut self) {
        let ch = self.channel.regs().st(self.channel.num());

        // Disable the channel. Keep the IEs enabled so the irqs still fire.
        ch.cr().write(|w| {
            w.set_teie(true);
            w.set_tcie(true);
        });
    }

    /// Return whether this transfer is still running.
    ///
    /// If this returns `false`, it can be because either the transfer finished, or
    /// it was requested to stop early with [`request_stop`](Self::request_stop).
    pub fn is_running(&mut self) -> bool {
        let ch = self.channel.regs().st(self.channel.num());
        ch.cr().read().en()
    }

    /// Gets the total remaining transfers for the channel
    /// Note: this will be zero for transfers that completed without cancellation.
    pub fn get_remaining_transfers(&self) -> u16 {
        let ch = self.channel.regs().st(self.channel.num());
        ch.ndtr().read().ndt()
    }

    /// Blocking wait until the transfer finishes.
    pub fn blocking_wait(mut self) {
        while self.is_running() {}

        // "Subsequent reads and writes cannot be moved ahead of preceding reads."
        fence(Ordering::SeqCst);

        core::mem::forget(self);
    }
}

impl<'a, C: Channel> Drop for Transfer<'a, C> {
    fn drop(&mut self) {
        self.request_stop();
        while self.is_running() {}

        // "Subsequent reads and writes cannot be moved ahead of preceding reads."
        fence(Ordering::SeqCst);
    }
}

impl<'a, C: Channel> Unpin for Transfer<'a, C> {}
impl<'a, C: Channel> Future for Transfer<'a, C> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        STATE.ch_wakers[self.channel.index()].register(cx.waker());

        if self.is_running() {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

// ==================================

/// Double-buffered DMA transfer.
pub struct DoubleBuffered<'a, C: Channel, W: Word> {
    channel: PeripheralRef<'a, C>,
    _phantom: PhantomData<W>,
}

impl<'a, C: Channel, W: Word> DoubleBuffered<'a, C, W> {
    /// Create a new read DMA transfer (peripheral to memory).
    pub unsafe fn new_read(
        channel: impl Peripheral<P = C> + 'a,
        _request: Request,
        peri_addr: *mut W,
        buf0: *mut W,
        buf1: *mut W,
        len: usize,
        options: TransferOptions,
    ) -> Self {
        into_ref!(channel);
        assert!(len > 0 && len <= 0xFFFF);

        let dir = Dir::PeripheralToMemory;
        let data_size = W::size();

        let channel_number = channel.num();
        let dma = channel.regs();

        // "Preceding reads and writes cannot be moved past subsequent writes."
        fence(Ordering::SeqCst);

        let mut this = Self {
            channel,
            _phantom: PhantomData,
        };
        this.clear_irqs();

        #[cfg(dmamux)]
        super::dmamux::configure_dmamux(&mut *this.channel, _request);

        let ch = dma.st(channel_number);
        ch.par().write_value(peri_addr as u32);
        ch.m0ar().write_value(buf0 as u32);
        ch.m1ar().write_value(buf1 as u32);
        ch.ndtr().write_value(regs::Ndtr(len as _));
        ch.fcr().write(|w| {
            if let Some(fth) = options.fifo_threshold {
                // FIFO mode
                w.set_dmdis(vals::Dmdis::DISABLED);
                w.set_fth(fth.into());
            } else {
                // Direct mode
                w.set_dmdis(vals::Dmdis::ENABLED);
            }
        });
        ch.cr().write(|w| {
            w.set_dir(dir.into());
            w.set_msize(data_size.into());
            w.set_psize(data_size.into());
            w.set_pl(vals::Pl::VERYHIGH);
            w.set_minc(true);
            w.set_pinc(false);
            w.set_teie(true);
            w.set_tcie(true);
            #[cfg(dma_v1)]
            w.set_trbuff(true);

            #[cfg(dma_v2)]
            w.set_chsel(_request);

            w.set_pburst(options.pburst.into());
            w.set_mburst(options.mburst.into());
            w.set_pfctrl(options.flow_ctrl.into());

            w.set_en(true);
        });

        this
    }

    fn clear_irqs(&mut self) {
        let channel_number = self.channel.num();
        let dma = self.channel.regs();
        let isrn = channel_number / 4;
        let isrbit = channel_number % 4;

        dma.ifcr(isrn).write(|w| {
            w.set_htif(isrbit, true);
            w.set_tcif(isrbit, true);
            w.set_teif(isrbit, true);
        });
    }

    /// Set the first buffer address.
    ///
    /// You may call this while DMA is transferring the other buffer.
    pub unsafe fn set_buffer0(&mut self, buffer: *mut W) {
        let ch = self.channel.regs().st(self.channel.num());
        ch.m0ar().write_value(buffer as _);
    }

    /// Set the second buffer address.
    ///
    /// You may call this while DMA is transferring the other buffer.
    pub unsafe fn set_buffer1(&mut self, buffer: *mut W) {
        let ch = self.channel.regs().st(self.channel.num());
        ch.m1ar().write_value(buffer as _);
    }

    /// Returh whether buffer0 is accessible (i.e. whether DMA is transferring buffer1 now)
    pub fn is_buffer0_accessible(&mut self) -> bool {
        let ch = self.channel.regs().st(self.channel.num());
        ch.cr().read().ct() == vals::Ct::MEMORY1
    }

    /// Set a waker to be woken when one of the buffers is being transferred.
    pub fn set_waker(&mut self, waker: &Waker) {
        STATE.ch_wakers[self.channel.index()].register(waker);
    }

    /// Request the transfer to stop.
    ///
    /// This doesn't immediately stop the transfer, you have to wait until [`is_running`](Self::is_running) returns false.
    pub fn request_stop(&mut self) {
        let ch = self.channel.regs().st(self.channel.num());

        // Disable the channel. Keep the IEs enabled so the irqs still fire.
        ch.cr().write(|w| {
            w.set_teie(true);
            w.set_tcie(true);
        });
    }

    /// Return whether this transfer is still running.
    ///
    /// If this returns `false`, it can be because either the transfer finished, or
    /// it was requested to stop early with [`request_stop`](Self::request_stop).
    pub fn is_running(&mut self) -> bool {
        let ch = self.channel.regs().st(self.channel.num());
        ch.cr().read().en()
    }

    /// Gets the total remaining transfers for the channel
    /// Note: this will be zero for transfers that completed without cancellation.
    pub fn get_remaining_transfers(&self) -> u16 {
        let ch = self.channel.regs().st(self.channel.num());
        ch.ndtr().read().ndt()
    }
}

impl<'a, C: Channel, W: Word> Drop for DoubleBuffered<'a, C, W> {
    fn drop(&mut self) {
        self.request_stop();
        while self.is_running() {}

        // "Subsequent reads and writes cannot be moved ahead of preceding reads."
        fence(Ordering::SeqCst);
    }
}

// ==============================

struct DmaCtrlImpl<'a, C: Channel>(PeripheralRef<'a, C>);

impl<'a, C: Channel> DmaCtrl for DmaCtrlImpl<'a, C> {
    fn get_remaining_transfers(&self) -> usize {
        let ch = self.0.regs().st(self.0.num());
        ch.ndtr().read().ndt() as usize
    }

    fn get_complete_count(&self) -> usize {
        STATE.complete_count[self.0.index()].load(Ordering::Acquire)
    }

    fn reset_complete_count(&mut self) -> usize {
        STATE.complete_count[self.0.index()].swap(0, Ordering::AcqRel)
    }

    fn set_waker(&mut self, waker: &Waker) {
        STATE.ch_wakers[self.0.index()].register(waker);
    }
}

/// Ringbuffer for receiving data using DMA circular mode.
pub struct ReadableRingBuffer<'a, C: Channel, W: Word> {
    cr: regs::Cr,
    channel: PeripheralRef<'a, C>,
    ringbuf: ReadableDmaRingBuffer<'a, W>,
}

impl<'a, C: Channel, W: Word> ReadableRingBuffer<'a, C, W> {
    /// Create a new ring buffer.
    pub unsafe fn new(
        channel: impl Peripheral<P = C> + 'a,
        _request: Request,
        peri_addr: *mut W,
        buffer: &'a mut [W],
        options: TransferOptions,
    ) -> Self {
        into_ref!(channel);

        let len = buffer.len();
        assert!(len > 0 && len <= 0xFFFF);

        let dir = Dir::PeripheralToMemory;
        let data_size = W::size();

        let channel_number = channel.num();
        let dma = channel.regs();

        // "Preceding reads and writes cannot be moved past subsequent writes."
        fence(Ordering::SeqCst);

        let mut w = regs::Cr(0);
        w.set_dir(dir.into());
        w.set_msize(data_size.into());
        w.set_psize(data_size.into());
        w.set_pl(vals::Pl::VERYHIGH);
        w.set_minc(true);
        w.set_pinc(false);
        w.set_teie(true);
        w.set_htie(options.half_transfer_ir);
        w.set_tcie(true);
        w.set_circ(true);
        #[cfg(dma_v1)]
        w.set_trbuff(true);
        #[cfg(dma_v2)]
        w.set_chsel(_request);
        w.set_pburst(options.pburst.into());
        w.set_mburst(options.mburst.into());
        w.set_pfctrl(options.flow_ctrl.into());
        w.set_en(true);

        let buffer_ptr = buffer.as_mut_ptr();
        let mut this = Self {
            channel,
            cr: w,
            ringbuf: ReadableDmaRingBuffer::new(buffer),
        };
        this.clear_irqs();

        #[cfg(dmamux)]
        super::dmamux::configure_dmamux(&mut *this.channel, _request);

        let ch = dma.st(channel_number);
        ch.par().write_value(peri_addr as u32);
        ch.m0ar().write_value(buffer_ptr as u32);
        ch.ndtr().write_value(regs::Ndtr(len as _));
        ch.fcr().write(|w| {
            if let Some(fth) = options.fifo_threshold {
                // FIFO mode
                w.set_dmdis(vals::Dmdis::DISABLED);
                w.set_fth(fth.into());
            } else {
                // Direct mode
                w.set_dmdis(vals::Dmdis::ENABLED);
            }
        });

        this
    }

    /// Start the ring buffer operation.
    ///
    /// You must call this after creating it for it to work.
    pub fn start(&mut self) {
        let ch = self.channel.regs().st(self.channel.num());
        ch.cr().write_value(self.cr);
    }

    /// Clear all data in the ring buffer.
    pub fn clear(&mut self) {
        self.ringbuf.clear(&mut DmaCtrlImpl(self.channel.reborrow()));
    }

    /// Read elements from the ring buffer
    /// Return a tuple of the length read and the length remaining in the buffer
    /// If not all of the elements were read, then there will be some elements in the buffer remaining
    /// The length remaining is the capacity, ring_buf.len(), less the elements remaining after the read
    /// OverrunError is returned if the portion to be read was overwritten by the DMA controller.
    pub fn read(&mut self, buf: &mut [W]) -> Result<(usize, usize), OverrunError> {
        self.ringbuf.read(&mut DmaCtrlImpl(self.channel.reborrow()), buf)
    }

    /// Read an exact number of elements from the ringbuffer.
    ///
    /// Returns the remaining number of elements available for immediate reading.
    /// OverrunError is returned if the portion to be read was overwritten by the DMA controller.
    ///
    /// Async/Wake Behavior:
    /// The underlying DMA peripheral only can wake us when its buffer pointer has reached the halfway point,
    /// and when it wraps around. This means that when called with a buffer of length 'M', when this
    /// ring buffer was created with a buffer of size 'N':
    /// - If M equals N/2 or N/2 divides evenly into M, this function will return every N/2 elements read on the DMA source.
    /// - Otherwise, this function may need up to N/2 extra elements to arrive before returning.
    pub async fn read_exact(&mut self, buffer: &mut [W]) -> Result<usize, OverrunError> {
        self.ringbuf
            .read_exact(&mut DmaCtrlImpl(self.channel.reborrow()), buffer)
            .await
    }

    /// The capacity of the ringbuffer
    pub const fn capacity(&self) -> usize {
        self.ringbuf.cap()
    }

    /// Set a waker to be woken when at least one byte is received.
    pub fn set_waker(&mut self, waker: &Waker) {
        DmaCtrlImpl(self.channel.reborrow()).set_waker(waker);
    }

    fn clear_irqs(&mut self) {
        let channel_number = self.channel.num();
        let dma = self.channel.regs();
        let isrn = channel_number / 4;
        let isrbit = channel_number % 4;

        dma.ifcr(isrn).write(|w| {
            w.set_htif(isrbit, true);
            w.set_tcif(isrbit, true);
            w.set_teif(isrbit, true);
        });
    }

    /// Request DMA to stop.
    ///
    /// This doesn't immediately stop the transfer, you have to wait until [`is_running`](Self::is_running) returns false.
    pub fn request_stop(&mut self) {
        let ch = self.channel.regs().st(self.channel.num());

        // Disable the channel. Keep the IEs enabled so the irqs still fire.
        ch.cr().write(|w| {
            w.set_teie(true);
            w.set_htie(true);
            w.set_tcie(true);
        });
    }

    /// Return whether DMA is still running.
    ///
    /// If this returns `false`, it can be because either the transfer finished, or
    /// it was requested to stop early with [`request_stop`](Self::request_stop).
    pub fn is_running(&mut self) -> bool {
        let ch = self.channel.regs().st(self.channel.num());
        ch.cr().read().en()
    }
}

impl<'a, C: Channel, W: Word> Drop for ReadableRingBuffer<'a, C, W> {
    fn drop(&mut self) {
        self.request_stop();
        while self.is_running() {}

        // "Subsequent reads and writes cannot be moved ahead of preceding reads."
        fence(Ordering::SeqCst);
    }
}

/// Ringbuffer for writing data using DMA circular mode.
pub struct WritableRingBuffer<'a, C: Channel, W: Word> {
    cr: regs::Cr,
    channel: PeripheralRef<'a, C>,
    ringbuf: WritableDmaRingBuffer<'a, W>,
}

impl<'a, C: Channel, W: Word> WritableRingBuffer<'a, C, W> {
    /// Create a new ring buffer.
    pub unsafe fn new(
        channel: impl Peripheral<P = C> + 'a,
        _request: Request,
        peri_addr: *mut W,
        buffer: &'a mut [W],
        options: TransferOptions,
    ) -> Self {
        into_ref!(channel);

        let len = buffer.len();
        assert!(len > 0 && len <= 0xFFFF);

        let dir = Dir::MemoryToPeripheral;
        let data_size = W::size();

        let channel_number = channel.num();
        let dma = channel.regs();

        // "Preceding reads and writes cannot be moved past subsequent writes."
        fence(Ordering::SeqCst);

        let mut w = regs::Cr(0);
        w.set_dir(dir.into());
        w.set_msize(data_size.into());
        w.set_psize(data_size.into());
        w.set_pl(vals::Pl::VERYHIGH);
        w.set_minc(true);
        w.set_pinc(false);
        w.set_teie(true);
        w.set_htie(options.half_transfer_ir);
        w.set_tcie(true);
        w.set_circ(true);
        #[cfg(dma_v1)]
        w.set_trbuff(true);
        #[cfg(dma_v2)]
        w.set_chsel(_request);
        w.set_pburst(options.pburst.into());
        w.set_mburst(options.mburst.into());
        w.set_pfctrl(options.flow_ctrl.into());
        w.set_en(true);

        let buffer_ptr = buffer.as_mut_ptr();
        let mut this = Self {
            channel,
            cr: w,
            ringbuf: WritableDmaRingBuffer::new(buffer),
        };
        this.clear_irqs();

        #[cfg(dmamux)]
        super::dmamux::configure_dmamux(&mut *this.channel, _request);

        let ch = dma.st(channel_number);
        ch.par().write_value(peri_addr as u32);
        ch.m0ar().write_value(buffer_ptr as u32);
        ch.ndtr().write_value(regs::Ndtr(len as _));
        ch.fcr().write(|w| {
            if let Some(fth) = options.fifo_threshold {
                // FIFO mode
                w.set_dmdis(vals::Dmdis::DISABLED);
                w.set_fth(fth.into());
            } else {
                // Direct mode
                w.set_dmdis(vals::Dmdis::ENABLED);
            }
        });

        this
    }

    /// Start the ring buffer operation.
    ///
    /// You must call this after creating it for it to work.
    pub fn start(&mut self) {
        let ch = self.channel.regs().st(self.channel.num());
        ch.cr().write_value(self.cr);
    }

    /// Clear all data in the ring buffer.
    pub fn clear(&mut self) {
        self.ringbuf.clear(&mut DmaCtrlImpl(self.channel.reborrow()));
    }

    /// Write elements from the ring buffer
    /// Return a tuple of the length written and the length remaining in the buffer
    pub fn write(&mut self, buf: &[W]) -> Result<(usize, usize), OverrunError> {
        self.ringbuf.write(&mut DmaCtrlImpl(self.channel.reborrow()), buf)
    }

    /// Write an exact number of elements to the ringbuffer.
    pub async fn write_exact(&mut self, buffer: &[W]) -> Result<usize, OverrunError> {
        self.ringbuf
            .write_exact(&mut DmaCtrlImpl(self.channel.reborrow()), buffer)
            .await
    }

    /// The capacity of the ringbuffer
    pub const fn capacity(&self) -> usize {
        self.ringbuf.cap()
    }

    /// Set a waker to be woken when at least one byte is received.
    pub fn set_waker(&mut self, waker: &Waker) {
        DmaCtrlImpl(self.channel.reborrow()).set_waker(waker);
    }

    fn clear_irqs(&mut self) {
        let channel_number = self.channel.num();
        let dma = self.channel.regs();
        let isrn = channel_number / 4;
        let isrbit = channel_number % 4;

        dma.ifcr(isrn).write(|w| {
            w.set_htif(isrbit, true);
            w.set_tcif(isrbit, true);
            w.set_teif(isrbit, true);
        });
    }

    /// Request DMA to stop.
    ///
    /// This doesn't immediately stop the transfer, you have to wait until [`is_running`](Self::is_running) returns false.
    pub fn request_stop(&mut self) {
        let ch = self.channel.regs().st(self.channel.num());

        // Disable the channel. Keep the IEs enabled so the irqs still fire.
        ch.cr().write(|w| {
            w.set_teie(true);
            w.set_htie(true);
            w.set_tcie(true);
        });
    }

    /// Return whether DMA is still running.
    ///
    /// If this returns `false`, it can be because either the transfer finished, or
    /// it was requested to stop early with [`request_stop`](Self::request_stop).
    pub fn is_running(&mut self) -> bool {
        let ch = self.channel.regs().st(self.channel.num());
        ch.cr().read().en()
    }
}

impl<'a, C: Channel, W: Word> Drop for WritableRingBuffer<'a, C, W> {
    fn drop(&mut self) {
        self.request_stop();
        while self.is_running() {}

        // "Subsequent reads and writes cannot be moved ahead of preceding reads."
        fence(Ordering::SeqCst);
    }
}
