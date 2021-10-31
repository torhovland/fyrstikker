use dashmap::DashMap;
use num_bigint::{BigUint, ToBigUint};

pub fn fyrstikk_tal_kombinasjonar(mut fyrstikker: usize) -> BigUint {
    let mut kombinasjonar = 0.to_biguint().unwrap();
    let rainbow_tabellar = lag_rainbow_tabellar(fyrstikker);
    let mut tabell_storleik = største_tabell(fyrstikker);
    let mut rainbow_tabell = hent_tabell(&rainbow_tabellar, tabell_storleik);

    let mut greiner = DashMap::new();
    greiner.insert(0, 1.to_biguint().unwrap());

    loop {
        let nye_greiner = DashMap::new();
        //nye_greiner.insert(0, 1.to_biguint().unwrap());

        let nye_kombinasjonar: BigUint = greiner
            .par_iter()
            .map(|grein| {
                rainbow_tabell
                    .iter()
                    .filter(|innslag| grein.key() + innslag.key() <= fyrstikker)
                    .map(|innslag| {
                        // Om vi veit at vi har 10 kombinasjonar som treng 20 fyrstikker, så blir det 10 * 3 kombinasjonar
                        // som treng 25 fyrstikker, fordi for kvar kombinasjon kan vi legge til 2, 3 eller 5.

                        let nye_treng = grein.key() + innslag.key();
                        let nye_gongar: BigUint = grein.value() * innslag.value();

                        nye_greiner
                            .entry(nye_treng)
                            .and_modify(|gongar| *gongar += &nye_gongar)
                            .or_insert(nye_gongar.clone());

                        nye_gongar
                    })
                    //.filter(|_| *grein.key() > 0usize)
                    .sum::<BigUint>()
            })
            .sum::<BigUint>();

        println!(
            "Iterasjon med tabellstorleik på {} fann {} kombinasjonar.",
            tabell_storleik, nye_kombinasjonar
        );

        if nye_kombinasjonar > 0.to_biguint().unwrap() {
            kombinasjonar += nye_kombinasjonar;
        } else if tabell_storleik == 1 {
            return kombinasjonar;
        } else {
            // På tide å finne ein tabell med færre siffer
            tabell_storleik /= 2;
            rainbow_tabell = hent_tabell(&rainbow_tabellar, tabell_storleik);
        }

        // Vi treng ikkje dei gamle greinene lenger. No skal vi jobbe vidare med dei nye, og forsøke å legge på eit nytt
        // siffer.
        greiner = nye_greiner;
    }
}

fn lag_rainbow_tabellar(fyrstikker: usize) -> DashMap<usize, DashMap<usize, BigUint>> {
    let rainbow_tabell = DashMap::new();
    let ny_rainbow = DashMap::new();

    let mut siffer = 1;

    [
        (2, 1usize),
        (3, 1usize),
        (4, 1usize),
        (5, 3usize),
        (6, 3usize),
        (7, 1usize),
    ]
    .into_iter()
    .for_each(|(treng, nye_gongar)| {
        ny_rainbow
            .entry(treng)
            .and_modify(|gongar| *gongar += nye_gongar)
            .or_insert_with(|| nye_gongar.to_biguint().unwrap());
    });

    rainbow_tabell.insert(siffer, ny_rainbow.clone());
    let mut siste_rainbow = ny_rainbow;

    loop {
        let ny_rainbow = DashMap::new();
        siffer *= 2;

        if siffer > største_tabell(fyrstikker) {
            return rainbow_tabell;
        }

        siste_rainbow.iter().for_each(|grein| {
            siste_rainbow.iter().for_each(|grein2| {
                // Om vi veit at vi har 10 kombinasjonar som treng 20 fyrstikker, så blir det 10 * 3 kombinasjonar
                // som treng 25 fyrstikker, fordi for kvar kombinasjon kan vi legge til 2, 3 eller 5.

                let nye_treng = grein.key() + grein2.key();
                let nye_gongar = grein.value() * grein2.value();

                ny_rainbow
                    .entry(nye_treng)
                    .and_modify(|gongar| *gongar += &nye_gongar)
                    .or_insert(nye_gongar.clone());
            });
        });

        rainbow_tabell.insert(siffer, ny_rainbow.clone());
        siste_rainbow = ny_rainbow;
    }
}

fn største_tabell(fyrstikker: usize) -> usize {
    let mut siffer = 1;

    loop {
        let neste = siffer * 2;
        if neste * 2 > fyrstikker {
            return siffer;
        } else {
            siffer = neste;
        }
    }
}

fn hent_tabell(
    rainbow_tabellar: &DashMap<usize, DashMap<usize, BigUint>>,
    siffer: usize,
) -> DashMap<usize, BigUint> {
    let tabell = rainbow_tabellar.get(&siffer).unwrap().clone();
    println!(
        "Hentar tabell for {} siffer. Den har {} innslag: {:?}",
        siffer,
        tabell.len(),
        tabell
    );
    tabell
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
