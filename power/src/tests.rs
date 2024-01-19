#[cfg(test)]

use serde_json::Value;
use serde_json::json;
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
    let join = with_rest("oneSampleTTest");
    let extra = json!({"tail": "1", "analysis": "alpha"});
    test_interface(&join(&extra), 0.034);
    let extra = json!({"tail": "2", "analysis": "alpha"});
    test_interface(&join(&extra), 0.067);
    let extra = json!({"tail": "1", "analysis": "power"});
    test_interface(&join(&extra), 0.967);
    let extra = json!({"tail": "2", "analysis": "power"});
    test_interface(&join(&extra), 0.934);
}

#[test]
fn deviation_from_zero_multiple_regression() {
    let join = with_rest("deviationFromZeroMultipleRegression");
    let f_squared = ES.sqrt();
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "alpha"});
    test_interface(&join(&extra), 0.006);
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "power"});
    test_interface(&join(&extra), 0.994);
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "es"});
    test_interface(&join(&extra), 0.574);
    let extra = json!({"nPredictors": "2", "es": f_squared, "analysis": "n"});
    test_interface(&join(&extra), 34.0);
}

#[test]
fn goodness_of_fit_chisq() {
    let df = "5";
    let join = with_rest("goodnessOfFitChisqTest");
    let extra = json!({"analysis": "alpha", "df": df});
    test_interface(&join(&extra), 0.254);
    let extra = json!({"analysis": "alpha", "df": df, "es": 0.628});
    test_interface(&join(&extra), 0.051);
    let extra = json!({"analysis": "power", "df": df});
    test_interface(&join(&extra), 0.788);
    let extra = json!({"analysis": "es", "df": df});
    test_interface(&join(&extra), 0.629);
    let extra = json!({"analysis": "n", "df": df});
    test_interface(&join(&extra), 79.0);
}

#[test]
fn increase_multiple_regression() {
    let p = "5";
    let q = "2";
    let f_squared = ES.sqrt();
    let join = with_rest("increaseMultipleRegression");
    let extra = json!({"analysis": "alpha", "p": p, "q": q, "es": f_squared});
    test_interface(&join(&extra), 0.006);
}

#[test]
fn independent_samples_t_test() {
    // G*Power only gives 0.392 if you put sample size group 1 and 2 both on n=50.
    // pwr.t.test(n=50, d=0.5, sig.level=NULL, power=0.95, type="two.sample", alternative="two.sided")
    let join = with_rest("independentSamplesTTest");
    let extra = json!({"analysis": "alpha", "tail": "2"});
    test_interface(&join(&extra), 0.398);

    let extra = json!({"tail": "1", "analysis": "n"});
    test_interface(&join(&extra), 88.0);
}
