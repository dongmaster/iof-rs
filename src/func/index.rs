pub fn of(haystack: &str, needle: &str) -> uint {
    let mut counter = 0;
    
    let mut haystack_vec = Vec::new();
    let mut needle_vec = Vec::new();
    
    let needlen = needle.len();
    let mut temp_str = "".to_string();
    
    for x in haystack.chars() {
        haystack_vec.push(x);
    }
    
    
    // New
    for x in needle.chars() {
        needle_vec.push(x);
    }
    
    
    let haystack_vec_len = haystack_vec.len() - 1;
    
    /*
    // Old
    for x in range(0, haystack_vec_len) {
        
        for y in range(0, needlen) {
            temp_str.push(lmao[x + y]);
        }
        
        //println!("Needle: {} temp_str: {}", needle, temp_str);
        
        if needle == temp_str {
            println!("Found {} at {}", needle, counter);
            break;
        }
        
        if x == haystack_vec_len - 4 {
            break;
        }
        
        counter += 1;
        temp_str = "".to_string();
        
        //temp_str.push_str(lmao[x]);
        //println!("{}", temp_str);
    }
    * */
    
    // New
    for x in range(0, haystack_vec_len) {
        /*
        if x == haystack_vec_len - 1 {
            break;
        }
        * */
        
        // Check if the first letter of the needle is the same as the current haystack letter
        // we fast now
        if needle_vec[0] == haystack_vec[x] {
            for y in range(0, needlen) {
                temp_str.push(haystack_vec[x + y]);
            }
        
            if needle == temp_str {
                break;
            }
        }
        
        counter += 1;
        temp_str = "".to_string();
    }
    
    
    return counter
}
