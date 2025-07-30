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
    pub dac_level: u16,
    pub path: Path,
}

impl Level {
    const fn new(vref: ReferenceVoltage, path_level: PathLevel, adc_level: u16) -> Self {
        Level {
            dac_level: adc_level,
            path: Path::new().with_path_level(path_level).with_reference(vref),
        }
    }
}

pub const NUM_LEVELS: usize = 256;

const OUTPUT_LEVELS_C: [Level; NUM_LEVELS] = [
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 0),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 2),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 4),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 3),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 14),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 24),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 39),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 59),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 86),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 50),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 69),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 227),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 299),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 387),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 202),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 254),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 770),
    Level::new(ReferenceVoltage::_1V024, PathLevel::One, 945),
    Level::new(ReferenceVoltage::_2V048, PathLevel::One, 574),
    Level::new(ReferenceVoltage::_2V048, PathLevel::One, 692),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 677),
    Level::new(ReferenceVoltage::_2V048, PathLevel::One, 980),
    Level::new(ReferenceVoltage::_2V50, PathLevel::One, 945),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 27),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 13),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 36),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 42),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 48),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 55),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 62),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 70),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 79),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 89),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 99),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 111),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 123),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 56),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 62),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 167),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 184),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 202),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 221),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 242),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 264),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 118),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 314),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 341),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 369),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 400),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 432),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 466),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 502),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 541),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 581),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 624),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 668),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 716),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 766),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 335),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 873),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 381),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Two, 991),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 432),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 459),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Two, 595),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 517),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Two, 669),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 581),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 615),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 650),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 687),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Two, 886),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Two, 934),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 807),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 851),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 896),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 23),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Two, 991),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 25),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 11),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 28),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 12),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 31),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 32),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 34),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 35),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 37),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 39),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 41),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 42),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 44),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 46),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 48),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 50),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 52),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 54),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 57),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 59),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 25),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 64),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 66),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 69),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 72),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 75),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 77),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 33),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 34),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 86),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 90),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 93),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 96),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 100),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 103),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 107),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 111),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 47),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 118),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 50),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 52),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 131),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 135),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 140),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 59),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 61),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 63),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 65),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 67),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 69),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 174),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 179),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 185),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 78),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 196),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 202),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 208),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 214),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 221),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 227),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 233),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 240),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 247),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 254),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 261),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 110),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 113),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 116),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 291),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 299),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 307),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 315),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 324),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 136),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 341),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 350),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 147),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 368),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 378),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 387),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 397),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 407),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 417),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 175),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 438),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 449),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 459),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 471),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 482),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 202),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 505),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 517),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 529),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 541),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 554),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 567),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 580),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 593),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 606),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 254),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 634),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 648),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 662),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 677),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 692),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 707),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 722),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 738),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 754),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 770),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 322),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 803),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 820),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 837),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 854),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 357),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 890),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 908),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 926),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 945),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 964),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 403),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 1003),
    Level::new(ReferenceVoltage::_1V024, PathLevel::Three, 1023),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 427),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 532),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 542),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 453),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 462),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 574),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 585),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 489),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 498),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 619),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 517),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 643),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 655),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 667),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 679),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 692),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 577),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 717),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 730),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 609),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 757),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 631),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 784),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 798),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 812),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 677),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 841),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 701),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 713),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 725),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 738),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 916),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 932),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 776),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 964),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 980),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 816),
    Level::new(ReferenceVoltage::_2V048, PathLevel::Three, 1013),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 844),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 858),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 872),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 886),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 901),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 915),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 930),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 945),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 960),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 976),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 991),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 1007),
    Level::new(ReferenceVoltage::_2V50, PathLevel::Three, 1023),
];

avr_progmem::progmem! {
    pub static progmem OUTPUT_LEVELS: [Level; NUM_LEVELS] = OUTPUT_LEVELS_C;
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
