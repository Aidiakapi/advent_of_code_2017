framework::day!(03, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<i32>;

fn calculate_position(value: u32) -> Result<Vec2> {
    let mut value = match value {
        0 => return Err(Error::InvalidInput("0 is invalid")),
        1 => return Ok(Vec2::zero()),
        _ => value - 2,
    };

    let mut radius = 1;
    while value >= radius * 8 {
        value -= radius * 8;
        radius += 1;
    }

    let value = value as i32;
    let radius = radius as i32;
    let along_edge = radius - 1 - value % (radius * 2);
    Ok(match value / (radius * 2) {
        0 => (radius, along_edge),   // right
        1 => (along_edge, -radius),  // up
        2 => (-radius, -along_edge), // left
        3 => (-along_edge, radius),  // bottom
        _ => unreachable!(),
    }
    .into())
}

fn pt1(&input: &u32) -> Result<i32> {
    let pos = calculate_position(input)?;
    Ok(pos.x.abs() + pos.y.abs())
}

fn pt2(&input: &u32) -> Result<u32> {
    let mut cells = HashMap::<Vec2, u32>::new();
    cells.insert(Vec2::zero(), 1);
    let mut i = 1;
    loop {
        i += 1;
        let pos = calculate_position(i)?;
        let value: u32 = pos
            .neighbors(&Offset::ALL)
            .filter_map(|p| cells.get(&p).cloned())
            .sum();
        if value > input {
            return Ok(value);
        }
        cells.insert(pos, value);
    }
}

fn parse(input: &[u8]) -> Result<u32> {
    use parsers::*;
    number::<u32>().execute(input)
}

tests! {
    test_pt!(parse, pt1,
        b"1" => 0,
        b"12" => 3,
        b"23" => 2,
        b"1024" => 31,
    );
}
