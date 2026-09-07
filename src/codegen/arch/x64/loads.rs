pub fn emit_load_num(out: &mut String, val: i64) {
    out.push_str(&format!("    mov ${}, %rax\n", val));
}

pub fn emit_load_float(out: &mut String, val: f64) {
    let bits = val.to_bits() as i64;
    out.push_str(&format!("    mov ${}, %rax\n", bits));
    out.push_str("    movq %rax, %xmm0\n");
}

pub fn emit_int_to_float(out: &mut String) {
    out.push_str("    cvtsi2sdq %rax, %xmm0\n");
    out.push_str("    movq %xmm0, %rax\n");
}

pub fn emit_float_to_int(out: &mut String) {
    out.push_str("    movq %rax, %xmm0\n");
    out.push_str("    cvttsd2siq %xmm0, %rax\n");
}

pub fn emit_load_str_label(out: &mut String, label: &str) {
    out.push_str(&format!("    lea {}(%rip), %rax\n", label));
}

pub fn emit_load_var(out: &mut String, offset: i32) {
    out.push_str(&format!("    mov -{}(%rbp), %rax\n", offset));
}

pub fn emit_store_var(out: &mut String, offset: i32) {
    out.push_str(&format!("    mov %rax, -{}(%rbp)\n", offset));
}

pub fn emit_allocate_var(out: &mut String, stack_offset: &mut i32) {
    *stack_offset += 8;
    out.push_str("    push %rax\n");
}

pub fn emit_push_temp(out: &mut String) {
    out.push_str("    push %rax\n");
}

pub fn emit_pop_temp(out: &mut String) {
    out.push_str("    pop %rax\n");
}
