use std::cmp::Ordering;

framework::day!(24, parse => pt1, pt2);

type Port = (u32, u32);

fn iterate_connections(ports: &[Port], mut f: impl FnMut(&[Port])) -> Result<()> {
    if ports.len() >= 64 {
        return Err(Error::InvalidInput("too many ports in input"));
    }
    let mut lookups = Vec::with_capacity(ports.len() * 2);
    lookups.extend(ports.iter().enumerate().map(|(i, &p)| (p, i)));
    let swapped = ports.iter().enumerate().map(|(i, &p)| ((p.1, p.0), i));
    lookups.extend(swapped.filter(|(p, _)| p.0 != p.1));
    let (forward, reverse) = lookups.split_at_mut(ports.len());
    forward.sort();
    reverse.sort();

    let mut used = 0u64;
    let mut port_stack = vec![];
    let mut index_stack = vec![];

    struct Context<'a, F> {
        used: &'a mut u64,
        port_stack: &'a mut Vec<Port>,
        index_stack: &'a mut Vec<usize>,
        ports: &'a [Port],
        forward: &'a [(Port, usize)],
        reverse: &'a [(Port, usize)],
        f: F,
    }
    fn visit<F: FnMut(&[Port])>(connector: u32, context: &mut Context<F>) {
        if !context.port_stack.is_empty() {
            (context.f)(context.port_stack);
        }
        for source in [context.forward, context.reverse] {
            let start = source.partition_point(|(p, _)| p.0 < connector);
            let end = source[start..].partition_point(|(p, _)| p.0 == connector) + start;
            for &(port, index) in &source[start..end] {
                let mask = 1 << index;
                if *context.used & mask != 0 {
                    continue;
                }
                *context.used |= mask;
                context.port_stack.push(context.ports[index]);
                context.index_stack.push(index);
                visit(port.1, context);
                context.index_stack.pop();
                context.port_stack.pop();
                *context.used &= !mask;
            }
        }
    }
    visit(
        0,
        &mut Context {
            used: &mut used,
            port_stack: &mut port_stack,
            index_stack: &mut index_stack,
            ports,
            forward,
            reverse,
            f: &mut f,
        },
    );

    Ok(())
}

fn pt1(ports: &[Port]) -> Result<u32> {
    let mut max_strength = 0;
    iterate_connections(ports, |connection| {
        max_strength = max_strength.max(connection.iter().map(|p| p.0 + p.1).sum());
    })
    .map(|_| max_strength)
}

fn pt2(ports: &[Port]) -> Result<u32> {
    let mut max_length = 0;
    let mut max_strength = 0;
    iterate_connections(ports, |connection| {
        match connection.len().cmp(&max_length) {
            Ordering::Less => return,
            Ordering::Greater => {
                max_length = connection.len();
                max_strength = 0;
            }
            Ordering::Equal => {}
        }
        max_strength = max_strength.max(connection.iter().map(|p| p.0 + p.1).sum());
    })
    .map(|_| max_strength)
}

fn parse(input: &[u8]) -> Result<Vec<Port>> {
    use parsers::*;
    let nr = number::<u32>();
    let port = nr.and(token(b'/').then(nr));
    port.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    test_pt!(parse, pt1, EXAMPLE => 31);
    test_pt!(parse, pt2, EXAMPLE => 19);
}
