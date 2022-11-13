framework::day!(19, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<usize>;

fn pts(input: &Input) -> Result<(usize, String)> {
    let mut visited = String::new();
    let mut pos = Some(Vec2::new(input.starting_point, 0));
    let mut dir = Offset::Y_POS;
    let mut steps = 0;
    loop {
        steps += 1;
        pos = pos.and_then(|p| p.neighbor(dir));
        match input.get(pos) {
            b'+' => {
                let pos = pos.unwrap();
                let next_char = input.get(pos.neighbor(dir));
                match (dir.has_x(), next_char) {
                    (true, b'-') | (false, b'|') => continue,
                    _ => {}
                }
                let can_rot_90 = input.get(pos.neighbor(dir.rot_90())) != b' ';
                let can_rot_270 = input.get(pos.neighbor(dir.rot_270())) != b' ';
                match (can_rot_90, can_rot_270) {
                    (true, false) => dir = dir.rot_90(),
                    (false, true) => dir = dir.rot_270(),
                    (true, true) => return Err(Error::InvalidInput("ambiguous corner")),
                    (false, false) => return Err(Error::InvalidInput("dead end")),
                }
            }
            b'-' | b'|' => {}
            c @ b'A'..=b'Z' => {
                visited.push(c as char);
            }
            b' ' => break,
            _ => return Err(Error::InvalidInput("unexpected char")),
        }
    }

    Ok((steps, visited))
}

fn pt1(input: &Input) -> Result<String> {
    pts(input).map(|(_, s)| s)
}

fn pt2(input: &Input) -> Result<usize> {
    pts(input).map(|(s, _)| s)
}

struct Input<'i> {
    data: &'i [u8],
    width: usize,
    starting_point: usize,
    height: usize,
}

trait Get<T> {
    fn get(&self, index: T) -> u8;
}

impl Get<Vec2> for Input<'_> {
    fn get(&self, index: Vec2) -> u8 {
        if index.x < self.width && index.y < self.height {
            self.data[index.x + index.y * (self.width + 1)]
        } else {
            b' '
        }
    }
}

impl Get<Option<Vec2>> for Input<'_> {
    fn get(&self, index: Option<Vec2>) -> u8 {
        index.map(|index| self.get(index)).unwrap_or(b' ')
    }
}

fn parse(input: &[u8]) -> Result<Input> {
    let line_width = input
        .iter()
        .position(|c| *c == b'\n')
        .ok_or(Error::InvalidInput("no newline"))?
        + 1;
    let starting_point = input[..line_width]
        .iter()
        .position(|c| *c == b'|')
        .ok_or(Error::InvalidInput("no starting point"))?;
    if input.len() % line_width != 0 {
        return Err(Error::InvalidInput("expected rectangular grid"));
    }
    let height = input.len() / line_width;
    for y in 0..height {
        if input[y * line_width + line_width - 1] != b'\n' {
            return Err(Error::InvalidInput(
                "expected line endings at regular interval",
            ));
        }
    }

    Ok(Input {
        data: input,
        width: line_width - 1,
        starting_point,
        height,
    })
}

tests! {
    // Explicitly listed the length here, to prevent auto-formatting from
    // messing things up (without failing compilation).
    const EXAMPLE: [u8; 97] = *br#"
    |          
    |  +--+    
    A  |  C    
F---|----E|--+ 
    |  |  |  D 
    +B-+  +--+ 
"#;

    test_pt!(parse, pt1, &EXAMPLE[1..] => "ABCDEF");
    test_pt!(parse, pt2, &EXAMPLE[1..] => 38);
}
