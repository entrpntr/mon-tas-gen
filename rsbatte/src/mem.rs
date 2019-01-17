pub mod cartridge;
pub mod memptrs;
pub mod rtc;

use crate::mem::cartridge::Cartridge;

pub struct Memory {
  cart: Cartridge,
  ioamhram: [u8; 0x200],
}

impl Memory {
  pub fn new(rom_data: &[u8]) -> Memory {
    let cart = Cartridge::new(rom_data);
    Memory {
      cart,
      ioamhram: INITIAL_IOAMHRAM,
    }
  }
  pub fn reset(&mut self) {
    self.cart.reset();
    self.ioamhram = INITIAL_IOAMHRAM;
  }

  #[inline]
  pub fn has_blit(&self, _cycle_counter: u64) -> bool {
    panic!("unimplemented");
  }
  pub fn set_end_time(&mut self, _cycle_counter: u64, _cycles: u64) {
    panic!("unimplemented");
  }
  #[inline]
  pub fn is_active(&self) -> bool {
    panic!("unimplemented");
  }
  #[inline]
  pub fn is_halted(&self) -> bool {
    panic!("unimplemented");
  }
  #[inline]
  pub fn next_event_time(&self) -> u64 {
    panic!("unimplemented");
  }
  pub fn event(&self, _cycle_counter: u64) -> u64 {
    panic!("unimplemented");
  }
  pub fn read_ff(&self, address: u16, cycle_counter: u64) -> u8 {
    if address >= 0xff80 {
      self.ioamhram[usize::from(address - 0xfe00)]
    } else {
      self.read_ff_nontrivial(address, cycle_counter)
    }
  }
  pub fn read(&self, address: u16, cycle_counter: u64) -> u8 {
    if address < 0xfe00 {
      self.cart.read(address)
    } else if address < 0xff00 || address >= 0xff80 {
      self.ioamhram[usize::from(address - 0xfe00)]
    } else {
      self.read_ff_nontrivial(address, cycle_counter)
    }
  }
  pub fn read_ff_nontrivial(&self, _address: u16, _cycle_counter: u64) -> u8 {
    panic!("unimplemented");
  }
  pub fn oam_dma(&mut self, src: u8) {
    panic!("unimplemented");
    // memcpy(ioamhram, cart.wramdata(data >> 4 & 1) + (data << 8 & 0xFFF), 0xa0);
    // display.oamChange(cycleCounter);
  }
  pub fn write(&self, address: u16, value: u8, cycle_counter: u64) {
    panic!("unimplemented");
  }

  #[inline] pub fn read_ioamhram(&self, address: u16) -> u8 { unsafe { *self.ioamhram.get_unchecked(usize::from(address - 0xfe00)) } }
  #[inline] pub fn read_memory_map(&self, address: u16) -> u8 { self.cart.read_memory_map(address) }
  #[inline] pub fn read_memory_map_u16_le(&self, address: u16) -> u16 { self.cart.read_memory_map_u16_le(address) }
  #[inline] pub fn set_interrupt(&mut self, rom_address: usize) { self.cart.set_interrupt(rom_address) }
  #[inline] pub fn clear_interrupt(&mut self, rom_address: usize) { self.cart.clear_interrupt(rom_address) }
  #[inline] pub fn is_interrupt(&mut self, memory_address: u16) -> usize { self.cart.is_interrupt(memory_address) }
  #[inline] pub fn set_current_rtc(&mut self, current_rtc: std::num::Wrapping<u32>) { self.cart.set_current_rtc(current_rtc) }
}

const INITIAL_IOAMHRAM: [u8; 0x200] = [
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x08, 0x01, 0xEF, 0xDE, 0x06, 0x4A, 0xCD, 0xBD, 0x08, 0x01, 0xEF, 0xDE, 0x06, 0x4A, 0xCD, 0xBD,
  0x08, 0x01, 0xEF, 0xDE, 0x06, 0x4A, 0xCD, 0xBD, 0x08, 0x01, 0xEF, 0xDE, 0x06, 0x4A, 0xCD, 0xBD,
  0x00, 0x90, 0xF7, 0x7F, 0xC0, 0xB1, 0xBC, 0xFB, 0x00, 0x90, 0xF7, 0x7F, 0xC0, 0xB1, 0xBC, 0xFB,
  0x00, 0x90, 0xF7, 0x7F, 0xC0, 0xB1, 0xBC, 0xFB, 0x00, 0x90, 0xF7, 0x7F, 0xC0, 0xB1, 0xBC, 0xFB,
  0x24, 0x13, 0xFD, 0x3A, 0x10, 0x10, 0xAD, 0x45, 0x24, 0x13, 0xFD, 0x3A, 0x10, 0x10, 0xAD, 0x45,
  0x24, 0x13, 0xFD, 0x3A, 0x10, 0x10, 0xAD, 0x45, 0x24, 0x13, 0xFD, 0x3A, 0x10, 0x10, 0xAD, 0x45,
  0xCF, 0x00, 0x7C, 0xFF, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xE1,
  0x80, 0x3F, 0x00, 0xFF, 0xBF, 0xFF, 0x3F, 0x00, 0xFF, 0xBF, 0x7F, 0xFF, 0x9F, 0xFF, 0xBF, 0xFF,
  0xFF, 0x00, 0x00, 0xBF, 0x00, 0x00, 0x70, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
  0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
  0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x7E, 0xFF, 0xFE,
  0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3E, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
  0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0xFF, 0xC1, 0x00, 0xFE, 0xFF, 0xFF, 0xFF,
  0xF8, 0xFF, 0x00, 0x00, 0x00, 0x8F, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
  0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
  0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
  0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
  0x45, 0xEC, 0x42, 0xFA, 0x08, 0xB7, 0x07, 0x5D, 0x01, 0xF5, 0xC0, 0xFF, 0x08, 0xFC, 0x00, 0xE5,
  0x0B, 0xF8, 0xC2, 0xCA, 0xF4, 0xF9, 0x0D, 0x7F, 0x44, 0x6D, 0x19, 0xFE, 0x46, 0x97, 0x33, 0x5E,
  0x08, 0xFF, 0xD1, 0xFF, 0xC6, 0x8B, 0x24, 0x74, 0x12, 0xFC, 0x00, 0x9F, 0x94, 0xB7, 0x06, 0xD5,
  0x40, 0x7A, 0x20, 0x9E, 0x04, 0x5F, 0x41, 0x2F, 0x3D, 0x77, 0x36, 0x75, 0x81, 0x8A, 0x70, 0x3A,
  0x98, 0xD1, 0x71, 0x02, 0x4D, 0x01, 0xC1, 0xFF, 0x0D, 0x00, 0xD3, 0x05, 0xF9, 0x00, 0x0B, 0x00
];