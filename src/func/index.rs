pub fn of(haystack: &str, needle: &str) -> uint {
    let mut counter = 0;
    
    let mut haystack_vec = Vec::new();
    let mut needle_vec = Vec::new();
    
    let needlen = needle.len();
    let mut temp_str = "".to_string();
    
    for x in haystack.chars() {
        haystack_vec.push(x);
    }
    
    for x in needle.chars() {
        needle_vec.push(x);
    }
    
    let haystack_vec_len = haystack_vec.len() - 1;
    
    for x in range(0, haystack_vec_len) {
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
