#[cfg(test)]
mod tests;

use serde::de::DeserializeOwned;
use std::any::Any;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub trait CallParsed<T> {
    fn call_parsed(&mut self, args: String) -> Result<(), serde_json::Error>;
}

macro_rules! call_parsed_impls {
    ( $head:ident, $( $tail:ident, )* ) => {
        impl<AXSUDYF3412341234UCG, $head, $( $tail ),*> CallParsed<($head, $( $tail ),*)> for AXSUDYF3412341234UCG
        where
            AXSUDYF3412341234UCG: FnMut($head, $( $tail ),*),
            $head: DeserializeOwned,
            $( $tail: DeserializeOwned ),*

        {
            fn call_parsed(&mut self, args: String) -> Result<(), serde_json::Error> {
                let mut graphemes: Vec<&str> = args.as_str().graphemes(true).collect();
                *graphemes.first_mut().unwrap() = "[";
                *graphemes.last_mut().unwrap() = "]";

                #[allow(non_snake_case)]
                let ($head, $( $tail ),*): ($head, $( $tail ),*) = serde_json::from_str(&graphemes.into_iter().collect::<String>())?;

                (self)($head, $( $tail ),*);
                Ok(())
            }
        }
        call_parsed_impls!($( $tail, )*);
    };
    () => {
        impl<F> CallParsed<()> for F
            where
                F: FnMut()
        {
            fn call_parsed(&mut self, _: String) -> Result<(), serde_json::Error> {
                (self)();
                Ok(())
            }
        }
    };
}

call_parsed_impls!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,);

pub struct LSystem {
    pub axiom: String,
    pub production_rules: HashMap<String, Box<dyn FnMut() -> String>>,
}

impl LSystem {
    pub fn new(
        axiom: String,
        production_rules: HashMap<String, Box<dyn FnMut() -> String>>,
    ) -> Self {
        Self {
            axiom,
            production_rules,
        }
    }

    pub fn register_production_rule(
        &mut self,
        token: String,
        replacement: impl 'static + FnMut() -> String,
    ) {
        self.production_rules.insert(token, Box::new(replacement));
    }

    pub fn step(&mut self) {
        let mut old_axiom = vec![(0, self.axiom.clone())];
        let mut new_axiom = vec![];

        for (token, replacement) in self.production_rules.iter_mut() {
            let mut old_axiom_extend = vec![];

            for (i, part) in &mut old_axiom {
                for j in part
                    .match_indices(token)
                    .map(|p| p.0)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                {
                    new_axiom.push((*i + j, (replacement)()));

                    old_axiom_extend.push((*i + j + token.len(), part.split_off(j + token.len())));
                    part.truncate(part.len() - token.len());
                }
            }

            old_axiom.extend_from_slice(&old_axiom_extend);
        }

        new_axiom.extend_from_slice(&old_axiom);
        new_axiom.sort_by(|a, b| a.0.cmp(&b.0));
        self.axiom = new_axiom.into_iter().map(|a| a.1).collect();
    }

    pub fn step_by(&mut self, n: usize) {
        (0..n).for_each(|_| self.step())
    }
}

pub struct LSystemExecutor {
    execution_rules: HashMap<String, Box<dyn Any>>,
}

impl LSystemExecutor {
    pub fn new() -> Self {
        Self {
            execution_rules: HashMap::new(),
        }
    }

    pub fn register_execution_rule<T>(
        &mut self,
        token: String,
        rule: impl 'static + CallParsed<T>,
    ) {
        self.execution_rules
            .insert(token, Box::new(rule) as Box<dyn Any>);
    }
}
