mod random_number_question {
    use rand::Rng;
    use std::collections::HashMap;

    pub fn create_integer_list(length: i32) -> Vec<u32> {
        let mut vector = Vec::new();
        for _ in 1..=length {
           let rand_number = rand::thread_rng().gen_range(1..101);
           vector.push(rand_number);
        }
        println!("create integer_list: {:?}", vector);
        vector
    }

    pub fn mean_output(list: &Vec<u32>) {
        let sum: u32 = list.iter().sum();
        let count = list.len() as u32;    
        let average = sum / count;
        println!("mean = {}", average)
    }

    pub fn sort_output(list: &mut Vec<u32>) {
        list.sort();
        let length = list.len();
        let mid_index = length /  2;
        let ans = list[mid_index];
        println!("sorted list: {:?}", list);
        println!("median: {}", ans);
    }

    pub fn mode_output(list: &Vec<u32>) {
        let mut count_map = HashMap::new();
        for num in list.iter() {
            let count = count_map.entry(num).or_insert(0);
            *count += 1;
        }
        println!("count map: {:?}", count_map);

        let mut max_count = 0;
        for value in count_map.values() {
            if value > &max_count {
                max_count = *value
            }
        } 

        let mut ans_list = Vec::new();
        for (number, count) in count_map.iter() {
            if &max_count == count {
                ans_list.push(number);
            }
        }
        println!("mode: {:?}", ans_list);
    }

}

pub fn output() {
   let mut data_list = random_number_question::create_integer_list(10); 
   random_number_question::mean_output(&data_list);
   random_number_question::sort_output(&mut data_list);
   random_number_question::mode_output(&data_list);
}