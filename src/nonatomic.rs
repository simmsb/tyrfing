use core::cell::UnsafeCell;

// for global variables that are only accessed by coroutines, we can guarantee
// that access is serialized, and therefore don't need to have any atomic
// behaviour.
//
// However I don't have a good way to prevent use from interrupts, so this just
// has to be 'trust me bro'

macro_rules! non_atomic_impl {
    ($name:ident, $type:ty) => {
        ::paste::paste! {
            pub struct [<NonAtomic $name>](UnsafeCell<$type>);

            unsafe impl Sync for [<NonAtomic $name>] {}
            unsafe impl Send for [<NonAtomic $name>] {}

            impl [<NonAtomic $name>] {
                pub const fn new(value: $type) -> Self {
                    Self(UnsafeCell::new(value))
                }

                #[inline(always)]
                pub fn load(&self) -> $type {
                    unsafe { self.0.get().read() }
                }

                #[inline(always)]
                pub fn store(&self, value: $type) {
                    unsafe { self.0.get().write(value) }
                }

                #[inline(always)]
                pub fn replace(&self, value: $type) -> $type {
                    unsafe { self.0.get().replace(value) }
                }
            }
        }
    };
}

non_atomic_impl!(U8, u8);
non_atomic_impl!(U16, u16);
non_atomic_impl!(U32, u32);
non_atomic_impl!(I8, i8);
non_atomic_impl!(I16, i16);
non_atomic_impl!(I32, i32);
non_atomic_impl!(Bool, bool);
