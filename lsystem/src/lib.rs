use std::{collections::HashMap, fmt::Debug};

type Iteration = Vec<char>;
pub type RuleSet = HashMap<char, Iteration>;

#[macro_export]
macro_rules! vectorize {
	($string_literal:literal) => {
		$string_literal.chars().collect()
	};
}

/// From [stack-overflow](https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal)
#[macro_export]
macro_rules! to_hash {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::{collections::HashMap};
        use std::iter::FromIterator;
        use std::array::IntoIter;
        HashMap::<char, Vec<char>>::from_iter(IntoIter::new([$(($k, $v),)*]))
    }};
}

// #[macro_export]
// macro_rules! lsystem_rules {
//     ($rules_name:ident, $($var:ident => $rule_str:literal),+) => {
//         let mut $rules_name = RuleSet::new();
//         lsystem_rules!($rules_name $($var => $rule_str),+)
//     };
//     ($rules_name:ident $var:ident => $rule_str:literal, $($next_var:ident => $next_rule_str:literal),+) => {
//         $rules_name.insert(stringify!($var).chars().next().unwrap(), vectorize!($rule_str));
//         lsystem_rules!($rules_name $($next_var => $next_rule_str),+)
//     };
//     ($rules_name:ident $var:ident => $rule_str:literal) => {
//         $rules_name.insert(stringify!($var).chars().next().unwrap(), vectorize!($rule_str));
//     };
// }

#[macro_export]
macro_rules! lsystem {
    ($axiom_str:literal, $($k:ident => $v:literal),* $(,)?) => {
        lsystem::LSystem::new_from_axiom_string_and_rules($axiom_str, lsystem::to_hash!(
            $(
                stringify!($k).chars().next().unwrap() => lsystem::vectorize!($v),
            )*
        ));
    };
}

#[derive(Clone)]
pub struct LSystem {
    axiom: Iteration,
    current_iteration: Iteration,
    rules: RuleSet
}

impl Debug for LSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let axiom_str: String = self.axiom.clone().into_iter().collect();
        write!(f, "lsystem!({}, ", axiom_str);
        for (k, v) in &self.rules {
            let rule_string: String = v.clone().into_iter().collect();
            write!(f, "{} => \"{}\", ", k, rule_string);
        }

        write!(f, ");")
    }
}

impl LSystem {
    pub fn new() -> Self {
        LSystem {
            axiom: vec![],
            current_iteration: vec![],
            rules: RuleSet::new(),
        }
    }

    pub fn new_from_axiom_vec(axiom: Iteration) -> Self {
        LSystem {
            axiom: axiom.clone(),
            current_iteration: axiom,
            rules: RuleSet::new(),
        }
    }

    pub fn new_from_axiom_string(axiom: &str) -> Self {
        LSystem {
            axiom: axiom.chars().collect(),
            current_iteration: axiom.chars().collect(),
            rules: RuleSet::new(),
        }
    }

    pub fn new_from_axiom_string_and_rules(axiom: &str, rules: RuleSet) -> Self {
        LSystem {
            axiom: axiom.chars().collect(),
            current_iteration: axiom.chars().collect(),
            rules: rules,
        }
    }

    pub fn new_from_axiom_vec_and_rules(axiom: &str, rules: RuleSet) -> Self {
        LSystem {
            axiom: axiom.chars().collect(),
            current_iteration: axiom.chars().collect(),
            rules: rules,
        }
    }

    pub fn get_axiom_str(&self) -> String {
        return self.axiom.clone().into_iter().collect();
    }

    pub fn get_rules(&self) -> &RuleSet {
        &self.rules
    }

    pub fn set_axiom(&mut self, axiom: Iteration) {
        self.axiom = axiom;
    }

    pub fn reset(&mut self) {
        self.current_iteration = self.axiom.clone();
    }

    pub fn add_rule(&mut self, var: char, replacment: Iteration) {
        self.rules.insert(var, replacment);
    }

    pub fn remove_rule(&mut self, var: &char) -> Option<Iteration> {
        self.rules.remove(var)
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    pub fn get_current(&self) -> &Iteration {
        &self.current_iteration
    }
}

impl Iterator for LSystem {
    type Item = Iteration;

    fn next(&mut self) -> Option<Self::Item> {
        let mut temp: Iteration = vec![];
        for variable in &self.current_iteration {
            if let Some(rule) = self.rules.get(variable) {
                temp.extend(rule);
            } else {
                temp.push(*variable);
            }
        }
        self.current_iteration = temp;
        Some(self.current_iteration.clone())
    }
}