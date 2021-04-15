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

pub fn get_morphemes() -> HashMap<Vec<Morpheme>, String> {
    let mut hashmap = HashMap::new();
    // stems + versions
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem1),
                        Morpheme::Version(Version::Processual)], "a".to_owned());
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem1),
                        Morpheme::Version(Version::Completive)], "ä".to_owned());
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem2),
                        Morpheme::Version(Version::Processual)], "e".to_owned());
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem2),
                        Morpheme::Version(Version::Completive)], "i".to_owned());
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem3),
                        Morpheme::Version(Version::Processual)], "u".to_owned());
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem3),
                        Morpheme::Version(Version::Completive)], "ü".to_owned());
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem0),
                        Morpheme::Version(Version::Processual)], "o".to_owned());
    hashmap.insert(vec![Morpheme::Stem(Stem::Stem0),
                        Morpheme::Version(Version::Completive)], "ö".to_owned());
    // function + specification + context
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Existential)], "a".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Existential)], "ä".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Existential)], "e".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Existential)], "i".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Functional)], "ai".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Functional)], "au".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Functional)], "ei".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Functional)], "eu".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Representational)], "ia/uä".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Representational)], "ie/uë".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Representational)], "io/üä".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Representational)], "iö/üë".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Amalgamative)], "ao".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Amalgamative)], "aö".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Amalgamative)], "eo".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Stative),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Amalgamative)], "eö".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Existential)], "u".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Existential)], "ü".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Existential)], "o".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Existential)], "ö".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Functional)], "ui".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Functional)], "iu".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Functional)], "oi".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Functional)], "ou".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Representational)], "ua/iä".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Representational)], "ue/ië".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Representational)], "uo/öä".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Representational)], "uö/öë".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Basic),
                        Morpheme::Context(Context::Amalgamative)], "oa".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Contential),
                        Morpheme::Context(Context::Amalgamative)], "öa".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Constitutive),
                        Morpheme::Context(Context::Amalgamative)], "oe".to_owned());
    hashmap.insert(vec![Morpheme::Function(Function::Dynamic),
                        Morpheme::Specification(Specification::Objective),
                        Morpheme::Context(Context::Amalgamative)], "öe".to_owned());
    // affiliation
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Consolidative)], "".to_owned());
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Associative)], "nļ/l".to_owned());
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Coalescent)], "rļ/r".to_owned());
    hashmap.insert(vec![Morpheme::Affiliation(Affiliation::Variative)], "řļ/ř".to_owned());
    // configuration + extension + affiliation + perspective + essence
    hashmap.insert(vec![Morpheme::Configuration(Configuration::Uniplex)], "".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(Configuration::Duplex)], "s".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexSimilar(ConfigurationRelation::Separate))], "t".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexSimilar(ConfigurationRelation::Connected))], "k".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexSimilar(ConfigurationRelation::Fused))], "p".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexSimilar(ConfigurationRelation::Separate))], "c".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexSimilar(ConfigurationRelation::Connected))], "ks".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexSimilar(ConfigurationRelation::Fused))], "ps".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexDissimilar(ConfigurationRelation::Separate))], "ţ".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexDissimilar(ConfigurationRelation::Connected))], "f".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexDissimilar(ConfigurationRelation::Fused))], "ç".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexDissimilar(ConfigurationRelation::Separate))], "ţs".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexDissimilar(ConfigurationRelation::Connected))], "fs".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexDissimilar(ConfigurationRelation::Fused))], "š".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexFuzzy(ConfigurationRelation::Separate))], "z".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexFuzzy(ConfigurationRelation::Connected))], "ž".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::MultiplexFuzzy(ConfigurationRelation::Fused))], "ż".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexFuzzy(ConfigurationRelation::Separate))], "č".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexFuzzy(ConfigurationRelation::Connected))], "kš".to_owned());
    hashmap.insert(vec![Morpheme::Configuration(
        Configuration::DuplexFuzzy(ConfigurationRelation::Fused))], "pš".to_owned());
    // extension
    hashmap.insert(vec![Morpheme::Extension(Extension::Delimitive)], "".to_owned());
    hashmap.insert(vec![Morpheme::Extension(Extension::Proximal)], "d/t".to_owned());
    hashmap.insert(vec![Morpheme::Extension(Extension::Incipient)], "g/k".to_owned());
    hashmap.insert(vec![Morpheme::Extension(Extension::Attenuative)], "b/p".to_owned());
    hashmap.insert(vec![Morpheme::Extension(Extension::Graduative)], "gz/g".to_owned());
    hashmap.insert(vec![Morpheme::Extension(Extension::Deplitive)], "bz/b".to_owned());
    // perspective + essence
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Essence(Essence::Normal)], "l/".to_owned());
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Agglomerative),
                        Morpheme::Essence(Essence::Normal)], "r".to_owned());
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Nomic),
                        Morpheme::Essence(Essence::Normal)], "v/w".to_owned());
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Abstract),
                        Morpheme::Essence(Essence::Normal)], "j/y".to_owned());
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Essence(Essence::Representative)], "tļ/l".to_owned());
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Agglomerative),
                        Morpheme::Essence(Essence::Representative)], "ř".to_owned());
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Nomic),
                        Morpheme::Essence(Essence::Representative)], "m/h".to_owned());
    hashmap.insert(vec![Morpheme::Perspective(Perspective::Abstract),
                        Morpheme::Essence(Essence::Representative)], "n/ç".to_owned());
    // cases
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Thematic))], "a".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Instrumental))], "ä".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Absolutive))], "e".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Affective))], "i".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Stimulative))], "ëi".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Effectuative))], "ö".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Ergative))], "o".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Dative))], "ü".to_owned());
    hashmap.insert(vec![Morpheme::Case(
        Case::Transrelative(Transrelative::Inducive))], "u".to_owned());
    // speaker
    hashmap.insert(vec![Morpheme::Party(Party::Speaker),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "l".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Speaker),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "r".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Speaker),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "ř".to_owned());
    // addressee
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "s".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "š".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "ž".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Neutral)], "n".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Beneficial)], "t".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Addressee),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Detrimental)], "d".to_owned());
    // third party
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "m".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "p".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "b".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Neutral)], "ň".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Beneficial)], "k".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Detrimental)], "g".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "z".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "ţ".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "ḑ".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Neutral)], "ż".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Beneficial)], "f".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                        Morpheme::Perspective(Perspective::Polyadic),
                        Morpheme::Effect(Effect::Detrimental)], "v".to_owned());
    // others
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "c".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "č".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "j".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Obviative),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "th".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Obviative),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "ph".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Obviative),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "kh".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Neutral)], "ll".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Beneficial)], "rr".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "řř".to_owned());
    hashmap.insert(vec![Morpheme::Party(Party::Provisional),
                        Morpheme::Perspective(Perspective::Monadic),
                        Morpheme::Effect(Effect::Detrimental)], "řř".to_owned());
    // illocution + expectation + validation
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Observational)], "á".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Observational)], "ái".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Observational)], "iá/uâ".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Recollective)], "â".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Recollective)], "áu".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Recollective)], "ié/oê".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Reportive)], "é".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Reportive)], "éi".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Reportive)], "ió/üâ".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Purportive)], "í".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Purportive)], "éu".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Purportive)], "iô/üê".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Performative),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Observational)], "êi".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Performative),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Observational)], "êu".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Performative),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Observational)], "eê".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Imaginary)], "ô".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Imaginary)], "óu".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Imaginary)], "uô/öê".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Conventional)], "ó".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Conventional)], "ói".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Conventional)], "uó/öâ".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Intuitive)], "û".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Intuitive)], "íu".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Intuitive)], "ué/iê".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Cognitive),
                        Morpheme::Validation(Validation::Inferential)], "ú".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Responsive),
                        Morpheme::Validation(Validation::Inferential)], "úi".to_owned());
    hashmap.insert(vec![Morpheme::Illocution(Illocution::Assertive),
                        Morpheme::Expectation(Expectation::Executive),
                        Morpheme::Validation(Validation::Inferential)], "uá/iâ".to_owned());
    hashmap
}

