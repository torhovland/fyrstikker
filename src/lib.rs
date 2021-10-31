use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use parking_lot::RwLock;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rstest::rstest;
use std::sync::Arc;

pub fn fyrstikk_tal_kombinasjonar(fyrstikker: usize) -> BigUint {
    // La vektor vere 1-basert, så vi ikkje treng å legge til og trekke frå 1 i koden lenger ned.
    let mut greiner = Arc::new(RwLock::new(vec![0.to_biguint().unwrap(); fyrstikker + 1]));

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
        greiner.write()[treng] += nye_gongar;
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
        let nye_greiner = Arc::new(RwLock::new(vec![0.to_biguint().unwrap(); fyrstikker + 1]));

        let nye_kombinasjonar: BigUint = greiner
            .clone()
            .read()
            .par_iter()
            .enumerate()
            .map(|(grein_treng, grein_gongar)| {
                let nye_greiner = Arc::clone(&nye_greiner);

                [
                    (2, 1usize),
                    (3, 1usize),
                    (4, 1usize),
                    (5, 3usize),
                    (6, 3usize),
                    (7, 1usize),
                ]
                .iter()
                .filter(|(treng, _)| grein_treng + treng <= fyrstikker)
                .map(|(treng, gongar)| {
                    // Om vi veit at vi har 10 kombinasjonar som treng 20 fyrstikker, så blir det 10 * 3 kombinasjonar
                    // som treng 25 fyrstikker, fordi for kvar kombinasjon kan vi legge til 2, 3 eller 5.

                    let nye_treng = grein_treng + treng;
                    let nye_gongar = grein_gongar * gongar;

                    nye_greiner.write()[nye_treng] += &nye_gongar;
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
