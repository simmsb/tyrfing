# Flashlight firmware, written in rust

This is flashlight firmware for the lume1-6af (attiny1616), written in Rust.

The UI is implemented using async rust, which makes adding new modes of operation fairly trivial.

The UI is (read: will be) inspired by the
[Andúril](https://github.com/ToyKeeper/anduril) firmware

## Thanks

This project relies on and makes modifications to:

- [Embassy](https://github.com/embassy-rs/embassy)
- [atxtiny-hal](https://github.com/G33KatWork/atxtiny-hal)
- [avr-device](https://github.com/Rahix/avr-device)

## EMBASSY on an attiny1616

This is some messing around with embassy on AVR (attiny1616)

The time driver ticks at 64hz and I've modified embassy to use a u32 tick
counter, which at 64hz won't overflow for just enough time for this to be useful
to me
