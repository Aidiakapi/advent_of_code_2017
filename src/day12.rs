framework::day!(12, false, parse => pt1, pt2);

type Pipe = u16;
type Connections = Vec<Pipe>;

fn pt1(input: &[Connections]) -> usize {
    let mut count = 0;
    graph::flood_fill(0, |&index| {
        count += 1;
        input[index].iter().map(|&nr| nr as usize)
    });
    count
}

fn pt2(input: &[Connections]) -> usize {
    let mut visited = vec![false; input.len()];
    let mut groups = 0;
    while let Some(start_index) = visited.iter().position(|&visited| !visited) {
        groups += 1;
        graph::dfs(start_index, |index| {
            if visited[index] {
                None
            } else {
                visited[index] = true;
                Some(input[index].iter().map(|&nr| nr as usize))
            }
            .into_iter()
            .flatten()
        });
    }
    groups
}

fn parse(input: &[u8]) -> Result<Vec<Connections>> {
    use parsers::*;
    let row = number::<Pipe>()
        .trailed(token(b" <-> "))
        .and(number::<Pipe>().sep_by(token(b", ")))
        .trailed(token(b'\n'));
    row.fold(Some(Vec::new()), |v, (index, connections)| {
        let mut v = v?;
        if index as usize != v.len() {
            None
        } else {
            v.push(connections);
            Some(v)
        }
    })
    .map_res(|opt| opt.ok_or(ParseError::Custom("indices are not consecutive")))
    .execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

    test_pt!(parse, pt1, EXAMPLE => 6);
    test_pt!(parse, pt2, EXAMPLE => 2);
}
