use ithkuil_learning::checks::*;
use ithkuil_learning::variants::*;
use ithkuil_learning::morpho_phonology::*;

mod common;

#[test]
fn slot41() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Function(Function::Stative),
                       Morpheme::Specification(Specification::Basic),
                       Morpheme::Context(Context::Amalgamative)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "ao".to_owned());
}

#[test]
fn slot42() {
    let data = Data::new();
    let variant = vec![Morpheme::W, Morpheme::Function(Function::Stative),
                       Morpheme::Specification(Specification::Basic),
                       Morpheme::Context(Context::Representational)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "ia".to_owned());
}

#[test]
fn slot43() {
    let data = Data::new();
    let variant = vec![Morpheme::Y, Morpheme::Function(Function::Stative),
                       Morpheme::Specification(Specification::Basic),
                       Morpheme::Context(Context::Representational)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "uä".to_owned());
}

#[test]
fn slot44() {
    let data = Data::new();
    let variant = vec![Morpheme::W, Morpheme::Function(Function::Dynamic),
                       Morpheme::Specification(Specification::Basic),
                       Morpheme::Context(Context::Representational)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "iä".to_owned());
}