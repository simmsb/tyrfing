use atxtiny_hal::vref::ReferenceVoltage;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PathLevel {
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
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[bitfield_struct::bitfield(u8)]
pub struct Path {
    #[bits(4)]
    pub reference: atxtiny_hal::vref::ReferenceVoltage,

    #[bits(4)]
    pub path_level: PathLevel,
}

#[derive(Clone, Copy)]
pub struct Level {
    pub dac_level: u8,
    pub path: Path,
}

impl Level {
    const fn new(vref: ReferenceVoltage, path_level: PathLevel, adc_level: u8) -> Self {
        Level {
            dac_level: adc_level,
            path: Path::new().with_path_level(path_level).with_reference(vref),
        }
    }
}

pub const NUM_LEVELS: usize = 256;

const OUTPUT_LEVELS_C: [Level; NUM_LEVELS - 1] = [
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 1),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 2),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 1),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 9),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 15),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 24),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 37),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 54),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 28),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 105),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 141),
    Level::new(ReferenceVoltage::_0V55, PathLevel::One, 186),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 53),
    Level::new(ReferenceVoltage::_1V10, PathLevel::One, 154),
    Level::new(ReferenceVoltage::_1V10, PathLevel::One, 193),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 176),
    Level::new(ReferenceVoltage::_1V50, PathLevel::One, 216),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 157),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 189),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 226),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 12),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 14),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 17),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 7),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 5),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 26),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 11),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 34),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 14),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 16),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 18),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 55),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 62),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 69),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 28),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 85),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 94),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 104),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 42),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 46),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 138),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 151),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 165),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 179),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 195),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 212),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 230),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Two, 249),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 59),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 145),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 156),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 74),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 181),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 194),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 208),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 98),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Two, 238),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 112),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 199),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 212),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 226),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 241),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Two, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 163),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 173),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 183),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 194),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 206),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 217),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 230),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 11),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 255),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 12),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 13),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 5),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 14),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 15),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 16),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 6),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 17),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 4),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 7),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 20),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 21),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 22),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 23),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 24),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 25),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 26),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 10),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 29),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 11),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 31),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 12),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 34),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 13),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 37),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 14),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 40),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 41),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 43),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 45),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 17),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 48),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 50),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 19),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 54),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 56),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 58),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 22),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 62),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 64),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 67),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 69),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 71),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 27),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 28),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 79),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 30),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 84),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 87),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 90),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 34),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 35),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 99),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 102),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 105),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 108),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 41),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 115),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 26),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 122),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 126),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 129),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 133),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 137),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 141),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 32),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 149),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 154),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 158),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 162),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 167),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 63),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 176),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 181),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 186),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 191),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 72),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 74),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 207),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 212),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 218),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 82),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 84),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 235),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 53),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 247),
    Level::new(ReferenceVoltage::_0V55, PathLevel::Three, 253),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 95),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 133),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 100),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 140),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 143),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 146),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 150),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 154),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 157),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 118),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 121),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 74),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 76),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 176),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 180),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 135),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 83),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 193),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 197),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 148),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 206),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 211),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 158),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 220),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 99),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 101),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 172),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 176),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 245),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 183),
    Level::new(ReferenceVoltage::_1V10, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 191),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 195),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 199),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 203),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 207),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 127),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 216),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 220),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 224),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 229),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 140),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 238),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 243),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 247),
    Level::new(ReferenceVoltage::_1V50, PathLevel::Three, 252),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 154),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 157),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 160),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 163),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 166),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 170),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 173),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 176),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 179),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 183),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 186),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 189),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 193),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 196),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 200),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 204),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 207),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 211),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 215),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 219),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 222),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 226),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 230),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 234),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 238),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 242),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 247),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 251),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
    Level::new(ReferenceVoltage::_4V34, PathLevel::Three, 255),
];

avr_progmem::progmem! {
    pub static progmem OUTPUT_LEVELS: [Level; NUM_LEVELS - 1] = OUTPUT_LEVELS_C;
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
