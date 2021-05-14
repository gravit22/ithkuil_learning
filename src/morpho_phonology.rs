use std::collections::HashMap;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct MorphemeContent {
    pub stems: Vec<Morpheme>,
    pub versions: Vec<Morpheme>,
    pub functions: Vec<Morpheme>,
    pub specifications: Vec<Morpheme>,
    pub contexts: Vec<Morpheme>,
    pub configurations: Vec<Morpheme>,
    pub extensions: Vec<Morpheme>,
    pub affiliations: Vec<Morpheme>,
    pub perspectives: Vec<Morpheme>,
    pub essences: Vec<Morpheme>,
    pub parties: Vec<Morpheme>,
    pub effects: Vec<Morpheme>,
    pub cases: Vec<Morpheme>,
    pub illocutions: Vec<Morpheme>,
    pub expectations: Vec<Morpheme>,
    pub validations: Vec<Morpheme>,
    pub consonant_forms: Vec<Morpheme>,
    pub vowel_forms: Vec<Morpheme>,
}

pub enum Formative {
    Noun,
    Verb,
}

pub enum Adjunct {
    Affixual,
    SpecialisedCsRoot,
    Modular,
    Register,
    Suppletive,
    PersonalReference,
    Mood,
    CaseScope,
    Bias,
    Parsing,
}

