use super::emit_adrp_add;
use super::loads::{emit_arm64_load_x29_offset, emit_arm64_store_x29_offset};
use crate::ast::BinaryOp;
use crate::codegen::target::OperatingSystem;

pub fn emit_try_begin(out: &mut String, catch_label: &str, os: OperatingSystem) {
    emit_adrp_add(out, "x9", "alya_catch_idx", os);
    out.push_str("    ldr x10, [x9]\n");
    emit_adrp_add(out, "x11", catch_label, os);
    emit_adrp_add(out, "x12", "alya_catch_stack_handler", os);
    out.push_str("    str x11, [x12, x10, lsl #3]\n");
    out.push_str("    mov x13, sp\n");
    emit_adrp_add(out, "x12", "alya_catch_stack_sp", os);
    out.push_str("    str x13, [x12, x10, lsl #3]\n");
    emit_adrp_add(out, "x12", "alya_catch_stack_bp", os);
    out.push_str("    str x29, [x12, x10, lsl #3]\n");
    out.push_str("    add x10, x10, #1\n");
    out.push_str("    str x10, [x9]\n");
}

pub fn emit_try_end(out: &mut String, end_label: &str, stack_delta: i32, os: OperatingSystem) {
    emit_adrp_add(out, "x9", "alya_catch_idx", os);
    out.push_str("    ldr x10, [x9]\n");
    out.push_str("    sub x10, x10, #1\n");
    out.push_str("    str x10, [x9]\n");
    if stack_delta > 0 {
        out.push_str(&format!("    add sp, sp, #{}\n", stack_delta));
    }
    out.push_str(&format!("    b {}\n", end_label));
}

pub fn emit_catch_begin(out: &mut String, catch_label: &str) {
    out.push_str(&format!("{}:\n", catch_label));
}

pub fn emit_catch_load_err(out: &mut String, os: OperatingSystem) {
    emit_adrp_add(out, "x9", "alya_err_msg", os);
    out.push_str("    ldr x0, [x9]\n");
}

pub fn emit_catch_end(out: &mut String, stack_delta: i32) {
    if stack_delta > 0 {
        out.push_str(&format!("    add sp, sp, #{}\n", stack_delta));
    }
}

pub fn emit_array_new(out: &mut String, count: usize) {
    out.push_str(&format!("    mov x0, #{}\n", count));
    out.push_str("    bl alya_array_new\n");
}

pub fn emit_array_set_imm(out: &mut String, index: usize) {
    out.push_str("    ldr x1, [sp]\n");
    out.push_str("    ldr x1, [x1, #16]\n");
    out.push_str(&format!("    mov x2, #{}\n", index * 8));
    out.push_str("    str x0, [x1, x2]\n");
}

pub fn emit_array_get(out: &mut String) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    cmp x1, #0\n");
    out.push_str("    b.lt alya_error_index_out_of_bounds\n");
    out.push_str("    ldr x2, [x0]\n");
    out.push_str("    cmp x1, x2\n");
    out.push_str("    b.ge alya_error_index_out_of_bounds\n");
    out.push_str("    ldr x0, [x0, #16]\n");
    out.push_str("    ldr x0, [x0, x1, lsl #3]\n");
}

pub fn emit_array_set(out: &mut String) {
    out.push_str("    mov x2, x0\n");
    out.push_str("    ldr x1, [sp], #16\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    cmp x1, #0\n");
    out.push_str("    b.lt alya_error_index_out_of_bounds\n");
    out.push_str("    ldr x3, [x0]\n");
    out.push_str("    cmp x1, x3\n");
    out.push_str("    b.ge alya_error_index_out_of_bounds\n");
    out.push_str("    ldr x0, [x0, #16]\n");
    out.push_str("    str x2, [x0, x1, lsl #3]\n");
}

pub fn emit_array_push(out: &mut String) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    bl alya_array_push\n");
}

pub fn emit_array_pop(out: &mut String) {
    out.push_str("    bl alya_array_pop\n");
}

pub fn emit_array_len(out: &mut String) {
    out.push_str("    cbz x0, 1f\n");
    out.push_str("    ldr x0, [x0]\n");
    out.push_str("1:\n");
}

pub fn emit_print_array(out: &mut String) {
    out.push_str("    bl alya_print_array\n");
}

pub fn emit_print_map(out: &mut String) {
    out.push_str("    bl alya_print_map\n");
}

pub fn emit_struct_new(
    out: &mut String,
    desc_label: &str,
    field_count: usize,
    os: OperatingSystem,
) {
    emit_adrp_add(out, "x0", desc_label, os);
    out.push_str(&format!("    mov x1, #{}\n", field_count));
    out.push_str("    bl alya_struct_new\n");
}

pub fn emit_struct_field_get(out: &mut String, field_idx: usize) {
    out.push_str(&format!("    ldr x0, [x0, #{}]\n", (field_idx + 1) * 8));
}

pub fn emit_struct_field_set_imm(out: &mut String, field_idx: usize) {
    out.push_str("    ldr x1, [sp]\n");
    out.push_str(&format!("    str x0, [x1, #{}]\n", (field_idx + 1) * 8));
}

pub fn emit_struct_field_set(out: &mut String, field_idx: usize) {
    out.push_str("    ldr x1, [sp], #16\n");
    out.push_str(&format!("    str x0, [x1, #{}]\n", (field_idx + 1) * 8));
}

pub fn emit_print_struct(out: &mut String) {
    out.push_str("    bl alya_print_struct\n");
}

pub fn emit_for_each_load_element(
    out: &mut String,
    arr_offset: i32,
    idx_offset: i32,
    var_offset: i32,
    end_label: &str,
) {
    emit_arm64_load_x29_offset(out, "x0", arr_offset, "x9");
    out.push_str(&format!("    cbz x0, {}\n", end_label));
    out.push_str("    ldr x1, [x0]\n");
    emit_arm64_load_x29_offset(out, "x2", idx_offset, "x9");
    out.push_str("    cmp x2, x1\n");
    out.push_str(&format!("    b.ge {}\n", end_label));
    out.push_str("    ldr x3, [x0, #16]\n");
    out.push_str("    ldr x0, [x3, x2, lsl #3]\n");
    emit_arm64_store_x29_offset(out, "x0", var_offset, "x9");
}

pub fn emit_string_equality_call(out: &mut String, op: BinaryOp) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x0, [sp], #16\n");
    out.push_str("    bl fn_streq\n");
    if matches!(op, BinaryOp::NotEqual) {
        out.push_str("    eor x0, x0, #1\n");
    }
}

pub fn emit_char_code_at(out: &mut String, done_label: &str) {
    out.push_str("    mov x1, x0\n");
    out.push_str("    ldr x2, [sp], #16\n");
    out.push_str("    mov x0, #0\n");
    out.push_str(&format!("    cbz x2, {}\n", done_label));
    out.push_str(&format!("    tbnz x1, #63, {}\n", done_label));
    out.push_str("    ldrb w0, [x2, x1]\n");
    out.push_str(&format!("{}:\n", done_label));
}

