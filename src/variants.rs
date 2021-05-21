use crate::morpho_phonology::*;
use crate::checks::*;

pub fn slot2_variants(data: &Data, key: &Vec<Vec<bool>>) -> Vec<Vec<Morpheme>> {
    let mut variants = Vec::new();
    for vowel_form in 0..data.morphemes_content.vowel_forms.len() {
        if key[0][vowel_form] == false {continue}
        for stem in 0..data.morphemes_content.stems.len() {
            if key[1][stem] == false {continue}
            for version in 0..data.morphemes_content.versions.len() {
                if key[2][version] == false {continue}
                variants.push(vec![
                    data.morphemes_content.vowel_forms[vowel_form].clone()
                    , data.morphemes_content.stems[stem].clone()
                    , data.morphemes_content.versions[version].clone()]);
            }
        }
    }
    variants
}

pub fn slot4_variants(data: &Data, key: &Vec<Vec<bool>>) -> Vec<Vec<Morpheme>> {
    let mut variants = Vec::new();
    for vowel_form in 0..data.morphemes_content.vowel_forms.len() {
        if key[0][vowel_form] == false {continue}
        for function in 0..data.morphemes_content.functions.len() {
            if key[1][function] == false {continue}
            for specification in 0..data.morphemes_content.specifications.len() {
                if key[2][specification] == false {continue}
                for context in 0..data.morphemes_content.contexts.len() {
                    if key[3][context] == false {continue}
                    variants.push(vec![
                        data.morphemes_content.vowel_forms[vowel_form].clone()
                        , data.morphemes_content.functions[function].clone()
                        , data.morphemes_content.specifications[specification].clone()
                        , data.morphemes_content.contexts[context].clone()]);
                }
            }
        }
    }
    variants
}

pub fn slot6_variants(data: &Data, key: &Vec<Vec<bool>>) -> Vec<Vec<Morpheme>> {
    let mut variants = Vec::new();
    for consonant_form in 0..data.morphemes_content.consonant_forms.len() {
        if key[0][consonant_form] == false {continue}
        for affiliation in 0..data.morphemes_content.affiliations.len() {
            if key[1][affiliation] == false {continue}
            for configuration in 0..data.morphemes_content.configurations.len() {
                if key[2][configuration] == false {continue}
                for extension in 0..data.morphemes_content.expectations.len() {
                    if key[3][extension] == false {continue}
                    for perspective in 0..data.morphemes_content.perspectives.len() {
                        let mut perspective = perspective;
                        if perspective == 4 {continue}
                        if key[4][perspective] == false {continue}
                        if perspective > 0 {
                            perspective += 1;
                        }
                        for essence in 0..data.morphemes_content.essences.len() {
                            if key[5][essence] == false {continue}
                            variants.push(vec![
                                data.morphemes_content.consonant_forms[consonant_form].clone()
                                , data.morphemes_content.affiliations[affiliation].clone()
                                , data.morphemes_content.configurations[configuration].clone()
                                , data.morphemes_content.extensions[extension].clone()
                                , data.morphemes_content.perspectives[perspective].clone()
                                , data.morphemes_content.essences[essence].clone()]);
                        }
                    }
                }
            }
        }
    }
    variants
}

pub fn referential_variants(data: &Data, key: &Vec<Vec<bool>>) -> Vec<Vec<Morpheme>> {
    let mut variants = Vec::new();
    for party in 0..data.morphemes_content.parties.len() {
        if key[0][party] == false {continue}
        for perspective in 0..data.morphemes_content.perspectives.len() {
            if perspective == 4 {continue}
            if key[1][perspective] == false {continue}
            if data.morphemes_content.perspectives[perspective] == Morpheme::Perspective(Perspective::Nomic)
                || data.morphemes_content.perspectives[perspective] == Morpheme::Perspective(Perspective::Abstract)
                || data.morphemes_content.perspectives[perspective] == Morpheme::Perspective(Perspective::Agglomerative) {
                continue;
            }
            if (data.morphemes_content.parties[party] == Morpheme::Party(Party::Provisional)
                || data.morphemes_content.parties[party] == Morpheme::Party(Party::Obviative)
                || data.morphemes_content.parties[party] == Morpheme::Party(Party::ThirdParty(Animacy::Mixed)))
                && data.morphemes_content.perspectives[perspective] != Morpheme::Perspective(Perspective::Monadic) {
                continue;
            }
            if data.morphemes_content.parties[party] == Morpheme::Party(Party::Speaker)
                && data.morphemes_content.perspectives[perspective] != Morpheme::Perspective(Perspective::Monadic) {
                continue;
            }
            for effect in 0..data.morphemes_content.effects.len() {
                if key[2][effect] == false {continue}
                for case in 0..data.morphemes_content.cases.len() {
                    if key[3][case] == false {continue}
                    variants.push(vec![
                        data.morphemes_content.parties[party].clone()
                        , data.morphemes_content.perspectives[perspective].clone()
                        , data.morphemes_content.effects[effect].clone()
                        , data.morphemes_content.cases[case].clone()]);
                }
            }
        }
    }
    variants
}

