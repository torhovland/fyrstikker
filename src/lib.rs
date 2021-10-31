use dashmap::DashMap;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

pub fn fyrstikk_tal_kombinasjonar(fyrstikker: usize) -> BigUint {
    let mut greiner = DashMap::new();

    // Tell opp 0 separat, sidan det er det einaste talet som får lov til å starte med 0.
    let mut kombinasjonar = if kan_skrive_null(fyrstikker) {
        One::one()
    } else {
        Zero::zero()
    };

    // Finn alle siffer vi kan starte med. Dette blir dei initielle greinene.
    kombinasjonar += [
        (2, 1usize), // Det er 1 siffer som treng to fyrstikker.
        (4, 1usize),
        (5, 3usize), // Det er 3 siffer som treng fem fyrstikker.
        (3, 1usize),
        (6, 2usize), // Ikkje ta med 0 som fyrste siffer. Vi har allereie handtert talet 0 over.
        (7, 1usize),
    ]
    .into_iter()
    .filter(|(treng, _)| treng <= &fyrstikker)
    .map(|(treng, nye_gongar)| {
        greiner
            .entry(treng)
            .and_modify(|gongar| *gongar += nye_gongar)
            .or_insert_with(|| nye_gongar.to_biguint().unwrap());

        nye_gongar
    })
    .sum::<BigUint>();

    // Finn fleire siffer så langt det går.
    loop {
        // For kvar grein (førre siffer), får vi eit sett av nye greiner med siffer som vi har plass til. Men i staden
        // for ei eksponensiell vekst av greiner, lagar vi heller ein delt hashmap der for eksempel desse hamnar under
        // same nøkkel:

        // 41 treng 6 fyrstikker  -> map[6] = 1 tal
        // 111 treng 6 fyrstikker -> map[6] = 1 tal
        // I nye_greiner blir det -> map[6] = 2 tal

        let nye_greiner = DashMap::new();

        let nye_kombinasjonar: BigUint = greiner
            .par_iter()
            .map(|grein| {
                [
                    (2, 1usize),
                    (3, 1usize),
                    (4, 1usize),
                    (5, 3usize),
                    (6, 3usize),
                    (7, 1usize),
                ]
                .iter()
                .filter(|(treng, _)| grein.key() + treng <= fyrstikker)
                .map(|(treng, gongar)| {
                    // Om vi veit at vi har 10 kombinasjonar som treng 20 fyrstikker, så blir det 10 * 3 kombinasjonar
                    // som treng 25 fyrstikker, fordi for kvar kombinasjon kan vi legge til 2, 3 eller 5.

                    let nye_treng = grein.key() + treng;
                    let nye_gongar = grein.value() * gongar;

                    nye_greiner
                        .entry(nye_treng)
                        .and_modify(|gongar| *gongar += &nye_gongar)
                        .or_insert(nye_gongar.clone());

                    nye_gongar
                })
                .sum::<BigUint>()
            })
            .sum();

        if nye_kombinasjonar > Zero::zero() {
            kombinasjonar += nye_kombinasjonar;
        } else {
            return kombinasjonar;
        }

        // Vi treng ikkje dei gamle greinene lenger. No skal vi jobbe vidare med dei nye, og forsøke å legge på eit nytt
        // siffer.
        greiner = nye_greiner;
    }
}

fn kan_skrive_null(fyrstikker: usize) -> bool {
    fyrstikker >= 6
}

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rstest::rstest;

#[rstest]
#[case(0, 0.to_biguint().unwrap())]
#[case(1, 0.to_biguint().unwrap())]
#[case(2, 1.to_biguint().unwrap())]
#[case(3, 2.to_biguint().unwrap())]
#[case(4, 4.to_biguint().unwrap())]
#[case(8, 47.to_biguint().unwrap())]
#[case(100, BigUint::parse_bytes(b"155609448901280828126891", 10).unwrap())]
fn fyrstikk_tal_kombinasjonar_test(#[case] input: usize, #[case] expected: BigUint) {
    println!(
        "Med {} fyrstikker forventar vi {} kombinasjonar.",
        input, expected
    );

    assert_eq!(expected, fyrstikk_tal_kombinasjonar(input))
}
