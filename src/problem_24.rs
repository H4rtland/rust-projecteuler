struct Permutations {
    list: Vec<i32>,
    current_list: Vec<i32>,
    indices: Vec<i32>,
    index: i32,
}

impl Permutations {
    fn new(list: Vec<i32>) -> Permutations {
        let l = list.len();
        let clone = list.clone();
        Permutations{list: list, indices: vec![0; l], index: 0, current_list: clone}
    }
}

impl Iterator for Permutations {
    type Item = (i32);
    fn next(&mut self) -> Option<(i32)> {
        //let mut current_indices = self.indices.clone();
        let mut carry = false;
        let mut i = self.indices.len();
        self.indices[i-1] += 1;
        while i >= 1 {
            i -= 1;
            if carry {
                self.indices[i] += 1;
                carry = false;
            }
            if self.indices[i] > (self.indices.len()-i-1) as i32 {
                carry = true;
                self.indices[i] = 0;
            }
        }
        //self.indices = current_indices.clone();
        let mut list_copy = self.list.clone();
        let mut new_list: Vec<i32> = vec![];
        for i in 0..self.list.len() {
            let popped_value = list_copy.remove(self.indices[i] as usize);
            new_list.push(popped_value);
        }
        self.current_list = new_list.clone();
        //println!("{:?}, {:?}", self.current_list, self.indices);
        self.index += 1;
        Some(self.index)
    }
}


pub fn problem_24() {
    let numbers: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut p = Permutations::new(numbers);
    println!("{:?}, {:?}", p.current_list, p.indices);
    loop {
        let index = p.next().unwrap();
        if index == 999999 {
            println!("{:?}, {:?}", p.current_list, p.indices);
            break;
        }
    }
    for i in p.current_list {
        print!("{}", i);
    }
    println!("")
}