/*
    --- Part Two ---
    During the second Go / No Go poll, the Elf in charge of the Rocket Equation Double-Checker stops the launch sequence. Apparently, you forgot to include additional fuel for the fuel you just added.

    Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2. However, that fuel also requires fuel, and that fuel requires fuel, and so on. Any mass that would require negative fuel should instead be treated as if it requires zero fuel; the remaining mass, if any, is instead handled by wishing really hard, which has no mass and is outside the scope of this calculation.

    So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:

    A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
    At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
    The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
    What is the sum of the fuel requirements for all of the modules on your spacecraft when also taking into account the mass of the added fuel? (Calculate the fuel requirements for each module separately, then add them all up at the end.)
*/

fn calculate_fuel(mass: u32) -> u32 {
    let fuel = mass / 3 - 2;
    fuel
}

fn calculate_total_fuel(mass: u32) -> u32 {
    let mut total_fuel = 0;

    let mut remaining_mass = mass;
    while (remaining_mass / 3) > 2 {
        let fuel = calculate_fuel(remaining_mass);
        total_fuel += fuel;
        remaining_mass = fuel;
    }

    total_fuel
}

pub fn solve() {
    let modules: [u32; 100] = [
        87819, 115026, 134815, 137411, 67764, 99126, 73336, 66216, 81346, 94695,
        76336, 148938, 100089, 67341, 101811, 83239, 58537, 146622, 140006, 95115,
        87728, 51664, 93463, 127521, 62195, 135326, 104650, 121170, 142794, 125892,
        112521, 81326, 110930, 125273, 70131, 52291, 116316, 50670, 82145, 89869,
        55474, 146525, 67064, 118129, 74723, 111269, 128051, 131256, 145221, 71059,
        137530, 94041, 92331, 134280, 133517, 59611, 113590, 96394, 64731, 53491,
        83163, 56863, 51928, 126075, 92833, 106741, 94873, 97241, 105203, 147315,
        108651, 67542, 111622, 83522, 125500, 149284, 70747, 78945, 125322, 141425,
        111995, 66892, 131105, 86896, 87588, 140571, 116504, 76218, 146224, 127819,
        59032, 102767, 137517, 126448, 141218, 102267, 78692, 96306, 56531, 80841];

    let total = modules.iter()
        .map(|&mass| calculate_total_fuel(mass))
        .fold(0, |accumulator, fuel| accumulator + fuel);

    println!("Actually total fuel: {}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

        #[test]
    fn test_calculate_total_fuel() {
        assert_eq!(calculate_total_fuel(12), 2);
        assert_eq!(calculate_total_fuel(14), 2);
        assert_eq!(calculate_total_fuel(1969), 966);
        assert_eq!(calculate_total_fuel(100756), 50346);
    }
}
