pub struct Day {
    p1: <Self as crate::Problem>::P1Result,
    p2: <Self as crate::Problem>::P2Result,
    input: String,
    games: Vec<DiceGame>,
}
#[derive(Debug, Default)]
struct DiceGame {
    game: u32,
    rounds: Vec<DiceRound>,
}

impl DiceGame {
    fn from_parsed_input(game: u32, rounds: Vec<Vec<(u32, &str)>>) -> Self {
        Self {
            game,
            rounds: rounds.iter().map(|r| DiceRound::from_list(r)).collect(),
        }
    }

    fn in_limit(&self, limit: &DiceRound) -> Option<u32> {
        self.rounds
            .iter()
            .all(|r| r.all_lt(limit))
            .then_some(self.game)
    }

    fn power(&self) -> u32 {
        let (r, g, b) = self.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
            (r.max(round.red), g.max(round.green), b.max(round.blue))
        });
        r * g * b
    }
}
#[derive(Debug, Default)]
struct DiceRound {
    red: u32,
    green: u32,
    blue: u32,
}

impl DiceRound {
    // I got lazy and couldn't be bothered doing FP here
    // You could use fold or something probably
    fn from_list(list: &[(u32, &str)]) -> Self {
        let mut me = DiceRound::default();
        for (count, color) in list {
            match *color {
                " red" => me.red += count,
                " green" => me.green += count,
                " blue" => me.blue += count,
                _ => panic!("Unknown color: {}", color),
            }
        }
        me
    }

    fn all_lt(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

impl crate::Problem for Day {
    const DAY: u32 = 2;

    type P1Result = u32;
    type P2Result = u32;

    fn new(input: String) -> Self {
        Self {
            p1: 0,
            p2: 0,
            input,
            games: Vec::new(),
        }
    }
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    fn do_p1(&mut self) -> Self::P1Result {
        let mut parser = nom::sequence::tuple((
            nom::bytes::complete::tag::<_, &str, nom::error::VerboseError<_>>("Game "),
            nom::character::complete::u32,
            nom::bytes::complete::tag(": "),
            nom::multi::separated_list0(
                nom::bytes::complete::tag("; "),
                nom::multi::separated_list0(
                    nom::bytes::complete::tag(", "),
                    nom::sequence::tuple((
                        nom::character::complete::u32,
                        nom::branch::alt((
                            nom::bytes::complete::tag(" green"),
                            nom::bytes::complete::tag(" red"),
                            nom::bytes::complete::tag(" blue"),
                        )),
                    )),
                ),
            ),
        ));
        self.games = self
            .input
            .lines()
            .map(|line| parser(line).unwrap())
            .map(|(_, (_, game, _, rounds))| DiceGame::from_parsed_input(game, rounds))
            .collect();
        let limit = DiceRound {
            red: 12,
            green: 13,
            blue: 14,
        };
        self.p1 = self.games.iter().filter_map(|g| g.in_limit(&limit)).sum();
        self.p1
    }
    fn do_p2(&mut self) -> Self::P2Result {
        self.p2 = self.games.iter().map(|g| g.power()).sum();
        self.p2
    }
}
