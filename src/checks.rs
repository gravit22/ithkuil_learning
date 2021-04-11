use crate::morpho_phonology::*;
use std::collections::HashMap;
use rand;
use rand::Rng;
use stopwatch::Stopwatch;

pub struct Data {
    pub morphemes: HashMap<Vec<Morpheme>, &'static str>,
    pub morphemes_names: HashMap<Morpheme, (&'static str, &'static str)>,
    pub morphemes_content: MorphemeContent,
    pub consonants: String,
    pub vowels: String,
    pub slot6_transformations: HashMap<&'static str, &'static str>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            morphemes: get_morphemes(),
            morphemes_names: get_morphemes_names(),
            morphemes_content: get_morphemes_content(),
            consonants: "pbmfvwtdnţḑszcżršžčjçykgxňř'hļl".to_owned(),
            vowels: "ieäöïaüëuo".to_owned(),
            slot6_transformations: get_slot6_transformations(),
        }
    }
}

pub fn is_win(probabilities: &Vec<f64>) -> bool {
    if probabilities.iter().filter(|x| **x > 0.2).collect::<Vec<&f64>>().is_empty() {
        return true
    } else {
        return false
    }
}

pub fn modify_probability(probabilities: &mut Vec<f64>, successful: bool, choice: usize) {
    let difficulty = 0.2;
    if successful {
        probabilities[choice] -= probabilities[choice] * 0.2;
    } else {
        probabilities[choice] += probabilities[choice] * 0.2;
    }
    probabilities[choice] = clamp(probabilities[choice]);
}

fn probability_choice(probabilities: &Vec<f64>) -> usize {
    let mut max = (0, 0.0);
    for (i, v) in probabilities.iter().enumerate() {
        if rand::thread_rng().gen_bool(*v) {
            return i;
        }
        if max.1 < *v {
            max = (i, *v);
        }
    }
    max.0
}

fn task(variant: &Vec<Morpheme>, data: &Data, from_beginning: bool) -> (String, String) {
    let start = if from_beginning {0} else {1};
    let mut string = String::new();
    let mut expected_answer = String::new();
    for morpheme in variant {
        string.push_str(data.morphemes_names.get(&morpheme).unwrap().1);
        string.push_str(" + ");
    }
    let mut e_a = false;
    for beginning in start..variant.len() {
        for end in beginning..variant.len() {
            if let Some(v) = data.morphemes.get(&variant[beginning..end+1]) {
                if variant.len() > 3 {
                    if variant[2] != Morpheme::Extension(Extension::Delimitive)
                        && variant[3] != Morpheme::Affiliation(Affiliation::Consolidative) {
                        e_a = true;
                    }
                }
                // vowel forms
                if v.contains('/') && v.chars().next().unwrap() == 'i' {
                    if variant[0] == Morpheme::Y {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[1]);
                    } else {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[0]);
                    }
                } else if v.contains('/') && v.chars().next().unwrap() == 'u' {
                    if variant[0] == Morpheme::W {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[1]);
                    } else {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[0]);
                    }
                }
                // slot 6 forms
                else if v.contains('/') && beginning == 1 {
                    if is_end_empty(&variant) {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[0]);
                    } else {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[1]);
                    }
                } else if (variant[beginning..end+1]
                == vec![Morpheme::Perspective(Perspective::Nomic), Morpheme::Essence(Essence::Representative)]
                    || variant[beginning..end+1]
                    == vec![Morpheme::Perspective(Perspective::Abstract), Morpheme::Essence(Essence::Representative)])
                    && expected_answer.chars().count() >= 2 {
                    let chars = expected_answer.chars().collect::<Vec<char>>();
                    let last_char = chars[chars.len()];
                    let second_to_last_char = chars[chars.len() - 1];
                    if "tkp".contains(last_char) && (data.consonants.contains(second_to_last_char)
                        || variant[0] == Morpheme::C) {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[1]);
                    } else {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[0]);
                    }
                } else if v.contains('/') && beginning == 3 {
                    if variant[2] == Morpheme::Configuration(Configuration::Uniplex) {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[0]);
                    } else {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[1]);
                    }
                } else {
                    expected_answer.push_str(v);
                }
            }
        }
    }
    transform_slot6(&mut expected_answer, data);
    //println!("{}", expected_answer);
    string.pop();
    string.pop();
    string.pop();
    (string, expected_answer)
}

pub fn generate_slot2(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
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
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn generate_slot4(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
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
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn generate_slot6(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
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
                        if data.morphemes_content.perspectives[perspective] == Morpheme::Perspective(Perspective::Polyadic) {
                            continue;
                        }
                        if perspective == 4 {continue}
                        if key[4][perspective] == false {continue}
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
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn generate_referentials(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
    let mut variants = Vec::new();
    for party in 0..data.morphemes_content.parties.len() {
        if key[0][party] == false {continue}
        for perspective in 0..data.morphemes_content.perspectives.len() {
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
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, true), choice)
}

pub fn generate_slot9iev(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
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
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn transform_slot6(string: &mut String, data: &Data) {
    let to_bytes = string.clone();
    let bytes = to_bytes.as_bytes();
    for (f_idx, &item1) in bytes.iter().enumerate() {
        for (s_idx, &item2) in bytes.iter().enumerate() {
            if s_idx <= f_idx {
                continue;
            }
            let s = String::from_utf8_lossy(&bytes[f_idx..s_idx + 1]);
            // testing combinations involving V or C
            for key in data.slot6_transformations.keys() {
                if key.contains("V") {
                    let mut from = String::from(*key);
                    from.remove(key.find("V").unwrap());
                    if from == s.trim() {
                        let mut b = f_idx - 1..f_idx;
                        if bytes[f_idx - 1].is_ascii_alphabetic() == false {
                            b = f_idx - 2..f_idx;
                        }
                        if data.vowels.contains(&string[b]) {
                            string.replace_range(f_idx..s_idx + 1, &data.slot6_transformations
                                .get(key).unwrap()[1..]);
                        }
                    }
                } else if key.contains("C") {
                    let mut from = String::from(*key);
                    from.remove(key.find("C").unwrap());
                    if from == s.trim() {
                        let mut b = s_idx..s_idx + 1;
                        if bytes[s_idx + 1].is_ascii_alphabetic() == false {
                            b = s_idx..s_idx + 1;
                        }
                        if data.consonants.contains(&string[b]) { // here is the problem
                            string.replace_range(f_idx..s_idx + 1, &data.slot6_transformations
                                .get(key).unwrap()[..key.len() - 2]);
                        }
                    }
                }
            }
            // testing regular combinations
            if data.slot6_transformations.contains_key(s.trim()) {
                string.replace_range(f_idx..s_idx + 1, data.slot6_transformations
                    .get(&string[f_idx..s_idx + 1]).unwrap());
            }
        }
    }
}

fn clamp(value: f64) -> f64 {
    if value < 0.01 {
        return 0.01;
    } else if value > 0.9 {
        return 0.9;
    }
    value
}

fn is_end_empty(variant: &Vec<Morpheme>) -> bool {
    if variant[2] == Morpheme::Configuration(Configuration::Uniplex)
        && variant[3] == Morpheme::Extension(Extension::Delimitive)
        && variant[4] == Morpheme::Perspective(Perspective::Monadic)
        && variant[5] == Morpheme::Essence(Essence::Normal) {
        return true
    }
    false
}
