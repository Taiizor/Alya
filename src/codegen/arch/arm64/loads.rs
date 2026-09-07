use super::emit_adrp_add;
use crate::codegen::target::OperatingSystem;

pub fn emit_load_reg_u64(out: &mut String, reg: &str, u: u64) {
    if u <= 65535 {
        out.push_str(&format!("    mov {}, #{}\n", reg, u));
        return;
    }
    let chunks = [
        (u & 0xFFFF) as u16,
        ((u >> 16) & 0xFFFF) as u16,
        ((u >> 32) & 0xFFFF) as u16,
        ((u >> 48) & 0xFFFF) as u16,
    ];

    let first_idx = chunks.iter().position(|&c| c != 0).unwrap_or(0);
    let shift_str = if first_idx == 0 {
        String::new()
    } else {
        format!(", lsl #{}", first_idx * 16)
    };
    out.push_str(&format!(
        "    movz {}, #{}{}\n",
        reg, chunks[first_idx], shift_str
    ));
    for (i, &chunk) in chunks.iter().enumerate().skip(first_idx + 1) {
        if chunk != 0 {
            out.push_str(&format!("    movk {}, #{}, lsl #{}\n", reg, chunk, i * 16));
        }
    }
}

pub fn emit_load_reg_imm64(out: &mut String, reg: &str, val: i64) {
    emit_load_reg_u64(out, reg, val as u64);
}

pub fn emit_load_num(out: &mut String, val: i64) {
    emit_load_reg_imm64(out, "x0", val);
}

pub fn emit_load_float(out: &mut String, val: f64) {
    let bits = val.to_bits();
    emit_load_reg_u64(out, "x0", bits);
    out.push_str("    fmov d0, x0\n");
}

pub fn emit_int_to_float(out: &mut String) {
    out.push_str("    scvtf d0, x0\n");
    out.push_str("    fmov x0, d0\n");
}

pub fn emit_float_to_int(out: &mut String) {
    out.push_str("    fmov d0, x0\n");
    out.push_str("    fcvtzs x0, d0\n");
}

pub fn emit_load_str_label(out: &mut String, label: &str, os: OperatingSystem) {
    emit_adrp_add(out, "x0", label, os);
}

pub fn emit_arm64_load_x29_offset(
    out: &mut String,
    dest_reg: &str,
    offset: i32,
    scratch_reg: &str,
) {
    if (0..=256).contains(&offset) {
        out.push_str(&format!("    ldr {}, [x29, #-{}]\n", dest_reg, offset));
    } else if (0..=4095).contains(&offset) {
        out.push_str(&format!("    sub {}, x29, #{}\n", scratch_reg, offset));
        out.push_str(&format!("    ldr {}, [{}]\n", dest_reg, scratch_reg));
    } else {
        emit_load_reg_imm64(out, scratch_reg, offset as i64);
        out.push_str(&format!("    sub {}, x29, {}\n", scratch_reg, scratch_reg));
        out.push_str(&format!("    ldr {}, [{}]\n", dest_reg, scratch_reg));
    }
}

pub fn emit_arm64_store_x29_offset(
    out: &mut String,
    src_reg: &str,
    offset: i32,
    scratch_reg: &str,
) {
    if (0..=256).contains(&offset) {
        out.push_str(&format!("    str {}, [x29, #-{}]\n", src_reg, offset));
    } else if (0..=4095).contains(&offset) {
        out.push_str(&format!("    sub {}, x29, #{}\n", scratch_reg, offset));
        out.push_str(&format!("    str {}, [{}]\n", src_reg, scratch_reg));
    } else {
        emit_load_reg_imm64(out, scratch_reg, offset as i64);
        out.push_str(&format!("    sub {}, x29, {}\n", scratch_reg, scratch_reg));
        out.push_str(&format!("    str {}, [{}]\n", src_reg, scratch_reg));
    }
}

pub fn emit_load_var(out: &mut String, offset: i32, _stack_offset: i32) {
    emit_arm64_load_x29_offset(out, "x0", offset, "x9");
}

pub fn emit_store_var(out: &mut String, offset: i32, _stack_offset: i32) {
    emit_arm64_store_x29_offset(out, "x0", offset, "x9");
}

pub fn emit_allocate_var(out: &mut String, stack_offset: &mut i32) {
    *stack_offset += 16;
    out.push_str("    str x0, [sp, #-16]!\n");
}

pub fn emit_push_temp(out: &mut String) {
    out.push_str("    str x0, [sp, #-16]!\n");
}

pub fn emit_pop_temp(out: &mut String) {
    out.push_str("    ldr x0, [sp], #16\n");
}
