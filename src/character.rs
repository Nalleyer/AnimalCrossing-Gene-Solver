use crate::color::Color;
use crate::gene::Gene;

pub fn character(gene4: &Gene) -> Color {
    match gene4.value() as u8 {
        0x00 => Color::White,
        0x01 => Color::White,
        0x03 => Color::White,
        0x04 => Color::White,
        0x05 => Color::White,
        0x07 => Color::White,
        0x0c => Color::Purple,
        0x0d => Color::Purple,
        0x0f => Color::Purple,
        0x10 => Color::Yellow,
        0x11 => Color::Yellow,
        0x13 => Color::Yellow,
        0x14 => Color::White,
        0x15 => Color::White,
        0x17 => Color::White,
        0x1c => Color::Purple,
        0x1d => Color::Purple,
        0x1f => Color::Purple,
        0x30 => Color::Yellow,
        0x31 => Color::Yellow,
        0x33 => Color::Yellow,
        0x34 => Color::Yellow,
        0x35 => Color::Yellow,
        0x37 => Color::Yellow,
        0x3c => Color::White,
        0x3d => Color::White,
        0x3f => Color::White,
        0x40 => Color::Red,
        0x41 => Color::Pink,
        0x43 => Color::White,
        0x44 => Color::Red,
        0x45 => Color::Pink,
        0x47 => Color::White,
        0x4c => Color::Red,
        0x4d => Color::Pink,
        0x4f => Color::Purple,
        0x50 => Color::Orange,
        0x51 => Color::Yellow,
        0x53 => Color::Yellow,
        0x54 => Color::Red,
        0x55 => Color::Pink,
        0x57 => Color::White,
        0x5c => Color::Red,
        0x5d => Color::Pink,
        0x5f => Color::Purple,
        0x70 => Color::Orange,
        0x71 => Color::Yellow,
        0x73 => Color::Yellow,
        0x74 => Color::Orange,
        0x75 => Color::Yellow,
        0x77 => Color::Yellow,
        0x7c => Color::Red,
        0x7d => Color::Pink,
        0x7f => Color::White,
        0xc0 => Color::Black,
        0xc1 => Color::Red,
        0xc3 => Color::Pink,
        0xc4 => Color::Black,
        0xc5 => Color::Red,
        0xc7 => Color::Pink,
        0xcc => Color::Black,
        0xcd => Color::Red,
        0xcf => Color::Pink,
        0xd0 => Color::Orange,
        0xd1 => Color::Orange,
        0xd3 => Color::Yellow,
        0xd4 => Color::Red,
        0xd5 => Color::Red,
        0xd7 => Color::White,
        0xdc => Color::Black,
        0xdd => Color::Red,
        0xdf => Color::Purple,
        0xf0 => Color::Orange,
        0xf1 => Color::Orange,
        0xf3 => Color::Yellow,
        0xf4 => Color::Orange,
        0xf5 => Color::Orange,
        0xf7 => Color::Yellow,
        0xfc => Color::Blue,
        0xfd => Color::Red,
        0xff => Color::White,
        _ => panic!("invalid gene"),
    }
}
