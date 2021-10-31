use num_bigint::{BigUint, ToBigUint};
use std::collections::HashMap;

pub fn fyrstikk_tal_kombinasjonar(fyrstikker: usize) -> BigUint {
    let mut greiner = HashMap::new();

    let mut kombinasjonar = if kan_skrive_null(fyrstikker) { 1 } else { 0 }
        .to_biguint()
        .unwrap();

    for (treng, nye_gongar) in [
        (2, 1usize),
        (4, 1usize),
        (5, 3usize),
        (3, 1usize),
        (6, 2usize), // Don't include 0 as the leading number. Zero itself is counted above.
        (7, 1usize),
    ] {
        if treng <= fyrstikker {
            greiner
                .entry(treng)
                .and_modify(|gongar| *gongar += nye_gongar)
                .or_insert_with(|| nye_gongar.to_biguint().unwrap());

            kombinasjonar += nye_gongar;
        }
    }

    loop {
        let mut nye_greiner = HashMap::new();
        let mut stopp = true;

        for (nye_treng, nye_gongar) in [
            (2, 1usize),
            (3, 1usize),
            (4, 1usize),
            (5, 3usize),
            (6, 3usize),
            (7, 1usize),
        ] {
            greiner
                .iter()
                .filter(|(treng, _)| *treng + nye_treng <= fyrstikker)
                .for_each(|(treng, gongar)| {
                    let t = treng + nye_treng;
                    let g = gongar * nye_gongar;

                    kombinasjonar += &g;
                    stopp = false;

                    nye_greiner
                        .entry(t)
                        .and_modify(|gongar| *gongar += &g)
                        .or_insert(g);
                });
        }

        if stopp {
            return kombinasjonar;
        }

        greiner = nye_greiner;
    }
}

fn kan_skrive_null(fyrstikker: usize) -> bool {
    fyrstikker >= 6
}

use rstest::rstest;

#[rstest]
#[case(0, 0.to_biguint().unwrap())]
#[case(1, 0.to_biguint().unwrap())]
#[case(2, 1.to_biguint().unwrap())]
#[case(3, 2.to_biguint().unwrap())]
#[case(4, 4.to_biguint().unwrap())]
#[case(8, 47.to_biguint().unwrap())]
fn fyrstikk_tal_kombinasjonar_test(#[case] input: usize, #[case] expected: BigUint) {
    println!(
        "Med {} fyrstikker forventar vi {} kombinasjonar.",
        input, expected
    );

    assert_eq!(expected, fyrstikk_tal_kombinasjonar(input))
}
