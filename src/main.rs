extern crate rand;
use rand::Rng;

fn kenchan_builder(limit: i32) -> String {
    let mut kenchan = "けんちゃん".to_string();
    let mut kenchan_size = 5;
    let mut rng = rand::thread_rng();

    loop {
        let rest = limit - kenchan_size;

        if rest < 2 {
            break;
        } else {
            match rng.gen::<u32>() % 3 {
                0 => {
                    kenchan = format!("{}{}", kenchan, "くん".to_string());
                    kenchan_size = kenchan_size + 2;
                },
                1 => {
                    kenchan = format!("{}{}", kenchan, "さん".to_string());
                    kenchan_size = kenchan_size + 2;
                },
                2 => {
                    if rest > 2 {
                        kenchan = format!("{}{}", kenchan, "ちゃん".to_string());
                        kenchan_size = kenchan_size + 3;
                    }
                },
                _ => {
                },
            }
        }
    }

    kenchan
}

fn main() {
    println!("{}", kenchan_builder(140));
}
