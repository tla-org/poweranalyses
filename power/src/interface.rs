use crate::power::TestKind;
use crate::string::json;
use crate::string::u8_to_string;
use crate::string::write_to_ptr;
use serde_json::Value;
use serde_json::json;

enum Analysis {
    N,
    Alpha,
    Power,
    ES,
}

impl Analysis {
    fn from_str(text: &str) -> Result<Analysis, String> {
        match text {
            "n" => Ok(Analysis::N),
            "alpha" => Ok(Analysis::Alpha),
            "power" => Ok(Analysis::Power),
            "es" => Ok(Analysis::ES),
            _ => Err(format!("Unknown analysis: {}", text)),
        }
    }
}

struct Received {
    test: TestKind,
    analysis: Analysis,
    n: f64,
    alpha: f64,
    power: f64,
    es: f64,
}

impl Received {
    fn from_str(text: &str) -> Result<Received, String> {
        let data: Value = json(text).unwrap();
        let test = TestKind::from_str(data["test"].as_str().unwrap(), &data)?;
        let analysis = Analysis::from_str(data["analysis"].as_str().unwrap())?;
        let n = data["n"].as_f64().unwrap();
        let alpha = data["alpha"].as_f64().unwrap();
        let power = data["power"].as_f64().unwrap();
        let es = data["es"].as_f64().unwrap();
        Ok(Received { test, analysis, n, alpha, power, es })
    }
}

fn round(x: f64, decimals: u32) -> f64 {
    let factor = i32::checked_pow(10, decimals);
    return match factor {
        Some(number) => (x * number as f64).round() / number as f64,
        None => x
    };
}

#[test]
fn rounding() {
    assert_eq!(round(1.234, 2), 1.23);
}

fn handle_received(recv: Received) -> Value {
    let test = recv.test;
    let tail = 1;
    match recv.analysis {
        Analysis::N => {
            let n = test.n(tail, recv.alpha, recv.power, recv.es);
            json!({"n": n})
        },
        Analysis::Alpha => {
            let alpha = round(test.alpha(tail, recv.n, recv.power, recv.es), 3);
            json!({"alpha": alpha})
        }
        Analysis::Power => {
            let power = round(test.power(tail, recv.n, recv.alpha, recv.es), 3);
            json!({"power": power})
        },
        Analysis::ES => {
            let es = round(test.es(tail, recv.n, recv.alpha, recv.power), 3);
            json!({"es": es})
        }
    }
}

#[no_mangle]
pub extern fn calculatePower(ptr: *mut u8) {
    let text = unsafe { u8_to_string(ptr) };
    let recv = Received::from_str(&text).unwrap();
    let result = handle_received(recv);
    write_to_ptr(ptr, &result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const ES: f64 = 0.5;
    const ALPHA: f64 = 0.05;
    const POWER: f64 = 0.95;
    const N: f64 = 50.0;

    fn test_interface(input: &Value, output: f64) {
        let analysis = input["analysis"].as_str().unwrap();
        let text = input.to_owned().to_string();
        let recv = Received::from_str(&text).unwrap();
        let result = handle_received(recv);
        assert_eq!(result[analysis].as_f64().unwrap(), output);
    }

    fn default_input() -> Value {
        json!({
            "n": N,
            "alpha": ALPHA,
            "power": POWER,
            "es": ES,
        })
    }

    #[test]
    fn one_sample_t_test() {
        let mut input = default_input();
        input["test"] = json!("OneSampleTTest");
        input["analysis"] = json!("alpha");
        input["tail"] = json!(1);
        test_interface(&input, 0.034);
    }
}
