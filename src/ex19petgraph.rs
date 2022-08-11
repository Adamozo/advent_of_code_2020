use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

use petgraph::graph::Graph;
use petgraph::prelude::NodeIndex;
use petgraph::Direction;

use fnv::FnvHashMap as HashMap;
use std::str::FromStr;

type RuleNum = usize;
type Letter = char;
type RulesSequenceType = Vec<RuleNum>;
type Rules = HashMap<RuleNum, Rule>;

pub struct Day19petgraph;

impl DaySolver for Day19petgraph {
    type Output = usize;

    const INFO: DayInfo = DayInfo::with_day_and_file_and_variant(
        "day_19_petgraph",
        "data_files/ex19.txt",
        "petgraph",
    );

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let (rules, messages) = _s.split_once("\n\n").unwrap();

        let tree = Tree::from_rules(get_rules(rules));

        let res = messages
            .lines()
            .filter(|message| tree.does_message_match(message))
            .count();

        Ok(res)
    }
}

#[derive(Debug, PartialEq)]
enum Rule {
    Letter(Letter),
    RulesSequence(RulesSequenceType),
    RulesSequenceAlternative(RulesSequenceType, RulesSequenceType),
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rule::*;

        match s.split_once('|') {
            Some((first, second)) => Ok(RulesSequenceAlternative(
                get_rules_sequence(first),
                get_rules_sequence(second),
            )),
            None => {
                if s.contains('"') {
                    Ok(Letter(s.trim().chars().nth(1).unwrap())) // s.chars -> '"', 'a', '"'
                } else {
                    Ok(RulesSequence(get_rules_sequence(s)))
                }
            },
        }
    }
}

fn get_rules_sequence(s: &str) -> Vec<RuleNum> {
    s.split(' ')
        .filter(|char| !char.is_empty())
        .map(|rule_num| rule_num.parse::<RuleNum>().unwrap())
        .collect()
}

fn get_rules(s: &str) -> Rules {
    s.lines()
        .map(|line| {
            let (num, rule_body) = line.trim().split_once(':').unwrap();
            (
                num.parse::<usize>().unwrap(),
                rule_body.parse::<Rule>().unwrap(),
            )
        })
        .collect()
}

struct Tree {
    root_index: NodeIndex,
    body: Graph<Letter, ()>,
}

impl Tree {
    fn from_rules(rules: Rules) -> Self {
        let mut body: Graph<Letter, ()> = Graph::new();
        let root_index = body.add_node(' ');

        let mut tree = Tree { root_index, body };

        let _res = tree.helper(NodeIndex::new(0), &0, &rules);

        tree
    }

    fn helper(
        &mut self,
        start_node_num: NodeIndex,
        rule_number: &RuleNum,
        rules: &Rules,
    ) -> Option<NodeIndex> {
        use Rule::*;

        match rules.get(rule_number).unwrap() {
            Letter(letter) => Some(self.body.add_node(*letter)),

            RulesSequence(sequence) => {
                self.rules_sequence_walk(start_node_num, rules, sequence);

                None
            },

            RulesSequenceAlternative(sequence_left, sequence_right) => {
                let leaves = self.get_node_leaves(&start_node_num, &mut Vec::new());

                for sequence in [sequence_left, sequence_right] {
                    let empty_node = self.body.add_node(' ');

                    self.connect_node_to_nodes(&leaves, &empty_node);
                    self.rules_sequence_walk(empty_node, rules, sequence);
                }

                None
            },
        }
    }

    fn get_node_children(&self, node: &NodeIndex) -> Vec<NodeIndex> {
        self.body
            .neighbors_directed(*node, Direction::Outgoing)
            .collect::<Vec<_>>()
    }

    fn does_message_match(&self, message: &str) -> bool {
        let message_vec: Vec<char> = message.chars().collect();

        self.recur_match(&self.root_index, &message_vec, 0)
    }

    fn recur_match(
        &self,
        current_node: &NodeIndex,
        message: &Vec<char>,
        checked_char: usize,
    ) -> bool {
        let node_value = *self.body.node_weight(*current_node).unwrap();

        if node_value == ' ' {
            self.get_node_children(current_node)
                .iter()
                .any(|node| self.recur_match(node, message, checked_char))
        } else if node_value == message[checked_char] {
            if checked_char == message.len() - 1 {
                self.get_node_children(current_node).is_empty()
            } else {
                self.get_node_children(current_node)
                    .iter()
                    .any(|node| self.recur_match(node, message, checked_char + 1))
            }
        } else {
            false
        }
    }

    fn rules_sequence_walk(
        &mut self,
        start_node_num: NodeIndex,
        rules: &Rules,
        sequence: &Vec<usize>,
    ) {
        for rule in sequence {
            if let Some(node_to_connect) = self.helper(start_node_num, rule, rules) {
                self.connect_node_to_node_leaves(start_node_num, node_to_connect);
            }
        }
    }

