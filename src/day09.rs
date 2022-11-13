framework::day!(09, parse => pt1, pt2);

#[derive(Debug, Clone)]
enum Stream<'i> {
    GroupOpen,
    GroupClose,
    Garbage(&'i AStr),
}

fn pt1(input: &[Stream]) -> u32 {
    input
        .iter()
        .fold((0, 0), |(score, nesting_level), value| match value {
            Stream::GroupOpen => (score, nesting_level + 1),
            Stream::GroupClose => (score + nesting_level, nesting_level - 1),
            Stream::Garbage(_) => (score, nesting_level),
        })
        .0
}

fn pt2(input: &[Stream]) -> u32 {
    input
        .iter()
        .filter_map(|s| match s {
            Stream::Garbage(g) => Some(g),
            _ => None,
        })
        .map(|garbage| {
            garbage
                .iter()
                .fold((0, false), |(count, is_escaped), &c| {
                    if is_escaped {
                        (count, false)
                    } else if c == b'!' {
                        (count, true)
                    } else {
                        (count + 1, false)
                    }
                })
                .0
        })
        .sum()
}

fn parse(input: &[u8]) -> Result<Vec<Stream>> {
    use parsers::*;
    let garbage = token(b'<')
        .then(
            take_while(false, |is_escaped, char| match (*is_escaped, char) {
                (true, _) => {
                    *is_escaped = false;
                    true
                }
                (false, b'!') => {
                    *is_escaped = true;
                    true
                }
                (false, b'>') => false,
                (false, _) => true,
            })
            .opt()
            .map(|n| n.unwrap_or(b"")),
        )
        .trailed(token(b'>'))
        .map(Stream::Garbage);
    let open = token((b'{', Stream::GroupOpen));
    let close = token((b'}', Stream::GroupClose));
    let section = open.or(close).or(garbage);
    token(b',').opt().then(section).repeat_into().execute(input)
}

tests! {
    test_pt!(parse, pt1,
        b"{}" => 1,
        b"{{{}}}" => 6,
        b"{{},{}}" => 5,
        b"{{{},{},{{}}}}" => 16,
        b"{<a>,<a>,<a>,<a>}" => 1,
        b"{{<ab>},{<ab>},{<ab>},{<ab>}}" => 9,
        b"{{<!!>},{<!!>},{<!!>},{<!!>}}" => 9,
        b"{{<a!>},{<a!>},{<a!>},{<ab>}}" => 3,
    );
    test_pt!(parse, pt2,
        b"<>" => 0,
        b"<random characters>" => 17,
        b"<<<<>" => 3,
        b"<{!>}>" => 2,
        b"<!!>" => 0,
        b"<!!!>>" => 0,
        b"<{o\"i!a,<{i<a>" => 10,
    );
}
