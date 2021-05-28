use ithkuil_learning::checks::*;
use ithkuil_learning::variants::*;
use ithkuil_learning::morpho_phonology::*;

mod common;

#[test]
fn slot21() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Stem(Stem::Stem1),
                       Morpheme::Version(Version::Processual)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "a".to_owned());
}