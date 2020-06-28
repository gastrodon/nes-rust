pub mod register;
pub mod cpu;
pub mod ppu;
pub mod apu;
pub mod rom;
pub mod memory;
pub mod mapper;
pub mod button;
pub mod input;
pub mod joypad;
pub mod audio;
pub mod display;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use cpu::Cpu;
use rom::Rom;
use button::Button;
use input::Input;
use display::Display;
use display::PIXELS_CAPACITY;
use audio::Audio;
use audio::BUFFER_CAPACITY;

pub struct Nes {
	cpu: Cpu
}

impl Nes {
	pub fn new(input: Box<dyn Input>, display: Box<dyn Display>, audio: Box<dyn Audio>) -> Self {
		Nes {
			cpu: Cpu::new(
				input,
				display,
				audio
			)
		}
	}

	pub fn set_rom(&mut self, rom: Rc<RefCell<Rom>>) {
		self.cpu.set_rom(rom.clone());
	}

	pub fn run(&mut self) {
		self.bootup();
		loop {
			self.step_frame();
			// @TODO: Fix sleep duration time
			// @TODO: timer should depend on platform
			//        (For example we use requestAnimationFrame on WASM + Web)
			//        so should we move it out from this class?
			std::thread::sleep(Duration::from_millis(2));
		}
	}

	pub fn step(&mut self) {
		self.cpu.step();
	}

	pub fn bootup(&mut self) {
		self.cpu.bootup();
	}

	pub fn reset(&mut self) {
		self.cpu.reset();
	}

	pub fn step_frame(&mut self) {
		self.cpu.step_frame();
	}

	// For WASM
	// @TODO: Are these methods really necessary?

	pub fn copy_pixels(&self, pixels: &mut [u8; PIXELS_CAPACITY]) {
		self.cpu.copy_pixels(pixels);
	}

	pub fn copy_sample_buffer(&mut self, buffer: &mut [f32; BUFFER_CAPACITY]) {
		self.cpu.copy_sample_buffer(buffer);
	}

	pub fn press_button(&mut self, button: Button) {
		self.cpu.press_button(button);
	}

	pub fn release_button(&mut self, button: Button) {
		self.cpu.release_button(button);
	}
}
