DEV := "/dev/cu.usbserial-A50285BI"

flash:
  cargo build --release --no-default-features --features "space_saving default_modes"
  cargo objcopy -q --release --no-default-features --features "space_saving default_modes" -- -O ihex target/out.hex
  avrdude -c serialupdi -p avr32dd20 -P {{DEV}} -U flash:w:target/out.hex:i


flash_d:
  cargo build --release
  cargo objcopy -q --release -- -O ihex target/out.hex
  avrdude -c serialupdi -p t1616 -P {{DEV}} -U flash:w:target/out.hex:i
