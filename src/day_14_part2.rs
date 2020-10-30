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

#[derive(Debug, Eq, Hash, PartialEq)]
struct Chemical {
    id: u32,
    num: i64,
}

impl Chemical {
    fn from_string(input: &str, name_map: &mut NameMap) -> Self {
        // Parses string in format: "7 A"
        let mut parts = input.trim().split(' ');
        let num = parts.next().unwrap().parse::<i64>().unwrap();
        let name = parts.next().unwrap();
        let id = name_map.get_id(name);

        Self { id, num }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Reaction {
    from: Vec<Chemical>,
    to: Chemical,
}

impl Reaction {
    fn from_string(input: &str, name_map: &mut NameMap) -> Self {
        let pieces: Vec<&str> = input.split("=>").collect();
        let from_chemicals: Vec<Chemical> = pieces[0]
            .split(',')
            .map(|s| Chemical::from_string(s, name_map))
            .collect();
        let to_chemical = Chemical::from_string(pieces[1], name_map);

        Self {
            from: from_chemicals,
            to: to_chemical,
        }
    }

    fn react(&self, supply: &mut HashMap<u32, i64>, count: i64) {
        for consumed_chemical in &self.from {
            let chemical_name = consumed_chemical.id;
            let entry = supply.entry(chemical_name).or_insert(0);
            *entry -= consumed_chemical.num * count;
        }

        let produced_chemical = &self.to;
        let chemical_name = produced_chemical.id;
        let entry = supply.entry(chemical_name).or_insert(0);
        *entry += produced_chemical.num * count;
    }
}

struct NameMap {
    map: HashMap<String, u32>,
    next_id: u32,
}

impl NameMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    fn get_id(&mut self, input: &str) -> u32 {
        let input_owned = input.to_owned();
        let id = self.map.get(&input_owned);
        if id.is_some() == true {
            *id.unwrap()
        } else {
            let next_id = self.next_id;
            self.next_id += 1;
            self.map.insert(input_owned, next_id);
            next_id
        }
    }
}

struct RecipeBook {
    reactions: HashMap<u32, Reaction>,
    name_map: NameMap,
}

impl RecipeBook {
    fn from_string(input: &str) -> Self {
        let mut name_map = NameMap::new();
        let reactions: HashMap<u32, Reaction> = input
            .trim()
            .lines()
            .map(|line| {
                let reaction = Reaction::from_string(line, &mut name_map);
                (reaction.to.id, reaction)
            })
            .collect();
        Self {
            reactions,
            name_map,
        }
    }

    fn produce_chemical(&self, supply: &mut HashMap<u32, i64>, chemical: &Chemical) {
        // Find reaction which produces chemical
        let reaction = &self.reactions[&chemical.id];

        // Calculate how many times the reaction needs to occur to produce the desired number of chemical
        let mut react_count = chemical.num / reaction.to.num;
        if chemical.num % reaction.to.num != 0 {
            react_count += 1;
        }

        // Perform reaction X times
        reaction.react(supply, react_count);
    }

    fn resolve_debt(&mut self, supply: &mut HashMap<u32, i64>) {
        loop {
            // Get all elements in the supply with a negative number (chemical debt)
            let ore_id = self.name_map.get_id("ORE");
            let chemical_debts: Vec<Chemical> = supply
                .iter()
                .filter(|&(&k, &v)| (v < 0) && (k != ore_id))
                .map(|(&k, &v)| Chemical {
                    id: k,
                    num: -v, // Negate to get the amount needed to produce
                })
                .collect();
            if chemical_debts.is_empty() == true {
                // No more chemical debt, return
                break;
            }

            chemical_debts
                .iter()
                .for_each(|chemical| self.produce_chemical(supply, chemical));
        }
    }

    fn calculate_ore_for_fuel(&mut self, num_fuel: i64) -> i64 {
        let mut supply: HashMap<u32, i64> = HashMap::new();

        let ore_id = self.name_map.get_id("ORE");
        let fuel_id = self.name_map.get_id("FUEL");

        let fuel = Chemical {
            id: fuel_id,
            num: num_fuel,
        };
        self.produce_chemical(&mut supply, &fuel);
        self.resolve_debt(&mut supply);

        // println!();
        // println!("Final supply: {:?}", supply);

        let ore_required = supply.get(&ore_id).unwrap_or(&0i64) * -1;
        ore_required
    }

    fn calculate_max_fuel(&mut self, ore_to_use: i64) -> i64 {
        let mut last_good = 0;
        let mut attempt = 1;

        // Find an upper bound on the answer
        loop {
            let ore = self.calculate_ore_for_fuel(attempt);
            //println!("{} ore makes {} fuel", ore, attempt);
            if ore <= ore_to_use {
                // Success. Increase the amount we attempt next.
                last_good = attempt;
                attempt *= 2;
            } else {
                // Fail. Move to next stage.
                break;
            }
        }

        // Now that the upper bound is known, binary search to find the answer
        let mut lower = last_good;
        let mut upper = attempt;
        while lower + 1 != upper {
            // Calculation ends with lower == upper - 1 since lower is always a good attempt and upper is always a bad one
            let middle = (lower + upper) / 2;
            let ore = self.calculate_ore_for_fuel(middle);
            //println!("{} ore makes {} fuel", ore, attempt);
            if ore <= ore_to_use {
                // Success. Increase the lower bound and thus the amount we attempt next.
                lower = middle;
            } else {
                // Fail. Decrease the upper bound and thus the amount we attempt next.
                upper = middle;
            }
        }

        lower
    }
}

#[aoc(day14, part2)]
pub fn solve(input: &str) -> i64 {
    let mut recipe_book = RecipeBook::from_string(input);
    let max_fuel = recipe_book.calculate_max_fuel(1_000_000_000_000);
    println!("Max fuel: {}", max_fuel);
    max_fuel
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
        let mut recipe_book = RecipeBook::from_string(input);
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
        let mut recipe_book = RecipeBook::from_string(input);
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
        let mut recipe_book = RecipeBook::from_string(input);
        let max_fuel = recipe_book.calculate_max_fuel(1_000_000_000_000);
        assert_eq!(max_fuel, 460664);
    }
}
