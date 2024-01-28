# Embassy on an attiny1616

This is some messing around with embassy on AVR (attiny1616)

The time driver ticks at 64hz and I've modified embassy to use a u32 tick
counter, which at 64hz won't overflow for just enough time for this to be useful
to me
