//! Represent the hex-grid by making +Y map to south, and +X map to south-east.

framework::day!(11, parse => pt1, pt2);

type Vec2 = framework::vecs::Vec2<i32>;

fn dist(p: Vec2) -> i32 {
    p.x.abs().max(p.y.abs()).max((p.x + p.y).abs())
}

fn pt1(input: &[Offset]) -> i32 {
    let target = input
        .iter()
        .fold(Vec2::zero(), |p, &o| p.neighbor(o).unwrap());
    dist(target)
}

fn pt2(input: &[Offset]) -> i32 {
    let mut current = Vec2::zero();
    input
        .iter()
        .map(|&o| {
            current = current.neighbor(o).unwrap();
            current
        })
        .map(dist)
        .max()
        .unwrap()
}

fn parse(input: &[u8]) -> Result<Vec<Offset>> {
    use parsers::*;
    #[rustfmt::skip]
    let dir = token((b"se", Offset::X_POS      ))
          .or(token((b"ne", Offset::X_POS_Y_NEG)))
          .or(token((b"nw", Offset::X_NEG      )))
          .or(token((b"sw", Offset::X_NEG_Y_POS)))
          .or(token((b'n' , Offset::Y_NEG      )))
          .or(token((b's' , Offset::Y_POS      )));
    dir.sep_by(token(b',')).execute(input)
}

tests! {
    test_pt!(parse, pt1,
        b"ne,ne,ne" => 3,
        b"ne,ne,sw,sw" => 0,
        b"ne,ne,s,s" => 2,
        b"se,sw,se,sw,sw" => 3,
    );
}
