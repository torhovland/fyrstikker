use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

pub fn fyrstikk_tal_kombinasjonar(fyrstikker: usize) -> usize {
    let mut kombinasjonar = HashMap::new();
    let mut greiner = vec![];

    for siffer in 1..10 {
        let treng = treng_fyrstikker(siffer);

        if treng <= fyrstikker {
            greiner.push(treng);
            let tal = kombinasjonar.entry(treng).or_insert(0usize);
            *tal += 1;
        }
    }

    loop {
        let mut nye_greiner = vec![];
        let mut stopp = true;

        let nye_greiner_for_sifre: Vec<_> = (0..10)
            .into_par_iter()
            .map(|siffer| {
                let treng = treng_fyrstikker(siffer);

                let nye_greiner_for_siffer: Vec<_> = greiner
                    .par_iter()
                    .map(|g| g + treng)
                    .filter(|g| g <= &fyrstikker)
                    .collect();

                nye_greiner_for_siffer
            })
            .collect();

        for nye_greiner_for_siffer in nye_greiner_for_sifre {
            if !nye_greiner_for_siffer.is_empty() {
                nye_greiner.extend(nye_greiner_for_siffer);
                stopp = false;
            }
        }

        nye_greiner.iter().for_each(|fyrstikker| {
            let tal = kombinasjonar.entry(*fyrstikker).or_insert(0usize);
            *tal += 1;
        });

        if stopp {
            return kombinasjonar.values().sum::<usize>()
                + if kan_skrive_null(fyrstikker) { 1 } else { 0 };
        }

        greiner = nye_greiner;
    }
}

fn kan_skrive_null(fyrstikker: usize) -> bool {
    fyrstikker >= 6
}

fn treng_fyrstikker(tal: usize) -> usize {
    digits(tal)
        .map(|i| match i {
            0 => 6,
            1 => 2,
            2 => 5,
            3 => 5,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 3,
            8 => 7,
            9 => 6,
            _ => {
                panic!("Unexpected digit.");
            }
        })
        .sum()
}

fn digits(mut num: usize) -> impl Iterator<Item = usize> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

use rstest::rstest;

#[rstest]
#[case(0, 0)]
#[case(1, 0)]
#[case(2, 1)]
#[case(3, 2)]
#[case(4, 4)]
#[case(8, 47)]
fn fyrstikk_tal_kombinasjonar_test(#[case] input: usize, #[case] expected: usize) {
    println!(
        "Med {} fyrstikker forventar vi {} kombinasjonar.",
        input, expected
    );

    assert_eq!(expected, fyrstikk_tal_kombinasjonar(input))
}
