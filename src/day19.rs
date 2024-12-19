use memoize::memoize;
use rustc_hash::FxHashSet as HashSet;

pub fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let mut patterns: Vec<String> = patterns.split(", ").map(String::from).collect();
    patterns.sort_by_key(|s| std::cmp::Reverse(s.len()));

    let designs = designs.lines().map(String::from).collect();

    (patterns, designs)
}

#[memoize]
fn possible_design(design: String, patterns: Vec<String>) -> bool {
    if design.is_empty() {
        return true;
    }
    let possible_patterns: Vec<String> = patterns
        .iter()
        .filter(|p| design.contains(p.as_str()))
        .map(|p| p.to_string())
        .collect();
    if possible_patterns.is_empty() {
        return false;
    }
    let possible_letters: HashSet<char> =
        possible_patterns.iter().flat_map(|p| p.chars()).collect();
    let all_letters = design.chars().collect::<HashSet<char>>();
    if all_letters.difference(&possible_letters).count() != 0 {
        return false;
    }

    possible_patterns.clone().into_iter().any(|pattern| {
        if design.starts_with(&pattern) {
            let d = &design[pattern.len()..];
            possible_design(d.to_owned(), possible_patterns.clone())
        } else {
            false
        }
    })
}

pub fn part1((patterns, designs): (Vec<String>, Vec<String>)) -> usize {
    designs
        .into_iter()
        .filter(|design| possible_design(String::from(design), patterns.clone()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb"
    };

    #[test]
    fn test_parse_input() {
        let (patterns, designs) = parse_input(INPUT);
        assert_eq!(patterns.len(), 8);
        assert_eq!(patterns[0], String::from("bwu"));
        assert_eq!(designs.len(), 8);
    }

    #[test]
    fn test_part1() {
        let (patterns, designs) = parse_input(INPUT);
        assert_eq!(part1((patterns, designs)), 6);
        // panic!();
    }

    #[test]
    fn test_long_input() {
        let input = indoc! {
            "urbu, wrrbwrg, rgug, uwb, uwg, wubwgu, bgrwu, gubbuu, wrrub, rbr, bgrr, rrgrrur, rgu, ugrrru, bgu, uubbwrb, ugru, bwb, uwuu, urb, gwubw, ubu, wubwu, ubb, rugrgw, guu, rbwg, rug, brgugb, rwbrbwu, wu, bgrb, wgburg, uuguw, wbggr, guurrbbu, rggw, guwr, rwgrbrb, urbb, bwggwg, wgr, bbwurrw, bgubr, uuu, grru, rbbrbgbr, buwub, urr, gwrgb, wrwrgubu, rurrb, bwbuub, bgbub, wguu, gbrburw, rrb, rwuu, wgwgwubb, rruw, rurru, uuw, uu, ruu, wrwb, wbrgu, gwbuu, bbgw, grgggbr, bwgww, wbuw, wwb, wubguurw, wbuu, uwurgw, ugwbbbr, rbg, uburg, wrbug, gu, bwuugb, rbuwrbur, wguur, wwwbgu, ugbb, guwubw, uwbug, rwub, uwwgug, uur, wrg, ubgbb, uw, wru, uww, uubwg, wgrgw, bu, ugb, rur, bbw, wbgru, ruug, rrbb, ugu, urbgb, bbgu, bgbwgg, gbrwb, brub, wbu, rwbuur, bgggrw, rg, bgwr, wgwwuwu, gbg, gw, ruuw, wr, bww, uwr, ubgg, guwrw, rrr, wburguu, wrbbrru, rwwrg, rwg, bgg, wgw, www, gggbrrur, gbw, rgrr, bb, bwrwww, rb, rgbbu, bubu, ubwubw, w, wgbb, gwwb, r, gru, wugr, b, bubwu, ub, uugggg, ruwuub, buw, uuruuub, gb, grbgr, wwrg, wwg, grwr, wwr, rbrb, wwrrub, uuwbwgw, gggr, rgubr, guggu, rgwwg, bggr, wrruu, grub, gbbbbr, ubw, rbubw, bbbwwww, wuu, uwuuwww, wwu, gww, wbw, rwb, gbwg, bwr, gubgu, urubwrub, uuuw, rrw, wwruu, ggg, bwrrg, bugw, urw, gggwgw, uwgrgwrr, ugr, rgru, bgr, wbbwb, brg, rwbw, bwuu, brw, bbwbuwu, grur, wgbbwu, ru, rwu, rrgb, urbwrub, ubug, bwu, wbgbbg, uuubu, wbrgwrwg, gbgg, buu, uugu, rrbuu, brbg, brwuw, rbbbw, rbgr, buggrug, gbwr, gwr, wrrw, urug, bbbb, bbrgu, rrbwuru, ugggw, bbb, rrbrw, ubg, bwubgw, ugw, urwgr, uwwrgw, ugrbwu, rggwwwb, grubb, brb, rww, ububwr, brruburb, bbrgrur, uwwbgg, gbbrr, rggu, bbwug, wrb, wgugu, br, bbu, gbr, wbg, gbuw, wwug, bru, grwrur, bbr, bugwur, wrug, rrwbu, rw, uug, rgr, ugrubu, rbb, brgwug, rgwbg, rub, gbbg, bur, grg, bubgg, ugg, rbwu, rwr, bub, rrguw, gubb, bbgr, wbbburg, bgw, wb, wubw, ggwuub, bwg, bwbbrw, ggw, grubrg, rgw, uwwuw, uwbr, grr, rgb, bwrggwg, brbwur, uru, bguuubr, wwuwg, wuruwwbw, grrwwrgr, bwbu, rbuuuw, ubru, uub, uwbb, ruw, gwg, rbur, grw, gbgbu, uwbugw, wbb, gurb, ugbwgub, rgubrw, bbugu, urg, wrr, bw, wbubruw, wwwr, rrrb, ggbbbg, wug, rgruu, guwb, uuuggug, brrbrr, gr, buwbguwu, rgg, rrwg, wwgwrr, rrg, wurur, ggwbbrg, rrwgr, rruwu, buww, gbu, rrwuruub, brgr, wguwuu, urbrbg, rru, wgb, urubb, rbw, guru, buru, gur, ww, u, rwgw, gug, wwurwuw, rrbgrw, bwuwubu, bgrw, gguuugub, rbu, rubub, wrw, ggb, buwugrg, wgu, wwrbuu, bgurbg, gwu, bbwgb, uwwrrb, urugu, wugwrrgr, gwbbr, rgubg, bwrww, ug, wrwr, brur, bwur, gwgguw, wuwub, burww, gubwru, gub, bug, bwww, gwgugg, gruw, wgg, wrub, uwuwbgu, bwbw, gwuw, ubr, gbb, ubggru, wwubuw, rgrgru, wg, wub, wrgww, bwwub, gbur, bwgu, ur, rgwrrbw, wguuurb, rwrrg, ubrbburg, rgbggww, rgbgrur, brr, wrggbwr, ggr, guw, bgwb, gwrbwg, wrwbug, ubggu, bgrrb, wbr, wgub, gwbw, grb, rwbubwur, ubgw, gwbb, ugwwggrb, ggww, wugwur, rubwg, bgugw, wbubbu, rbbw, bgug, uggu, bgwrwrw, gwb, bgb, uwug, gurguug, wrrbw, gg
            
            wuguwwwgbrbwubuuuguburugbbwgwgubwbugbgrrugwwwgbgbubgrw"
        };
        let parsed = parse_input(input);
        assert_eq!(part1(parsed), 0);
    }
}
