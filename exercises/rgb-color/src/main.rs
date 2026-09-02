

#[derive(Default, Debug, Clone)]
struct RGBColor {
    red: u8,
    green: u8,
    blue: u8
}




fn main() {
    let color1 = RGBColor { red: 122, green: 60, blue: 90 };
    let display: Vec<Vec<RGBColor>> = vec![vec![color1.clone()]];
    println!("My color is RGB({:?}, {:?}, {:?})", color1.red, color1.green, color1.blue);
}
