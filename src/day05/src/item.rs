use crate::item_type::ItemType;

#[derive(Debug, PartialEq)]
pub struct Item {
    pub item_type: ItemType,
    pub id: u32,
    pub child: Option<Box<Item>>,
}

impl Item {
    fn set_child(&mut self, child: Item) {
        self.child = Some(Box::new(child));
    }
}