pub fn slot8_variants(data: &Data, key: &Vec<Vec<bool>>) -> Vec<Vec<Morpheme>> {
    let mut variants = Vec::new();
    for vowel_form in 0..data.morphemes_content.vowel_forms.len() {
        let mut part1 = Vec::new();
        let mut part2 = Vec::new();
        if key[0][vowel_form] == false {continue}
        for valence in 0..data.morphemes_content.valences.len() {
            if key[1][valence] {part1.push(data.morphemes_content.valences[valence].clone())}
        }
        for phase in 0..data.morphemes_content.phases.len() {
            if key[2][phase] { part1.push(data.morphemes_content.phases[phase].clone()) }
        }
        for effect in 0..data.morphemes_content.effects8.len() {
            if key[3][effect] { part1.push(data.morphemes_content.effects8[effect].clone()) }
        }
        for level in 0..data.morphemes_content.levels.len() {
            if key[4][level] { part1.push(data.morphemes_content.levels[level].clone()) }
        }
        for aspect in 0..data.morphemes_content.aspects.len() {
            if key[5][aspect] { part1.push(data.morphemes_content.aspects[aspect].clone()) }
        }
        for mood in 0..data.morphemes_content.moods.len() {
            if key[6][mood] { part2.push(data.morphemes_content.moods[mood].clone()) }
        }
        for case_scope in 0..data.morphemes_content.case_scopes.len() {
            if key[7][case_scope] { part2.push(data.morphemes_content.case_scopes[case_scope].clone()) }
        }
        for p1 in &part1 {
            for p2 in &part2 {
                variants.push(vec![
                    data.morphemes_content.vowel_forms[vowel_form].clone(),
                    p1.clone(), p2.clone()]);
            }
        }
    }
    variants
}

pub fn slot9iev_variants(data: &Data, key: &Vec<Vec<bool>>) -> Vec<Vec<Morpheme>> {
    let mut variants = Vec::new();
    for vowel_form in 0..data.morphemes_content.vowel_forms.len() {
        if key[0][vowel_form] == false {continue}
        for illocution in 0..data.morphemes_content.illocutions.len() {
            if key[1][illocution] == false {continue}
            for expectation in 0..data.morphemes_content.expectations.len() {
                if key[2][expectation] == false {continue}
                for validation in 0..data.morphemes_content.validations.len() {
                    if key[3][validation] == false {continue}
                    if data.morphemes_content.illocutions[illocution] == Morpheme::Illocution(Illocution::Performative)
                        && data.morphemes_content.validations[validation] != Morpheme::Validation(Validation::Observational) {
                        continue;
                    }
                    variants.push(vec![
                        data.morphemes_content.vowel_forms[vowel_form].clone()
                        , data.morphemes_content.illocutions[illocution].clone()
                        , data.morphemes_content.expectations[expectation].clone()
                        , data.morphemes_content.validations[validation].clone()]);
                }
            }
        }
    }
    variants
}

pub fn slot2_key_options() -> (Vec<Vec<bool>>, Vec<Vec<&'static str>>) {
    let mut key = Vec::new();
    key.push(vec![true, true, true]);
    key.push(vec![true, true, true, true]);
    key.push(vec![true, true]);

    let mut options = Vec::new();
    options.push(vec!["(C)", "(w)", "(y)"]);
    options.push(vec!["S1", "S2", "S3", "S0"]);
    options.push(vec!["PRC", "CPT"]);
    (key, options)
}

pub fn slot4_key_options() -> (Vec<Vec<bool>>, Vec<Vec<&'static str>>) {
    let mut key = Vec::new();
    key.push(vec![true, true, true]);
    key.push(vec![true, true]);
    key.push(vec![true, true, true, true]);
    key.push(vec![true, true, true, true]);

    let mut options = Vec::new();
    options.push(vec!["(C)", "(w)", "(y)"]);
    options.push(vec!["STA", "DYN"]);
    options.push(vec!["BSC", "CTE", "CSV", "OBJ"]);
    options.push(vec!["EXS", "FNC", "RPS", "AMG"]);
    (key, options)
}

