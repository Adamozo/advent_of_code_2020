use aoc_utils::DayInfo;
use aoc_utils::DaySolver;
use fnv::FnvHashMap as HashMap;
use std::fmt::Debug;
use std::str::FromStr;

type RuleNum = usize;
type Letter = char;
type SubRuleType = Vec<RuleNum>;
type SubRulesAlternative = Vec<SubRuleType>;

pub struct Day19;

impl DaySolver for Day19 {
    type Output = usize;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_19", "data_files/ex19.txt", "base");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let (rules, messages) = _s.split_once("\n\n").unwrap();

        let tree = Tree::try_from(rules.parse::<Rules>()?)?;

        let result: usize = messages
            .lines()
            .filter(|message| tree.does_message_match(message))
            .count();

        Ok(result)
    }
}

#[derive(Debug, PartialEq)]
enum Rule {
    Letter(Letter),
    SubRules(SubRuleType),
    AlternativeSubRules(SubRulesAlternative),
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rule::*;

        if s.contains('|') {
            let body: SubRulesAlternative = s
                .trim()
                .split('|')
                .map(|sub_rules| {
                    sub_rules
                        .trim()
                        .split(' ')
                        .map(|rule_num| rule_num.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            Ok(AlternativeSubRules(body))
        } else if s.contains('"') {
            Ok(Letter(s.trim().replace('"', "").parse::<char>()?))
        } else {
            let body: SubRuleType = s
                .trim()
                .split(' ')
                .map(|rule_num| rule_num.parse::<usize>().unwrap())
                .collect();
            Ok(SubRules(body))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rules {
    body: HashMap<RuleNum, Rule>,
}

impl FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let body: HashMap<RuleNum, Rule> = s
            .lines()
            .map(|line| {
                let (num, rule_body) = line.split_once(':').unwrap();

                (
                    num.parse::<usize>().unwrap(),
                    rule_body.parse::<Rule>().unwrap(),
                )
            })
            .collect();

        Ok(Rules { body })
    }
}

impl Rules {
    fn get(&self, rule_num: &RuleNum) -> &Rule {
        self.body.get(rule_num).unwrap()
    }
}

#[derive(PartialEq, Debug)]
struct Tree {
    root_index: usize,
    nodes: Vec<Node>,
}

impl Tree {
    fn helper(
        &mut self,
        start_node_num: usize,
        rule_number: &RuleNum,
        rules: &Rules,
    ) -> Option<usize> {
        use Rule::*;
        match rules.get(rule_number) {
            Letter(body) => {
                self.nodes.push(Node {
                    value: Some(*body),
                    children: vec![],
                });
                Some(self.get_last_index())
            },

            SubRules(rules_sequence) => {
                for rule in rules_sequence {
                    if let Some(node_to_add) = self.helper(start_node_num, rule, rules) {
                        self.connect_node_to_node_leaves(start_node_num, node_to_add);
                    }
                }

                None
            },

            AlternativeSubRules(rules_sequence_collection) => {
                let leaves = self.get_node_leaves_indexes(&start_node_num, &mut Vec::new());

                for rules_sequence in rules_sequence_collection {
                    self.nodes.push(Node {
                        value: None,
                        children: vec![],
                    });
                    let empty_node_index = self.get_last_index();

                    self.connect_node_to_nodes(&leaves, &empty_node_index);

                    for rule in rules_sequence {
                        if let Some(node_to_add) = self.helper(empty_node_index, rule, rules) {
                            self.connect_node_to_node_leaves(empty_node_index, node_to_add);
                        }
                    }
                }

                None
            },
        }
    }

    fn connect_node_to_node_leaves(&mut self, parent_node_index: usize, child_node_index: usize) {
        for leaf in self.get_node_leaves_indexes(&parent_node_index, &mut Vec::new()) {
            self.nodes[leaf].children.push(child_node_index);
        }
    }

    fn connect_node_to_nodes(&mut self, nodes: &Vec<usize>, node: &usize) {
        for index in nodes {
            self.nodes[*index].children.push(*node);
        }
    }

    fn get_node_leaves_indexes(&self, node_index: &usize, result: &mut Vec<usize>) -> Vec<usize> {
        if self.get_node(node_index).is_leaf() && !result.contains(node_index) {
            result.push(*node_index);
        } else {
            for child in self.get_node_children(node_index) {
                self.get_node_leaves_indexes(child, result);
            }
        }

        result.to_vec()
    }

    fn new(root_value: Option<Letter>) -> Self {
        Self {
            root_index: 0,
            nodes: vec![Node::new(root_value)],
        }
    }

    fn get_last_index(&self) -> usize {
        self.nodes.len() - 1
    }

    fn get_node(&self, index: &usize) -> &Node {
        &self.nodes[*index]
    }

    fn get_node_children(&self, index: &usize) -> &Vec<usize> {
        &self.get_node(index).children
    }

    fn does_message_match(&self, message: &str) -> bool {
        let message_vec: Vec<char> = message.chars().collect();

        self.recur_match(&self.root_index, &message_vec, 0)
    }

    fn recur_match(
        &self,
        current_node_num: &usize,
        message: &Vec<char>,
        checked_char: usize,
    ) -> bool {
        if self.get_node(current_node_num).is_empty() {
            self.get_node_children(current_node_num)
                .iter()
                .any(|node_index| self.recur_match(node_index, message, checked_char))
        } else if self.get_node(current_node_num).value.unwrap() == message[checked_char] {
            if checked_char == message.len() - 1 {
                self.get_node_children(current_node_num).is_empty()
            } else {
                self.get_node_children(current_node_num)
                    .iter()
                    .any(|node_index| self.recur_match(node_index, message, checked_char + 1))
            }
        } else {
            false
        }
    }
}

impl TryFrom<Rules> for Tree {
    type Error = anyhow::Error;

    fn try_from(rules: Rules) -> Result<Self, Self::Error> {
        let mut searching_tree = Tree::new(None);

        searching_tree.helper(0, &0, &rules);

        Ok(searching_tree)
    }
}

#[derive(PartialEq, Debug)]
struct Node {
    value: Option<Letter>,
    children: Vec<usize>,
}

impl Node {
    fn new(value: Option<Letter>) -> Self {
        Self {
            value,
            children: vec![],
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn is_empty(&self) -> bool {
        self.value.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1 2" => Rule::SubRules(vec![1,2]))]
    #[test_case("\"a\"" => Rule::Letter('a'))]
    #[test_case("1 2 | 2 1" => Rule::AlternativeSubRules(vec![vec![1,2], vec![2,1]]) )]
    fn ex19_rule_fromstr(body: &str) -> Rule {
        body.parse::<Rule>().unwrap()
    }

    #[test]
    fn ex19_rules_fromstr() {
        use Rule::*;
        let input = "0: 1 2\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"";

        let mut body: HashMap<usize, Rule> = HashMap::default();
        body.insert(0, SubRules(vec![1, 2]));
        body.insert(1, Letter('a'));
        body.insert(2, AlternativeSubRules(vec![vec![1, 3], vec![3, 1]]));
        body.insert(3, Letter('b'));

        assert_eq!(input.parse::<Rules>().unwrap(), Rules { body })
    }

    #[test]
    fn ex19_rules_get() {
        use Rule::*;

        let input = "0: 1 2\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"";
        let rules = input.parse::<Rules>().unwrap();

        assert_eq!(*rules.get(&0), SubRules(vec![1, 2]));
        assert_eq!(*rules.get(&1), Letter('a'));
        assert_eq!(
            *rules.get(&2),
            AlternativeSubRules(vec![vec![1, 3], vec![3, 1]])
        )
    }

    #[test_case(Node { value: None, children: vec![] } => true)]
    #[test_case(Node { value: None, children: vec![1] } => false)]
    fn ex19_node_is_leaf(node: Node) -> bool {
        node.is_leaf()
    }

    #[test_case(Node { value: None, children: vec![] } => true)]
    #[test_case(Node { value: Some('a'), children: vec![] } => false)]
    fn ex19_node_is_empty(node: Node) -> bool {
        node.is_empty()
    }

    #[test]
    fn ex19_tree_tryfrom() {
        let input = "0: 1 2\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"";
        let tree = Tree::try_from(input.parse::<Rules>().unwrap()).unwrap();

        let nodes = vec![
            Node {
                value: None,
                children: vec![1],
            },
            Node {
                value: Some('a'),
                children: vec![2, 5],
            },
            Node {
                value: None,
                children: vec![3],
            },
            Node {
                value: Some('a'),
                children: vec![4],
            },
            Node {
                value: Some('b'),
                children: vec![],
            },
            Node {
                value: None,
                children: vec![6],
            },
            Node {
                value: Some('b'),
                children: vec![7],
            },
            Node {
                value: Some('a'),
                children: vec![],
            },
        ];

        assert_eq!(
            tree,
            Tree {
                root_index: 0,
                nodes: nodes
            }
        )
    }

    #[test_case("aab" => true)]
    #[test_case("aba" => true)]
    #[test_case("bba" => false)]
    fn ex19_tree_does_message_match(message: &str) -> bool {
        let input = "0: 1 2\n1: \"a\"\n2: 1 3 | 3 1\n3: \"b\"";
        let tree = Tree::try_from(input.parse::<Rules>().unwrap()).unwrap();

        tree.does_message_match(message)
    }

    #[test]
    fn ex19_tree_get_node_children() {
        /*
                      0
                    /   \
                   1     4
                  / \
                 2   3
        */

        let nodes = vec![
            Node {
                value: None,
                children: vec![1, 4],
            },
            Node {
                value: None,
                children: vec![2, 3],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
        ];

        let tree = Tree {
            root_index: 0,
            nodes: nodes,
        };

        assert_eq!(tree.get_node_children(&0), &vec![1, 4]);
        assert_eq!(tree.get_node_children(&1), &vec![2, 3]);
        assert_eq!(tree.get_node_children(&4), &Vec::<usize>::new())
    }

    #[test]
    fn ex19_tree_get_node() {
        /*
                      0
                    /   \
                   1     4
                  / \
                 2   3
        */

        let nodes = vec![
            Node {
                value: None,
                children: vec![1, 4],
            },
            Node {
                value: None,
                children: vec![2, 3],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
        ];

        let tree = Tree {
            root_index: 0,
            nodes: nodes,
        };

        assert_eq!(
            tree.get_node(&0),
            &Node {
                value: None,
                children: vec![1, 4]
            }
        );
        assert_eq!(
            tree.get_node(&3),
            &Node {
                value: None,
                children: vec![]
            }
        )
    }

    #[test]
    fn ex19_tree_get_last_index() {
        /*
                      0
                    /   \
                   1     4
                  / \
                 2   3
        */

        let nodes = vec![
            Node {
                value: None,
                children: vec![1, 4],
            },
            Node {
                value: None,
                children: vec![2, 3],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
        ];

        let tree = Tree {
            root_index: 0,
            nodes: nodes,
        };

        assert_eq!(tree.get_last_index(), 4)
    }

    #[test]
    fn ex19_tree_get_node_leaves_indexes() {
        /*
                      0
                    /   \
                   1     4
                  / \
                 2   3
        */

        let nodes = vec![
            Node {
                value: None,
                children: vec![1, 4],
            },
            Node {
                value: None,
                children: vec![2, 3],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
            Node {
                value: None,
                children: vec![],
            },
        ];

        let tree = Tree {
            root_index: 0,
            nodes: nodes,
        };

        assert_eq!(tree.get_node_leaves_indexes(&4, &mut Vec::new()), vec![4]);
        assert_eq!(
            tree.get_node_leaves_indexes(&0, &mut Vec::new()),
            vec![2, 3, 4]
        );
        assert_eq!(
            tree.get_node_leaves_indexes(&1, &mut Vec::new()),
            vec![2, 3]
        )
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day19::solve_default_file().unwrap(), 2)
    }
}
