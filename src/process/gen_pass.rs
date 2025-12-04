use rand::seq::{IndexedRandom, SliceRandom};

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVXYZ"; // I,O removed
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz"; // l removed
const NUMBER: &[u8] = b"123456789"; // 0 removed
const SYMBOL: &[u8] = b"!@$%&^*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
        chars.extend_from_slice(UPPER);
    }
    if lower {
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
        chars.extend_from_slice(LOWER);
    }
    if number {
        password.push(*NUMBER.choose(&mut rng).expect("NUMUBER won't be empty"));
        chars.extend_from_slice(NUMBER);
    }
    if symbol {
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL should not be empty"));
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..(length as usize - password.len()) {
        let c = chars
            .choose(&mut rng)
            .expect("Chars won't be empty in normal usage");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn::zxcvbn(&password, &[]);
    eprintln!("Password strength: {}/4", estimate.score());
    Ok(())
}
