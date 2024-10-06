use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use rand::Rng;
use std::{thread, time::Duration};

const NUM_LEDS: usize = 10; // Adjust this to the number of LEDs you have

struct LedState {
    current_color: (u8, u8, u8),
    target_color: (u8, u8, u8),
    ticks_since_last_target: u32,
    ticks_to_next_target: u32,
}

fn main() {
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();
    let mut rng = rand::thread_rng();

    // Initialize the LEDs
    let mut led_states = Vec::with_capacity(NUM_LEDS);

    for _ in 0..NUM_LEDS {
        let initial_color = generate_random_fire_color(&mut rng);
        let target_color = generate_random_fire_color(&mut rng);
        let ticks_to_next_target = rng.gen_range(5..=20);
        led_states.push(LedState {
            current_color: initial_color,
            target_color,
            ticks_since_last_target: 0,
            ticks_to_next_target,
        });
    }

    loop {
        let mut rgb_values = Vec::with_capacity(NUM_LEDS);

        for led in &mut led_states {
            led.ticks_since_last_target += 1;

            let t = led.ticks_since_last_target as f32 / led.ticks_to_next_target as f32;
            let t = t.min(1.0); // Clamp t to a maximum of 1.0

            // Use one of the interpolation functions here
            let new_color = interpolate_color_ease_in_out(led.current_color, led.target_color, t);
            // Alternatively, use linear interpolation:
            // let new_color = interpolate_color(led.current_color, led.target_color, t);

            // If we've reached the target, set up a new target
            if led.ticks_since_last_target >= led.ticks_to_next_target {
                led.current_color = led.target_color;
                led.target_color = generate_random_fire_color(&mut rng);
                led.ticks_since_last_target = 0;
                led.ticks_to_next_target = rng.gen_range(10..=15);
            }

            rgb_values.push(new_color);
        }

        adapter.write_rgb(&rgb_values).unwrap();
        thread::sleep(Duration::from_millis(15)); // Adjust delay as needed for desired flicker speed
    }
}


/// Generates a random fire-like color.
///
/// # Arguments
///
/// * `rng` - A mutable reference to a random number generator.
///
/// # Returns
///
/// A tuple `(u8, u8, u8)` representing the RGB color.
fn generate_random_fire_color<R: Rng + ?Sized>(rng: &mut R) -> (u8, u8, u8) {
    let red = rng.gen_range(150..=200);
    let green = (((red as f32)*0.2) as i8) + rng.gen_range(-20..=10);
    let blue = rng.gen_range(0..=2);
    (red, green as u8, blue)
}

/// Interpolates between two colors using linear interpolation.
///
/// # Arguments
///
/// * `start` - The starting color as a tuple `(u8, u8, u8)`.
/// * `end` - The target color as a tuple `(u8, u8, u8)`.
/// * `t` - The interpolation factor (0.0 to 1.0).
///
/// # Returns
///
/// A new color tuple representing the interpolated color.
fn interpolate_color(start: (u8, u8, u8), end: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let r = interpolate(start.0, end.0, t);
    let g = interpolate(start.1, end.1, t);
    let b = interpolate(start.2, end.2, t);
    (r, g, b)
}

/// Interpolates between two colors using an ease-in-out function for smoother transitions.
///
/// # Arguments
///
/// * `start` - The starting color as a tuple `(u8, u8, u8)`.
/// * `end` - The target color as a tuple `(u8, u8, u8)`.
/// * `t` - The interpolation factor (0.0 to 1.0).
///
/// # Returns
///
/// A new color tuple representing the interpolated color with easing.
fn interpolate_color_ease_in_out(start: (u8, u8, u8), end: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t_eased = ease_in_out(t);
    interpolate_color(start, end, t_eased)
}

/// Linear interpolation between two `u8` values.
///
/// # Arguments
///
/// * `start` - The starting value.
/// * `end` - The target value.
/// * `t` - The interpolation factor (0.0 to 1.0).
///
/// # Returns
///
/// The interpolated value as a `u8`.
fn interpolate(start: u8, end: u8, t: f32) -> u8 {
    let start_f = start as f32;
    let end_f = end as f32;
    let value = start_f + t * (end_f - start_f);
    value.round() as u8
}

/// Ease-in-out function for smoother interpolation.
///
/// This function accelerates at the beginning and decelerates towards the end.
///
/// # Arguments
///
/// * `t` - The interpolation factor (0.0 to 1.0).
///
/// # Returns
///
/// A new interpolation factor adjusted for easing.
fn ease_in_out(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}