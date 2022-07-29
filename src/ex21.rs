use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use std::str::FromStr;

pub struct Day21;

type Foods = Vec<Food>;
type AllergensSources = Vec<String>;

impl DaySolver for Day21 {
    type Output = usize;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_21", "data_files/ex21.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let foods: Foods = _s
            .lines()
            .map(|line| line.parse::<Food>().unwrap())
            .collect();

        let allergens_sources = get_possible_allergens_sources(&foods);

        let res = count_ingridients_without_allergens(&foods, &allergens_sources);

        Ok(res)
    }
}

#[derive(Debug, PartialEq)]
struct Food {
    allergens: Vec<String>,
    ingridients: Vec<String>,
}

impl FromStr for Food {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ingridients_unextracted, allergens_unextracted) = s.split_once(" (contains ").unwrap();
        let ingridients: Vec<String> = ingridients_unextracted
            .split(" ")
            .map(|s| s.to_owned())
            .collect();
        let allergens: Vec<String> = allergens_unextracted
            .trim_end_matches(")")
            .split(", ")
            .map(|s| s.to_owned())
            .collect();

        Ok(Food {
            allergens,
            ingridients,
        })
    }
}

fn count_ingridients_without_allergens(foods: &Foods, allergens_sources: &Vec<String>) -> usize {
    foods
        .into_iter()
        .map(|food| {
            (&food.ingridients)
                .into_iter()
                .filter(|ingridient| !allergens_sources.contains(ingridient))
                .count()
        })
        .sum()
}

fn get_vec_intersection(vec1: &Vec<String>, vec2: &Vec<String>) -> Vec<String> {
    vec1.into_iter()
        .filter(|vec1_element| vec2.contains(vec1_element))
        .map(|value| value.to_owned())
        .collect()
}

fn get_possible_allergens_sources(foods: &Foods) -> AllergensSources {
    let mut allergens_sources: HashMap<String, Vec<String>> = HashMap::default();

    for food in foods {
        for allergen in &food.allergens {
            if let Some((key, value)) = allergens_sources.get_key_value(allergen) {
                let _unused = allergens_sources.insert(
                    key.to_owned(),
                    get_vec_intersection(value, &food.ingridients),
                );
            } else {
                allergens_sources.insert(allergen.to_owned(), food.ingridients.to_owned());
            }
        }
    }

    let mut res = Vec::new();

    for ingridient in allergens_sources.values().flatten() {
        if !res.contains(ingridient) {
            res.push(ingridient.to_owned());
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)" => 
        Food {  allergens: vec!["dairy".to_owned(), "fish".to_owned()], 
                ingridients: vec!["mxmxvkd".to_owned(), "kfcds".to_owned(), "sqjhc".to_owned(), "nhms".to_owned()] })]
    #[test_case("sqjhc fvjkl (contains soy)" => 
        Food {  allergens: vec!["soy".to_owned()], 
                ingridients: vec!["sqjhc".to_owned(), "fvjkl".to_owned()] })]
    fn ex21_food_from_str(s: &str) -> Food {
        s.parse::<Food>().unwrap()
    }

    #[test]
    fn ex21_get_possible_allergens_sources() {
        let t = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

        let foods: Vec<Food> = t
            .lines()
            .map(|line| line.parse::<Food>().unwrap())
            .collect();
        assert_eq!(
            get_possible_allergens_sources(&foods),
            vec!["mxmxvkd", "sqjhc", "fvjkl"]
        )
    }

    #[test_case(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()], vec!["2".to_string(), "4".to_string(), "6".to_string()] => vec!["2".to_string(), "4".to_string()])]
    fn ex21_get_vec_intersection(vec1: Vec<String>, vec2: Vec<String>) -> Vec<String> {
        get_vec_intersection(&vec1, &vec2)
    }

    #[test]
    fn ex21_count_ingridients_without_allergens() {
        let foods = vec![
            Food {
                allergens: vec!["dairy".to_string(), "fish".to_string()],
                ingridients: vec![
                    "mxmxvkd".to_string(),
                    "kfcds".to_string(),
                    "sqjhc".to_string(),
                    "nhms".to_string(),
                ],
            },
            Food {
                allergens: vec!["dairy".to_string()],
                ingridients: vec![
                    "trh".to_string(),
                    "fvjkl".to_string(),
                    "sbzzf".to_string(),
                    "mxmxvkd".to_string(),
                ],
            },
            Food {
                allergens: vec!["soy".to_string()],
                ingridients: vec!["sqjhc".to_string(), "fvjkl".to_string()],
            },
            Food {
                allergens: vec!["fish".to_string()],
                ingridients: vec![
                    "sqjhc".to_string(),
                    "mxmxvkd".to_string(),
                    "sbzzf".to_string(),
                ],
            },
        ];
        assert_eq!(
            count_ingridients_without_allergens(
                &foods,
                &vec![
                    "mxmxvkd".to_string(),
                    "sqjhc".to_string(),
                    "fvjkl".to_string()
                ]
            ),
            5
        )
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day21::solve_default_file().unwrap(), 5)
    }
}