pub fn get_morphemes_names() -> HashMap<Morpheme, (String, String)> {
    let mut names = HashMap::new();
    names.insert(Morpheme::Stem(Stem::Stem1), ("Stem 1".to_owned(), "S1".to_owned()));
    names.insert(Morpheme::Stem(Stem::Stem2), ("Stem 2".to_owned(), "S2".to_owned()));
    names.insert(Morpheme::Stem(Stem::Stem3), ("Stem 3".to_owned(), "S3".to_owned()));
    names.insert(Morpheme::Stem(Stem::Stem0), ("Stem 0".to_owned(), "S0".to_owned()));
    names.insert(Morpheme::Version(Version::Processual), ("Processual".to_owned(), "PRC".to_owned()));
    names.insert(Morpheme::Version(Version::Completive), ("Completive".to_owned(), "CPT".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::Uniplex), ("Uniplex".to_owned(), "UNI".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexSimilar(
        ConfigurationRelation::Separate)), ("Multiplex-similar: separate".to_owned(), "MSS".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexSimilar(
        ConfigurationRelation::Connected)), ("Multiplex-similar: connected".to_owned(), "MSC".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexSimilar(
        ConfigurationRelation::Fused)), ("Multiplex-similar: fused".to_owned(), "MSF".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexDissimilar(
        ConfigurationRelation::Separate)), ("Multiplex-dissimilar: separate".to_owned(), "MDS".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexDissimilar(
        ConfigurationRelation::Connected)), ("Multiplex-dissimilar: connected".to_owned(), "MDC".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexDissimilar(
        ConfigurationRelation::Fused)), ("Multiplex-dissimilar: fused".to_owned(), "MDF".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexFuzzy(
        ConfigurationRelation::Separate)), ("Multiplex-fuzzy: separate".to_owned(), "MFS".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexFuzzy(
        ConfigurationRelation::Connected)), ("Multiplex-fuzzy: connected".to_owned(), "MFC".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::MultiplexFuzzy(
        ConfigurationRelation::Fused)), ("Multiplex-fuzzy: fused".to_owned(), "MFF".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexSimilar(
        ConfigurationRelation::Separate)), ("Duplex-similar: separate".to_owned(), "DSS".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexSimilar(
        ConfigurationRelation::Connected)), ("Duplex-similar: connected".to_owned(), "DSC".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexSimilar(
        ConfigurationRelation::Fused)), ("Duplex-similar: fused".to_owned(), "DSF".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexDissimilar(
        ConfigurationRelation::Separate)), ("Duplex-dissimilar: separate".to_owned(), "DDS".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexDissimilar(
        ConfigurationRelation::Connected)), ("Duplex-dissimilar: connected".to_owned(), "DDC".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexDissimilar(
        ConfigurationRelation::Fused)), ("Duplex-dissimilar: fused".to_owned(), "DDF".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexFuzzy(
        ConfigurationRelation::Separate)), ("Duplex-fuzzy: separate".to_owned(), "DFS".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexFuzzy(
        ConfigurationRelation::Connected)), ("Duplex-fuzzy: connected".to_owned(), "DFC".to_owned()));
    names.insert(Morpheme::Configuration(Configuration::DuplexFuzzy(
        ConfigurationRelation::Fused)), ("Duplex-fuzzy: fused".to_owned(), "DFF".to_owned()));
    names.insert(Morpheme::Extension(Extension::Delimitive), ("Delimitive".to_owned(), "DEL".to_owned()));
    names.insert(Morpheme::Extension(Extension::Proximal), ("Proximal".to_owned(), "PRX".to_owned()));
    names.insert(Morpheme::Extension(Extension::Incipient), ("Incipient".to_owned(), "ICP".to_owned()));
    names.insert(Morpheme::Extension(Extension::Attenuative), ("Attenuative".to_owned(), "ATV".to_owned()));
    names.insert(Morpheme::Extension(Extension::Graduative), ("Graduative".to_owned(), "GRA".to_owned()));
    names.insert(Morpheme::Extension(Extension::Deplitive), ("Deplitive".to_owned(), "DPL".to_owned()));
    names.insert(Morpheme::Affiliation(Affiliation::Consolidative), ("Consolidative".to_owned(), "CSL".to_owned()));
    names.insert(Morpheme::Affiliation(Affiliation::Associative), ("Associative".to_owned(), "ASO".to_owned()));
    names.insert(Morpheme::Affiliation(Affiliation::Coalescent), ("Coalescent".to_owned(), "COA".to_owned()));
    names.insert(Morpheme::Affiliation(Affiliation::Variative), ("Variative".to_owned(), "VAR".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Monadic), ("Monadic".to_owned(), "M".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Polyadic), ("Polyadic".to_owned(), "P".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Agglomerative), ("Agglomerative".to_owned(), "G".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Nomic), ("Nomic".to_owned(), "N".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Abstract), ("Abstract".to_owned(), "A".to_owned()));
    names.insert(Morpheme::Essence(Essence::Normal), ("Normal".to_owned(), "NRM".to_owned()));
    names.insert(Morpheme::Essence(Essence::Representative), ("Representative".to_owned(), "RPV".to_owned()));
    names.insert(Morpheme::Function(Function::Stative), ("Stative".to_owned(), "STA".to_owned()));
    names.insert(Morpheme::Function(Function::Dynamic), ("Dynamic".to_owned(), "DYN".to_owned()));
    names.insert(Morpheme::Specification(Specification::Basic), ("Basic".to_owned(), "BSC".to_owned()));
    names.insert(Morpheme::Specification(Specification::Contential), ("Contential".to_owned(), "CTE".to_owned()));
    names.insert(Morpheme::Specification(Specification::Constitutive), ("Constitutive".to_owned(), "CSV".to_owned()));
    names.insert(Morpheme::Specification(Specification::Objective), ("Objective".to_owned(), "OBJ".to_owned()));
    names.insert(Morpheme::Context(Context::Existential), ("Existential".to_owned(), "EXS".to_owned()));
    names.insert(Morpheme::Context(Context::Functional), ("Functional".to_owned(), "FNC".to_owned()));
    names.insert(Morpheme::Context(Context::Representational), ("Representational".to_owned(), "RPS".to_owned()));
    names.insert(Morpheme::Context(Context::Amalgamative), ("Amalgamative".to_owned(), "AMG".to_owned()));
    names.insert(Morpheme::Party(Party::Speaker), ("Speaker".to_owned(), "I".to_owned()));
    names.insert(Morpheme::Party(Party::Addressee), ("Addressee".to_owned(), "you".to_owned()));
    names.insert(Morpheme::Party(Party::ThirdParty(Animacy::Animate)),
                 ("3rd party animate".to_owned(), "he/she/they".to_owned()));
    names.insert(Morpheme::Party(Party::ThirdParty(Animacy::Inanimate)),
                 ("3rd party inanimate".to_owned(), "it/these things/those things".to_owned()));
    names.insert(Morpheme::Party(Party::ThirdParty(Animacy::Mixed)),
                 ("3rd party mixed".to_owned(), "animate+inanimate".to_owned()));
    names.insert(Morpheme::Party(Party::Obviative),
                 ("Obviative/Resumptive".to_owned(), "3rd party other than most recently referenced".to_owned()));
    names.insert(Morpheme::Party(Party::Provisional),
                 ("Provisional".to_owned(), "whatever".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Monadic), ("Monadic".to_owned(), "M".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Polyadic), ("Polyadic".to_owned(), "P".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Nomic), ("Nomic".to_owned(), "N".to_owned()));
    names.insert(Morpheme::Perspective(Perspective::Abstract), ("Abstract".to_owned(), "A".to_owned()));
    names.insert(Morpheme::Effect(Effect::Neutral), ("Neutral".to_owned(), "NEUTRAL".to_owned()));
    names.insert(Morpheme::Effect(Effect::Beneficial), ("Beneficial".to_owned(), "BENEFICIAL".to_owned()));
    names.insert(Morpheme::Effect(Effect::Detrimental), ("Detrimental".to_owned(), "DETRIMENTAL".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Thematic)), ("Thematic".to_owned(), "THM".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Instrumental)), ("Instrumental".to_owned(), "INS".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Absolutive)), ("Absolutive".to_owned(), "ABS".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Stimulative)), ("Stimulative".to_owned(), "STM".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Affective)), ("Affective".to_owned(), "AFF".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Effectuative)), ("Effectuative".to_owned(), "EFF".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Ergative)), ("Ergative".to_owned(), "ERG".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Dative)), ("Dative".to_owned(), "DAT".to_owned()));
    names.insert(Morpheme::Case(Case::Transrelative(Transrelative::Inducive)), ("Inducive".to_owned(), "IND".to_owned()));
    names.insert(Morpheme::Illocution(Illocution::Assertive), ("Assertive".to_owned(), "ASR".to_owned()));
    names.insert(Morpheme::Illocution(Illocution::Performative), ("Performative".to_owned(), "PFM".to_owned()));
    names.insert(Morpheme::Expectation(Expectation::Cognitive), ("Cognitive".to_owned(), "COG".to_owned()));
    names.insert(Morpheme::Expectation(Expectation::Responsive), ("Responsive".to_owned(), "RSP".to_owned()));
    names.insert(Morpheme::Expectation(Expectation::Executive), ("Executive".to_owned(), "EXE".to_owned()));
    names.insert(Morpheme::Validation(Validation::Observational), ("Observational".to_owned(), "OBS".to_owned()));
    names.insert(Morpheme::Validation(Validation::Recollective), ("Recollective".to_owned(), "REC".to_owned()));
    names.insert(Morpheme::Validation(Validation::Reportive), ("Reportive".to_owned(), "RPR".to_owned()));
    names.insert(Morpheme::Validation(Validation::Purportive), ("Purportive".to_owned(), "PUP".to_owned()));
    names.insert(Morpheme::Validation(Validation::Imaginary), ("Imaginary".to_owned(), "IMA".to_owned()));
    names.insert(Morpheme::Validation(Validation::Conventional), ("Conventional.to_owned()".to_owned(), "CVN".to_owned()));
    names.insert(Morpheme::Validation(Validation::Intuitive), ("Intuitive".to_owned(), "ITU".to_owned()));
    names.insert(Morpheme::Validation(Validation::Inferential), ("Inferential".to_owned(), "INF".to_owned()));
    names.insert(Morpheme::V, ("Vowel".to_owned(), "(V)".to_owned()));
    names.insert(Morpheme::C, ("Consonant".to_owned(), "(C)".to_owned()));
    names.insert(Morpheme::W, ("W".to_owned(), "(w)".to_owned()));
    names.insert(Morpheme::Y, ("Y".to_owned(), "(y)".to_owned()));
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

pub fn get_slot6_transformations() -> HashMap<String, String> {
    let out = [("pp".to_owned(), "mp".to_owned()), ("tt".to_owned(), "nt".to_owned())
        , ("kk".to_owned(), "nk".to_owned()), ("ll".to_owned(), "pļ".to_owned()), ("pb".to_owned(), "mb".to_owned())
        , ("kg".to_owned(), "ng".to_owned()), ("çy".to_owned(), "nd".to_owned()), ("rr".to_owned(), "ns".to_owned()),
        ("rř".to_owned(), "nš".to_owned()), ("řr".to_owned(), "ňs".to_owned())
        , ("řř".to_owned(), "ňš".to_owned()), ("Cgm".to_owned(), "Cx".to_owned()), ("Cgn".to_owned(), "Cň".to_owned()),
        ("ngn".to_owned(), "ňn".to_owned()), ("Cçx".to_owned(), "Cxw".to_owned())
        , ("Cbm".to_owned(), "Cv".to_owned()), ("Cbn".to_owned(), "Cḑ".to_owned()), ("ff".to_owned(), "vw".to_owned()),
        ("ţţ".to_owned(), "ḑy".to_owned())]
        .iter().cloned().collect();
    out
}