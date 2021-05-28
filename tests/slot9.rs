use ithkuil_learning::checks::*;
use ithkuil_learning::variants::*;
use ithkuil_learning::morpho_phonology::*;

mod common;

#[test]
fn slot91() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Illocution(Illocution::Assertive),
                       Morpheme::Expectation(Expectation::Responsive),
    Morpheme::Validation(Validation::Observational)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "ái".to_owned());
}