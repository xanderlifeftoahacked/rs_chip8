fn main() {
    let test1 = 0x80;
    let test2 = 0x00FF;
    let value_a: u8 = 240;
    let value_b: u8 = 60;
    let res: u16 = value_a as u16 + value_b as u16;
    println!("{:016b}", test2);
    println!("{:016b}", test1);
}
