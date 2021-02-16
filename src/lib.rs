#[cfg(test)]
mod tests;

use serde::de::DeserializeOwned;
use std::{collections::HashMap, marker::PhantomData};
use unicode_segmentation::UnicodeSegmentation;

pub trait CallParsed<State, T> {
    fn call_parsed(&mut self, state: &mut State, args: String) -> Result<(), serde_json::Error>;
}

trait CallParsedErased<State> {
    fn call_parsed(&mut self, state: &mut State, args: String) -> Result<(), serde_json::Error>;
}

struct Wrapper<State, T, C> {
    data: C,
    phantom: PhantomData<(State, T)>,
}

impl<State, T, C: CallParsed<State, T>> CallParsedErased<State> for Wrapper<State, T, C> {
    fn call_parsed(&mut self, state: &mut State, args: String) -> Result<(), serde_json::Error> {
        self.data.call_parsed(state, args)
    }
}

macro_rules! call_parsed_impls {
    ( $head:ident, $( $tail:ident, )* ) => {
        impl<AXSUDYF3412341234UCG, EUCBNAJHXIZAD81923IX, $head, $( $tail ),*> CallParsed<EUCBNAJHXIZAD81923IX, ($head, $( $tail ),*)> for AXSUDYF3412341234UCG
        where
            AXSUDYF3412341234UCG: FnMut(&mut EUCBNAJHXIZAD81923IX, $head, $( $tail ),*),
            $head: DeserializeOwned,
            $( $tail: DeserializeOwned ),*

        {
            fn call_parsed(&mut self, state8348912731: &mut EUCBNAJHXIZAD81923IX, args: String) -> Result<(), serde_json::Error> {              
                let mut graphemes: Vec<&str> = args.as_str().graphemes(true).collect();
                graphemes.first_mut().map(|g| *g = "[");
                graphemes.last_mut().map(|g| *g = "]");

                #[allow(non_snake_case)]
                let ($head, $( $tail ),*): ($head, $( $tail ),*) = serde_json::from_str(&graphemes.into_iter().collect::<String>())?;

                (self)(state8348912731, $head, $( $tail ),*);
                Ok(())
            }
        }

        call_parsed_impls!($( $tail, )*);
    };
    () => {
        impl<State, F> CallParsed<State, ()> for F
            where
                F: FnMut(&mut State)
        {
            fn call_parsed(&mut self, state: &mut State, _: String) -> Result<(), serde_json::Error> {
                (self)(state);
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
    pub fn new(axiom: String) -> Self {
        Self::with_rules(axiom, HashMap::new())
    }

    pub fn with_rules(
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

pub struct LSystemExecutor<State> {
    pub state: State,
    execution_rules: Vec<(String, Box<dyn CallParsedErased<State>>)>,
}

impl<State: 'static> LSystemExecutor<State> {
    pub fn new(state: State) -> Self {
        Self {
            execution_rules: vec![],
            state,
        }
    }

    pub fn register_execution_rule<T: 'static>(
        &mut self,
        token: String,
        rule: impl 'static + CallParsed<State, T>,
    ) {
        self.execution_rules.push((
            token,
            Box::new(Wrapper {
                data: rule,
                phantom: PhantomData,
            }) as Box<dyn CallParsedErased<State>>,
        ));
    }

    pub fn execute(&mut self, system: &LSystem) -> Result<(), serde_json::Error> {
        let mut instructions = system.axiom.clone();

        while instructions.len() != 0 {
            if let Some((token, rule)) = self
                .execution_rules
                .iter_mut()
                .find(|e| instructions.starts_with(&e.0))
            {
                let _: String = instructions.drain(..token.len()).collect();
                if instructions.chars().next() != Some('(') {
                    rule.call_parsed(&mut self.state, "".into())?;
                    continue;
                }

                for i in 0..instructions.len() {
                    let res = rule.call_parsed(&mut self.state, instructions[..=i].to_string());
                    if res.is_ok() {
                        let _: String = instructions.drain(..i).collect();
                        break;
                    }
                    if i == instructions.len() - 1 {
                        res?;
                    }
                }
            } else {
                instructions.remove(0);
            }
        }

        Ok(())
    }
}
