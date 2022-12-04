pub fn part_1(input: &str) -> u64 {
    let (n, vec) = parse_input(input);
    let mut gamma = 0;

    for i in 0..n {
        let mask = 2_u32.pow(i);
        let ones = vec.iter().filter(|x| *x & mask == mask).count();
        if ones > vec.len() / 2 {
            gamma += mask
        }
    }

    let mask = 2_u32.pow(n) - 1;
    let espilon = !gamma & mask;
    (gamma * espilon) as u64
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn filter_report(bit_index: u32, list: &[u32], criteria: BitCriteria) -> Vec<u32> {
    let mask = 2_u32.pow(bit_index);
    let ones: Vec<u32> = list
        .to_vec()
        .into_iter()
        .filter(|x| x & mask == mask)
        .collect();
    let zeros: Vec<u32> = list
        .to_vec()
        .into_iter()
        .filter(|x| x & mask != mask)
        .collect();

    match criteria {
        BitCriteria::MostCommon => {
            if ones.len() >= zeros.len() {
                ones
            } else {
                zeros
            }
        }
        BitCriteria::LeastCommon => {
            if zeros.len() <= ones.len() {
                zeros
            } else {
                ones
            }
        }
    }
}

fn get_rating(n_bits: u32, list: &[u32], criteria: BitCriteria) -> u32 {
    let mut candidates = list.to_vec();
    for i in (0..n_bits).rev() {
        candidates = filter_report(i, &candidates, criteria);
        if candidates.len() == 1 {
            break;
        }
    }

    *candidates.get(0).unwrap()
}
pub fn part_2(input: &str) -> u64 {
    let (n, vec) = parse_input(input);

    let oxygen = get_rating(n, &vec, BitCriteria::MostCommon);
    let co2 = get_rating(n, &vec, BitCriteria::LeastCommon);

    (oxygen * co2) as u64
}

fn parse_input(input: &str) -> (u32, Vec<u32>) {
    let lines = input.lines();
    let binaries: Vec<u32> = lines.map(|x| u32::from_str_radix(x, 2).unwrap()).collect();

    (input.clone().lines().next().unwrap().len() as u32, binaries)
}
