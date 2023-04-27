fn main() {
    let trivial_num = 111;
    for top in 100..=999 {
        for bottom in 100..=999 {
            if top % trivial_num == 0 && bottom % trivial_num == 0 {
                continue;
            }
            if bottom * (top / 10) == top * (bottom % 100) && top % 10 == bottom / 100 {
                println!("{} / {} = {} / {}", top, bottom, top / 10, bottom % 100);
            }
        }
    }
}
