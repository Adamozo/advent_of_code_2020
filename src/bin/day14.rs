use advent_of_code::ex14::Day14;
use aoc_utils::DaySolver;

fn main() {
    Day14::timeit_solve_default_file();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use advent_of_code::ex14;
    
    #[test]
    fn ex14_process_mask() {
        let result = ex14::process_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!((result.0, !result.1), (64, 2));
    }

    #[test]
    fn ex14_process_input() {
        use aoc_utils::read_to_string;
        let dict: HashMap<u64, u64> = HashMap::from([
            (7, 101),
            (8, 0)
        ]);
        assert_eq!(ex14::prepare_input(&read_to_string("data_files/ex14.txt").unwrap()).unwrap(), (64, 2, dict));
    }
}
