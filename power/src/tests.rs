#[cfg(test)]

use crate::power::TestKind;
use crate::string::json;
use crate::string::u8_to_string;
use crate::string::write_to_ptr;
use serde_json::Value;
use serde_json::json;
use crate::power::Tail;
use crate::interface::handle_received;

const ES: f64 = 0.5;
const ALPHA: f64 = 0.05;
const POWER: f64 = 0.95;
const N: f64 = 50.0;

/// Join two JSON objects; note that the second object takes precedence.
fn join_json(a: &Value, b: &Value) -> Value {
    let mut result = a.to_owned();
    for (key, value) in b.as_object().unwrap() {
        result[key] = value.to_owned();
    }
    result
}

#[test]
fn json_joining() {
    let a = json!({"a": 1});
    let b = json!({"b": 2});
    let c = json!({"a": 1, "b": 2});
    assert_eq!(join_json(&a, &b), c);
}

fn with_base(input: &Value) -> Value {
    let base = default_input();
    join_json(&base, input)
}

fn test_interface(input: &Value, output: f64) {
    let analysis = input["analysis"].as_str().unwrap();
    let text = input.to_owned().to_string();
    let returned = handle_received(&text);
    let result = returned[analysis].as_f64().unwrap();
    assert_eq!(result, output);
}

fn default_input() -> Value {
    json!({
        "n": N,
        "alpha": ALPHA,
        "power": POWER,
        "es": ES,
    })
}

fn with_rest(test: &str) -> impl Fn(&Value) -> Value {
    let extra = json!({"test": test});
    move | input | join_json(&with_base(input), &extra)
}

#[test]
fn one_sample_t_test() {
    let join = with_rest("OneSampleTTest");
    let extra = json!({"tail": 1, "analysis": "alpha"});
    test_interface(&join(&extra), 0.034);
    let extra = json!({"tail": 2, "analysis": "alpha"});
    test_interface(&join(&extra), 0.067);
    let extra = json!({"tail": 1, "analysis": "power"});
    test_interface(&join(&extra), 0.967);
    let extra = json!({"tail": 2, "analysis": "power"});
    test_interface(&join(&extra), 0.934);
}

#[test]
fn deviation_from_zero_multiple_regression() {
    let join = with_rest("DeviationFromZeroMultipleRegression");
    let f_squared = ES.sqrt();
    let extra = json!({"nPredictors": 2, "es": f_squared, "analysis": "alpha"});
    test_interface(&join(&extra), 0.006);
    let extra = json!({"nPredictors": 2, "es": f_squared, "analysis": "power"});
    test_interface(&join(&extra), 0.994);
    let extra = json!({"nPredictors": 2, "es": f_squared, "analysis": "es"});
    test_interface(&join(&extra), 0.574);
    let extra = json!({"nPredictors": 2, "es": f_squared, "analysis": "n"});
    test_interface(&join(&extra), 34.0);
}

