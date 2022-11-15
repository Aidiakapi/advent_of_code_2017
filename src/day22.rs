framework::day!(22, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<i32>;

#[derive(Debug, Clone)]
struct State {
    board: HashMap<Vec2, Cell>,
    position: Vec2,
    direction: Offset,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Cell {
    #[default]
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State {
    pub fn step<F: FnMut(Cell) -> Cell>(&mut self, mut transform: F) -> Cell {
        let cell = self.board.entry(self.position).or_default();

        self.direction = match *cell {
            Cell::Clean => self.direction.rot_270(),
            Cell::Weakened => self.direction,
            Cell::Infected => self.direction.rot_90(),
            Cell::Flagged => self.direction.rot_180(),
        };
        *cell = transform(*cell);
        self.position = self.position.neighbor(self.direction).unwrap();
        *cell
    }
}

fn pts<F: FnMut(Cell) -> Cell>(state: &State, iter: usize, mut transform: F) -> usize {
    let mut state = state.clone();
    (0..iter)
        .filter(|_| state.step(&mut transform) == Cell::Infected)
        .count()
}

fn pt1(state: &State) -> usize {
    pts(state, 10_000, |c| {
        if c == Cell::Clean {
            Cell::Infected
        } else {
            Cell::Clean
        }
    })
}

fn pt2(state: &State) -> usize {
    pts(state, 10_000_000, |c| match c {
        Cell::Clean => Cell::Weakened,
        Cell::Weakened => Cell::Infected,
        Cell::Infected => Cell::Flagged,
        Cell::Flagged => Cell::Clean,
    })
}

fn parse(input: &[u8]) -> Result<State> {
    let mut board = HashMap::new();
    let mut width = None;
    let mut x = 0;
    let mut y = 0;
    for &c in input.iter() {
        let cell_value = match c {
            b'\n' => {
                if x == 0 {
                    return Err(Error::InvalidInput("empty line in input"));
                }
                if width.is_some() && width != Some(x) {
                    return Err(Error::InvalidInput(
                        "different lines have different lengths",
                    ));
                }
                width = Some(x);
                x = 0;
                y += 1;
                continue;
            }
            b'.' => Cell::Clean,
            b'#' => Cell::Infected,
            _ => return Err(Error::InvalidInput("unexpected character in input")),
        };
        board.insert(Vec2::new(x, y), cell_value);
        x += 1;
    }

    let width = width.ok_or(Error::InvalidInput("empty input"))?;
    if x != 0 {
        return Err(Error::InvalidInput("expected newline at the end"));
    }
    let height = y;
    if width % 2 == 0 || height % 2 == 0 {
        return Err(Error::InvalidInput(
            "input grid has an even dimension, which does not unambiguously specify a middle",
        ));
    }

    Ok(State {
        board,
        position: Vec2::new(width / 2, height / 2),
        direction: Offset::Y_NEG,
    })
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
..#
#..
...
";

    test_pt!(parse, pt1, EXAMPLE => 5587);
    test_pt!(parse, pt2, EXAMPLE => 2511944);
}