pub enum Word {
    Formative(Formative),
    Adjunct,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Morpheme {
    Stem(Stem),
    Version(Version),
    Root,
    Function(Function),
    Specification(Specification),
    Context(Context),
    Configuration(Configuration),
    Extension(Extension),
    Affiliation(Affiliation),
    Perspective(Perspective),
    Essence(Essence),
    Valence,
    Mood,
    Aspect,
    Phase,
    Level,
    Case(Case),
    Illocution(Illocution),
    Expectation(Expectation),
    Validation(Validation),
    Bias,
    CaseScope,
    Relation(Relation),
    Party(Party),
    Effect(Effect),
    V,
    C,
    Y,
    W,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Stem {
    Stem1,
    Stem2,
    Stem3,
    Stem0,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Version {
    Processual,
    Completive,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Function {
    Stative,
    Dynamic,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Context {
    Existential,
    Functional,
    Representational,
    Amalgamative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Specification {
    Basic,
    Contential,
    Constitutive,
    Objective,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum ConfigurationRelation {
    Separate,
    Connected,
    Fused,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Configuration {
    Uniplex,
    Duplex,
    MultiplexSimilar(ConfigurationRelation),
    MultiplexDissimilar(ConfigurationRelation),
    MultiplexFuzzy(ConfigurationRelation),
    DuplexSimilar(ConfigurationRelation),
    DuplexDissimilar(ConfigurationRelation),
    DuplexFuzzy(ConfigurationRelation),
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Extension {
    Delimitive,
    Proximal,
    Incipient,
    Attenuative,
    Graduative,
    Deplitive,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Affiliation {
    Consolidative,
    Associative,
    Coalescent,
    Variative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Perspective {
    Monadic,
    Agglomerative,
    Nomic,
    Abstract,
    Polyadic,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Essence {
    Normal,
    Representative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Transrelative {
    Thematic,
    Instrumental,
    Absolutive,
    Affective,
    Stimulative,
    Effectuative,
    Ergative,
    Dative,
    Inducive,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Appositive {
    Possessive,
    Proprietive,
    Genitive,
    Attributive,
    Productive,
    Interpretative,
    Originative,
    Interdependent,
    Partitive,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Associative {
    Applicative,
    Purposive,
    Transmissive,
    Deferential,
    Contrastive,
    Transpositive,
    Commutative,
    Comparative,
    Considerative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Adverbial {
    Functive,
    Transformative,
    Classificative,
    Resultative,
    Consumptive,
    Concessive,
    Aversive,
    Conversive,
    Situative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Relational {
    Pertinential,
    Assimilative,
    Essive,
    Correlative,
    Compositive,
    Comitative,
    Utilitative,
    Relative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Affinitive {
    Activative,
    Descriptive,
    Terminative,
    Selective,
    Conformative,
    Dependent,
    Predicative,
    Vocative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum SpatioTemporal1 {
    Locative,
    Attendant,
    Allative,
    Ablative,
    Orientative,
    Interrelative,
    Intrative,
    Navigative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum SpatioTemporal2 {
    Concursive,
    Assessive,
    Periodic,
    Prolapsive,
    Precursive,
    Postcursive,
    Elapsive,
    Prolimitive,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Case {
    Transrelative(Transrelative),
    Appositive(Appositive),
    Associative(Associative),
    Adverbial(Adverbial),
    Relational(Relational),
    Affinitive(Affinitive),
    SpatioTemporal1(SpatioTemporal1),
    SpatioTemporal2(SpatioTemporal2),
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Illocution {
    Assertive,
    Performative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Expectation {
    Cognitive,
    Responsive,
    Executive,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Validation {
    Observational,
    Recollective,
    Purportive,
    Reportive,
    Imaginary,
    Conventional,
    Intuitive,
    Inferential,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Relation {
    Framed,
    Unframed,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Animacy {
    Animate,
    Inanimate,
    Mixed,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Party {
    Speaker,
    Addressee,
    ThirdParty(Animacy),
    Obviative,
    Provisional,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Effect {
    Neutral,
    Beneficial,
    Detrimental,
}

pub fn get_morphemes() -> HashMap<Vec<Morpheme>, &'static str> {
    let mut hashmap = HashMap::new();
    // stems + versions
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem1),
                        Morpheme::Version(Version::Processual)], "a");
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem1),
                        Morpheme::Version(Version::Completive)], "ä");
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem2),
                        Morpheme::Version(Version::Processual)], "e");
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem2),
                        Morpheme::Version(Version::Completive)], "i");
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem3),
                        Morpheme::Version(Version::Processual)], "u");
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem3),
                        Morpheme::Version(Version::Completive)], "ü");
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem0),
                        Morpheme::Version(Version::Processual)], "o");
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem0),
                        Morpheme::Version(Version::Completive)], "ö");
    // function + specification + context
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Existential)], "a");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Existential)], "ä");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Existential)], "e");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Existential)], "i");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Functional)], "ai");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Functional)], "au");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Functional)], "ei");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Functional)], "eu");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Representational)], "ia/uä");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Representational)], "ie/uë");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Representational)], "io/üä");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Representational)], "iö/üë");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Amalgamative)], "ao");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Amalgamative)], "aö");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Amalgamative)], "eo");
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Amalgamative)], "eö");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Existential)], "u");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Existential)], "ü");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Existential)], "o");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Existential)], "ö");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Functional)], "ui");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Functional)], "iu");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Functional)], "oi");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Functional)], "ou");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Representational)], "ua/iä");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Representational)], "ue/ië");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Representational)], "uo/öä");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Representational)], "uö/öë");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Amalgamative)], "oa");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Amalgamative)], "öa");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Amalgamative)], "oe");
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Amalgamative)], "öe");
    // affiliation
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Consolidative)], "");
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Associative)], "nļ/l");
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Coalescent)], "rļ/r");
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Variative)], "řļ/ř");
    // configuration + extension + affiliation + perspective + essence
    hashmap.insert(vec![Morpheme::Configuration(Configuration::Uniplex)], "");
    hashmap.insert(vec![Morpheme::Configuration(Configuration::Duplex)], "s");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexSimilar(ConfigurationRelation::Separate))], "t");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexSimilar(ConfigurationRelation::Connected))], "k");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexSimilar(ConfigurationRelation::Fused))], "p");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexSimilar(ConfigurationRelation::Separate))], "c");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexSimilar(ConfigurationRelation::Connected))], "ks");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexSimilar(ConfigurationRelation::Fused))], "ps");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexDissimilar(ConfigurationRelation::Separate))], "ţ");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexDissimilar(ConfigurationRelation::Connected))], "f");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexDissimilar(ConfigurationRelation::Fused))], "ç");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexDissimilar(ConfigurationRelation::Separate))], "ţs");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexDissimilar(ConfigurationRelation::Connected))], "fs");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexDissimilar(ConfigurationRelation::Fused))], "š");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexFuzzy(ConfigurationRelation::Separate))], "z");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexFuzzy(ConfigurationRelation::Connected))], "ž");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexFuzzy(ConfigurationRelation::Fused))], "ż");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexFuzzy(ConfigurationRelation::Separate))], "č");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexFuzzy(ConfigurationRelation::Connected))], "kš");
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexFuzzy(ConfigurationRelation::Fused))], "pš");
    // extension
    hashmap.insert(vec![Morpheme::Extension(Extension::Delimitive)], "");
    hashmap.insert(vec![Morpheme::Extension(Extension::Proximal)], "d/t");
    hashmap.insert(vec![Morpheme::Extension(Extension::Incipient)], "g/k");
    hashmap.insert(vec![Morpheme::Extension(Extension::Attenuative)], "b/p");
    hashmap.insert(vec![Morpheme::Extension(Extension::Graduative)], "gz/g");
    hashmap.insert(vec![Morpheme::Extension(Extension::Deplitive)], "bz/b");
    // perspective + essence
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Essence(Essence::Normal)], "l/");
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Agglomerative),
                        Morpheme::Essence(Essence::Normal)], "r");
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Nomic),
                        Morpheme::Essence(Essence::Normal)], "v/w");
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Abstract),
                        Morpheme::Essence(Essence::Normal)], "j/y");
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Essence(Essence::Representative)], "tļ/l");
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Agglomerative),
                        Morpheme::Essence(Essence::Representative)], "ř");
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Nomic),
                        Morpheme::Essence(Essence::Representative)], "m/h");
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Abstract),
                        Morpheme::Essence(Essence::Representative)], "n/ç");
    // cases
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Thematic))], "a");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Instrumental))], "ä");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Absolutive))], "e");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Affective))], "i");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Stimulative))], "ëi");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Effectuative))], "ö");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Ergative))], "o");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Dative))], "ü");
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Inducive))], "u");
    // speaker
    hashmap.insert(vec![Morpheme::Party(Party::Speaker),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "l");
    hashmap.insert(vec![Morpheme::Party(Party::Speaker),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "r");
    hashmap.insert(vec![Morpheme::Party(Party::Speaker),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "ř");
    // addressee
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "s");
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "š");
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "ž");
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Neutral)], "n");
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Beneficial)], "t");
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Detrimental)], "d");
    // third party
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "m");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "p");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "b");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Neutral)], "ň");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Beneficial)], "k");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Detrimental)], "g");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "z");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "ţ");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "ḑ");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Neutral)], "ż");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Beneficial)], "f");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Detrimental)], "v");
    // others
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "c");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "č");
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "j");
    hashmap.insert(vec![Morpheme::Party(Party::Obviative),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "th");
    hashmap.insert(vec![Morpheme::Party(Party::Obviative),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "ph");
    hashmap.insert(vec![Morpheme::Party(Party::Obviative),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "kh");
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "ll");
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "rr");
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "řř");
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "řř");
    // illocution + expectation + validation
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Observational)], "á");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Observational)], "ái");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Observational)], "iá/uâ");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Recollective)], "â");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Recollective)], "áu");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Recollective)], "ié/oê");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Reportive)], "é");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Reportive)], "éi");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Reportive)], "ió/üâ");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Purportive)], "í");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Purportive)], "éu");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Purportive)], "iô/üê");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Performative),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Observational)], "êi");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Performative),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Observational)], "êu");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Performative),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Observational)], "eê");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Imaginary)], "ô");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Imaginary)], "óu");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Imaginary)], "uô/öê");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Conventional)], "ó");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Conventional)], "ói");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Conventional)], "uó/öâ");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Intuitive)], "û");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Intuitive)], "íu");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Intuitive)], "ué/iê");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Inferential)], "ú");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Inferential)], "úi");
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Inferential)], "uá/iâ");
    hashmap
}

