use std::mem::transmute;
fn main() {
    let precision = 24;
    let exp:i32= 30;
    let biased:i32 = 127 + exp;
    assert!(biased < 256 && biased >= 0);
    let subtraction = (exp - precision + 1) as f64; 
    let gap = f64::powf(2.0,subtraction);
    // left shift biased exponent value to bits 30-23
    let exp_mask =  (biased as u32) << 23;
    let mut prev = unsafe{
        transmute::<u32, f32>(exp_mask)
    };
    println!("Expected gap: {gap}, 0x{exp_mask:X}");
    for x in 1..20 {
        let mantissa_mask = x;
        let raw = mantissa_mask | exp_mask;
        let float = unsafe {
            transmute::<u32,f32>(raw)
        };
        let real_gap =  float - prev;
        println!("{:2} : {:.10E}            gap: {:8}        0x{raw:X}",x,float,real_gap);
        prev = float;
    }
    
}
