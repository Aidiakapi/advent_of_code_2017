framework::day!(04, parse => pt1, pt2);

fn pt1(input: &[Vec<Vec<u8>>]) -> usize {
    input
        .iter()
        .filter(|passphrase| passphrase.iter().all_unique())
        .count()
}

fn to_frequency_list(letters: &[u8]) -> Vec<(u8, usize)> {
    let mut frequency_list = Vec::new();
    for &letter in letters {
        match frequency_list.binary_search_by_key(&letter, |&(l, _)| l) {
            Ok(index) => frequency_list[index].1 += 1,
            Err(index) => frequency_list.insert(index, (letter, 1)),
        }
    }

    frequency_list
}

fn pt2(input: &[Vec<Vec<u8>>]) -> usize {
    input
        .iter()
        .filter(|passphrase| {
            passphrase
                .iter()
                .map(|password| to_frequency_list(password))
                .all_unique()
        })
        .count()
}

fn parse(input: &[u8]) -> Result<Vec<Vec<Vec<u8>>>> {
    use parsers::*;
    let letter = pattern!(b'a'..=b'z');
    let word = letter.repeat_into();
    word.sep_by(token(b' ')).sep_by(token(b'\n')).execute(input)
}

tests! {
    test_pt!(parse, pt1,
        b"aa bb cc dd ee" => 1,
        b"aa bb cc dd aa" => 0,
        b"aa bb cc dd aaa" => 1,
    );
    test_pt!(parse, pt2,
        b"abcde fghij" => 1,
        b"abcde xyz ecdab" => 0,
        b"a ab abc abd abf abj" => 1,
        b"iiii oiii ooii oooi oooo" => 1,
        b"oiii ioii iioi iiio" => 0,
    );
}
