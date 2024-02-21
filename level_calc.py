from dataclasses import dataclass
from enum import Enum
from typing import List 
import itertools
from pprint import pprint

class RefVoltage(Enum):
    v0_5 = "ReferenceVoltage::_0V55"
    v1_1 = "ReferenceVoltage::_1V10"
    v1_5 = "ReferenceVoltage::_1V50"
    v2_5 = "ReferenceVoltage::_2V50"

    def voltage(self):
        match self:
            case RefVoltage.v0_5:
                return 0.55
            case RefVoltage.v1_1:
                return 1.1
            case RefVoltage.v1_5:
                return 1.5
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

@dataclass
class Level:
    ref: RefVoltage
    path_level: PathLevel
    adc_level: int

    def calc_output(self) -> float:
        adc_out = (self.adc_level / 255.0) * self.ref.voltage()
        return adc_out * self.path_level.output_scale()

def closest_level(levels: List[Level], desired: float) -> Level:
    return min(levels, key=lambda l: abs(l.calc_output() - desired))

def calc_levels() -> List[Level]:
    levels = [Level(ref, path_level, adc_level) for ref, path_level, adc_level in itertools.product(
        RefVoltage,
        PathLevel,
        range(256)
    )]

    highest_output = max(levels, key=lambda l: l.calc_output()).calc_output()

    factor = 4

    return [closest_level(levels, highest_output * ((x / 254) ** factor)) for x in range(255)]

pprint(calc_levels())

print("[" + ", ".join([f"Level::new({l.ref.value}, {l.path_level.value}, {l.adc_level})" for l in calc_levels()]) + "]")
