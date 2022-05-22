fn main() {
    let mut _fb_num = 1000;

    println!("The Fibonacci number is {}", fb(_fb_num));
}



fn fb(mut _fb_num: u32) -> u32 {
    if _fb_num == 0 {
        return 0;
    } else if _fb_num == 1 {
        return 1;
    } else {
        let mut _fibo1 = 0; 
        let mut _fibo2 = 1; 
        
        while _fb_num > 1 {
            let next: u32 = _fibo1 + _fibo2;
            _fibo1 = _fibo2; 
            _fibo2 = next; 
            _fb_num -= 1;
        }

        return _fibo2;
    }
}
