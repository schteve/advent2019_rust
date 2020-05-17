/*
    --- Part Two ---
    After collecting ORE for a while, you check your cargo hold: 1 trillion (1_000_000_000_000) units of ORE.

    With that much ore, given the examples above:

    The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
    The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
    The 2210736 ORE-per-FUEL example could produce 460664 FUEL.
    Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?
*/

use std::collections::HashMap;

#[derive(Debug)]
struct Chemical {
    name: String,
    num: i64,
}

#[derive(Debug)]
struct Reaction {
    from: Vec<Chemical>,
    to: Chemical,
}

fn react(supply: &mut HashMap<String, i64>, reaction: &Reaction) {
    for consumed_chemical in &reaction.from {
        let chemical_name = consumed_chemical.name.clone();
        let entry = supply.entry(chemical_name).or_insert(0);
        *entry -= consumed_chemical.num;
    }

    let produced_chemical = &reaction.to;
    let chemical_name = produced_chemical.name.clone();
    let entry = supply.entry(chemical_name).or_insert(0);
    *entry += produced_chemical.num;
}

fn produce_chemical(supply: &mut HashMap<String, i64>, reactions: &Vec<Reaction>, chemical: Chemical) {
    // Find reaction which produces chemical
    let reaction = reactions.iter()
                            .find(|&r| r.to.name == chemical.name);
    let reaction = match reaction {
        Some(r) => r,
        None => {
            println!("No reaction for: {:?}", chemical);
            return;
        }
    };

    // Calculate how many times the reaction needs to occur to produce the desired number of chemical
    let mut react_count = chemical.num / reaction.to.num;
    if chemical.num % reaction.to.num != 0 {
        react_count += 1;
    }

    // println!();
    // println!("Supply: {:?}", supply);
    // println!("React {} times: {:?}", react_count, reaction);

    // Perform reaction X times
    for _ in 0..react_count {
        react(supply, &reaction);
    }
}

fn resolve_debt(supply: &mut HashMap<String, i64>, reactions: &Vec<Reaction>) {
    loop {
        // Get all elements in the supply with a negative number (chemical debt)
        let chemical_debts = supply.iter()
                                    .filter(|&(k, v)| (*v < 0) && (k != "ORE"))
                                    .map(|(k, v)| Chemical {
                                            name: k.clone(),
                                            num: -v, // Invert to get the amount needed to produce
                                        })
                                    .collect::<Vec<Chemical>>();

        if chemical_debts.len() == 0 {
            break;
        }

        for chemical in chemical_debts {
            produce_chemical(supply, reactions, chemical);
        }
    }
}

fn chemical_from_string(input: &str) -> Chemical {
    // Parses string in format: "7 A"
    // println!("Parse: {}", input);

    let mut parts = input.trim().split(" ");
    let num = parts.next().unwrap().parse::<i64>().unwrap();
    let name = parts.next().unwrap().to_string();

    Chemical {
        name: name,
        num: num,
    }
}

fn parse_reactions(input: &str) -> Vec<Reaction> {
    let mut reactions = Vec::new();

    for line in input.trim().lines() {
        let mut halves = line.split("=>");
        let first_half = halves.next().unwrap();
        let second_half = halves.next().unwrap();

        // Parse first half of the line: 7 A, 1 B
        let from_chemicals = first_half.trim()
                                .split(",")
                                .map(|s| chemical_from_string(s))
                                .collect();

        // Parse second half
        let to_chemical = chemical_from_string(second_half.trim());

        let new_reaction = Reaction {
            from: from_chemicals,
            to: to_chemical,
        };
        reactions.push(new_reaction);
    }

    reactions
}

fn calculate_ore_for_fuel(reactions: &Vec<Reaction>, num_fuel: i64) -> i64 {
    let mut supply: HashMap::<String, i64> = HashMap::new();

    let fuel = Chemical {
        name: "FUEL".to_string(),
        num: num_fuel
    };
    produce_chemical(&mut supply, reactions, fuel);
    resolve_debt(&mut supply, reactions);

    // println!();
    // println!("Final supply: {:?}", supply);

    let ore_required = supply.get("ORE").unwrap_or(&0i64) * -1;
    ore_required
}

fn calculate_max_fuel(reactions: &Vec<Reaction>, ore_to_use: i64) -> i64 {
    let mut supply: HashMap::<String, i64> = HashMap::new();

    let mut counter = 0;
    while (supply.get("ORE").unwrap_or(&0i64) * -1) <= ore_to_use {
        let fuel = Chemical {
            name: "FUEL".to_string(),
            num: 1,
        };
        produce_chemical(&mut supply, reactions, fuel);
        resolve_debt(&mut supply, reactions);

        counter += 1;
        if counter >= 1000 {
            counter = 0;
            println!("Ore used: {}", supply.get("ORE").unwrap_or(&0i64) * -1);
        }
    }

    let max_fuel = *supply.get("FUEL").unwrap_or(&0i64) - 1; // Subtract one since the last attempt to create fuel necessarily failed.
    max_fuel

    /*let mut lower_bound = 1;
    let mut upper_bound = ore_to_use;
    while lower_bound < upper_bound {
        let mut this_attempt = (lower_bound + upper_bound) / 2;
        let ore_used = calculate_ore_for_fuel(reactions, this_attempt);
        println!("Attempt {} made {}", this_attempt, ore_used);

        if ore_used > ore_to_use {
            // Fail. Decrease upper bound.
            upper_bound = this_attempt;
            println!("Upper--");
        } else {
            // Success. Increase lower bound.
            lower_bound = this_attempt;
            println!("Lower++");
        }
    }

    lower_bound*/
}

#[aoc(day14, part2)]
pub fn solve(input: &str) -> i64 {
    let reactions = parse_reactions(&input);
    let max_fuel = calculate_max_fuel(&reactions, 1_000_000_000_000);
    println!("Max fuel: {}", max_fuel);
    max_fuel
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_ore_for_fuel() {
        let input = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
";
        let reactions = parse_reactions(input);
        let ore = calculate_ore_for_fuel(&reactions, 1);
        assert_eq!(ore, 31);

        let input = "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";
        let reactions = parse_reactions(input);
        let ore = calculate_ore_for_fuel(&reactions, 1);
        assert_eq!(ore, 165);

        let input = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";
        let reactions = parse_reactions(input);
        let ore = calculate_ore_for_fuel(&reactions, 1);
        assert_eq!(ore, 13312);

        let input = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
";
        let reactions = parse_reactions(input);
        let ore = calculate_ore_for_fuel(&reactions, 1);
        assert_eq!(ore, 180697);

        let input = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
";
        let reactions = parse_reactions(input);
        let ore = calculate_ore_for_fuel(&reactions, 1);
        assert_eq!(ore, 2210736);
    }

    #[test]
    #[ignore]
    fn test_calculate_max_fuel() {
        let input = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";
        let reactions = parse_reactions(input);
        let max_fuel = calculate_max_fuel(&reactions, 1_000_000_000_000);
        assert_eq!(max_fuel, 82892753);

        let input = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
";
        let reactions = parse_reactions(input);
        let max_fuel = calculate_max_fuel(&reactions, 1_000_000_000_000);
        assert_eq!(max_fuel, 5586022);

        let input = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
";
        let reactions = parse_reactions(input);
        let max_fuel = calculate_max_fuel(&reactions, 1_000_000_000_000);
        assert_eq!(max_fuel, 460664);
    }
}
