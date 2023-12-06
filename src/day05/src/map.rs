use std::ops::Range;
use crate::item_type::ItemType;

#[derive(Debug, PartialEq)]
pub struct Map {
    pub from: ItemType,
    pub to: ItemType,
    pub destination_range_start: u32,
    pub source_range_start: u32,
    pub range_length: u32,
    pub destinations: Range<u32>,
    pub sources: Range<u32>,
}