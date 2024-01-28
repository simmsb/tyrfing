"""
    Just print every byte we receive on stdout and flush it on STOP

    glasgow script glasgow_i2c_terminal.py i2c-target --port B -V 3.3 --pin-scl 0 --pin-sda 1 -A 0x03
"""

import sys
from glasgow.applet.interface.i2c_target import I2CTargetInterface

class _PrintI2CTargetInterface(I2CTargetInterface):
    def __init__(self, *args, **kwargs):
        self.i = 1
        super().__init__(*args, **kwargs)

    async def on_start(self):
        sys.stdout.write("[START]")
        sys.stdout.flush()
        pass

    async def on_stop(self):
        sys.stdout.write("[STOP]\n")
        sys.stdout.flush()

    async def on_restart(self):
        sys.stdout.write("[RESTART]")
        sys.stdout.flush()
        pass

    async def on_write(self, data):
        sys.stdout.write(chr(data))
        return True

    async def on_read(self):
        sys.stdout.write("[READ]")
        sys.stdout.flush()

        #self.i += 1
        #self.i &= 0xFF
        #return self.i
        return 0xAA

iface = _PrintI2CTargetInterface(iface.lower, iface._logger)

while True:
    await iface.read_event()
