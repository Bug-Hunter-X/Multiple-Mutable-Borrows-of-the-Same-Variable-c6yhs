fn main() {
    let mut x = 5;
    { // The scope here makes sure only one mutable reference is used
        let y = &mut x;
        *y = 6;
    }
    let z = &mut x;
    *z = 7;
}