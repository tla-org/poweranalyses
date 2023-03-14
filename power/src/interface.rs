use crate::power::TestKind;
use crate::string::json;
use crate::string::u8_to_string;
use crate::string::write_to_ptr;
use serde_json::Value;
use serde_json::json;
use crate::power::Tail;

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
    fn from_json(data: &Value) -> Result<Received, String> {
        let test = TestKind::from_str(data["test"].as_str().unwrap(), data)?;
        let analysis = Analysis::from_str(data["analysis"].as_str().unwrap())?;
        let n = data["n"].as_f64().unwrap();
        let alpha = data["alpha"].as_f64().unwrap();
        let power = data["power"].as_f64().unwrap();
        let es = data["es"].as_f64().unwrap();
        Ok(Received { test, analysis, n, alpha, power, es })
    }
}

fn round(x: f64, decimals: u32) -> f64 {
    let factor = i64::checked_pow(10, decimals);
    match factor {
        Some(number) => (x * number as f64).round() / number as f64,
        None => x
    }
}

#[test]
fn rounding() {
    assert_eq!(round(1.234, 2), 1.23);
}

pub fn handle_received(text: &str) -> Value {
    let data: Value = json(text).unwrap();
    let recv = Received::from_json(&data).unwrap();
    let tail = match Tail::from_json(&data) {
        Some(tail) => tail,
        None => Tail::OneSided,
    };
    let test = recv.test;
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
            println!("{tail:?}");
            println!("{}", test.power(tail.clone(), recv.n, recv.alpha, recv.es));
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
    let result = handle_received(&text);
    write_to_ptr(ptr, &result.to_string());
}
