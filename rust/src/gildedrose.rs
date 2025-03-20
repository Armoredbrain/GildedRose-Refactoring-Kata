use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            match self.items[i].name.as_str() {
                "Sulfuras, Hand of Ragnaros" => {
                    self.items[i].quality = self.items[i].quality;
                    self.items[i].sell_in = self.items[i].sell_in;
                }
                "Backstage passes to a TAFKAL80ETC concert" => {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;

                        if self.items[i].sell_in < 11 && self.items[i].quality < 50 {
                            self.items[i].quality = self.items[i].quality + 1;
                        }

                        if self.items[i].sell_in < 6 && self.items[i].quality < 50 {
                            self.items[i].quality = self.items[i].quality + 1;
                        }
                    }

                    self.items[i].sell_in = self.items[i].sell_in - 1;

                    if self.items[i].sell_in < 0 {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                }
                _ => {
                    if self.items[i].name != "Aged Brie" && self.items[i].quality > 0 {
                        self.items[i].quality = self.items[i].quality - 1;
                    } else {
                        if self.items[i].quality < 50 {
                            self.items[i].quality = self.items[i].quality + 1;
                        }
                    }

                    self.items[i].sell_in = self.items[i].sell_in - 1;

                    if self.items[i].sell_in < 0 {
                        if self.items[i].name != "Aged Brie" {
                            if self.items[i].quality > 0 {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        } else {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("fixme", rose.items[0].name);
    }
}
