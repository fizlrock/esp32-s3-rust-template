mod freq_meter;

use std::time::Duration;

use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{Input, InputPin, PinDriver, Pull},
    ledc::{config, LedcDriver, LedcTimerDriver},
    prelude::Peripherals,
};

use esp_idf_hal::prelude::*;

use esp_idf_svc::timer::EspTimerService;
use freq_meter::FreqMeter;
use log::info;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Старт приложения!");

    // Получаем перефирийные устройства для работы с таймером
    let peripherals = Peripherals::take().unwrap();

    let mut fm = FreqMeter::new(peripherals.pcnt0, peripherals.pins.gpio20);

    let ets = EspTimerService::new().unwrap();

    let tim = ets
        .timer(move || {
            fm.measure_fr();

            let f = fm.get_fr();

            info!("freq: {}", f);
        })
        .unwrap();

    tim.every(Duration::from_millis(500)).unwrap();

    FreeRtos::delay_ms(100000);
}
