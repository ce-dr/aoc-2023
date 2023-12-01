pub struct Day {
    p1: <Self as crate::Problem>::P1Result,
    p2: <Self as crate::Problem>::P2Result,
    input: String,
}

impl crate::Problem for Day {
    const DAY: u32 = 7;

    type P1Result = u32;
    type P2Result = u32;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
        }
    }

    fn do_p1(&mut self) -> Self::P1Result {
        log::debug!("{}", self.input);
        self.p1
    }
    fn do_p2(&mut self) -> Self::P2Result {
        self.p2
    }
}
