pub fn qsqrt(number : f32) -> f32
{

    let x2 : f32 = number * 0.5;
    let threehalfs : f32 = 1.5;

    let mut i = number.to_bits();                     // evil floating point bit level hacking 
    i = 0x5f3759df - (i >> 1);                             // what the fuck?
    let y = f32::from_bits(i);
    y * (threehalfs - x2 * y * y)                          // 1st iteration
    //y * (threehalfs - x2 * y * y)                        // 2nd iteration, this can be removed
}