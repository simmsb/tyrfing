use core::hint::unreachable_unchecked;

use crate::nonatomic::{NonAtomicBool, NonAtomicU8};
use atxtiny_hal::{
    dac::{Dac, Enabled},
    embedded_hal::digital::OutputPin,
    gpio::{Input, Output, Pin, Porta, Portb, Stateless, U},
    vref::{DACReferenceVoltage, ReferenceVoltage, VrefExt},
};
use avr_device::attiny1616::{DAC0, VREF};
use avr_hal_generic::prelude::*;

use crate::{
    adc::{Temperature, Voltage},
    sleep::Mug,
};

const NUM_LEVELS: usize = 256;

static DESIRED_LEVEL: NonAtomicU8 = NonAtomicU8::new(0);
static GRADUAL_LEVEL: NonAtomicU8 = NonAtomicU8::new(0);
static TORCH_IS_ON: NonAtomicBool = NonAtomicBool::new(false);

pub async fn blink(blinks: u8) {
    // TODO: calculate blink level off current level
    let current_level = DESIRED_LEVEL.load();

    for _ in 0..blinks {
        set_level(40);
        embassy_time::Timer::after_millis(100).await;
        set_level(current_level);
        embassy_time::Timer::after_millis(100).await;
    }
}

pub fn set_level(value: u8) {
    DESIRED_LEVEL.store(value);
    GRADUAL_LEVEL.store(value);
}

pub fn set_level_gradual(value: u8) {
    GRADUAL_LEVEL.store(value);
}

