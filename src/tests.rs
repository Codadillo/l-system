use crate::*;

#[test]
// This test taken from http://www.paulbourke.net/fractals/lsys/
fn test_single_system_production() {
    let mut system = LSystem::new("F+F+F+F".into());
    system.register_rule("F".into(), || "F+F-F-FF+F+F-F ".into());
    system.step();
    assert_eq!(
        system.axiom,
        "F+F-F-FF+F+F-F +F+F-F-FF+F+F-F +F+F-F-FF+F+F-F +F+F-F-FF+F+F-F ".to_owned()
    );
}

#[test]
// This test taken from https://en.wikipedia.org/wiki/L-system#Example_1:_Algae
fn test_multi_system_production() {
    let mut system = LSystem::new("A".into());
    system.register_rule("A".into(), || "AB".into());
    system.register_rule("B".into(), || "A".into());

    system.step();
    assert_eq!(system.axiom, "AB".to_owned());

    system.step_by(2);
    assert_eq!(system.axiom, "ABAAB".to_owned());

    system.step_by(4);
    assert_eq!(
        system.axiom,
        "ABAABABAABAABABAABABAABAABABAABAAB".to_owned()
    );
}

#[test]
fn test_basic_executor() {
    type State = (i32, i32, i32);

    let mut executor = LSystemExecutor::new((0, 0, 0));

    executor.register_rule("A".into(), |state: &mut State| state.0 += 1);
    executor.register_rule("B".into(), |state: &mut State, b: i32| state.1 += b);
    executor.register_rule("C".into(), |state: &mut State, c: i32, m: i32| {
        state.2 += if state.0 > state.1 { c } else { m * c }
    });

    executor
        .execute(&LSystem::new("AAC(3, 0)B(6)B(1)AC(3, 2)".into()))
        .unwrap();

    assert_eq!(executor.state, (3, 7, 9));
}

#[test]
fn test_non_primitive_executor() {
    let mut executor = LSystemExecutor::new(0);

    executor.register_rule("A".into(), |state: &mut i32, complex: Vec<(i32, f64)>| {
        *state += complex.into_iter().map(|(a, b)| a as f64 * b).sum::<f64>() as i32
    });

    executor
        .execute(&LSystem::new(
            "A([[1, 2], [3, 4.1]])XXXXA([])XA([[0, 4], [1, 0.1]])XA([[3, 1.2]])".into(),
        ))
        .unwrap();

    assert_eq!(executor.state, 17);
}
