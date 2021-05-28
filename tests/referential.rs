use ithkuil_learning::checks::*;
use ithkuil_learning::variants::*;
use ithkuil_learning::morpho_phonology::*;

mod common;

#[test]
fn referential1() {
    let data = Data::new();
    let variant = vec![Morpheme::Party(Party::Speaker),
                       Morpheme::Perspective(Perspective::Monadic),
                       Morpheme::Effect(Effect::Beneficial),
    Morpheme::Case(Case::Transrelative(Transrelative::Thematic))];
    let out = task(&variant, &data, true);
    assert_eq!(out.1, "ra".to_owned());
}