pub fn is_torch_on() -> bool {
    TORCH_IS_ON.load()
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum PathLevel {
    One = 0,
    Two = 1,
    Three = 2,
}

impl PathLevel {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[bitfield_struct::bitfield(u8)]
struct Path {
    #[bits(4)]
    reference: atxtiny_hal::vref::ReferenceVoltage,

    #[bits(4)]
    path_level: PathLevel,
}

#[derive(Clone, Copy)]
struct Level {
    dac_level: u8,
    path: Path,
}

impl Level {
    const fn new(vref: ReferenceVoltage, path_level: PathLevel, adc_level: u8) -> Self {
        Level {
            dac_level: adc_level,
            path: Path::new().with_path_level(path_level).with_reference(vref),
        }
    }
}

//
// const fn sort_paths<const N: usize>(mut arr: [(Level, f32); N]) -> [(Level, f32); N] {
//     loop {
//         let mut swapped = false;
//         let mut i = 1;
//         while i < arr.len() {
//             if arr[i-1].1 > arr[i].1 {
//                 let left = arr[i-1];
//                 let right = arr[i];
//                 arr[i-1] = right;
//                 arr[i] = left;
//                 swapped = true;
//             }
//             i += 1;
//         }
//         if !swapped {
//             break;
//         }
//     }
//     arr
// }
//
// const fn calculate_output(level: &Level) -> f32 {
//     let vref = match level.path.reference() {
//         ReferenceVoltage::_0V55 => 0.5,
//         ReferenceVoltage::_1V10 => 1.1,
//         ReferenceVoltage::_2V50 => 2.5,
//         ReferenceVoltage::_4V34 => panic!("Not used"),
//         ReferenceVoltage::_1V50 => 1.5,
//     };
//
//     let adc_out = (level.adc_level as f32 / 255.0) * vref;
//
//     let scale = match level.path.path_level() {
//         PathLevel::One => 1.0,
//         PathLevel::Two => 100.0,
//         PathLevel::Three => 100.0 * 100.0,
//     };
//
//     return adc_out * scale;
// }
//
// const fn closest_level(levels: &[(Level, f32)], out: f32) -> Level {
//     let mut selected = levels.first().unwrap().0;
//     let mut best_diff = f32::MAX;
//
//     let mut i = 0;
//     while i < levels.len() {
//         let diff = levels[i].1 - out;
//         let diff = if diff < 0.0 {
//             -diff
//         } else {
//             diff
//         };
//
//         if diff < best_diff {
//             selected = levels[i].0;
//             best_diff = diff;
//         }
//
//         i += 1;
//     }
//
//     selected
// }
//
// const fn calculate_levels() -> [Level; NUM_LEVELS] {
//     const NUM_PATHS: usize = 3;
//     const NUM_REFS: usize = 4;
//     const TOTAL_COMBINATIONS: usize = NUM_LEVELS * NUM_PATHS * NUM_REFS;
//
//     let mut arr: [MaybeUninit<(Level, f32)>; TOTAL_COMBINATIONS] = MaybeUninit::uninit_array();
//
//     let paths = [
//         PathLevel::One,
//         PathLevel::Two,
//         PathLevel::Three
//     ];
//
//     let refs = [
//         ReferenceVoltage::_0V55,
//         ReferenceVoltage::_1V10,
//         ReferenceVoltage::_1V50,
//         ReferenceVoltage::_2V50
//     ];
//
//     let mut i = 0;
//     let mut l = 0;
//     while l < NUM_LEVELS {
//         let mut p = 0;
//         while p < NUM_PATHS {
//             let mut r = 0;
//             while r < NUM_REFS {
//                 let vref = refs[r];
//                 let path = paths[p];
//
//                 let level = Level {
//                     adc_level: l as u8,
//                     path: Path::new()
//                         .with_path_level(path)
//                         .with_reference(vref),
//                 };
//
//                 let output = calculate_output(&level);
//
//                 arr[i].write((level, output));
//                 i += 1;
//
//                 r += 1
//             }
//             p += 1;
//         }
//         l += 1;
//     }
//
//     let possible_paths = sort_paths(unsafe { MaybeUninit::array_assume_init(arr) });
//
//     let largest_output = possible_paths.last().unwrap().1;
//
//     let mut selected_outputs: [MaybeUninit<Level>; NUM_LEVELS] = MaybeUninit::uninit_array();
//
//     let mut i = 0;
//     while i < NUM_LEVELS {
//         let x = (i as f32 / NUM_LEVELS as f32);
//         let x = x * x * x * x;
//
//         let desired_output = largest_output * x;
//
//         let closest = closest_level(&possible_paths, desired_output);
//
//         selected_outputs[i].write(closest);
//
//         i += 1;
//     }
//
//     unsafe { MaybeUninit::array_assume_init(selected_outputs) }
// }

// avr_progmem::progmem! {
//     #[allow(long_running_const_eval)]
//     static progmem OUTPUT_LEVELS: [Level; NUM_LEVELS] = calculate_levels();
// }

const OUTPUT_LEVELS_C: [Level; NUM_LEVELS - 1] = [
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 1),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 2),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 4),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 7),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 11),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 4),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 28),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 15),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 58),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 29),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 107),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 141),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 67),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 233),
    Level::new(ReferenceVoltage::_1V10, PathLevel::One, 146),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 133),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 98),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 119),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 239),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 171),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 203),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 239),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 13),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 15),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 17),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 20),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 5),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 26),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 29),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 33),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 37),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 42),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 47),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 52),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 58),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 64),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 71),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 79),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 19),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 95),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 23),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 114),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 125),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 136),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 148),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 59),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 174),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 69),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 204),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 220),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 237),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 255),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 137),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 147),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 158),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 169),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 180),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 193),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 151),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 161),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 234),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 182),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 194),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 206),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 131),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 139),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 245),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 156),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 165),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 174),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 184),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 194),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 204),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 215),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 227),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 239),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 251),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 12),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 13),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 13),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 14),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 15),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 15),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 16),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 17),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 17),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 4),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 7),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 20),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 21),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 8),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 5),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 24),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 9),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 26),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 27),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 28),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 29),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 30),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 31),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 12),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 34),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 35),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 8),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 38),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 39),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 15),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 42),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 44),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 10),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 47),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 49),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 50),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 52),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 54),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 56),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 58),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 22),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 62),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 14),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 66),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 68),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 70),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 16),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 75),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 77),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 29),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 82),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 31),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 87),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 90),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 34),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 21),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 36),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 101),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 104),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 107),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 110),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 113),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 116),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 44),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 123),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 127),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 130),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 49),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 137),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 141),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 145),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 149),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 56),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 157),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 59),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 165),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 62),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 174),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 178),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 67),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 187),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 192),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 72),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 74),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 206),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 211),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 217),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 222),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 50),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 233),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 238),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 244),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 249),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 96),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 98),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 100),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 140),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 143),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 146),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 149),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 112),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 156),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 117),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 163),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 122),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 125),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 174),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 178),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 133),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 136),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 189),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 85),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 197),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 201),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 151),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 210),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 214),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 160),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 98),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 100),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 170),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 104),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 241),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 246),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 184),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 191),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 195),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 119),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 202),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 206),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 210),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 214),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 131),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 222),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 136),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 231),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 235),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 239),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 146),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 248),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 253),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 154),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 157),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 160),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 163),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 166),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 168),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 171),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 174),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 177),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 181),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 184),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 187),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 190),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 193),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 197),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 200),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 203),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 207),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 210),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 214),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 217),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 221),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 224),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 228),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 232),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 236),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 239),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 243),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 247),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 251),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
];
avr_progmem::progmem! {
    static progmem OUTPUT_LEVELS: [Level; NUM_LEVELS - 1] = OUTPUT_LEVELS_C;
}

