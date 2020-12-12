#[derive(Debug, Default)]
pub struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

pub fn parse<'a>(input: &'a str) -> Vec<Passport<'a>> {
    input
        .split("\n\n")
        .map(|batch| {
            let mut pp = Passport::default();

            for entry in batch.split_whitespace() {
                let (key, value) = entry.split_once(':').unwrap();
                match key {
                    "byr" => pp.byr = Some(value),
                    "iyr" => pp.iyr = Some(value),
                    "eyr" => pp.eyr = Some(value),
                    "hgt" => pp.hgt = Some(value),
                    "hcl" => pp.hcl = Some(value),
                    "ecl" => pp.ecl = Some(value),
                    "pid" => pp.pid = Some(value),
                    "cid" => pp.cid = Some(value),
                    _ => panic!("invalid input"),
                }
            }

            pp
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let passports = parse(input);

    passports
        .iter()
        .filter(|pp| pp.byr.is_some())
        .filter(|pp| pp.iyr.is_some())
        .filter(|pp| pp.eyr.is_some())
        .filter(|pp| pp.hgt.is_some())
        .filter(|pp| pp.hcl.is_some())
        .filter(|pp| pp.ecl.is_some())
        .filter(|pp| pp.pid.is_some())
        //.filter(|pp| pp.cid.is_some())
        .count()
}

pub fn part2(input: &str) -> usize {
    let passports = parse(input);

    // each field has strict rules about what values are valid for automatic validation:
    passports
        .iter()
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        .filter(|pp| {
            pp.byr
                .map(|byr| {
                    byr.len() == 4
                        && byr.chars().all(|c| c.is_ascii_digit())
                        && byr >= "1920"
                        && byr <= "2002"
                })
                .unwrap_or(false)
        })
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        .filter(|pp| {
            pp.iyr
                .map(|iyr| {
                    iyr.len() == 4
                        && iyr.chars().all(|c| c.is_ascii_digit())
                        && iyr >= "2010"
                        && iyr <= "2020"
                })
                .unwrap_or(false)
        })
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        .filter(|pp| {
            pp.eyr
                .map(|eyr| {
                    eyr.len() == 4
                        && eyr.chars().all(|c| c.is_ascii_digit())
                        && eyr >= "2020"
                        && eyr <= "2030"
                })
                .unwrap_or(false)
        })
        // hgt (Height) - a number followed by either cm or in: If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        .filter(|pp| {
            if let Some(hgt) = pp.hgt {
                if let Some(cm) = hgt.strip_suffix("cm") {
                    return cm >= "150" && cm <= "193";
                } else if let Some(inches) = hgt.strip_suffix("in") {
                    return inches >= "59" && inches <= "76";
                }
            }

            false
        })
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        .filter(|pp| {
            pp.hcl
                .map(|hcl| {
                    if let Some(color) = hcl.strip_prefix('#') {
                        color.chars().all(|c| {
                            c.is_ascii_digit() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c)
                        })
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
        })
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        .filter(|pp| {
            pp.ecl
                .map(|ecl| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl))
                .unwrap_or(false)
        })
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        .filter(|pp| {
            pp.pid
                .map(|pid| pid.chars().all(|c| c.is_ascii_digit()) && pid.len() == 9)
                .unwrap_or(false)
        })
        // cid (Country ID) - ignored, missing or not.
        //.filter(|pp| pp.cid.is_some())
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = include_str!("test-input");
        let solution = part1(input);
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test-input2");
        let solution = part2(input);
        assert_eq!(solution, 4);
    }
}