pub fn slot6_key_options() -> (Vec<Vec<bool>>, Vec<Vec<&'static str>>) {
    let mut key = Vec::new();
    key.push(vec![true, true]);
    key.push(vec![true, true, true, true]);
    key.push(vec![
        true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true,
    ]);
    key.push(vec![true, true, true, true, true, true]);
    key.push(vec![true, true, true, true]);
    key.push(vec![true, true]);

    let mut options = Vec::new();
    options.push(vec!["(C)", "(V)"]);
    options.push(vec!["CSL", "ASO", "COA", "VAR"]);
    options.push(vec![
        "UNI", "DUP", "MSS", "MSC", "MSF", "MDS", "MDC", "MDF", "MFS", "MFC",
        "MFF", "DSS", "DSC", "DSF", "DDS", "DDC", "DDF", "DFS", "DFC", "DFF",
    ]);
    options
        .push(vec!["DEL", "PRX", "ICP", "ATV", "GRA", "DPL"]);
    options.push(vec!["M", "G", "N", "A"]);
    options.push(vec!["NRM", "RPV"]);
    (key, options)
}

pub fn slot8_key_options() -> (Vec<Vec<bool>>, Vec<Vec<&'static str>>) {
    let mut key = Vec::new();
    key.push(vec![true, true, true]);
    key.push(vec![true, true, true, true, true, true, true, true, true]);
    key.push(vec![true, true, true, true, true, true, true, true, true]);
    key.push(vec![true, true, true, true, true, true, true, true, true]);
    key.push(vec![true, true, true, true, true, true, true, true, true]);
    key.push(vec![true, true, true, true, true, true, true, true, true,
                  true, true, true, true, true, true, true, true, true,
                  true, true, true, true, true, true, true, true, true,
                  true, true, true, true, true, true, true, true, true]);
    key.push(vec![true, true, true, true, true, true]);
    key.push(vec![true, true, true, true, true, true]);

    let mut options = Vec::new();
    options.push(vec!["(C)", "(w)", "(y)"]);
    options.push(vec!["MNO", "PRL", "CRO", "RCP", "CPL", "DUP", "DEM", "CNG", "PTI"]);
    options.push(vec!["PCT", "ITR", "REP", "ITM", "RCT", "FRE", "FRG", "VAC", "FLC"]);
    options.push(vec!["1:BEN", "2:BEN", "3:BEN", "SLF:BEN", "UNK", "SLF:DET", "3:DET", "2:DET", "1:DET"]);
    options.push(vec!["MIN", "SBE", "IFR", "DFT", "EQU", "SUR", "SPL", "SPQ", "MAX"]);
    options.push(vec!["RTR", "PRS", "HAB", "PRG", "IMM", "PCS", "REG", "SMM", "ATP",
                      "RSM", "CSS", "PAU", "RGR", "PCL", "CNT", "ICS", "EXP", "IRP",
                      "PMP", "CLM", "DLT", "TMP", "XPD", "LIM", "EPD", "PTC", "PPR",
                      "DCL", "CCL", "CUL", "IMD", "TRD", "TNS", "ITC", "MTV", "SQN"]);
    options.push(vec!["FAC", "SUB", "ASM", "SPC", "COU", "HYP"]);
    options.push(vec!["CCN", "CCA", "CCS", "CCQ", "CCP", "CCV"]);
    (key, options)
}

pub fn referential_key_options() -> (Vec<Vec<bool>>, Vec<Vec<&'static str>>) {
    let mut key = Vec::new();
    key
        .push(vec![true, true, true, true, true, true, true]);
    key.push(vec![true, true, true, true]);
    key.push(vec![true, true, true]);
    key
        .push(vec![true, true, true, true, true, true, true, true, true]);

    let mut options = Vec::new();
    options.push(vec![
        "I",
        "you",
        "he/she/they",
        "it/these things/those things",
        "animate+inanimate",
        "3rd party other than most recently referenced",
        "whatever",
    ]);
    options.push(vec!["M", "P", "N", "A"]);
    options
        .push(vec!["NEUTRAL", "BENEFICIAL", "DETRIMENTAL"]);
    options.push(vec![
        "THM", "INS", "ABS", "AFF", "STM", "EFF", "ERG", "DAT", "IND",
    ]);
    (key, options)
}

pub fn slot9iev_key_options() -> (Vec<Vec<bool>>, Vec<Vec<&'static str>>) {
    let mut key = Vec::new();
    key.push(vec![true, true, true]);
    key.push(vec![true, true]);
    key.push(vec![true, true, true]);
    key
        .push(vec![true, true, true, true, true, true, true, true]);

    let mut options = Vec::new();
    options.push(vec!["(C)", "(w)", "(y)"]);
    options.push(vec!["ASR", "PFM"]);
    options.push(vec!["COG", "RSP", "EXE"]);
    options
        .push(vec!["OBS", "REC", "RPR", "PUP", "IMA", "CVN", "ITU", "INF"]);
    (key, options)
}