use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use fnv::FnvHashSet as HashSet;
use std::str::FromStr;

pub struct Day21;

type AllergensSources = Vec<String>;
type Allergens = HashSet<String>;
type Ingridients = HashSet<String>;

impl DaySolver for Day21 {
    type Output = usize;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_21", "data_files/ex21.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let foods = _s.parse::<Foods>().unwrap();

        let allergens_sources = foods.get_possible_allergens_sources();

        let res = foods.count_ingridients_without_allergens(&allergens_sources);

        Ok(res)
    }
}

#[derive(Debug, PartialEq)]
struct Food {
    allergens: Allergens,
    ingridients: Ingridients,
}

impl FromStr for Food {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ingridients_unextracted, allergens_unextracted) = s.split_once(" (contains ").unwrap();
        let ingridients: Ingridients = ingridients_unextracted
            .split(' ')
            .map(|s| s.to_owned())
            .collect();
        let allergens: Allergens = allergens_unextracted
            .trim_end_matches(')')
            .split(", ")
            .map(|s| s.to_owned())
            .collect();

        Ok(Food {
            allergens,
            ingridients,
        })
    }
}

struct Foods {
    foods: Vec<Food>,
}

impl FromStr for Foods {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let foods: Vec<Food> = s
            .lines()
            .map(|line| line.parse::<Food>().unwrap())
            .collect();

        Ok(Foods { foods })
    }
}

impl Foods {
    fn count_ingridients_without_allergens(&self, allergens_sources: &[String]) -> usize {
        self.foods
            .iter()
            .map(|food| {
                (&food.ingridients)
                    .iter()
                    .filter(|ingridient| !allergens_sources.contains(ingridient))
                    .count()
            })
            .sum()
    }

    fn get_possible_allergens_sources(&self) -> AllergensSources {
        let mut allergens_sources: HashMap<String, Ingridients> = HashMap::default();

        for food in &self.foods {
            for allergen in &food.allergens {
                if let Some((key, value)) = allergens_sources.get_key_value(allergen) {
                    let intersection: Ingridients = value
                        .intersection(&food.ingridients)
                        .map(|element| element.to_owned())
                        .collect();
                    let _unused = allergens_sources.insert(key.to_owned(), intersection);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn hashset(data: &[String]) -> HashSet<String> {
        HashSet::from_iter(data.iter().cloned())
    }

    #[test_case("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)" => 
        Food {  allergens: hashset(&["dairy".to_owned(), "fish".to_owned()]), 
                ingridients: hashset(&["mxmxvkd".to_owned(), "kfcds".to_owned(), "sqjhc".to_owned(), "nhms".to_owned()]) })]
    #[test_case("sqjhc fvjkl (contains soy)" => 
        Food {  allergens: hashset(&["soy".to_owned()]), 
                ingridients: hashset(&["sqjhc".to_owned(), "fvjkl".to_owned()]) })]
    fn ex21_food_from_str(s: &str) -> Food {
        s.parse::<Food>().unwrap()
    }

    #[test]
    fn ex21_get_possible_allergens_sources() {
        let t = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

        let foods = t.parse::<Foods>().unwrap();
        assert_eq!(
            foods.get_possible_allergens_sources(),
            vec!["mxmxvkd", "sqjhc", "fvjkl"]
        )
    }

    #[test]
    fn ex21_count_ingridients_without_allergens() {
        let foods = vec![
            Food {
                allergens: hashset(&["dairy".to_string(), "fish".to_string()]),
                ingridients: hashset(&[
                    "mxmxvkd".to_string(),
                    "kfcds".to_string(),
                    "sqjhc".to_string(),
                    "nhms".to_string(),
                ]),
            },
            Food {
                allergens: hashset(&["dairy".to_string()]),
                ingridients: hashset(&[
                    "trh".to_string(),
                    "fvjkl".to_string(),
                    "sbzzf".to_string(),
                    "mxmxvkd".to_string(),
                ]),
            },
            Food {
                allergens: hashset(&["soy".to_string()]),
                ingridients: hashset(&["sqjhc".to_string(), "fvjkl".to_string()]),
            },
            Food {
                allergens: hashset(&["fish".to_string()]),
                ingridients: hashset(&[
                    "sqjhc".to_string(),
                    "mxmxvkd".to_string(),
                    "sbzzf".to_string(),
                ]),
            },
        ];

        let allergens_sources = &vec![
            "mxmxvkd".to_string(),
            "sqjhc".to_string(),
            "fvjkl".to_string(),
        ];

        assert_eq!(
            Foods { foods }.count_ingridients_without_allergens(allergens_sources),
            5
        )
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day21::solve_default_file().unwrap(), 5)
    }
}
