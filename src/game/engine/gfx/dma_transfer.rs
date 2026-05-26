use crate::runtime::Cpu;

pub fn hdma_transfer_attrmap_and_tilemap_to_wram_bank3(cpu: &mut Cpu) {
    cpu.pc = 0x4000;
    cpu.set_hl(0x4006);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn hdma_transfer_attrmap_and_tilemap_to_wram_bank3_function(cpu: &mut Cpu) {
    cpu.pc = 0x4006;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_attrmap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_tilemap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_to_wram_bank3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_to_wram_bank3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn hdma_transfer_tilemap_to_wram_bank3(cpu: &mut Cpu) {
    cpu.pc = 0x402d;
    cpu.set_hl(0x4033);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn hdma_transfer_tilemap_to_wram_bank3_function(cpu: &mut Cpu) {
    cpu.pc = 0x4033;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_tilemap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_to_wram_bank3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn hdma_transfer_attrmap_to_wram_bank3(cpu: &mut Cpu) {
    cpu.pc = 0x4047;
    cpu.set_hl(0x404d);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn hdma_transfer_attrmap_to_wram_bank3_function(cpu: &mut Cpu) {
    cpu.pc = 0x404d;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_attrmap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_to_wram_bank3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn hdma_transfer_tilemap_and_attrmap_overworld(cpu: &mut Cpu) {
    cpu.pc = 0x4061;
    cpu.set_hl(0x4067);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn hdma_transfer_tilemap_and_attrmap_overworld_function(cpu: &mut Cpu) {
    cpu.pc = 0x4067;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_attrmap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_tilemap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(crate::defs::R_VBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128_to_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128_to_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mobile_hdma_transfer_tilemap_and_attrmap_overworld(cpu: &mut Cpu) {
    cpu.pc = 0x4099;
    cpu.set_hl(0x4061);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0x40a2);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn mobile_hdma_transfer_tilemap_and_attrmap_overworld_function(cpu: &mut Cpu) {
    cpu.pc = 0x40a2;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_attrmap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_tilemap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(crate::defs::R_VBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_no_di(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_no_di(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1040d4(cpu: &mut Cpu) {
    cpu.pc = 0x40d4;
    cpu.set_hl(0x40da);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn function1040d4_function(cpu: &mut Cpu) {
    cpu.pc = 0x40da;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_de(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffd6 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VDMA_SRC_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffd6);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VDMA_SRC_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_DEST_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_DEST_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffe8, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    wait_dma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn function1040fb(cpu: &mut Cpu) {
    cpu.pc = 0x40fb;
    cpu.set_hl(0x4101);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn function1040fb_function(cpu: &mut Cpu) {
    cpu.pc = 0x4101;
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = 0x03;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd800);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_to_wram_bank3(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn _hdma_transfer_tilemap_and_attrmap_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4110;
    cpu.set_hl(0x4116);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn _hdma_transfer_tilemap_and_attrmap_menu_function(cpu: &mut Cpu) {
    cpu.pc = 0x4116;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_attrmap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_tilemap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(crate::defs::R_VBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline124_to_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline124_to_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn mobile_hdma_transfer_tilemap_and_attrmap_menu(cpu: &mut Cpu) {
    cpu.pc = 0x4148;
    cpu.set_hl(0x414e);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn mobile_hdma_transfer_tilemap_and_attrmap_menu_function(cpu: &mut Cpu) {
    cpu.pc = 0x414e;
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_attrmap_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.c = 0xff;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    pad_map_for_hdma_transfer(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd400);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128_to_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128_to_bg_map(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn call_in_safe_gfx_mode(cpu: &mut Cpu) {
    cpu.pc = 0x4177;
    cpu.a = cpu.read_byte(0xffd4);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.read_byte(0xffde);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0;
    cpu.flag_z = true;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(0xffde, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x06;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(crate::defs::R_VBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    call_in_safe_gfx_mode_hl_(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffde, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffd4, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn hdma_transfer_to_wram_bank3(cpu: &mut Cpu) {
    cpu.pc = 0x419d;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    _load_hdma_parameters(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x23;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(0xffe8, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    become wait_dma_transfer(cpu);
}

pub(crate) fn wait_dma_transfer(cpu: &mut Cpu) {
    cpu.pc = 0x41a4;
    become wait_dma_transfer_loop(cpu);
}

pub(crate) fn wait_dma_transfer_loop(cpu: &mut Cpu) {
    cpu.pc = 0x41a4;
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = cpu.read_byte(0xffe8);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.a;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become wait_dma_transfer_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn hdma_transfer_wait_for_scanline128_to_bg_map(cpu: &mut Cpu) {
    cpu.pc = 0x41ad;
    cpu.a = cpu.read_byte({ const N: i32 = (0xffd6 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xffd6);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = { let value = ((2 as i32) * (crate::defs::SCREEN_HEIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become hdma_transfer_wait_for_scanline128(cpu);
}

pub(crate) fn hdma_transfer_wait_for_scanline124_to_bg_map(cpu: &mut Cpu) {
    cpu.pc = 0x41b7;
    cpu.a = cpu.read_byte({ const N: i32 = (0xffd6 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xffd6);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = { let value = ((2 as i32) * (crate::defs::SCREEN_HEIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become hdma_transfer_wait_for_scanline124(cpu);
}

pub(crate) fn hdma_transfer_no_di(cpu: &mut Cpu) {
    cpu.pc = 0x41c1;
    cpu.a = cpu.read_byte({ const N: i32 = (0xffd6 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.read_byte(0xffd6);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = { let value = ((2 as i32) * (crate::defs::SCREEN_HEIGHT as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_SRC_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VDMA_SRC_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x1f;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VDMA_DEST_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VDMA_DEST_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x80;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.b = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become hdma_transfer_no_di_loop1(cpu);
}

pub(crate) fn hdma_transfer_no_di_loop1(cpu: &mut Cpu) {
    cpu.pc = 0x41e4;
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become hdma_transfer_no_di_loop1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become hdma_transfer_no_di_loop2(cpu);
}

pub(crate) fn hdma_transfer_no_di_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x41e9;
    cpu.a = cpu.read_byte(crate::defs::R_STAT as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::STAT_MODE as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become hdma_transfer_no_di_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_LEN as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(crate::defs::R_LY as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become hdma_transfer_no_di_loop3(cpu);
}

pub(crate) fn hdma_transfer_no_di_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x41f8;
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become hdma_transfer_no_di_loop3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become hdma_transfer_no_di_loop3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::R_VDMA_LEN as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn hdma_transfer_wait_for_scanline124(cpu: &mut Cpu) {
    cpu.pc = 0x4205;
    cpu.b = 0x7b;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become _continue_hdma_transfer(cpu);
}

pub(crate) fn hdma_transfer_wait_for_scanline128(cpu: &mut Cpu) {
    cpu.pc = 0x4209;
    cpu.b = 0x7f;
    cpu.pc += 2;
    cpu.cycle(8);
    become _continue_hdma_transfer(cpu);
}

pub(crate) fn _continue_hdma_transfer(cpu: &mut Cpu) {
    cpu.pc = 0x420b;
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_SRC_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VDMA_SRC_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x1f;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VDMA_DEST_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0xf0;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VDMA_DEST_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x80;
    cpu.a |= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = false;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.e = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.c;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.d = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    become _continue_hdma_transfer_ly_loop(cpu);
}

pub(crate) fn _continue_hdma_transfer_ly_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4225;
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.d;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_c {
        cpu.cycle(12);
        become _continue_hdma_transfer_ly_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.disable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    become _continue_hdma_transfer_rstat_loop_1(cpu);
}

pub(crate) fn _continue_hdma_transfer_rstat_loop_1(cpu: &mut Cpu) {
    cpu.pc = 0x422b;
    cpu.a = cpu.read_byte(crate::defs::R_STAT as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::STAT_MODE as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _continue_hdma_transfer_rstat_loop_1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    become _continue_hdma_transfer_rstat_loop_2(cpu);
}

pub(crate) fn _continue_hdma_transfer_rstat_loop_2(cpu: &mut Cpu) {
    cpu.pc = 0x4231;
    cpu.a = cpu.read_byte(crate::defs::R_STAT as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = crate::defs::STAT_MODE as u8;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become _continue_hdma_transfer_rstat_loop_2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_LEN as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(crate::defs::R_LY as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    let value = cpu.c;
    let result = value.wrapping_add(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (value & 0x0f) + 1 > 0x0f;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(crate::defs::R_LY as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    become _continue_hdma_transfer_final_ly_loop(cpu);
}

pub(crate) fn _continue_hdma_transfer_final_ly_loop(cpu: &mut Cpu) {
    cpu.pc = 0x4240;
    let value = cpu.read_byte(cpu.hl());
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 1;
    cpu.cycle(8);
    if cpu.flag_z {
        cpu.cycle(12);
        become _continue_hdma_transfer_final_ly_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become _continue_hdma_transfer_final_ly_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(crate::defs::R_VDMA_LEN as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let mask = 1u8 << ((7) & 7);
    let addr = cpu.hl();
    let mut value = cpu.read_byte(addr);
    value &= !mask;
    cpu.write_byte(addr, value);
    cpu.pc += 2;
    cpu.cycle(16);
    cpu.enable_interrupts_after_next_instruction();
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn _load_hdma_parameters(cpu: &mut Cpu) {
    cpu.pc = 0x424e;
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_SRC_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(crate::defs::R_VDMA_SRC_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte({ const N: i32 = (0xffd6 as i32) + (1 as i32); N as u16 });
    cpu.pc += 2;
    cpu.cycle(12);
    let value = 0x1f;
    cpu.a &= value;
    cpu.flag_z = cpu.a == 0;
    cpu.flag_n = false;
    cpu.flag_h = true;
    cpu.flag_c = false;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VDMA_DEST_HIGH as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.a = cpu.read_byte(0xffd6);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_VDMA_DEST_LOW as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn pad_tilemap_for_hdma_transfer(cpu: &mut Cpu) {
    cpu.pc = 0x425f;
    cpu.c = { let value = (0x7f as i32); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.cycle(12);
    become pad_map_for_hdma_transfer(cpu);
}

pub(crate) fn pad_attrmap_for_hdma_transfer(cpu: &mut Cpu) {
    cpu.pc = 0x4263;
    cpu.c = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    become pad_map_for_hdma_transfer(cpu);
}

pub(crate) fn pad_map_for_hdma_transfer(cpu: &mut Cpu) {
    cpu.pc = 0x4265;
    cpu.a = cpu.read_byte(0xffaf);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.write_byte(0xffaf, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = crate::defs::SCREEN_HEIGHT as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become pad_map_for_hdma_transfer_loop1(cpu);
}

pub(crate) fn pad_map_for_hdma_transfer_loop1(cpu: &mut Cpu) {
    cpu.pc = 0x426d;
    cpu.b = crate::defs::SCREEN_WIDTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become pad_map_for_hdma_transfer_loop2(cpu);
}

pub(crate) fn pad_map_for_hdma_transfer_loop2(cpu: &mut Cpu) {
    cpu.pc = 0x426f;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become pad_map_for_hdma_transfer_loop2(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.read_byte(0xffaf);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.b = { let value = ((crate::defs::TILEMAP_WIDTH as i32) - (crate::defs::SCREEN_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    cpu.pc += 2;
    cpu.cycle(8);
    become pad_map_for_hdma_transfer_loop3(cpu);
}

pub(crate) fn pad_map_for_hdma_transfer_loop3(cpu: &mut Cpu) {
    cpu.pc = 0x4279;
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become pad_map_for_hdma_transfer_loop3(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become pad_map_for_hdma_transfer_loop1(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(0xffaf, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn hdma_transfer2bpp(cpu: &mut Cpu) {
    cpu.pc = 0x4284;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x06;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::far_copy_bytes(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub fn hdma_transfer1bpp(cpu: &mut Cpu) {
    cpu.pc = 0x42b2;
    become hdma_transfer1bpp_loop(cpu);
}

pub(crate) fn hdma_transfer1bpp_loop(cpu: &mut Cpu) {
    cpu.pc = 0x42b2;
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x10;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.pc += 2;
    cpu.cycle(8);
    if cpu.flag_c {
        cpu.cycle(16);
        become hdma_transfer1bpp_bankswitch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    if cpu.flag_z {
        cpu.cycle(16);
        become hdma_transfer1bpp_bankswitch(cpu);
    }
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.de());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.c = 0x10;
    cpu.pc += 2;
    cpu.cycle(8);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer1bpp_bankswitch(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_de(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_hl(0x80 as u16);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.de();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.set_bc(0x0100);
    cpu.pc += 3;
    cpu.cycle(12);
    let lhs = cpu.hl();
    let rhs = cpu.bc();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.a = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0x10;
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_sub(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (lhs & 0x0f) < (value & 0x0f);
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.c = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.cycle(12);
    become hdma_transfer1bpp_loop(cpu);
}

pub(crate) fn hdma_transfer1bpp_bankswitch(cpu: &mut Cpu) {
    cpu.pc = 0x42d6;
    cpu.a = cpu.read_byte(crate::defs::R_WBK as u16);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.af());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = 0x06;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.push_word(cpu.hl());
    cpu.pc += 1;
    cpu.cycle(16);
    cpu.a = cpu.b;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.c;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    let lhs = cpu.hl();
    let rhs = cpu.hl();
    let (result, carry) = lhs.overflowing_add(rhs);
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff;
    cpu.flag_c = carry;
    cpu.set_hl(result);
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.c = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.b = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.h = cpu.d;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.l = cpu.e;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_de(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::gfx::far_copy_bytes_double_double_bank_switch(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_hl(value);
    cpu.pc += 1;
    cpu.cycle(12);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.push_word(cpu.bc());
    cpu.pc += 1;
    cpu.cycle(16);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    crate::game::home::delay::delay_frame(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_bc(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.d = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.e = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128(cpu);
    assert_eq!(cpu.pc, next_pc);
    let value = cpu.pop_word();
    cpu.set_af(value);
    cpu.pc += 1;
    cpu.cycle(12);
    cpu.write_byte(crate::defs::R_WBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn hdma_transfer_only_top_four_rows(cpu: &mut Cpu) {
    cpu.pc = 0x4303;
    cpu.set_hl(0x4309);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.cycle(16);
    become call_in_safe_gfx_mode(cpu);
}

pub(crate) fn hdma_transfer_only_top_four_rows_function(cpu: &mut Cpu) {
    cpu.pc = 0x4309;
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xc4a0 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_only_top_four_rows_copy(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.set_hl({ const N: i32 = (0xd000 as i32) + (0x80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::SCREEN_WIDTH as u16 as i32)) + (0 as i32)) + (0xcdd9 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_only_top_four_rows_copy(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x1;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl({ const N: i32 = (0xd000 as i32) + (0x80 as i32); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::TILEMAP_WIDTH as u16 as i32)) + (0 as i32)) + (0x9c00 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.a = 0x0;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.write_byte(crate::defs::R_VBK as u16, cpu.a);
    cpu.pc += 2;
    cpu.cycle(12);
    cpu.c = 0x8;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.set_hl(0xd000);
    cpu.pc += 3;
    cpu.cycle(12);
    cpu.set_de({ const N: i32 = ((((0 as i32) * (crate::defs::TILEMAP_WIDTH as u16 as i32)) + (0 as i32)) + (0x9c00 as i32)); N as u16 });
    cpu.pc += 3;
    cpu.cycle(12);
    let next_pc = cpu.pc + 3;
    cpu.push_word(next_pc);
    cpu.cycle(24);
    hdma_transfer_wait_for_scanline128(cpu);
    assert_eq!(cpu.pc, next_pc);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn hdma_transfer_only_top_four_rows_copy(cpu: &mut Cpu) {
    cpu.pc = 0x433a;
    cpu.b = 4;
    cpu.pc += 2;
    cpu.cycle(8);
    become hdma_transfer_only_top_four_rows_outer_loop(cpu);
}

pub(crate) fn hdma_transfer_only_top_four_rows_outer_loop(cpu: &mut Cpu) {
    cpu.pc = 0x433c;
    cpu.c = crate::defs::SCREEN_WIDTH as u8;
    cpu.pc += 2;
    cpu.cycle(8);
    become hdma_transfer_only_top_four_rows_inner_loop(cpu);
}

pub(crate) fn hdma_transfer_only_top_four_rows_inner_loop(cpu: &mut Cpu) {
    cpu.pc = 0x433e;
    cpu.a = cpu.read_byte(cpu.de());
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.write_byte(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    cpu.set_de(cpu.de().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycle(8);
    let value = cpu.c;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.c = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become hdma_transfer_only_top_four_rows_inner_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.a = cpu.l;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = { let value = ((crate::defs::TILEMAP_WIDTH as i32) - (crate::defs::SCREEN_WIDTH as i32)); if value < 0 { i8::try_from(value).expect("n8 expression out of range") as u8 } else { u8::try_from(value).expect("n8 expression out of range") } };
    let lhs = cpu.a;
    let (result, carry) = lhs.overflowing_add(value);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) > 0x0f;
    cpu.flag_c = carry;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.l = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    cpu.a = cpu.h;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = 0;
    let carry_in = u8::from(cpu.flag_c);
    let lhs = cpu.a;
    let (partial, carry1) = lhs.overflowing_add(value);
    let (result, carry2) = partial.overflowing_add(carry_in);
    cpu.flag_z = result == 0;
    cpu.flag_n = false;
    cpu.flag_h = (lhs & 0x0f) + (value & 0x0f) + carry_in > 0x0f;
    cpu.flag_c = carry1 || carry2;
    cpu.a = result;
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.h = cpu.a;
    cpu.pc += 1;
    cpu.cycle(4);
    let value = cpu.b;
    let result = value.wrapping_sub(1);
    cpu.flag_z = result == 0;
    cpu.flag_n = true;
    cpu.flag_h = (value & 0x0f) == 0;
    cpu.b = result;
    cpu.pc += 1;
    cpu.cycle(4);
    if !cpu.flag_z {
        cpu.cycle(12);
        become hdma_transfer_only_top_four_rows_outer_loop(cpu);
    }
    cpu.pc += 2;
    cpu.cycle(8);
    cpu.pc = cpu.pop_word();
    cpu.cycle(16);
}

pub(crate) fn call_in_safe_gfx_mode_hl_(cpu: &mut Cpu) {
    match cpu.hl() {
        0x4006 => become hdma_transfer_attrmap_and_tilemap_to_wram_bank3_function(cpu),
        0x4033 => become hdma_transfer_tilemap_to_wram_bank3_function(cpu),
        0x404d => become hdma_transfer_attrmap_to_wram_bank3_function(cpu),
        0x4067 => become hdma_transfer_tilemap_and_attrmap_overworld_function(cpu),
        0x40a2 => become mobile_hdma_transfer_tilemap_and_attrmap_overworld_function(cpu),
        0x40da => become function1040d4_function(cpu),
        0x4101 => become function1040fb_function(cpu),
        0x4116 => become _hdma_transfer_tilemap_and_attrmap_menu_function(cpu),
        0x414e => become mobile_hdma_transfer_tilemap_and_attrmap_menu_function(cpu),
        0x4309 => become hdma_transfer_only_top_four_rows_function(cpu),
        n => panic!("unhandled hl: {n:04x}"),
    }
}
