#[cfg(test)]
mod test_solutions {
    use crate::{solution_part_1, solution_part_2};

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution_part_1("testData.txt", 100), "69758324");
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(solution_part_2("testData.txt", 10000000), 149245887792);
    }
}