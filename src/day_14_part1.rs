/*
    --- Day 14: Space Stoichiometry ---
    As you approach the rings of Saturn, your ship's low fuel indicator turns on. There isn't any fuel here, but the rings have plenty of raw material. Perhaps your ship's Inter-Stellar Refinery Union brand nanofactory can turn these raw materials into fuel.

    You ask the nanofactory to produce a list of the reactions it can perform that are relevant to this process (your puzzle input). Every reaction turns some quantities of specific input chemicals into some quantity of an output chemical. Almost every chemical is produced by exactly one reaction; the only exception, ORE, is the raw material input to the entire process and is not produced by a reaction.

    You just need to know how much ORE you'll need to collect before you can produce one unit of FUEL.

    Each reaction gives specific quantities for its inputs and output; reactions cannot be partially run, so only whole integer multiples of these quantities can be used. (It's okay to have leftover chemicals when you're done, though.) For example, the reaction 1 A, 2 B, 3 C => 2 D means that exactly 2 units of chemical D can be produced by consuming exactly 1 A, 2 B and 3 C. You can run the full reaction as many times as necessary; for example, you could produce 10 D by consuming 5 A, 10 B, and 15 C.

    Suppose your nanofactory produces the following list of reactions:

    10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL
    The first two reactions use only ORE as inputs; they indicate that you can produce as much of chemical A as you want (in increments of 10 units, each 10 costing 10 ORE) and as much of chemical B as you want (each costing 1 ORE). To produce 1 FUEL, a total of 31 ORE is required: 1 ORE to produce 1 B, then 30 more ORE to produce the 7 + 7 + 7 + 7 = 28 A (with 2 extra A wasted) required in the reactions to convert the B into C, C into D, D into E, and finally E into FUEL. (30 A is produced because its reaction requires that it is created in increments of 10.)

    Or, suppose you have the following list of reactions:

    9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL
    The above list of reactions requires 165 ORE to produce 1 FUEL:

    Consume 45 ORE to produce 10 A.
    Consume 64 ORE to produce 24 B.
    Consume 56 ORE to produce 40 C.
    Consume 6 A, 8 B to produce 2 AB.
    Consume 15 B, 21 C to produce 3 BC.
    Consume 16 C, 4 A to produce 4 CA.
    Consume 2 AB, 3 BC, 4 CA to produce 1 FUEL.
    Here are some larger examples:

    13312 ORE for 1 FUEL:

    157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    180697 ORE for 1 FUEL:

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
    2210736 ORE for 1 FUEL:

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
    Given the list of reactions in your puzzle input, what is the minimum amount of ORE required to produce exactly 1 FUEL?
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
        let mut parts = input.trim().split(" ");
        let num = parts.next().unwrap().parse::<i64>().unwrap();
        let name = parts.next().unwrap();
        let id = name_map.get_id(name);

        Self {
            id: id,
            num: num,
        }
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
        let from_chemicals: Vec<Chemical> = pieces[0].split(",")
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
        let reactions: HashMap<u32, Reaction> = input.trim().lines()
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
            let chemical_debts: Vec<Chemical> = supply.iter()
                                                    .filter(|&(&k, &v)| (v < 0) && (k != ore_id))
                                                    .map(|(&k, &v)| Chemical {
                                                            id: k,
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

    fn calculate_ore_for_fuel(&mut self, num_fuel: i64) -> i64 {
        let mut supply: HashMap::<u32, i64> = HashMap::new();

        let ore_id = self.name_map.get_id("ORE");
        let fuel_id = self.name_map.get_id("FUEL");

        let fuel = Chemical {
            id: fuel_id,
            num: num_fuel
        };
        self.produce_chemical(&mut supply, &fuel);
        self.resolve_debt(&mut supply);

        // println!();
        // println!("Final supply: {:?}", supply);

        let ore_required = supply.get(&ore_id).unwrap_or(&0i64) * -1;
        ore_required
    }
}

#[aoc(day14, part1)]
pub fn solve(input: &str) -> i64 {
    let mut recipe_book = RecipeBook::from_string(input);
    let ore = recipe_book.calculate_ore_for_fuel(1);
    println!("Ore required: {}", ore);
    ore
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
        let mut recipe_book = RecipeBook::from_string(input);
        let ore = recipe_book.calculate_ore_for_fuel(1);
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
        let mut recipe_book = RecipeBook::from_string(input);
        let ore = recipe_book.calculate_ore_for_fuel(1);
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
        let mut recipe_book = RecipeBook::from_string(input);
        let ore = recipe_book.calculate_ore_for_fuel(1);
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
        let mut recipe_book = RecipeBook::from_string(input);
        let ore = recipe_book.calculate_ore_for_fuel(1);
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
        let mut recipe_book = RecipeBook::from_string(input);
        let ore = recipe_book.calculate_ore_for_fuel(1);
        assert_eq!(ore, 2210736);
    }
}