pub struct PowerPaths {
    vref: DACReferenceVoltage<0>,
    dac: Dac<DAC0, Enabled>,
    path1: Pin<Porta, U<7>, Output<Stateless>>,
    path2: Pin<Portb, U<5>, Output<Stateless>>,
    path3: Pin<Portb, U<4>, Output<Stateless>>,
    buck: Pin<Portb, U<3>, Output<Stateless>>,
    fet: Pin<Portb, U<2>, Output<Stateless>>,
    wakelock: Mug,
    buck_is_on: bool,
}

impl PowerPaths {
    pub fn new(
        vref: DACReferenceVoltage<0>,
        mut dac: Dac<DAC0, Enabled>,
        path1: atxtiny_hal::gpio::PA7<Input>,
        path2: atxtiny_hal::gpio::PB5<Input>,
        path3: atxtiny_hal::gpio::PB4<Input>,
        buck: atxtiny_hal::gpio::PB3<Input>,
        fet: atxtiny_hal::gpio::PB2<Input>,
    ) -> Self {
        dac.run_in_standby(true);
        let mut s = Self {
            vref,
            dac,
            path1: path1.into_stateless_push_pull_output(),
            path2: path2.into_stateless_push_pull_output(),
            path3: path3.into_stateless_push_pull_output(),
            buck: buck.into_stateless_push_pull_output(),
            fet: fet.into_stateless_push_pull_output(),
            wakelock: Mug::new(),
            buck_is_on: false,
        };
        s.off();
        s
    }

    fn off(&mut self) {
        self.fet.set_low().unwrap_infallible();
        self.buck.set_low().unwrap_infallible();
        self.path1.set_low().unwrap_infallible();
        self.path2.set_low().unwrap_infallible();
        self.path3.set_low().unwrap_infallible();
        self.buck_is_on = false;
    }

    fn turbo_level(&mut self) {
        self.buck.set_low().unwrap_infallible();
        self.fet.set_high().unwrap_infallible();
        self.path1.set_high().unwrap_infallible();
        self.path2.set_high().unwrap_infallible();
        self.path3.set_high().unwrap_infallible();
        self.buck_is_on = false;
    }

    fn buck_level(&mut self) {
        self.fet.set_low().unwrap_infallible();
        self.buck.set_high().unwrap_infallible();
        self.buck_is_on = true;
    }

