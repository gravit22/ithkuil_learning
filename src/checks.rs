use crate::morpho_phonology::*;
use std::collections::HashMap;
use rand;
use rand::Rng;
use crate::variants::*;

pub struct Data {
    pub morphemes: HashMap<Vec<Morpheme>, &'static str>,
    pub morphemes_names: HashMap<Morpheme, (&'static str, &'static str)>,
    pub morphemes_content: MorphemeContent,
    pub consonants: &'static str,
    pub vowels: &'static str,
    pub slot6_transformations: HashMap<&'static str, &'static str>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            morphemes: get_morphemes(),
            morphemes_names: get_morphemes_names(),
            morphemes_content: get_morphemes_content(),
            consonants: "pbmfvwtdnţḑszcżršžčjçykgxňř'hļl",
            vowels: "ieäöïaüëuo",
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
    probabilities.iter().enumerate();
    rand::thread_rng().gen_bool(0.2);

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

pub fn task(variant: &Vec<Morpheme>, data: &Data, from_beginning: bool) -> (String, String) {
    let start = if from_beginning {0} else {1};
    let mut string = String::new();
    let mut expected_answer = String::new();
    for morpheme in variant {
        string.push_str(&data.morphemes_names.get(&morpheme).unwrap().1);
        string.push_str(" + ");
    }
    for beginning in start..variant.len() {
        for end in beginning..variant.len() {
            if let Some(v) = data.morphemes.get(&variant[beginning..end+1]) {
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
                // slot 8 forms
                else if is_slot8(&variant[1]) != None && v.contains('/') {
                    let out = is_slot8(&variant[1]).unwrap();
                    if out == "pat 1" {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[0]);
                    } else {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[1]);
                    }
                }
                // slot 6 forms
                else if v.contains('/') && beginning == 1 { // choose whether affiliation is single or not
                    if is_end_empty(&variant) {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[0]);
                    } else {
                        expected_answer.push_str(v.split('/').collect::<Vec<&str>>()[1]);
                    }
                } else if (variant[beginning..=end]
                == vec![Morpheme::Perspective(Perspective::Nomic), Morpheme::Essence(Essence::Representative)]
                    || variant[beginning..=end]
                    == vec![Morpheme::Perspective(Perspective::Abstract), Morpheme::Essence(Essence::Representative)])
                    && expected_answer.chars().count() >= 2 {
                    // choose from 2 alternative combined forms of perspective and essence
                    let chars = expected_answer.chars().collect::<Vec<char>>();
                    let last_char = chars[chars.len() - 1];
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
                } else if v.contains('/') {
                    if expected_answer.is_empty() {
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
    string.pop();
    string.pop();
    string.pop();
    (string, expected_answer)
}

pub fn generate_slot2(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
    let mut variants = slot2_variants(data, key);
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn generate_slot4(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
    let mut variants = slot4_variants(data, key);
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn generate_slot6(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
    let mut variants = slot6_variants(data, key);
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn generate_referentials(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
    let mut variants = referential_variants(data, key);
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, true), choice)
}

pub fn generate_slot8(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
    let mut variants = slot8_variants(data, key);
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn generate_slot9iev(data: &Data, probabilities: &mut Vec<f64>, key: &Vec<Vec<bool>>) -> ((String, String), usize) {
    let mut variants = slot9iev_variants(data, key);
    if probabilities.is_empty() {
        for _ in 0..variants.len() {
            probabilities.push(0.3);
        }
    }
    let choice = probability_choice(&probabilities);
    (task(&variants[choice], data, false), choice)
}

pub fn transform_slot6(string: &mut String, data: &Data) {
    // convert string into bites for easy iterations of chars(or easy replacement)
    let to_bytes = string.clone();
    let bytes = to_bytes.as_bytes();
    for (f_idx, &item1) in bytes.iter().enumerate() {
        for (s_idx, &item2) in bytes.iter().enumerate() {
            // if beginning is after or equal to end then there is nothing to do
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
                        if bytes[s_idx].is_ascii_alphabetic() == false {
                            b = s_idx..s_idx + 1;
                        }
                        if data.consonants.contains(&string[b]) { // here is the problem
                            string.replace_range(f_idx..s_idx + 1, &data.slot6_transformations
                                .get(key).unwrap()[1..key.len() - 1]);
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

fn is_slot8(variant: &Morpheme) -> Option<&str> {
    match variant {
        Morpheme::Valence(_) | Morpheme::Phase(_) |
        Morpheme::Effect(_) | Morpheme::Level(_) => Some("pat 1"),
        Morpheme::Aspect(_) => Some("pat 2"),
        _ => None,
    }
}
