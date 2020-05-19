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

impl Chemical {
    fn from_string(input: &str) -> Self {
        // Parses string in format: "7 A"
        let mut parts = input.trim().split(" ");
        let num = parts.next().unwrap().parse::<i64>().unwrap();
        let name = parts.next().unwrap().to_string();

        Self {
            name: name,
            num: num,
        }
    }
}

#[derive(Debug)]
struct Reaction {
    from: Vec<Chemical>,
    to: Chemical,
}

impl Reaction {
    fn from_string(input: &str) -> Self {
        let pieces: Vec<&str> = input.split("=>").collect();
        let from_chemicals: Vec<Chemical> = pieces[0].split(",")
                                                    .map(|s| Chemical::from_string(s))
                                                    .collect();
        let to_chemical = Chemical::from_string(pieces[1]);

        Self {
            from: from_chemicals,
            to: to_chemical,
        }
    }

    fn react(&self, supply: &mut HashMap<String, i64>) {
        for consumed_chemical in &self.from {
            let chemical_name = consumed_chemical.name.clone();
            let entry = supply.entry(chemical_name).or_insert(0);
            *entry -= consumed_chemical.num;
        }

        let produced_chemical = &self.to;
        let chemical_name = produced_chemical.name.clone();
        let entry = supply.entry(chemical_name).or_insert(0);
        *entry += produced_chemical.num;
    }
}

struct RecipeBook {
    reactions: Vec<Reaction>,
}

impl RecipeBook {
    fn from_string(input: &str) -> Self {
        let reactions: Vec<Reaction> = input.trim().lines()
                                                .map(|line| Reaction::from_string(line))
                                                .collect();
        Self {
            reactions,
        }
    }

    fn produce_chemical(&self, supply: &mut HashMap<String, i64>, chemical: &Chemical) {
        // Find reaction which produces chemical
        let reaction = self.reactions.iter()
                                    .find(|&r| r.to.name == chemical.name)
                                    .expect("Reaction not found");

        // Calculate how many times the reaction needs to occur to produce the desired number of chemical
        let mut react_count = chemical.num / reaction.to.num;
        if chemical.num % reaction.to.num != 0 {
            react_count += 1;
        }

        // Perform reaction X times
        (0..react_count).for_each(|_| reaction.react(supply));
    }

    fn resolve_debt(&self, supply: &mut HashMap<String, i64>) {
        loop {
            // Get all elements in the supply with a negative number (chemical debt)
            let chemical_debts: Vec<Chemical> = supply.iter()
                                                    .filter(|&(k, v)| (*v < 0) && (k != "ORE"))
                                                    .map(|(k, v)| Chemical {
                                                            name: k.clone(),
                                                            num: -v, // Negate to get the amount needed to produce
                                                        })
                                                    .collect();

            if chemical_debts.len() == 0 {
                // No more chemical debt, return
                break;
            }

            chemical_debts.iter().for_each(|chemical| self.produce_chemical(supply, chemical));
        }
    }

    fn calculate_max_fuel(&self, ore_to_use: i64) -> i64 {
        let mut supply: HashMap::<String, i64> = HashMap::new();

        let mut counter = 0;
        while (supply.get("ORE").unwrap_or(&0i64) * -1) <= ore_to_use {
            let fuel = Chemical {
                name: "FUEL".to_string(),
                num: 1,
            };
            self.produce_chemical(&mut supply, &fuel);
            self.resolve_debt(&mut supply);

            counter += 1;
            if counter >= 1000 {
                counter = 0;
                println!("Ore used: {}", supply.get("ORE").unwrap_or(&0i64) * -1);
            }
        }

        let max_fuel = *supply.get("FUEL").unwrap_or(&0i64) - 1; // Subtract one since the last attempt to create fuel necessarily failed.
        max_fuel
    }
}

#[aoc(day14, part2)]
pub fn solve(input: &str) -> i64 {
    let recipe_book = RecipeBook::from_string(input);
    let max_fuel = recipe_book.calculate_max_fuel(1_000_000_000_000);
    println!("Max fuel: {}", max_fuel);
    max_fuel
}

#[cfg(test)]
mod test {
    use super::*;

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
        let recipe_book = RecipeBook::from_string(input);
        let max_fuel = recipe_book.calculate_max_fuel(1_000_000_000_000);
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
        let recipe_book = RecipeBook::from_string(input);
        let max_fuel = recipe_book.calculate_max_fuel(1_000_000_000_000);
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
        let recipe_book = RecipeBook::from_string(input);
        let max_fuel = recipe_book.calculate_max_fuel(1_000_000_000_000);
        assert_eq!(max_fuel, 460664);
    }
}
