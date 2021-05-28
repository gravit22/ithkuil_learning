use ithkuil_learning::checks::*;
use ithkuil_learning::variants::*;
use ithkuil_learning::morpho_phonology::*;

mod common;

#[test]
fn slot81() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Valence(Valence::Monoactive),
                       Morpheme::Mood(Mood::Factual)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "ah".to_owned());
}

#[test]
fn slot82() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Effect(Effect::Beneficial1),
                       Morpheme::Mood(Mood::Factual)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "iah".to_owned());
}

#[test]
fn slot83() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Aspect(Aspect::Preemptive),
                       Morpheme::Mood(Mood::Factual)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "iaw".to_owned());
}