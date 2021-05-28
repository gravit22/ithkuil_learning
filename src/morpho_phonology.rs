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
    pub valences: Vec<Morpheme>,
    pub phases: Vec<Morpheme>,
    pub effects8: Vec<Morpheme>,
    pub levels: Vec<Morpheme>,
    pub aspects: Vec<Morpheme>,
    pub moods: Vec<Morpheme>,
    pub case_scopes: Vec<Morpheme>,
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
    Valence(Valence),
    Mood(Mood),
    Aspect(Aspect),
    Phase(Phase),
    Level(Level),
    Case(Case),
    Illocution(Illocution),
    Expectation(Expectation),
    Validation(Validation),
    Bias,
    CaseScope(CaseScope),
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
pub enum Valence {
    Monoactive,
    Parallel,
    Corollary,
    Reciprocal,
    Complementary,
    Duplicative,
    Demonstrative,
    Contingent,
    Participative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Phase {
    Punctual,
    Iterative,
    Repetitive,
    Intermittent,
    Recurrent,
    Frequentative,
    Fragmentative,
    Vacillative,
    Fluctuative,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Level {
    Minimal,
    Subequative,
    Inferior,
    Deficient,
    Equative,
    Surpassive,
    Superlative,
    Superequative,
    Maximal,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Aspect {
    Retrospective,
    Prospective,
    Habitual,
    Progressive,
    Imminent,
    Precessive,
    Regulative,
    Summative,
    Anticipatory,
    Resumptive,
    Cessative,
    Pausal,
    Regressive,
    Preclusive,
    Continuative,
    Incessative,
    Experiential,
    Interruptive,
    Preemptive,
    Climactic,
    Dilatory,
    Temporary,
    Expenditive,
    Limitative,
    Expeditive,
    Protractive,
    Preparatory,
    Disclusive,
    Conclusive,
    Culminative,
    Intermediative,
    Tardative,
    Transitional,
    Intercommutative,
    Motive,
    Sequential,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Mood {
    Factual,
    Subjunctive,
    Assumptive,
    Speculative,
    Counterfactive,
    Hypothetical,
}

#[derive(PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum CaseScope {
    Natural,
    Antecedent,
    Subaltern,
    Qualifier,
    Precedent,
    Successive,
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
    Beneficial1,
    Beneficial2,
    Beneficial3,
    BeneficialSelf,
    Detrimental1,
    Detrimental2,
    Detrimental3,
    DetrimentalSelf,
    Unknown,
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
    //  Valence or Phase or Level or Effect or Aspect
    hashmap.insert(vec![Morpheme::Valence(Valence::Monoactive)], "a");
    hashmap.insert(vec![Morpheme::Valence(Valence::Parallel)], "ä");
    hashmap.insert(vec![Morpheme::Valence(Valence::Corollary)], "e");
    hashmap.insert(vec![Morpheme::Valence(Valence::Reciprocal)], "i");
    hashmap.insert(vec![Morpheme::Valence(Valence::Complementary)], "ëi");
    hashmap.insert(vec![Morpheme::Valence(Valence::Duplicative)], "ö");
    hashmap.insert(vec![Morpheme::Valence(Valence::Demonstrative)], "o");
    hashmap.insert(vec![Morpheme::Valence(Valence::Contingent)], "ü");
    hashmap.insert(vec![Morpheme::Valence(Valence::Participative)], "u");
    hashmap.insert(vec![Morpheme::Phase(Phase::Punctual)], "ai");
    hashmap.insert(vec![Morpheme::Phase(Phase::Iterative)], "au");
    hashmap.insert(vec![Morpheme::Phase(Phase::Repetitive)], "ei");
    hashmap.insert(vec![Morpheme::Phase(Phase::Intermittent)], "eu");
    hashmap.insert(vec![Morpheme::Phase(Phase::Recurrent)], "ëu");
    hashmap.insert(vec![Morpheme::Phase(Phase::Frequentative)], "ou");
    hashmap.insert(vec![Morpheme::Phase(Phase::Fragmentative)], "oi");
    hashmap.insert(vec![Morpheme::Phase(Phase::Vacillative)], "iu");
    hashmap.insert(vec![Morpheme::Phase(Phase::Fluctuative)], "ui");
    hashmap.insert(vec![Morpheme::Effect(Effect::Beneficial1)], "ia/uä");
    hashmap.insert(vec![Morpheme::Effect(Effect::Beneficial2)], "ie/uë");
    hashmap.insert(vec![Morpheme::Effect(Effect::Beneficial3)], "io/üä");
    hashmap.insert(vec![Morpheme::Effect(Effect::BeneficialSelf)], "iö/üë");
    hashmap.insert(vec![Morpheme::Effect(Effect::Unknown)], "eë");
    hashmap.insert(vec![Morpheme::Effect(Effect::DetrimentalSelf)], "uö/öë");
    hashmap.insert(vec![Morpheme::Effect(Effect::Detrimental3)], "uo/öä");
    hashmap.insert(vec![Morpheme::Effect(Effect::Detrimental2)], "ue/ië");
    hashmap.insert(vec![Morpheme::Effect(Effect::Detrimental1)], "ua/iä");
    hashmap.insert(vec![Morpheme::Level(Level::Minimal)], "ao");
    hashmap.insert(vec![Morpheme::Level(Level::Subequative)], "aö");
    hashmap.insert(vec![Morpheme::Level(Level::Inferior)], "eo");
    hashmap.insert(vec![Morpheme::Level(Level::Deficient)], "eö");
    hashmap.insert(vec![Morpheme::Level(Level::Equative)], "oë");
    hashmap.insert(vec![Morpheme::Level(Level::Surpassive)], "öe");
    hashmap.insert(vec![Morpheme::Level(Level::Superlative)], "oe");
    hashmap.insert(vec![Morpheme::Level(Level::Superequative)], "öa");
    hashmap.insert(vec![Morpheme::Level(Level::Maximal)], "oa");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Retrospective)], "a");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Prospective)], "ä");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Habitual)], "e");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Progressive)], "i");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Imminent)], "ëi");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Precessive)], "ö");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Regulative)], "o");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Summative)], "ü");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Anticipatory)], "u");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Resumptive)], "ai");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Cessative)], "au");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Pausal)], "ei");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Regressive)], "eu");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Preclusive)], "ëu");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Continuative)], "ou");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Incessative)], "oi");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Experiential)], "iu");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Interruptive)], "ui");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Preemptive)], "ia/uä");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Climactic)], "ie/uë");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Dilatory)], "io/üä");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Temporary)], "iö/üë");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Expenditive)], "eë");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Limitative)], "uö/öë");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Expeditive)], "uo/öä");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Protractive)], "ue/ië");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Preparatory)], "ua/iä");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Disclusive)], "ao");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Conclusive)], "aö");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Culminative)], "eo");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Intermediative)], "eö");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Tardative)], "oë");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Transitional)], "öe");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Intercommutative)], "oe");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Motive)], "öa");
    hashmap.insert(vec![Morpheme::Aspect(Aspect::Sequential)], "oa");
    // Mood or Case-Scope
    hashmap.insert(vec![Morpheme::Mood(Mood::Factual)], "h/w");
    hashmap.insert(vec![Morpheme::Mood(Mood::Subjunctive)], "hl/hw");
    hashmap.insert(vec![Morpheme::Mood(Mood::Assumptive)], "hr/hlw");
    hashmap.insert(vec![Morpheme::Mood(Mood::Speculative)], "hm/hly");
    hashmap.insert(vec![Morpheme::Mood(Mood::Counterfactive)], "hn/hnw");
    hashmap.insert(vec![Morpheme::Mood(Mood::Hypothetical)], "hň/hny");
    hashmap.insert(vec![Morpheme::CaseScope(CaseScope::Natural)], "h/w");
    hashmap.insert(vec![Morpheme::CaseScope(CaseScope::Antecedent)], "hl/hw");
    hashmap.insert(vec![Morpheme::CaseScope(CaseScope::Subaltern)], "hr/hlw");
    hashmap.insert(vec![Morpheme::CaseScope(CaseScope::Qualifier)], "hm/hly");
    hashmap.insert(vec![Morpheme::CaseScope(CaseScope::Precedent)], "hn/hnw");
    hashmap.insert(vec![Morpheme::CaseScope(CaseScope::Successive)], "hň/hny");
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
    //  Valence or Phase or Level or Effect or Aspect
    names.insert(Morpheme::Valence(Valence::Monoactive), ("Monoactive", "MNO"));
    names.insert(Morpheme::Valence(Valence::Parallel), ("Parallel", "PRL"));
    names.insert(Morpheme::Valence(Valence::Corollary), ("Corollary", "CRO"));
    names.insert(Morpheme::Valence(Valence::Reciprocal), ("Reciprocal", "RCP"));
    names.insert(Morpheme::Valence(Valence::Complementary), ("Complementary", "CPL"));
    names.insert(Morpheme::Valence(Valence::Duplicative), ("Duplicative", "DUP"));
    names.insert(Morpheme::Valence(Valence::Demonstrative), ("Demonstrative", "DEM"));
    names.insert(Morpheme::Valence(Valence::Contingent), ("Contingent", "CNG"));
    names.insert(Morpheme::Valence(Valence::Participative), ("Participative", "PTI"));
    names.insert(Morpheme::Phase(Phase::Punctual), ("Punctual", "PCT"));
    names.insert(Morpheme::Phase(Phase::Iterative), ("Iterative", "ITR"));
    names.insert(Morpheme::Phase(Phase::Repetitive), ("Repetitive", "REP"));
    names.insert(Morpheme::Phase(Phase::Intermittent), ("Intermittent", "ITM"));
    names.insert(Morpheme::Phase(Phase::Recurrent), ("Recurrent", "RCT"));
    names.insert(Morpheme::Phase(Phase::Frequentative), ("Frequentative", "FRE"));
    names.insert(Morpheme::Phase(Phase::Fragmentative), ("Fragmentative", "FRG"));
    names.insert(Morpheme::Phase(Phase::Vacillative), ("Vacillative", "VAC"));
    names.insert(Morpheme::Phase(Phase::Fluctuative), ("Fluctuative", "FLC"));
    names.insert(Morpheme::Effect(Effect::Beneficial1), ("Beneficial First", "1:BEN"));
    names.insert(Morpheme::Effect(Effect::Beneficial2), ("Beneficial Second", "2:BEN"));
    names.insert(Morpheme::Effect(Effect::Beneficial3), ("Beneficial Third", "3:BEN"));
    names.insert(Morpheme::Effect(Effect::BeneficialSelf), ("Beneficial Self", "SLF:BEN"));
    names.insert(Morpheme::Effect(Effect::Unknown), ("Unknown", "UNK"));
    names.insert(Morpheme::Effect(Effect::DetrimentalSelf), ("Detrimental Self", "SLF:DET"));
    names.insert(Morpheme::Effect(Effect::Detrimental3), ("Detrimental Third", "3:DET"));
    names.insert(Morpheme::Effect(Effect::Detrimental2), ("Detrimental Second", "2:DET"));
    names.insert(Morpheme::Effect(Effect::Detrimental1), ("Detrimental First", "1:DET"));
    names.insert(Morpheme::Level(Level::Minimal), ("Minimal", "MIN"));
    names.insert(Morpheme::Level(Level::Subequative), ("Subequative", "SBE"));
    names.insert(Morpheme::Level(Level::Inferior), ("Inferior", "IFR"));
    names.insert(Morpheme::Level(Level::Deficient), ("Deficient", "DFT"));
    names.insert(Morpheme::Level(Level::Equative), ("Equative", "EQU"));
    names.insert(Morpheme::Level(Level::Surpassive), ("Surpassive", "SUR"));
    names.insert(Morpheme::Level(Level::Superlative), ("Superlative", "SPL"));
    names.insert(Morpheme::Level(Level::Superequative), ("Superequative", "SPQ"));
    names.insert(Morpheme::Level(Level::Maximal), ("Maximal", "MAX"));
    names.insert(Morpheme::Aspect(Aspect::Retrospective), ("Retrospective", "RTR"));
    names.insert(Morpheme::Aspect(Aspect::Prospective), ("Prospective", "PRS"));
    names.insert(Morpheme::Aspect(Aspect::Habitual), ("Habitual", "HAB"));
    names.insert(Morpheme::Aspect(Aspect::Progressive), ("Progressive", "PRG"));
    names.insert(Morpheme::Aspect(Aspect::Imminent), ("Imminent", "IMM"));
    names.insert(Morpheme::Aspect(Aspect::Precessive), ("Precessive", "PCS"));
    names.insert(Morpheme::Aspect(Aspect::Regulative), ("Regulative", "REG"));
    names.insert(Morpheme::Aspect(Aspect::Summative), ("Summative", "SMM"));
    names.insert(Morpheme::Aspect(Aspect::Anticipatory), ("Anticipatory", "ATP"));
    names.insert(Morpheme::Aspect(Aspect::Resumptive), ("Resumptive", "RSM"));
    names.insert(Morpheme::Aspect(Aspect::Cessative), ("Cessative", "CSS"));
    names.insert(Morpheme::Aspect(Aspect::Pausal), ("Pausal", "PAU"));
    names.insert(Morpheme::Aspect(Aspect::Regressive), ("Regressive", "RGR"));
    names.insert(Morpheme::Aspect(Aspect::Preclusive), ("Preclusive", "PCL"));
    names.insert(Morpheme::Aspect(Aspect::Continuative), ("Continuative", "CNT"));
    names.insert(Morpheme::Aspect(Aspect::Incessative), ("Incessative", "ICS"));
    names.insert(Morpheme::Aspect(Aspect::Experiential), ("Experiential", "EXP"));
    names.insert(Morpheme::Aspect(Aspect::Interruptive), ("Interruptive", "IRP"));
    names.insert(Morpheme::Aspect(Aspect::Preemptive), ("Preemptive", "PMP"));
    names.insert(Morpheme::Aspect(Aspect::Climactic), ("Climactic", "CLM"));
    names.insert(Morpheme::Aspect(Aspect::Dilatory), ("Dilatory", "DLT"));
    names.insert(Morpheme::Aspect(Aspect::Temporary), ("Temporary", "TMP"));
    names.insert(Morpheme::Aspect(Aspect::Expenditive), ("Expenditive", "XPD"));
    names.insert(Morpheme::Aspect(Aspect::Limitative), ("Limitative", "LIM"));
    names.insert(Morpheme::Aspect(Aspect::Expeditive), ("Expeditive", "EPD"));
    names.insert(Morpheme::Aspect(Aspect::Protractive), ("Protractive", "PTC"));
    names.insert(Morpheme::Aspect(Aspect::Preparatory), ("Preparatory", "PPR"));
    names.insert(Morpheme::Aspect(Aspect::Disclusive), ("Disclusive", "DCL"));
    names.insert(Morpheme::Aspect(Aspect::Conclusive), ("Conclusive", "CCL"));
    names.insert(Morpheme::Aspect(Aspect::Culminative), ("Culminative", "CUL"));
    names.insert(Morpheme::Aspect(Aspect::Intermediative), ("Intermediative", "IMD"));
    names.insert(Morpheme::Aspect(Aspect::Tardative), ("Tardative", "TRD"));
    names.insert(Morpheme::Aspect(Aspect::Transitional), ("Transitional", "TNS"));
    names.insert(Morpheme::Aspect(Aspect::Intercommutative), ("Intercommutative", "ITC"));
    names.insert(Morpheme::Aspect(Aspect::Motive), ("Motive", "MTV"));
    names.insert(Morpheme::Aspect(Aspect::Sequential), ("Sequential", "SQN"));
    // Mood or Case-Scope
    names.insert(Morpheme::Mood(Mood::Factual), ("Factual", "FAC"));
    names.insert(Morpheme::Mood(Mood::Subjunctive), ("Subjunctive", "SUB"));
    names.insert(Morpheme::Mood(Mood::Assumptive), ("Assumptive", "ASM"));
    names.insert(Morpheme::Mood(Mood::Speculative), ("Speculative", "SPC"));
    names.insert(Morpheme::Mood(Mood::Counterfactive), ("Counterfactive", "COU"));
    names.insert(Morpheme::Mood(Mood::Hypothetical), ("Hypothetical", "HYP"));
    names.insert(Morpheme::CaseScope(CaseScope::Natural), ("Natural", "CCN"));
    names.insert(Morpheme::CaseScope(CaseScope::Antecedent), ("Antecedent", "CCA"));
    names.insert(Morpheme::CaseScope(CaseScope::Subaltern), ("Subaltern", "CCS"));
    names.insert(Morpheme::CaseScope(CaseScope::Qualifier), ("Qualifier", "CCQ"));
    names.insert(Morpheme::CaseScope(CaseScope::Precedent), ("Precedent", "CCP"));
    names.insert(Morpheme::CaseScope(CaseScope::Successive), ("Successive", "CCV"));
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
    let valences = vec![Morpheme::Valence(Valence::Monoactive),
                        Morpheme::Valence(Valence::Parallel),
                        Morpheme::Valence(Valence::Corollary),
                        Morpheme::Valence(Valence::Reciprocal),
                        Morpheme::Valence(Valence::Complementary),
                        Morpheme::Valence(Valence::Duplicative),
                        Morpheme::Valence(Valence::Demonstrative),
                        Morpheme::Valence(Valence::Contingent),
                        Morpheme::Valence(Valence::Participative)];
    let phases = vec![Morpheme::Phase(Phase::Punctual),
    Morpheme::Phase(Phase::Iterative),
    Morpheme::Phase(Phase::Repetitive),
    Morpheme::Phase(Phase::Intermittent),
    Morpheme::Phase(Phase::Recurrent),
    Morpheme::Phase(Phase::Frequentative),
    Morpheme::Phase(Phase::Fragmentative),
    Morpheme::Phase(Phase::Vacillative),
    Morpheme::Phase(Phase::Fluctuative)];
    let effects8 = vec![Morpheme::Effect(Effect::Beneficial1),
    Morpheme::Effect(Effect::Beneficial2),
    Morpheme::Effect(Effect::Beneficial3),
    Morpheme::Effect(Effect::BeneficialSelf),
    Morpheme::Effect(Effect::Unknown),
    Morpheme::Effect(Effect::DetrimentalSelf),
    Morpheme::Effect(Effect::Detrimental3),
    Morpheme::Effect(Effect::Detrimental2),
    Morpheme::Effect(Effect::Detrimental1)];
    let levels = vec![Morpheme::Level(Level::Minimal),
    Morpheme::Level(Level::Subequative),
    Morpheme::Level(Level::Inferior),
    Morpheme::Level(Level::Deficient),
    Morpheme::Level(Level::Equative),
    Morpheme::Level(Level::Surpassive),
    Morpheme::Level(Level::Superlative),
    Morpheme::Level(Level::Superequative),
    Morpheme::Level(Level::Maximal)];
    let aspects = vec![Morpheme::Aspect(Aspect::Retrospective),
    Morpheme::Aspect(Aspect::Prospective),
    Morpheme::Aspect(Aspect::Habitual),
    Morpheme::Aspect(Aspect::Progressive),
    Morpheme::Aspect(Aspect::Imminent),
    Morpheme::Aspect(Aspect::Precessive),
    Morpheme::Aspect(Aspect::Regulative),
    Morpheme::Aspect(Aspect::Summative),
    Morpheme::Aspect(Aspect::Anticipatory),
    Morpheme::Aspect(Aspect::Resumptive),
    Morpheme::Aspect(Aspect::Cessative),
    Morpheme::Aspect(Aspect::Pausal),
    Morpheme::Aspect(Aspect::Regressive),
    Morpheme::Aspect(Aspect::Preclusive),
    Morpheme::Aspect(Aspect::Continuative),
    Morpheme::Aspect(Aspect::Incessative),
    Morpheme::Aspect(Aspect::Experiential),
    Morpheme::Aspect(Aspect::Interruptive),
    Morpheme::Aspect(Aspect::Preemptive),
    Morpheme::Aspect(Aspect::Climactic),
    Morpheme::Aspect(Aspect::Dilatory),
    Morpheme::Aspect(Aspect::Temporary),
    Morpheme::Aspect(Aspect::Expenditive),
    Morpheme::Aspect(Aspect::Limitative),
    Morpheme::Aspect(Aspect::Expeditive),
    Morpheme::Aspect(Aspect::Protractive),
    Morpheme::Aspect(Aspect::Preparatory),
    Morpheme::Aspect(Aspect::Disclusive),
    Morpheme::Aspect(Aspect::Conclusive),
    Morpheme::Aspect(Aspect::Culminative),
    Morpheme::Aspect(Aspect::Intermediative),
    Morpheme::Aspect(Aspect::Tardative),
    Morpheme::Aspect(Aspect::Transitional),
    Morpheme::Aspect(Aspect::Intercommutative),
    Morpheme::Aspect(Aspect::Motive),
    Morpheme::Aspect(Aspect::Sequential)];
    let moods = vec![Morpheme::Mood(Mood::Factual),
    Morpheme::Mood(Mood::Subjunctive),
    Morpheme::Mood(Mood::Assumptive),
    Morpheme::Mood(Mood::Speculative),
    Morpheme::Mood(Mood::Counterfactive),
    Morpheme::Mood(Mood::Hypothetical)];
    let case_scopes = vec![Morpheme::CaseScope(CaseScope::Natural),
    Morpheme::CaseScope(CaseScope::Antecedent),
    Morpheme::CaseScope(CaseScope::Subaltern),
    Morpheme::CaseScope(CaseScope::Qualifier),
    Morpheme::CaseScope(CaseScope::Precedent),
    Morpheme::CaseScope(CaseScope::Successive)];
    MorphemeContent {stems, versions, configurations, extensions, affiliations
        , perspectives, essences, functions, specifications, contexts, parties, effects, cases
        , illocutions, expectations, validations, vowel_forms, consonant_forms, valences, phases,
    effects8, levels, aspects, moods, case_scopes}
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