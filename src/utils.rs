#[derive(Debug)]
pub struct CombinationsGetter<'a, T: 'a> {
    arrays: Vec<(usize, &'a [T])>,
    end: bool,
}

impl<'a, T> CombinationsGetter<'a, T> {
    pub fn new<'b: 'a, K: AsRef<[T]>>(arrays: &'b [K]) -> Self {
        let arrays: Vec<(usize, &'a [T])> =
            arrays.iter().map(|array| (0, array.as_ref())).collect();

        Self { arrays, end: false }
    }

    fn arrange_next_combination(&mut self) {
        for (index, array) in self.arrays.iter_mut() {
            if (array.len() - 1) == *index as usize {
                self.end = true;
            } else {
                self.end = false;
                *index += 1;
                break;
            }
        }
    }
}

impl<'a, T> Iterator for CombinationsGetter<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }

        let mut combination = Vec::new();
        for (index, array) in self.arrays.iter() {
            combination.push(&array[*index as usize])
        }

        self.arrange_next_combination();

        Some(combination)
    }
}
