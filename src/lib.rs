use num_bigint::{BigUint, ToBigUint};
use std::collections::HashMap;

pub fn fyrstikk_tal_kombinasjonar(fyrstikker: usize) -> BigUint {
    let mut greiner = HashMap::new();

    let mut kombinasjonar = if kan_skrive_null(fyrstikker) { 1 } else { 0 }
        .to_biguint()
        .unwrap();

    for (treng, nye_gongar) in [
        (2, 1.to_biguint().unwrap()),
        (3, 1.to_biguint().unwrap()),
        (4, 1.to_biguint().unwrap()),
        (5, 3.to_biguint().unwrap()),
        (6, 2.to_biguint().unwrap()), // Don't include 0 as the leading number. Zero itself is counted above.
        (7, 1.to_biguint().unwrap()),
    ] {
        if treng <= fyrstikker {
            greiner
                .entry(treng)
                .and_modify(|gongar| *gongar += nye_gongar.clone())
                .or_insert_with(|| nye_gongar.clone());

            kombinasjonar += nye_gongar;
        }
    }

    loop {
        let mut nye_greiner = HashMap::new();
        let mut stopp = true;

        for (nye_treng, nye_gongar) in [
            (2, 1.to_biguint().unwrap()),
            (3, 1.to_biguint().unwrap()),
            (4, 1.to_biguint().unwrap()),
            (5, 3.to_biguint().unwrap()),
            (6, 3.to_biguint().unwrap()),
            (7, 1.to_biguint().unwrap()),
        ] {
            greiner
                .iter()
                .filter(|(treng, _)| *treng + nye_treng <= fyrstikker)
                .for_each(|(treng, gongar)| {
                    let t = treng + nye_treng;
                    let g = gongar * nye_gongar.clone();

                    nye_greiner
                        .entry(t)
                        .and_modify(|gongar| *gongar += g.clone())
                        .or_insert_with(|| g.clone());

                    kombinasjonar += g;
                    stopp = false;
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
