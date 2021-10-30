use num_bigint::{BigUint, ToBigUint};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

pub fn fyrstikk_tal_kombinasjonar(fyrstikker: usize) -> BigUint {
    let mut kombinasjonar = HashMap::new();
    let mut greiner = HashMap::new();

    if kan_skrive_null(fyrstikker) {
        kombinasjonar.insert(6usize, 1.to_biguint().unwrap());
    }

    for (treng, nye_gongar) in [
        (2, 1.to_biguint().unwrap()),
        (3, 1.to_biguint().unwrap()),
        (4, 1.to_biguint().unwrap()),
        (5, 3.to_biguint().unwrap()),
        (6, 2.to_biguint().unwrap()),
        (7, 1.to_biguint().unwrap()),
    ] {
        if treng <= fyrstikker {
            greiner
                .entry(treng)
                .and_modify(|gongar| *gongar += nye_gongar.clone())
                .or_insert_with(|| nye_gongar.clone());
            kombinasjonar
                .entry(treng)
                .and_modify(|gongar| *gongar += nye_gongar.clone())
                .or_insert(nye_gongar);
        }
    }

    loop {
        let mut nye_greiner = HashMap::new();
        let mut stopp = true;

        let nye_greiner_for_sifre: Vec<_> = [
            (2, 1.to_biguint().unwrap()),
            (3, 1.to_biguint().unwrap()),
            (4, 1.to_biguint().unwrap()),
            (5, 3.to_biguint().unwrap()),
            (6, 3.to_biguint().unwrap()),
            (7, 1.to_biguint().unwrap()),
        ]
        .into_par_iter()
        .map(|(nye_treng, nye_gongar)| {
            let nye_greiner_for_siffer: HashMap<_, _> = greiner
                .iter()
                .map(|(treng, gongar)| (treng + nye_treng, gongar * nye_gongar.clone()))
                .filter(|(treng, _)| treng <= &fyrstikker)
                .collect();

            nye_greiner_for_siffer
        })
        .collect();

        for nye_greiner_for_siffer in nye_greiner_for_sifre {
            for (treng, nye_gongar) in nye_greiner_for_siffer {
                nye_greiner
                    .entry(treng)
                    .and_modify(|gongar| *gongar += nye_gongar.clone())
                    .or_insert_with(|| nye_gongar.clone());
                kombinasjonar
                    .entry(treng)
                    .and_modify(|gongar| *gongar += nye_gongar.clone())
                    .or_insert(nye_gongar);
                stopp = false;
            }
        }

        if stopp {
            return kombinasjonar.values().sum();
        }

        greiner = nye_greiner;
    }
}

fn kan_skrive_null(fyrstikker: usize) -> bool {
    fyrstikker >= 6
}

// fn treng_fyrstikker(tal: usize) -> usize {
//     digits(tal)
//         .map(|i| match i {
//             0 => 6,
//             1 => 2,
//             2 => 5,
//             3 => 5,
//             4 => 4,
//             5 => 5,
//             6 => 6,
//             7 => 3,
//             8 => 7,
//             9 => 6,
//             _ => {
//                 panic!("Unexpected digit.");
//             }
//         })
//         .sum()
// }

// fn digits(mut num: usize) -> impl Iterator<Item = usize> {
//     let mut divisor = 1;
//     while num >= divisor * 10 {
//         divisor *= 10;
//     }

//     std::iter::from_fn(move || {
//         if divisor == 0 {
//             None
//         } else {
//             let v = num / divisor;
//             num %= divisor;
//             divisor /= 10;
//             Some(v)
//         }
//     })
// }

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
