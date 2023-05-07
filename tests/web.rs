//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

// only get compiled when running test - helper functions

// create a starting spaceship pattern
#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2,3), (3,1), (3,2), (3, 3)]);
    universe
}

// check positon of cells after one tick (manually calculated)
#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

// test

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();
    let expected_universe = expected_spaceship();

    input_universe.tick();
    let input_map = (0..input_universe.get_cells().len()).map(|i| {
        input_universe.get_cells()[i] == true
    }).collect::<Vec<bool>>();

    let expected_map = (0..expected_universe.get_cells().len()).map(|i| {
        expected_universe.get_cells()[i] == true
    }).collect::<Vec<bool>>();

    assert_eq!(&input_map, &expected_map);
}