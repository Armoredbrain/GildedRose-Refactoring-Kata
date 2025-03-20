mod gildedrose;

use std::{
    fs::File,
    io::{Read, Write},
};

use gildedrose::{GildedRose, Item};

use similar::{ChangeTag, TextDiff};

fn main() {
    let items = vec![
        Item::new("+5 Dexterity Vest", 10, 20),
        Item::new("Aged Brie", 2, 0),
        Item::new("Elixir of the Mongoose", 5, 7),
        Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
        Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49),
        // this conjured item does not work properly yet
        Item::new("Conjured Mana Cake", 3, 6),
    ];
    let mut rose = GildedRose::new(items);

    let mut text = String::new();

    println!("OMGHAI!");
    for i in 0..=30 {
        println!("-------- day {} --------", i);
        text.push_str(&format!("-------- day {} -------- \n", i));
        println!("name, sellIn, quality");
        text.push_str("name, sellIn, quality \n");
        for item in &rose.items {
            println!("{}", item);
            text.push_str(&format!("{} \n", item));
        }
        println!();
        rose.update_quality();
    }

    let mut result = File::create("src/test_contender").unwrap();
    let _echo = result.write_all(text.as_bytes());

    let mut original_file = File::open("src/test_original").unwrap();
    let mut contender_file = File::open("src/test_contender").unwrap();

    let mut original = String::new();
    let mut contender = String::new();

    original_file.read_to_string(&mut original).unwrap();
    contender_file.read_to_string(&mut contender).unwrap();

    let diff = TextDiff::from_lines(&original, &contender);
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "---",
            ChangeTag::Insert => "+++",
            ChangeTag::Equal => " ",
        };

        print!("{}   {}", sign, change);
    }

    if original != contender {
        eprintln!("NOPE");
    } else {
        eprintln!("GG");
    }
}
