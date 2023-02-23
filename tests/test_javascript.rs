use headless_chrome::{Browser, Tab};
use std::error::Error;
use std::time::Duration;
use tokio;
use std::thread;

fn serve() {
    println!("Starting server at http://localhost:8085");
    std::process::Command::new("/home/rik/git/poweranalyses.org/script/serve.sh")
        .arg("8085")
        .output()
        .expect("failed to run serve.sh");
}

const TIMEOUT: Duration = Duration::from_secs(2);

fn get_attribute(attrs: Vec<String>, attr: &str) -> Option<String> {
    let n = attrs.len();
    for i in 0..n {
        if attrs[i] == attr {
            return Some(attrs[i + 1].to_string());
        };
    }
    None
}

fn get_float(tab: &Tab, id: &str) -> Result<String, Box<dyn Error>> {
    let selector = format!("#{id}");
    let elem = tab.wait_for_element_with_custom_timeout(&selector, TIMEOUT)?;
    let attrs = elem.get_attributes()?.unwrap();
    let value: String = get_attribute(attrs, "value").unwrap();
    Ok(value)
}

#[tokio::test]
async fn browse_wikipedia() -> Result<(), Box<dyn Error>> {
    thread::spawn(serve);
    std::thread::sleep(Duration::from_secs(1));

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.navigate_to("http://localhost:8085")?;

    // tab.wait_for_element("input#searchInput")?.click()?;
    let timeout = Duration::from_secs(2);
    let elem = tab.wait_for_element_with_custom_timeout("#n", timeout)?;
    println!("{:?}", elem.get_content());

    let value = get_float(&tab, "n").unwrap();
    println!("value: {value}");

    assert!(false);

    Ok(())
}