    fn connect_node_to_node_leaves(&mut self, parrent_node: NodeIndex, node_to_connect: NodeIndex) {
        for leaf in self.get_node_leaves(&parrent_node, &mut Vec::new()) {
            self.body.add_edge(leaf, node_to_connect, ());
        }
    }

    fn connect_node_to_nodes(&mut self, nodes: &Vec<NodeIndex>, node_to_connect: &NodeIndex) {
        for node in nodes {
            self.body.add_edge(*node, *node_to_connect, ());
        }
    }

    fn get_node_leaves(&self, node: &NodeIndex, result: &mut Vec<NodeIndex>) -> Vec<NodeIndex> {
        let node_children = self.get_node_children(node);

        if node_children.is_empty() && !result.contains(node) {
            result.push(*node)
        } else {
            for child in node_children {
                self.get_node_leaves(&child, result);
            }
        }

        result.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::algo::is_isomorphic_matching;
    use test_case::test_case;

    #[test_case("1 2" => Rule::RulesSequence(vec![1,2]))]
    #[test_case(r#" "a""# => Rule::Letter('a'))]
    #[test_case("1 2 | 2 1" => Rule::RulesSequenceAlternative(vec![1,2], vec![2,1] ))]
    fn ex19_rule_fromstr(body: &str) -> Rule {
        body.parse::<Rule>().unwrap()
    }

    #[test]
    fn ex19_rules_fromstr() {
        use Rule::*;
        let input = r#"0: 1 2
        1: "a"
        2: 1 3 | 3 1
        3: "b""#;

        let mut rules: Rules = HashMap::default();
        rules.insert(0, RulesSequence(vec![1, 2]));
        rules.insert(1, Letter('a'));
        rules.insert(2, RulesSequenceAlternative(vec![1, 3], vec![3, 1]));
        rules.insert(3, Letter('b'));

        assert_eq!(get_rules(input), rules)
    }

    #[test]
    fn data_from_default_file() {
        assert_eq!(Day19petgraph::solve_default_file().unwrap(), 2)
    }

    #[test]
    fn ex19_tree_from_rules() {
        let input = r#"0: 1 2
        1: "a"
        2: 1 3 | 3 1
        3: "b""#;

        let tree = Tree::from_rules(get_rules(input));

        let mut body: Graph<char, ()> = Graph::new();

        let root_index = body.add_node(' ');
        let node_1 = body.add_node('a');
        let node_2 = body.add_node(' ');
        let node_3 = body.add_node('a');
        let node_4 = body.add_node('b');
        let node_5 = body.add_node(' ');
        let node_6 = body.add_node('b');
        let node_7 = body.add_node('a');

        body.add_edge(root_index, node_1, ());
        body.add_edge(node_1, node_2, ());
        body.add_edge(node_1, node_5, ());
        body.add_edge(node_2, node_3, ());
        body.add_edge(node_3, node_4, ());
        body.add_edge(node_5, node_6, ());
        body.add_edge(node_6, node_7, ());

        assert!(is_isomorphic_matching(
            &body,
            &tree.body,
            |a, b| a == b,
            |a, b| a == b
        ))
    }

    #[test]

    fn ex19_get_node_children() {
        let input = r#"0: 1 2
        1: "a"
        2: 1 3 | 3 1
        3: "b""#;

        let tree = Tree::from_rules(get_rules(input));

        assert_eq!(
            tree.get_node_children(&NodeIndex::new(1)),
            vec![NodeIndex::new(5), NodeIndex::new(2)]
        );
        assert_eq!(
            tree.get_node_children(&NodeIndex::new(5)),
            vec![NodeIndex::new(6)]
        )
    }

    #[test_case("aab" => true)]
    #[test_case("aba" => true)]
    #[test_case("bba" => false)]
    fn ex19_tree_does_message_match(message: &str) -> bool {
        let input = r#"0: 1 2
        1: "a"
        2: 1 3 | 3 1
        3: "b""#;

        let tree = Tree::from_rules(get_rules(input));

        tree.does_message_match(message)
    }

    #[test_case(&NodeIndex::new(0) => vec![NodeIndex::new(7), NodeIndex::new(4)])]
    #[test_case(&NodeIndex::new(3) => vec![NodeIndex::new(4)])]
    #[test_case(&NodeIndex::new(4) => vec![NodeIndex::new(4)])]
    fn ex19_get_node_leaves(node: &NodeIndex) -> Vec<NodeIndex> {
        let input = r#"0: 1 2
        1: "a"
        2: 1 3 | 3 1
        3: "b""#;

        let tree = Tree::from_rules(get_rules(input));

        tree.get_node_leaves(node, &mut Vec::new())
    }
}
