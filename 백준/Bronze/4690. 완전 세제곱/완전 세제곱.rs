fn main() {
    for a in 2..=100 {
        for b in 2..=100 {
            for c in (b + 1)..=100 {
                for d in (c + 1)..=100 {
                    if a * a * a == (b * b * b + c * c * c + d * d * d) {
                        println!("Cube = {}, Triple = ({},{},{})", a, b, c, d);
                    }
                    if a * a * a < (b * b * b + c * c * c + d * d * d) {
                        break;
                    }
                }
            }
        }
    }
}
