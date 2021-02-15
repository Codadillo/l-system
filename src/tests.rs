use crate::*;

#[test]
// This test taken from http://www.paulbourke.net/fractals/lsys/
fn test_single_system_production() {
    let mut system = LSystem::new("F+F+F+F".into(), HashMap::new());
    system.register_production_rule("F".into(), || "F+F-F-FF+F+F-F ".into());
    system.step();
    assert_eq!(system.axiom, "F+F-F-FF+F+F-F +F+F-F-FF+F+F-F +F+F-F-FF+F+F-F +F+F-F-FF+F+F-F ".to_owned());
}

#[test]
// This test taken from https://en.wikipedia.org/wiki/L-system#Example_1:_Algae
fn test_multi_system_production() {
    let mut system = LSystem::new("A".into(), HashMap::new());
    system.register_production_rule("A".into(), || "AB".into());
    system.register_production_rule("B".into(), || "A".into());

    system.step();
    assert_eq!(system.axiom, "AB".to_owned());

    system.step_by(2);
    assert_eq!(system.axiom, "ABAAB".to_owned());

    system.step_by(4);
    assert_eq!(system.axiom, "ABAABABAABAABABAABABAABAABABAABAAB".to_owned());
}
