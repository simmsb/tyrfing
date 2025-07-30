from dataclasses import dataclass
from enum import Enum
from typing import List 
import itertools
from pprint import pprint

class RefVoltage(Enum):
    v1_024 = "ReferenceVoltage::_1V024"
    v2_048 = "ReferenceVoltage::_2V048"
    v2_5 = "ReferenceVoltage::_2V50"
    # v4_3 = "ReferenceVoltage::_4V34"

    def voltage(self):
        match self:
            case RefVoltage.v1_024:
                return 1.024
            case RefVoltage.v2_048:
                return 2.048
            case RefVoltage.v2_5:
                return 2.5

class PathLevel(Enum):
    one = "PathLevel::One"
    two = "PathLevel::Two"
    three = "PathLevel::Three"

    def output_scale(self):
        match self:
            case PathLevel.one:
                return 1
            case PathLevel.two:
                return 100
            case PathLevel.three:
                return 100 * 100

adc_range = 2**10
num_levels = 256

@dataclass
class Level:
    ref: RefVoltage
    path_level: PathLevel
    adc_level: int

    def calc_output(self) -> float:
        adc_out = (self.adc_level / adc_range) * self.ref.voltage()
        return adc_out * self.path_level.output_scale()

def closest_level(levels: List[Level], desired: float) -> Level:
    return min(levels, key=lambda l: abs(l.calc_output() - desired))

def calc_levels() -> List[Level]:
    levels = [Level(ref, path_level, adc_level) for ref, path_level, adc_level in itertools.product(
        [RefVoltage.v1_024, RefVoltage.v2_048, RefVoltage.v2_5],
        PathLevel,
        range(adc_range)
        )]

    highest_output = max(levels, key=lambda l: l.calc_output()).calc_output()

    factor = 4

    return [closest_level(levels, highest_output * ((x / (num_levels - 1)) ** factor)) for x in range(num_levels)]

pprint(calc_levels())

print("[" + ", ".join([f"Level::new({l.ref.value}, {l.path_level.value}, {l.adc_level})" for l in calc_levels()]) + "]")
