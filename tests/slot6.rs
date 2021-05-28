use ithkuil_learning::checks::*;
use ithkuil_learning::variants::*;
use ithkuil_learning::morpho_phonology::*;

mod common;

#[test]
fn slot61() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Affiliation(Affiliation::Associative),
    Morpheme::Configuration(Configuration::MultiplexSimilar(ConfigurationRelation::Connected)),
    Morpheme::Extension(Extension::Incipient), Morpheme::Perspective(Perspective::Nomic),
    Morpheme::Essence(Essence::Normal)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "lnkw".to_owned());
}

#[test]
fn slot62() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Affiliation(Affiliation::Associative),
                       Morpheme::Configuration(Configuration::Uniplex),
                       Morpheme::Extension(Extension::Delimitive), Morpheme::Perspective(Perspective::Monadic),
                       Morpheme::Essence(Essence::Normal)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "nļ".to_owned());
}

#[test]
fn slot63() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Affiliation(Affiliation::Associative),
                       Morpheme::Configuration(Configuration::Uniplex),
                       Morpheme::Extension(Extension::Proximal), Morpheme::Perspective(Perspective::Agglomerative),
                       Morpheme::Essence(Essence::Normal)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "ldr".to_owned());
}

#[test]
fn slot64() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Affiliation(Affiliation::Consolidative),
                       Morpheme::Configuration(Configuration::Uniplex),
                       Morpheme::Extension(Extension::Delimitive), Morpheme::Perspective(Perspective::Nomic),
                       Morpheme::Essence(Essence::Normal)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "v".to_owned());
}

#[test]
fn slot65() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Affiliation(Affiliation::Consolidative),
                       Morpheme::Configuration(Configuration::MultiplexSimilar(ConfigurationRelation::Connected)),
                       Morpheme::Extension(Extension::Proximal), Morpheme::Perspective(Perspective::Nomic),
                       Morpheme::Essence(Essence::Representative)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "kth".to_owned());
}

#[test]
fn slot66() {
    let data = Data::new();
    let variant = vec![Morpheme::C, Morpheme::Affiliation(Affiliation::Associative),
                       Morpheme::Configuration(Configuration::MultiplexSimilar(ConfigurationRelation::Connected)),
                       Morpheme::Extension(Extension::Graduative), Morpheme::Perspective(Perspective::Nomic),
                       Morpheme::Essence(Essence::Representative)];
    let out = task(&variant, &data, false);
    assert_eq!(out.1, "lnx".to_owned());
}







