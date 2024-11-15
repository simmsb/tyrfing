DEV := "/dev/cu.usbserial-A50285BI"

flash:
  cargo build --release --no-default-features --features "space_saving has_fet default_modes"
  cargo objcopy -q --release --no-default-features --features "space_saving has_fet default_modes" -- -O ihex target/out.hex
  pymcuprog -d attiny1616 -t uart -u {{DEV}} write -f target/out.hex --erase --verify

flash_nofet:
  cargo build --release --no-default-features --features "space_saving default_modes"
  cargo objcopy -q --release --no-default-features --features "space_saving default_modes" -- -O ihex target/out.hex
  pymcuprog -d attiny1616 -t uart -u {{DEV}} write -f target/out.hex --erase --verify

flash_d:
  cargo build --release
  cargo objcopy -q --release -- -O ihex target/out.hex
  pymcuprog -d attiny1616 -t uart -u {{DEV}} write -f target/out.hex --erase --verify
