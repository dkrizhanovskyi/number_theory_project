use number_theory_project::ml::prediction::LinearRegression;
use number_theory_project::plugins::{load_plugin, Plugin};
use number_theory_project::strategy::factorization_strategy::{TrialDivisionStrategy, PollardsRhoStrategy};
use number_theory_project::strategy::FactorizationStrategy; // Импортируем трейт FactorizationStrategy
use number_theory_project::observer::computation_observer::{ComputationManager, ComputationObserver};
use number_theory_project::observer::Subject; // Импортируем трейт Subject
use number_theory_project::factory::curve_factory::{EllipticCurveFactory};
use number_theory_project::factory::CurveFactory; // Импортируем трейт CurveFactory

use ndarray::arr1;

fn main() {
    // Example 1: Machine Learning - Linear Regression
    let x_train = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    let y_train = arr1(&[2.0, 4.0, 6.0, 8.0, 10.0]);

    let mut model = LinearRegression::new();
    model.fit(&x_train, &y_train);

    let x_test = arr1(&[6.0, 7.0]);
    let predictions = model.predict(&x_test);

    println!("Predictions: {:?}", predictions);

    // Example 2: Plugin System
    let plugin = load_plugin("sample_plugin");
    plugin.execute();

    // Example 3: Strategy Pattern
    let trial_strategy = TrialDivisionStrategy;
    let pollards_strategy = PollardsRhoStrategy;

    let num = 91;
    println!("Factorization using Trial Division: {:?}", trial_strategy.factorize(num));
    println!("Factorization using Pollard's Rho: {:?}", pollards_strategy.factorize(num));

    // Example 4: Observer Pattern
    let mut manager = ComputationManager::new();
    let observer1 = Box::new(ComputationObserver::new("Observer 1"));
    let observer2 = Box::new(ComputationObserver::new("Observer 2"));

    manager.register_observer(observer1);
    manager.register_observer(observer2);

    manager.run_computation();

    // Example 5: Factory Pattern
    let curve_factory = EllipticCurveFactory;
    let curve = curve_factory.create_curve("Secp256k1");

    println!("Using curve: {}", curve.get_name());
    println!("{}", curve.generate_point());
}
