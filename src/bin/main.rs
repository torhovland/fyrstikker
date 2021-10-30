use fyrstikker::fyrstikk_tal_kombinasjonar;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Du må oppgje kor mange fyrstikker du har.");

    let fyrstikker = arg
        .parse::<usize>()
        .expect("Du har ikkje oppgitt eit gyldig nummer.");

    let tal = fyrstikk_tal_kombinasjonar(fyrstikker);
    println!("Med {} fyrstikker kan du lage {} tal.", fyrstikker, tal);
}
