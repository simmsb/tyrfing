flash:
  cargo build --release --no-default-features --features "space_saving"
  cargo objcopy -q --release --no-default-features --features "space_saving" -- -O ihex target/out.hex
  pymcuprog -d attiny1616 -t uart -u /dev/cu.usbserial-A50285BI write -f target/out.hex --erase --verify

flash_d:
  cargo build --release
  cargo objcopy -q --release -- -O ihex target/out.hex
  pymcuprog -d attiny1616 -t uart -u /dev/cu.usbserial-A50285BI write -f target/out.hex --erase --verify
