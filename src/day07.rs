framework::day!(07, true, parse => pt1, pt2);

fn get_root(input: &[Shoutout]) -> Result<&Shoutout> {
    let seen = input
        .iter()
        .flat_map(|shoutout| shoutout.carrying.iter().map(|vs| vs.iter()).flatten())
        .collect::<HashSet<_>>();
    input
        .iter()
        .find(|shoutout| !seen.contains(&shoutout.name))
        .ok_or(Error::NoSolution)
}

fn pt1(input: &[Shoutout]) -> Result<&AStr> {
    get_root(input).map(|shoutout| &*shoutout.name)
}

fn pt2(input: &[Shoutout]) -> Result<i32> {
    let node_map = input
        .iter()
        .map(|shoutout| (&*shoutout.name, shoutout))
        .collect::<HashMap<_, _>>();

    fn calc_weight(name: &AStr, node_map: &HashMap<&AStr, &Shoutout>) -> u32 {
        let node = node_map.get(&name).unwrap();
        let carried_weight = if let Some(carried) = &node.carrying {
            carried.iter().map(|c| calc_weight(c, node_map)).sum()
        } else {
            0
        };
        node.weight + carried_weight
    }
    let calc_weight = |name| calc_weight(name, &node_map);
    let mut current = get_root(input)?;
    let mut required_adjustment = 0i32;
    loop {
        let carrying = current.carrying.as_ref().unwrap();
        let distinct = carrying.iter().map(|c| calc_weight(c)).find_distinct();
        match distinct {
            DistinctResult::SingleDistinct(v) => {
                current = node_map[&*carrying[v.index]];
                required_adjustment = v.common as i32 - v.distinct as i32;
            }
            DistinctResult::Unique(_) => return Ok(current.weight as i32 + required_adjustment),
            _ => return Err(Error::NoSolution),
        }
    }
}

struct Shoutout {
    name: AString,
    weight: u32,
    carrying: Option<Vec<AString>>,
}

fn parse(input: &[u8]) -> Result<Vec<Shoutout>> {
    use parsers::*;
    let letter = pattern!(b'a'..=b'z');
    let word = letter.repeat_into::<AString>();
    let base = word
        .clone()
        .trailed(token(b" ("))
        .and(number::<u32>())
        .trailed(token(b')'));
    let carrying = token(b" -> ").then(word.sep_by(token(b", "))).opt();

    let shoutout = base
        .and(carrying)
        .map(|((name, weight), carrying)| Shoutout {
            name,
            weight,
            carrying,
        });
    shoutout.sep_by(token(b'\n')).execute(input)
}

tests! {
    const EXAMPLE: &'static [u8] = b"\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    test_pt!(parse, pt1, EXAMPLE => b"tknk");
    test_pt!(parse, pt2, EXAMPLE => 60);
}