pub fn get_morphemes_names() -> HashMap<Morpheme, (&'static str, &'static str)> {
    let mut names = HashMap::new();
    names.insert(Morpheme::Stem(Stem::Stem1), ("Stem 1", "S1"));
    names.insert(Morpheme::Stem(Stem::Stem2), ("Stem 2", "S2"));
    names.insert(Morpheme::Stem(Stem::Stem3), ("Stem 3", "S3"));
    names.insert(Morpheme::Stem(Stem::Stem0), ("Stem 0", "S0"));
    names.insert(Morpheme::Version(Version::Processual), ("Processual", "PRC"));
    names.insert(Morpheme::Version(Version::Completive), ("Completive", "CPT"));
    names.insert(Morpheme::Configuration(Configuration::Uniplex), ("Uniplex", "UNI"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexSimilar(
        ConfigurationRelation::Separate)), ("Multiplex-similar: separate", "MSS"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexSimilar(
        ConfigurationRelation::Connected)), ("Multiplex-similar: connected", "MSC"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexSimilar(
        ConfigurationRelation::Fused)), ("Multiplex-similar: fused", "MSF"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexDissimilar(
        ConfigurationRelation::Separate)), ("Multiplex-dissimilar: separate", "MDS"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexDissimilar(
        ConfigurationRelation::Connected)), ("Multiplex-dissimilar: connected", "MDC"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexDissimilar(
        ConfigurationRelation::Fused)), ("Multiplex-dissimilar: fused", "MDF"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexFuzzy(
        ConfigurationRelation::Separate)), ("Multiplex-fuzzy: separate", "MFS"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexFuzzy(
        ConfigurationRelation::Connected)), ("Multiplex-fuzzy: connected", "MFC"));
    names.insert(Morpheme::Configuration(Configuration::MultiplexFuzzy(
        ConfigurationRelation::Fused)), ("Multiplex-fuzzy: fused", "MFF"));
    names.insert(Morpheme::Configuration(Configuration::DuplexSimilar(
        ConfigurationRelation::Separate)), ("Duplex-similar: separate", "DSS"));
    names.insert(Morpheme::Configuration(Configuration::DuplexSimilar(
        ConfigurationRelation::Connected)), ("Duplex-similar: connected", "DSC"));
    names.insert(Morpheme::Configuration(Configuration::DuplexSimilar(
        ConfigurationRelation::Fused)), ("Duplex-similar: fused", "DSF"));
    names.insert(Morpheme::Configuration(Configuration::DuplexDissimilar(
        ConfigurationRelation::Separate)), ("Duplex-dissimilar: separate", "DDS"));
    names.insert(Morpheme::Configuration(Configuration::DuplexDissimilar(
        ConfigurationRelation::Connected)), ("Duplex-dissimilar: connected", "DDC"));
    names.insert(Morpheme::Configuration(Configuration::DuplexDissimilar(
        ConfigurationRelation::Fused)), ("Duplex-dissimilar: fused", "DDF"));
    names.insert(Morpheme::Configuration(Configuration::DuplexFuzzy(
        ConfigurationRelation::Separate)), ("Duplex-fuzzy: separate", "DFS"));
    names.insert(Morpheme::Configuration(Configuration::DuplexFuzzy(
        ConfigurationRelation::Connected)), ("Duplex-fuzzy: connected", "DFC"));
    names.insert(Morpheme::Configuration(Configuration::DuplexFuzzy(
        ConfigurationRelation::Fused)), ("Duplex-fuzzy: fused", "DFF"));
    names.insert(Morpheme::Extension(Extension::Delimitive), ("Delimitive", "DEL"));
    names.insert(Morpheme::Extension(Extension::Proximal), ("Proximal", "PRX"));
    names.insert(Morpheme::Extension(Extension::Incipient), ("Incipient", "ICP"));
    names.insert(Morpheme::Extension(Extension::Attenuative), ("Attenuative", "ATV"));
    names.insert(Morpheme::Extension(Extension::Graduative), ("Graduative", "GRA"));
    names.insert(Morpheme::Extension(Extension::Deplitive), ("Deplitive", "DPL"));
    names.insert(Morpheme::Affiliation(Affiliation::Consolidative), ("Consolidative", "CSL"));
    names.insert(Morpheme::Affiliation(Affiliation::Associative), ("Associative", "ASO"));
    names.insert(Morpheme::Affiliation(Affiliation::Coalescent), ("Coalescent", "COA"));
    names.insert(Morpheme::Affiliation(Affiliation::Variative), ("Variative", "VAR"));
    names.insert(Morpheme::Perspective(Perspective::Monadic), ("Monadic", "M"));
    names.insert(Morpheme::Perspective(Perspective::Polyadic), ("Polyadic", "P"));
    names.insert(Morpheme::Perspective(Perspective::Agglomerative), ("Agglomerative", "G"));
    names.insert(Morpheme::Perspective(Perspective::Nomic), ("Nomic", "N"));
    names.insert(Morpheme::Perspective(Perspective::Abstract), ("Abstract", "A"));
    names.insert(Morpheme::Essence(Essence::Normal), ("Normal", "NRM"));
    names.insert(Morpheme::Essence(Essence::Representative), ("Representative", "RPV"));
    names.insert(Morpheme::Function(Function::Stative), ("Stative", "STA"));
    names.insert(Morpheme::Function(Function::Dynamic), ("Dynamic", "DYN"));
    names.insert(Morpheme::Specification(Specification::Basic), ("Basic", "BSC"));
    names.insert(Morpheme::Specification(Specification::Contential), ("Contential", "CTE"));
    names.insert(Morpheme::Specification(Specification::Constitutive), ("Constitutive", "CSV"));
    names.insert(Morpheme::Specification(Specification::Objective), ("Objective", "OBJ"));
    names.insert(Morpheme::Context(Context::Existential), ("Existential", "EXS"));
    names.insert(Morpheme::Context(Context::Functional), ("Functional", "FNC"));
    names.insert(Morpheme::Context(Context::Representational), ("Representational", "RPS"));
    names.insert(Morpheme::Context(Context::Amalgamative), ("Amalgamative", "AMG"));
    names.insert(Morpheme::Party(Party::Speaker), ("Speaker", "I"));
    names.insert(Morpheme::Party(Party::Addressee), ("Addressee", "you"));
    names.insert(Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                 ("3rd party animate", "he/she/they"));
    names.insert(Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                 ("3rd party inanimate", "it/these things/those things"));
    names.insert(Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                 ("3rd party mixed", "animate+inanimate"));
    names.insert(Morpheme::Party(Party::Obviative),
                 ("Obviative/Resumptive", "3rd party other than most recently referenced"));
    names.insert(Morpheme::Party(Party::Provisional),
                 ("Provisional", "whatever"));
    names.insert(Morpheme::Perspective(Perspective::Monadic), ("Monadic", "M"));
    names.insert(Morpheme::Perspective(Perspective::Polyadic), ("Polyadic", "P"));
    names.insert(Morpheme::Perspective(Perspective::Nomic), ("Nomic", "N"));
    names.insert(Morpheme::Perspective(Perspective::Abstract), ("Abstract", "A"));
    names.insert(Morpheme::Effect(Effect::Neutral), ("Neutral", "NEUTRAL"));
    names.insert(Morpheme::Effect(Effect::Beneficial), ("Beneficial", "BENEFICIAL"));
    names.insert(Morpheme::Effect(Effect::Detrimental), ("Detrimental", "DETRIMENTAL"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Thematic)), ("Thematic", "THM"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Instrumental)), ("Instrumental", "INS"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Absolutive)), ("Absolutive", "ABS"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Stimulative)), ("Stimulative", "STM"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Affective)), ("Affective", "AFF"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Effectuative)), ("Effectuative", "EFF"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Ergative)), ("Ergative", "ERG"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Dative)), ("Dative", "DAT"));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Inducive)), ("Inducive", "IND"));
    names.insert(Morpheme::Illocution(Illocution::Assertive), ("Assertive", "ASR"));
    names.insert(Morpheme::Illocution(Illocution::Performative), ("Performative", "PFM"));
    names.insert(Morpheme::Expectation(Expectation::Cognitive), ("Cognitive", "COG"));
    names.insert(Morpheme::Expectation(Expectation::Responsive), ("Responsive", "RSP"));
    names.insert(Morpheme::Expectation(Expectation::Executive), ("Executive", "EXE"));
    names.insert(Morpheme::Validation(Validation::Observational), ("Observational", "OBS"));
    names.insert(Morpheme::Validation(Validation::Recollective), ("Recollective", "REC"));
    names.insert(Morpheme::Validation(Validation::Reportive), ("Reportive", "RPR"));
    names.insert(Morpheme::Validation(Validation::Purportive), ("Purportive", "PUP"));
    names.insert(Morpheme::Validation(Validation::Imaginary), ("Imaginary", "IMA"));
    names.insert(Morpheme::Validation(Validation::Conventional), ("Conventional", "CVN"));
    names.insert(Morpheme::Validation(Validation::Intuitive), ("Intuitive", "ITU"));
    names.insert(Morpheme::Validation(Validation::Inferential), ("Inferential", "INF"));
    names.insert(Morpheme::V, ("Vowel", "(V)"));
    names.insert(Morpheme::C, ("Consonant", "(C)"));
    names.insert(Morpheme::W, ("W", "(w)"));
    names.insert(Morpheme::Y, ("Y", "(y)"));
    names
}

