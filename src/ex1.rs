pub fn run() {
    let nums: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];

    for i in 0..(nums.len()-1){
        for j in (i+1)..nums.len(){
            if nums[i] + nums[j] == 2020{
                println!("{}", nums[i]*nums[j]);
            }
        }
    }
}