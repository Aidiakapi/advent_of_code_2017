framework::day!(16, parse => pt1, pt2);

#[derive(Debug, Clone, Copy)]
enum Move {
    Spin(u8),
    Exchange(u8, u8),
    Partner(u8, u8),
}

fn initial_order<const N: usize>() -> [u8; N] {
    assert!(N <= 16);
    util::init_array(|idx| b'a' + idx as u8)
}

fn dance<const N: usize>(order: &mut [u8; N], moves: &[Move]) {
    for mv in moves {
        match mv {
            &Move::Spin(n) => order.rotate_right(n as usize),
            &Move::Exchange(a, b) => order.swap(a as usize, b as usize),
            &Move::Partner(a, b) => {
                let a = order.iter().position(|&c| c == a).unwrap();
                let b = order.iter().position(|&c| c == b).unwrap();
                order.swap(a, b);
            }
        }
    }
}

fn pt1(moves: &[Move]) -> AString {
    let mut order = initial_order::<16>();
    dance(&mut order, moves);
    order.to_vec()
}

fn pt2(moves: &[Move]) -> AString {
    let mut order = initial_order::<16>();
    let mut seen = HashMap::new();
    seen.insert(order, 0);

    let mut index = 0;
    loop {
        index += 1;
        dance(&mut order, moves);
        if let Some(previous_index) = seen.insert(order, index) {
            let interval = index - previous_index;
            let remainder = (1_000_000_000 - index) % interval;
            for _ in 0..remainder {
                dance(&mut order, moves);
            }
            return order.to_vec();
        }
    }
}

fn parse(input: &[u8]) -> Result<Vec<Move>> {
    use parsers::*;
    let nr = number::<u8>();
    let spin = token(b's').then(nr);
    let exchange = token(b'x').then(nr).trailed(token(b'/')).and(nr);
    let partner = token(b'p').then(any()).trailed(token(b'/')).and(any());

    let mv = spin
        .map(|n| Move::Spin(n))
        .or(exchange.map(|(a, b)| Move::Exchange(a, b)))
        .or(partner.map(|(a, b)| Move::Partner(a, b)));
    mv.sep_by(token(b',')).execute(input)
}

tests! {
    test_pt!(parse, pt1, |moves| {
        let mut order = super::initial_order::<5>();
        super::dance(&mut order, &moves);
        order.to_vec()
    },
        b"s1,x3/4,pe/b" => b"baedc"
    );
}
