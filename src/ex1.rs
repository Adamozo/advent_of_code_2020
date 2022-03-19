pub fn check_sum(nums: &[i32], sum: &i32) -> Vec<i32>{
    let mut v: Vec<i32> = (*nums).to_vec();
    v.sort_unstable();

    let mut res: Vec<i32> = vec![];
    let mut right_border: u32 = (v.len()-1) as u32;
    let mut index: u32 = 0;

    while index != right_border {
        if v[index as usize] + v[right_border as usize] == *sum {
            res.push(v[index as usize]*v[right_border as usize]);
            index += 1;
        }
        else if v[index as usize] + v[right_border as usize] < *sum {
            index += 1;
        }

        else{
            right_border -= 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[1721, 979, 366, 299, 675, 1456], &2020 => vec![514579]; "test from website")]
    #[test_case(&[2040, -20, 366, 299, 675, 1456], &2020 => vec![-40800]; "negative value in array")]
    #[test_case(&[-2040, 20, 366, 299, 675, 1456], &(-2020) => vec![-40800]; "negative sum result")]
    #[test_case(&[1721, 979, 366, 0, 675, 1456], &2020 => Vec::<i32>::new(); "lack of matched pair")]
    #[test_case(&[2, 2018, 20, 2000], &2020 => vec![4036, 40000]; "2 matching pairs")]
    #[test_case(&[2, 2018, 20, 2000, 40], &2020 => vec![4036, 40000]; "2 matching pairs one exstra")]
    fn test_check_sum(nums: &[i32], sum: &i32) -> Vec<i32> {
        check_sum(nums, sum)
    }
}