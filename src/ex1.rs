use test_case::test_case;

#[test_case(&[1721, 979, 366, 299, 675, 1456], &2020 => vec![(299, 1721)]; "test from website")]
#[test_case(&[1721, 979, 366, 0, 675, 1456], &2020 => Vec::<(u32,u32)>::new(); "lack of matched pair")]
#[test_case(&[2, 2018, 20, 2000], &2020 => vec![(2, 2018), (20, 2000)]; "2 matching pairs")]
#[test_case(&[2, 2018, 20, 2000, 40], &2020 => vec![(2, 2018), (20, 2000)]; "2 matching pairs one exstra")]
pub fn check_sum(nums: &[u32], sum: &u32) -> Vec<(u32, u32)>{
    let mut v: Vec<u32> = (*nums).to_vec();
    v.sort_unstable();

    let mut res: Vec<(u32, u32)> = vec![];
    let mut right_border: u32 = (v.len()-1) as u32;
    let mut index = 0;

    while index != right_border {
        if v[index as usize] + v[right_border as usize] == *sum {
            res.push((v[index as usize], v[right_border as usize]));
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