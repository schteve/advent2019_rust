/*
    --- Day 1: The Tyranny of the Rocket Equation ---
    Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from fifty stars.

    Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

    The Elves quickly load you into a spacecraft and prepare to launch.

    At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of fuel required yet.

    Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.

    For example:

    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    For a mass of 1969, the fuel required is 654.
    For a mass of 100756, the fuel required is 33583.
    The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.

    What is the sum of the fuel requirements for all of the modules on your spacecraft?
*/

fn calculate_fuel(mass: u32) -> u32 {
    let fuel = mass / 3 - 2;
    fuel
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
        .map(|&mass| calculate_fuel(mass))
        .fold(0, |accumulator, fuel| accumulator + fuel);

    println!("Total fuel: {}", total);
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
}
