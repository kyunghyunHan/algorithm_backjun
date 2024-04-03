use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let  pi = std::f64::consts::PI;
    let mut i: i32 = 1;
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    loop {
        let mut input = String::new();
        reader.read_line(&mut input)?;
        let v = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<f64>>();

        let d = v[0];
        let r = v[1];
        let t = v[2];

        if r == 0.0 {
            break;
        }
        let dis = d / 63360. * pi * r;
        let mph = dis / t * 3600.;
        writeln!(writer, "Trip #{}: {:.2} {:.2}", i, dis, mph)?;
        i += 1;
    }

    writer.flush()?;
    Ok(())
}
