use std::{convert::TryFrom, num::ParseIntError};

use crate::utils::CombinationsGetter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemOption {
    price: u32,
}

impl Default for ItemOption {
    fn default() -> Self {
        Self {
            price: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct ItemOptionCategory {
    item_options: Vec<ItemOption>,
}

impl<'a> AsRef<[ItemOption]> for ItemOptionCategory {
    fn as_ref(&self) -> &[ItemOption] {
        self.item_options.as_ref()
    }
}

pub trait ToItemOptionCategory<T> {
    fn to_item_option_category(&self) -> ItemOptionCategory;
}

pub trait ToItemOption: Copy {
    fn to_item_option(self) -> ItemOption;
}

impl ToItemOption for u32 {
    fn to_item_option(self) -> ItemOption {
        ItemOption { price: self }
    }
}

impl<T: ToItemOption, K: AsRef<[T]>> ToItemOptionCategory<T> for K {
    fn to_item_option_category(&self) -> ItemOptionCategory {
        ItemOptionCategory {
            item_options: self
                .as_ref()
                .iter()
                .copied()
                .map(ToItemOption::to_item_option)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct ShopCart {
    item_option_categories: Vec<ItemOptionCategory>,
    budget: u32,
}

impl ShopCart {
    pub fn new(budget: u32) -> Self {
        Self {
            item_option_categories: Vec::new(),
            budget,
        }
    }

    pub fn add_option_category<T: ToItemOptionCategory<K>, K>(&mut self, item_category: T) {
        self.item_option_categories
            .push(item_category.to_item_option_category())
    }

    fn get_all_combinations(&self) -> CombinationsGetter<ItemOption> {
        CombinationsGetter::new(&self.item_option_categories)
    }

    pub fn get_cost(&self) -> i64 {
        self.get_all_combinations()
            .map(|combination| combination.iter().map(|item| item.price).sum::<u32>().into())
            .filter(|result| *result <= i64::from(self.budget))
            .max()
            .unwrap_or(-1)
    }
}

#[derive(Debug)]
pub enum ShopCartParseError {
    MissingField,
    MissingLine,
    ParseIntError(ParseIntError),
}

impl TryFrom<&str> for ShopCart {
    type Error = ShopCartParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut lines = input.lines().map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u32>().map_err(ShopCartParseError::ParseIntError))
        });

        let mut header = lines.next().ok_or(ShopCartParseError::MissingLine)?;

        let budget = header.next().ok_or(ShopCartParseError::MissingField)??;
        let category_lengths = header;

        let mut item_option_categories = Vec::new();

        for category_length in category_lengths {
            let data: Result<Vec<u32>, ShopCartParseError> = lines
                .next()
                .ok_or(ShopCartParseError::MissingLine)?
                .take(usize::try_from(category_length?).unwrap())
                .collect();

            let data = data?;

            item_option_categories.push(ItemOptionCategory {
                item_options: data
                    .iter()
                    .copied()
                    .map(|price| ItemOption { price })
                    .collect(),
            })
        }

        Ok(ShopCart {
            item_option_categories,
            budget,
        })
    }
}

mod tests {
    use crate::ShopCart;
    use std::convert::TryFrom;

    fn test_input(input: &str, expected: i64) {
        let shopcart = ShopCart::try_from(
           input,
        )
        .unwrap();

        assert_eq!(shopcart.get_cost(), expected)
    }

    #[test]
    pub fn test_input_1() {
        test_input(
            r"10 2 3
              3 1
              5 2 8",
            9,
        )
    }

    #[test]
    pub fn test_input_2() {
        test_input(
            r"5 1 1
              4
              5",
            -1,
        )
    }
}
