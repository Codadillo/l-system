use crate::{turtle::*, LSystemExecutor};
use serde::de::DeserializeOwned;

// I like the macro but hate this file
macro_rules! make_builder {
    ( $struct_name:ident, $( $function:ident, )+ ) => {
        #[derive(Default)]
        pub struct $struct_name {
            $( pub $function: Option<String>, )+
        }

        impl $struct_name {
            pub fn new() -> Self {
                Self::default()
            }

            $(
                pub fn $function(mut self, token: String) -> Self {
                    self.$function = Some(token);
                    self
                }
            )+
        }
    };
}

make_builder!(BasicTurtleRegister, pop, push, rotate, travel, flush,);

impl BasicTurtleRegister {
    pub fn register_parametric_with_defaults<S, I, F, P, T, E>(
        &mut self,
        executor: &mut LSystemExecutor<S>,
        getter: impl 'static + Clone + Fn(&mut S) -> &mut T,
    ) where
        S: 'static,
        I: 'static + DeserializeOwned,
        F: 'static + DeserializeOwned,
        P: 'static + DeserializeOwned,
        E: std::fmt::Debug,
        T: BasicTurtle<I, F, P, Error = E>,
    {
        self.pop.get_or_insert("]".into());
        self.push.get_or_insert("[".into());
        self.rotate.get_or_insert("+".into());
        self.travel.get_or_insert("F".into());
        self.flush.get_or_insert(".".into());
        self.register_parametric(executor, getter);
    }

    pub fn register_parametric<S, I, F, P, T, E>(
        &self,
        executor: &mut LSystemExecutor<S>,
        getter: impl 'static + Clone + Fn(&mut S) -> &mut T,
    ) where
        S: 'static,
        I: 'static + DeserializeOwned,
        F: 'static + DeserializeOwned,
        P: 'static + DeserializeOwned,
        E: std::fmt::Debug,
        T: BasicTurtle<I, F, P, Error = E>,
    {
        if let Some(token) = &self.pop {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S| getter(s).pop());
        }
        if let Some(token) = &self.push {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S| getter(s).push());
        }
        if let Some(token) = &self.rotate {
            let getter = getter.clone();
            executor
                .register_rule(token.into(), move |s: &mut S, r: F| getter(s).rotate(r));
        }
        if let Some(token) = &self.travel {
            let getter = getter.clone();
            executor
                .register_rule(token.into(), move |s: &mut S, d: I| getter(s).travel(d));
        }
        if let Some(token) = &self.flush {
            let getter = getter.clone();
            executor
                .register_rule(token.into(), move |s: &mut S| getter(s).flush().unwrap());
        }
    }
}

make_builder!(
    FancyTurtleRegister,
    pop,
    push,
    rotate,
    travel,
    flush,
    set_color,
    scale_color,
    increment_color,
    set_line_width,
    scale_line_width,
    increment_line_width,
);

impl FancyTurtleRegister {
    pub fn register_parametric_with_defaults<S, I, F, P, T, C, E>(
        &mut self,
        executor: &mut LSystemExecutor<S>,
        getter: impl 'static + Clone + Fn(&mut S) -> &mut T,
    ) where
        S: 'static,
        I: 'static + DeserializeOwned,
        F: 'static + DeserializeOwned,
        P: 'static + DeserializeOwned,
        C: 'static + DeserializeOwned,
        E: std::fmt::Debug,
        T: FancyTurtle<I, F, P, C, Error = E>,
    {
        self.pop.get_or_insert("]".into());
        self.push.get_or_insert("[".into());
        self.rotate.get_or_insert("+".into());
        self.travel.get_or_insert("F".into());
        self.flush.get_or_insert(".".into());

        self.set_color.get_or_insert("^".into());
        self.scale_color.get_or_insert('"'.into());
        self.increment_color.get_or_insert(">".into());
        self.set_line_width.get_or_insert("_".into());
        self.scale_line_width.get_or_insert("/".into());
        self.increment_line_width.get_or_insert("$".into());

        self.register_parametric(executor, getter);
    }

    pub fn register_parametric<S, I, F, P, T, C, E>(
        &self,
        executor: &mut LSystemExecutor<S>,
        getter: impl 'static + Clone + Fn(&mut S) -> &mut T,
    ) where
        S: 'static,
        I: 'static + DeserializeOwned,
        F: 'static + DeserializeOwned,
        P: 'static + DeserializeOwned,
        C: 'static + DeserializeOwned,
        E: std::fmt::Debug,
        T: FancyTurtle<I, F, P, C, Error = E>,
    {
        if let Some(token) = &self.pop {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S| getter(s).pop());
        }
        if let Some(token) = &self.push {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S| getter(s).push());
        }
        if let Some(token) = &self.rotate {
            let getter = getter.clone();
            executor
                .register_rule(token.into(), move |s: &mut S, r: F| getter(s).rotate(r));
        }
        if let Some(token) = &self.travel {
            let getter = getter.clone();
            executor
                .register_rule(token.into(), move |s: &mut S, d: I| getter(s).travel(d));
        }
        if let Some(token) = &self.flush {
            let getter = getter.clone();
            executor
                .register_rule(token.into(), move |s: &mut S| getter(s).flush().unwrap());
        }
        if let Some(token) = &self.set_color {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S, c: C| {
                getter(s).set_color(c)
            });
        }
        if let Some(token) = &self.scale_color {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S, c: C| {
                getter(s).scale_color(c)
            });
        }
        if let Some(token) = &self.increment_color {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S, c: C| {
                getter(s).increment_color(c)
            });
        }
        if let Some(token) = &self.set_line_width {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S, w: F| {
                getter(s).set_line_width(w)
            });
        }
        if let Some(token) = &self.scale_line_width {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S, w: F| {
                getter(s).scale_line_width(w)
            });
        }
        if let Some(token) = &self.increment_line_width {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S, w: F| {
                getter(s).increment_line_width(w)
            });
        }
        if let Some(token) = &self.flush {
            let getter = getter.clone();
            executor.register_rule(token.into(), move |s: &mut S, w: F| {
                getter(s).scale_line_width(w)
            });
        }
    }
}
