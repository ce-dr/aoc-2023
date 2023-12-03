#[derive(Debug, Default)]
pub struct Day {
    p1: <Self as crate::Problem>::P1Result,
    p2: <Self as crate::Problem>::P2Result,
    input: String,
    input_indexable: Vec<char>,
    input_w: usize,
}

fn contains_ctl(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_punctuation() && c != '.')
}

impl Day {
    fn has_adjacent(&self, num_start: usize) -> Option<u32> {
        let end = self.input[num_start..]
            .find(|c: char| !c.is_numeric())
            .unwrap_or(self.input.len());
        if [
            (num_start.wrapping_sub(self.input_w + 1))
                ..(num_start.wrapping_sub(self.input_w) + end + 1),
            (num_start.wrapping_sub(1))..num_start + end + 1,
            (num_start + self.input_w - 1)..(num_start + self.input_w + end + 1),
        ]
        .into_iter()
        .any(|r| self.input.get(r).is_some_and(contains_ctl))
        {
            self.input[num_start..num_start + end].parse::<u32>().ok()
        } else {
            None
        }
    }

    /// panics if idx does not point to a valid character
    /// also panics if idx points to a number at the end of the
    /// input. This wasn't a problem for my input, sorry.
    fn number_bounds_at(&self, idx: usize) -> (usize, usize) {
        (
            self.input_indexable[..idx]
                .iter()
                .rposition(|c| !c.is_numeric())
                .unwrap()
                + 1,
            self.input_indexable[idx..]
                .iter()
                .position(|c| !c.is_numeric())
                .unwrap()
                + idx,
        )
    }

    fn gear_number(&self, idx: usize) -> Option<u32> {
        let mut ranges: Vec<(usize, usize)> = [
            idx.wrapping_sub(self.input_w + 1),
            idx.wrapping_sub(self.input_w),
            idx.wrapping_sub(self.input_w - 1),
            idx.wrapping_sub(1),
            idx + 1,
            idx + self.input_w - 1,
            idx + self.input_w,
            idx + self.input_w + 1,
        ]
        .into_iter()
        .filter_map(|i| {
            self.input_indexable
                .get(i)
                .is_some_and(|c| c.is_numeric())
                .then_some(i)
        })
        .map(|i| self.number_bounds_at(i))
        .collect();
        ranges.dedup();

        if ranges.len() == 2 {
            Some(
                ranges
                    .into_iter()
                    .filter_map(|(s, e)| self.input.get(s..e))
                    .map(|s: &str| s.parse::<u32>().unwrap())
                    .product(),
            )
        } else {
            None
        }
    }
}
impl crate::Problem for Day {
    const DAY: u32 = 3;

    type P1Result = u32;
    type P2Result = u32;

    fn new(input: String) -> Self {
        Self {
            input,
            ..Default::default()
        }
    }

    fn do_p1(&mut self) -> Self::P1Result {
        self.input_w = crate::utils::grid_dimensions(&self.input).0 + 1;
        // rust doesn't have a "filter_map with internal state" or "iter windows"
        // iterator yet so unfortunately this mutable state is required
        let mut st: bool = true;
        self.p1 = self
            .input
            .char_indices()
            .filter_map(|(i, c)| {
                if !c.is_numeric() {
                    st = true;
                    None
                } else if st && c.is_numeric() {
                    st = false;
                    Some(i)
                } else {
                    None
                }
            })
            .filter_map(|i| self.has_adjacent(i))
            .sum();

        self.p1
    }
    fn do_p2(&mut self) -> Self::P2Result {
        self.input_indexable = self.input.chars().collect();
        assert!(self.input_indexable.len() == self.input.len());

        self.p2 = self
            .input
            .char_indices()
            .filter(|(_, c)| *c == '*')
            .map(|(i, _)| i)
            .filter_map(|i| self.gear_number(i))
            .sum();
        self.p2
    }
}