    fn set_vref(&mut self, vref: ReferenceVoltage) {
        let mut vref_p = unsafe { VREF::steal() }.constrain();
        DACReferenceVoltage::voltage(&mut vref_p, vref);
    }

    fn set_path(&mut self, path: PathLevel) {
        let (one, two, three) = match path {
            PathLevel::One => (true, false, false),
            PathLevel::Two => (true, true, false),
            PathLevel::Three => (true, true, true),
        };

        self.path1.set_state(one.into()).unwrap_infallible();
        self.path2.set_state(two.into()).unwrap_infallible();
        self.path3.set_state(three.into()).unwrap_infallible();
    }

    async fn set_power_level(&mut self, level: u8) {
        if level == 0 {
            self.off();
            self.wakelock.decaffeinate();
            TORCH_IS_ON.store(false);
        } else if level == 255 {
            #[cfg(feature = "has_fet")]
            self.turbo_level();
            self.wakelock.caffeinate();
            TORCH_IS_ON.store(true);
        } else {
            self.wakelock.caffeinate();
            let config = OUTPUT_LEVELS.load_at(level as usize);

            if !self.buck_is_on {
                self.buck_level();
                embassy_time::Timer::after_millis(8).await;
            }

            self.dac.dac_set_value(config.dac_level);
            self.set_vref(config.path.reference());
            self.set_path(config.path.path_level());
            TORCH_IS_ON.store(true);
        }
    }
}

const INSTANT_STOP_TEMP: Temperature<u16> = Temperature::kelvin_times_64_from_celcius(50);
const MAX_TEMP: Temperature<u16> = Temperature::kelvin_times_64_from_celcius(40);
const MIN_VOLTS: Voltage<u16> = Voltage::volts_to_adc_output(3.2);

#[embassy_executor::task]
pub async fn power_controller(mut paths: PowerPaths) {
    //let mut actual_level = 0u8;
    let mut previous_level = 0u8;

    let mut accumulated_over_temp = 0u32;

    // let mut l = unsafe { atxtiny_hal::avr_device::attiny1616::PORTB::steal().split().pb0.into_push_pull_output() };

    loop {
        let gradual_level = GRADUAL_LEVEL.load();
        let desired_level = DESIRED_LEVEL.load();

        let delta = if desired_level.abs_diff(gradual_level) > 50 {
            3
        } else {
            1
        };

        let desired_level = if desired_level < gradual_level {
            desired_level + delta
        } else if desired_level > gradual_level {
            desired_level - delta
        } else {
            desired_level
        };
        DESIRED_LEVEL.store(desired_level);

        let mut actual_level = DESIRED_LEVEL.load();

        let volts = crate::sensing::VOLTAGE.get();

        if volts < MIN_VOLTS {
            actual_level = 0;

            // l.set_high().unwrap_infallible();
        } // else {
          //     l.set_low().unwrap_infallible();
          // }

        let temp = crate::sensing::TEMPERATURE.get().kelvin_times_64();

        if temp > INSTANT_STOP_TEMP.0 {
            actual_level = 0;

            // TODO: do somethin with the aux
        }

        let temp_diff = (temp as i32) - (MAX_TEMP.0 as i32);

        accumulated_over_temp = accumulated_over_temp.saturating_add_signed(temp_diff);

        const TICKS_PER_SEC: u32 = 100;
        let power_decrease = 10 * (accumulated_over_temp / 64) / (TICKS_PER_SEC * 5);

        // decrease level by 10 for every second of 5c over temp?

        actual_level = actual_level.saturating_sub(power_decrease as u8);

        if actual_level != previous_level {
            previous_level = actual_level;

            paths.set_power_level(actual_level).await;
        }

        embassy_time::Timer::after(embassy_time::Duration::from_hz(TICKS_PER_SEC)).await;
    }
}