pub fn get_morphemes_content() -> MorphemeContent {
    let stems = vec![Morpheme::Stem(Stem::Stem1),
                     Morpheme::Stem(Stem::Stem2), Morpheme::Stem(Stem::Stem3),
                     Morpheme::Stem(Stem::Stem0)];
    let versions = vec![Morpheme::Version(Version::Processual),
                        Morpheme::Version(Version::Completive)];
    let configurations = vec![Morpheme::Configuration(Configuration::Uniplex),
                              Morpheme::Configuration(Configuration::Duplex),
                              Morpheme::Configuration(Configuration::MultiplexSimilar(
                                  ConfigurationRelation::Separate)),
                              Morpheme::Configuration(Configuration::MultiplexSimilar(
                                  ConfigurationRelation::Connected)),
                              Morpheme::Configuration(Configuration::MultiplexSimilar(
                                  ConfigurationRelation::Fused)),
                              Morpheme::Configuration(Configuration::MultiplexDissimilar(
                                  ConfigurationRelation::Separate)),
                              Morpheme::Configuration(Configuration::MultiplexDissimilar(
                                  ConfigurationRelation::Connected)),
                              Morpheme::Configuration(Configuration::MultiplexDissimilar(
                                  ConfigurationRelation::Fused)),
                              Morpheme::Configuration(Configuration::MultiplexFuzzy(
                                  ConfigurationRelation::Separate)),
                              Morpheme::Configuration(Configuration::MultiplexFuzzy(
                                  ConfigurationRelation::Connected)),
                              Morpheme::Configuration(Configuration::MultiplexFuzzy(
                                  ConfigurationRelation::Fused)),
                              Morpheme::Configuration(Configuration::DuplexSimilar(
                                  ConfigurationRelation::Separate)),
                              Morpheme::Configuration(Configuration::DuplexSimilar(
                                  ConfigurationRelation::Connected)),
                              Morpheme::Configuration(Configuration::DuplexSimilar(
                                  ConfigurationRelation::Fused)),
                              Morpheme::Configuration(Configuration::DuplexDissimilar(
                                  ConfigurationRelation::Separate)),
                              Morpheme::Configuration(Configuration::DuplexDissimilar(
                                  ConfigurationRelation::Connected)),
                              Morpheme::Configuration(Configuration::DuplexDissimilar(
                                  ConfigurationRelation::Fused)),
                              Morpheme::Configuration(Configuration::DuplexFuzzy(
                                  ConfigurationRelation::Separate)),
                              Morpheme::Configuration(Configuration::DuplexFuzzy(
                                  ConfigurationRelation::Connected)),
                              Morpheme::Configuration(Configuration::DuplexFuzzy(
                                  ConfigurationRelation::Fused))];
    let extensions = vec![Morpheme::Extension(Extension::Delimitive),
                          Morpheme::Extension(Extension::Proximal),
                          Morpheme::Extension(Extension::Incipient),
                          Morpheme::Extension(Extension::Attenuative),
                          Morpheme::Extension(Extension::Graduative),
                          Morpheme::Extension(Extension::Deplitive)];
    let affiliations = vec![Morpheme::Affiliation(Affiliation::Consolidative),
                            Morpheme::Affiliation(Affiliation::Associative),
                            Morpheme::Affiliation(Affiliation::Coalescent),
                            Morpheme::Affiliation(Affiliation::Variative)];
    let perspectives = vec![Morpheme::Perspective(Perspective::Monadic),
                           Morpheme::Perspective(Perspective::Polyadic),
                           Morpheme::Perspective(Perspective::Agglomerative),
                           Morpheme::Perspective(Perspective::Nomic),
                           Morpheme::Perspective(Perspective::Abstract)];
    let essences = vec![Morpheme::Essence(Essence::Normal),
                       Morpheme::Essence(Essence::Representative)];
    let functions = vec![Morpheme::Function(Function::Stative),
                         Morpheme::Function(Function::Dynamic)];
    let specifications = vec![Morpheme::Specification(Specification::Basic),
                              Morpheme::Specification(Specification::Contential),
                              Morpheme::Specification(Specification::Constitutive),
                              Morpheme::Specification(Specification::Objective)];
    let contexts = vec![Morpheme::Context(Context::Existential),
                        Morpheme::Context(Context::Functional),
                        Morpheme::Context(Context::Representational),
                        Morpheme::Context(Context::Amalgamative)];
    let effects = vec![Morpheme::Effect(Effect::Neutral),
                       Morpheme::Effect(Effect::Beneficial),
                       Morpheme::Effect(Effect::Detrimental)];
    let parties = vec![Morpheme::Party(Party::Speaker),
                            Morpheme::Party(Party::Addressee),
                            Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                            Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                            Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                            Morpheme::Party(Party::Obviative),
                            Morpheme::Party(Party::Provisional)];
    let cases = vec![Morpheme::Case(Case::Transrelative(Transrelative::Thematic)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Instrumental)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Absolutive)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Stimulative)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Affective)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Effectuative)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Ergative)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Dative)),
                     Morpheme::Case(Case::Transrelative(Transrelative::Inducive))];
    let illocutions = vec![Morpheme::Illocution(Illocution::Assertive),
                           Morpheme::Illocution(Illocution::Performative)];
    let expectations = vec![Morpheme::Expectation(Expectation::Cognitive),
                            Morpheme::Expectation(Expectation::Responsive),
                            Morpheme::Expectation(Expectation::Executive)];
    let validations = vec![Morpheme::Validation(Validation::Observational),
    Morpheme::Validation(Validation::Recollective),
    Morpheme::Validation(Validation::Reportive),
    Morpheme::Validation(Validation::Purportive),
    Morpheme::Validation(Validation::Imaginary),
    Morpheme::Validation(Validation::Conventional),
    Morpheme::Validation(Validation::Intuitive),
    Morpheme::Validation(Validation::Inferential)];
    let vowel_forms = vec![Morpheme::C, Morpheme::W, Morpheme::Y];
    let consonant_forms = vec![Morpheme::C, Morpheme::V];
    MorphemeContent {stems, versions, configurations, extensions, affiliations
        , perspectives, essences, functions, specifications, contexts, parties, effects, cases
        , illocutions, expectations, validations, vowel_forms, consonant_forms}
}

pub fn get_slot6_transformations() -> HashMap<&'static str, &'static str> {
    let out = [("pp", "mp"), ("tt", "nt")
        , ("kk", "nk"), ("ll", "pļ"), ("pb", "mb")
        , ("kg", "ng"), ("çy", "nd"), ("rr", "ns"),
        ("rř", "nš"), ("řr", "ňs")
        , ("řř", "ňš"), ("Cgm", "Cx"), ("Cgn", "Cň"),
        ("ngn", "ňn"), ("Cçx", "Cxw")
        , ("Cbm", "Cv"), ("Cbn", "Cḑ"), ("ff", "vw"),
        ("ţţ", "ḑy")]
        .iter().cloned().collect();
    out
}