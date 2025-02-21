fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // this is where the error occurs
    *y = 6; 
    *z = 7;
}