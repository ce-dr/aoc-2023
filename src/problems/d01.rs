pub struct Day {
    p1: <Self as crate::Problem>::P1Result,
    p2: <Self as crate::Problem>::P2Result,
    input: String,
}

#[derive(Debug, PartialEq)]
pub enum NumberParser {
    Empty,
    O,
    On,
    T,
    Tw,
    Th,
    Thr,
    Thre,
    F,
    Fo,
    Fou,
    Fi,
    Fiv,
    S,
    Si,
    Se,
    Sev,
    Seve,
    E,
    Ei,
    Eig,
    Eigh,
    N,
    Ni,
    Nin,
}

/// I'm sure there is a bug in this that would be found through fuzzing.
/// It works on my input though
/// It is very overengineered but I had lots of fun writing a state-machine
/// parser. It's faster than any other solutions I've tried
/// You'll notice I had to special-case the scenarios where we're mid-parse
/// of one number but start finding a different one (e.g. "fone") which would
/// miss the "one" if you didn't account for the state transition Self::Fo(n) -> Self::On
impl NumberParser {
    fn follow(&mut self, c: char) -> Option<u32> {
        match (&self, c) {
            (Self::O, 'n') => {
                *self = Self::On;
                None
            }
            (Self::On, 'e') => {
                *self = Self::E;
                Some(1)
            }
            (Self::On, 'i') => {
                *self = Self::Ni;
                None
            }
            (Self::T, 'w') => {
                *self = Self::Tw;
                None
            }
            (Self::Tw, 'o') => {
                *self = Self::O;
                Some(2)
            }
            (Self::T, 'h') => {
                *self = Self::Th;
                None
            }
            (Self::Th, 'r') => {
                *self = Self::Thr;
                None
            }
            (Self::Thr, 'e') => {
                *self = Self::Thre;
                None
            }
            (Self::Thre, 'e') => {
                *self = Self::E;
                Some(3)
            }
            (Self::Thre, 'i') => {
                *self = Self::Ei;
                None
            }
            (Self::F, 'o') => {
                *self = Self::Fo;
                None
            }
            (Self::Fo, 'u') => {
                *self = Self::Fou;
                None
            }
            (Self::Fo, 'n') => {
                *self = Self::On;
                None
            }
            (Self::Fou, 'r') => {
                *self = Self::Empty;
                Some(4)
            }
            (Self::F, 'i') => {
                *self = Self::Fi;
                None
            }
            (Self::Fi, 'v') => {
                *self = Self::Fiv;
                None
            }
            (Self::Fiv, 'e') => {
                *self = Self::E;
                Some(5)
            }
            (Self::S, 'i') => {
                *self = Self::Si;
                None
            }
            (Self::Si, 'x') => {
                *self = Self::Empty;
                Some(6)
            }
            (Self::S, 'e') => {
                *self = Self::Se;
                None
            }
            (Self::Se, 'v') => {
                *self = Self::Sev;
                None
            }
            (Self::Se, 'i') => {
                *self = Self::Ei;
                None
            }
            (Self::Sev, 'e') => {
                *self = Self::Seve;
                None
            }
            (Self::Seve, 'n') => {
                *self = Self::Empty;
                Some(7)
            }
            (Self::Seve, 'i') => {
                *self = Self::Ei;
                None
            }
            (Self::E, 'i') => {
                *self = Self::Ei;
                None
            }
            (Self::Ei, 'g') => {
                *self = Self::Eig;
                None
            }
            (Self::Eig, 'h') => {
                *self = Self::Eigh;
                None
            }
            (Self::Eigh, 't') => {
                *self = Self::T;
                Some(8)
            }
            (Self::N, 'i') => {
                *self = Self::Ni;
                None
            }
            (Self::Ni, 'n') => {
                *self = Self::Nin;
                None
            }
            (Self::Nin, 'e') => {
                *self = Self::E;
                Some(9)
            }
            (_, c) => match c {
                'o' => {
                    *self = Self::O;
                    None
                }
                't' => {
                    *self = Self::T;
                    None
                }
                'f' => {
                    *self = Self::F;
                    None
                }
                's' => {
                    *self = Self::S;
                    None
                }
                'e' => {
                    *self = Self::E;
                    None
                }
                'n' => {
                    *self = Self::N;
                    None
                }
                c if c.is_numeric() => {
                    *self = Self::Empty;
                    Some(c.to_digit(10).unwrap())
                }
                _ => {
                    *self = Self::Empty;
                    None
                }
            },
        }
    }
}

fn parse_input(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut c = line.chars().filter_map(|c| c.to_digit(10));
            let first = c.next().unwrap();
            let last = c.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

impl crate::Problem for Day {
    const DAY: u32 = 1;

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
        self.p1 = parse_input(&self.input);
        self.p1
    }
    fn do_p2(&mut self) -> Self::P2Result {
        for line in self.input.lines() {
            let mut parser = NumberParser::Empty;
            let mut nums = line.chars().filter_map(|c| parser.follow(c));
            let first = nums.next().unwrap();
            let last = nums.last().unwrap_or(first);
            self.p2 += first * 10 + last;
        }
        self.p2
    }
}
