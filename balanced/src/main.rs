fn balanced(input: &String) -> bool {
    println!("Checking {:?}", input);
    let number_of_ls = input.matches('L').count();
    let number_of_rs = input.matches('R').count();
    number_of_rs == number_of_ls
}

fn main() {
    //let s = "RLRRLLRLRL".chars();
    //let s = "RLRRRLLRLL".chars();
    let s = "LLLLRRRR".chars();
    println!("{:?}", s);

    let mut pairs = 0;
    let mut buffer = "".to_string();
    for _char in s {
        //println!("char: {:?}", _char);
        if buffer.is_empty() {
            buffer.push(_char);
            //println!("{:?}", buffer);
        } else {
            buffer.push(_char);

            if balanced(&buffer) {
                println!("{:?} is balanced", buffer);
                buffer = "".to_string();
                println!("Clearing buffer {:?}", buffer);
                pairs += 1;
            }
        }
    }
    println!("Pairs: {:?}", pairs);
}
