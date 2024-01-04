use std::thread::sleep;
use std::time::Duration;
use thirtyfour_sync::components::select::SelectElement;
use thirtyfour_sync::http::reqwest_sync::ReqwestDriverSync;
use thirtyfour_sync::prelude::*;
use thirtyfour_sync::GenericWebDriver;

// https://googlechromelabs.github.io/chrome-for-testing/
// https://github.com/stevepryde/thirtyfour_sync
pub(crate) fn play_audio() {
    let voice_name = "Katja";
    let phrase = "Wie geht es Ihnen?";

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).expect("Failed to create session");

    driver
        .get("https://www.naturalreaders.com/online/")
        .expect("Failed to load page");

    SelectElement::new(
        &driver
            .find_element(By::Id("pw_languages"))
            .expect("Failed to find language dropdown"),
    )
    .expect("Failed to find language dropdown")
    .select_by_visible_text("German")
    .expect("Failed to select German language");
    sleep(Duration::from_secs(1));

    let voice_button = driver
        .find_elements(By::Css(
            "div[class=pw-voice-content] button[class=pw-voice-item]",
        ))
        .expect("Failed to find the voice buttons")
        .into_iter()
        .filter(|e| e.text().unwrap().contains(voice_name))
        .next()
        .unwrap();
    voice_button.click().expect(&format!(
        "Failed to click on {} German voice button",
        voice_name
    ));
    sleep(Duration::from_secs(1));
    voice_button.click().expect(&format!(
        "Failed to click on {} German voice button",
        voice_name
    ));
    sleep(Duration::from_secs(1));

    click_on(&driver, By::Css("div[class=pw-voice-footer] > button"));
    sleep(Duration::from_secs(1));

    click_on(&driver, By::Css("div[id=switch-pw-card] > a[class=nr-btn]"));
    sleep(Duration::from_secs(5));

    let input_element = driver.find_element(By::Id("inputDiv")).unwrap();
    input_element.focus().unwrap();
    input_element.clear().unwrap();
    input_element.send_keys(phrase).unwrap();
    sleep(Duration::from_secs(1));

    driver
        .find_elements(By::Css(
            "div[class=pw-read-bar] > div[class=pw-read-controls] button",
        ))
        .expect("Failed to find the element")
        .get(2)
        .unwrap()
        .click()
        .expect("Failed to click the element");
    sleep(Duration::from_secs(1));

    sleep(Duration::from_secs(10));

    driver.quit().expect("Failed to quit driver");
}

fn click_on(driver: &GenericWebDriver<ReqwestDriverSync>, by: By) {
    driver
        .find_element(by)
        .expect("Failed to find the element")
        .click()
        .expect("Failed to click the element");
}
