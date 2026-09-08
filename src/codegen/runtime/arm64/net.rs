use crate::codegen::target::OperatingSystem;
use super::emit_adrp_add;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_mac = matches!(os, OperatingSystem::MacOS);
    let p = if is_mac { "_" } else { "" };
    let _ = (is_mac, p);

    // fn_net_socket: creates a TCP socket (2, 1, 0)
    out.push_str(".align 2\n");
    out.push_str(".global fn_net_socket\n");
    out.push_str("fn_net_socket:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    mov x0, #2\n");
    out.push_str("    mov x1, #1\n");
    out.push_str("    mov x2, #0\n");
    out.push_str(&format!("    bl {}socket\n", p));
    out.push_str("    cmp x0, #0\n");
    out.push_str("    bge .L_arm64_socket_ok\n");
    out.push_str("    mvn x0, xzr\n"); // -1
    out.push_str(".L_arm64_socket_ok:\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_net_connect: connect(host, port) -> socket or -1
    out.push_str(".align 2\n");
    out.push_str(".global fn_net_connect\n");
    out.push_str("fn_net_connect:\n");
    out.push_str("    stp x29, x30, [sp, #-80]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    stp x21, x22, [sp, #32]\n");
    out.push_str("    mov x19, x0\n"); // host
    out.push_str("    mov x20, x1\n"); // port
    out.push_str("    cbz x19, .L_arm64_conn_fail\n");

    // Clear sockaddr_in at [sp, #48] (16 bytes)
    out.push_str("    str xzr, [sp, #48]\n");
    out.push_str("    str xzr, [sp, #56]\n");
    if is_mac {
        out.push_str("    mov w2, #0x0210\n");
        out.push_str("    strh w2, [sp, #48]\n");
    } else {
        out.push_str("    mov w2, #2\n");
        out.push_str("    strh w2, [sp, #48]\n");
    }
    // htons(port)
    out.push_str("    rev16 w2, w20\n");
    out.push_str("    strh w2, [sp, #50]\n");

    // inet_addr(host)
    out.push_str("    mov x0, x19\n");
    out.push_str(&format!("    bl {}inet_addr\n", p));
    out.push_str("    cmn w0, #1\n");
    out.push_str("    bne .L_arm64_conn_have_ip\n");

    // gethostbyname(host)
    out.push_str("    mov x0, x19\n");
    out.push_str(&format!("    bl {}gethostbyname\n", p));
    out.push_str("    cbz x0, .L_arm64_conn_fail\n");
    out.push_str("    ldr x0, [x0, #24]\n"); // h_addr_list
    out.push_str("    cbz x0, .L_arm64_conn_fail\n");
    out.push_str("    ldr x0, [x0]\n");
    out.push_str("    cbz x0, .L_arm64_conn_fail\n");
    out.push_str("    ldr w0, [x0]\n");
    out.push_str(".L_arm64_conn_have_ip:\n");
    out.push_str("    str w0, [sp, #52]\n"); // sin_addr

    // socket(2, 1, 0)
    out.push_str("    mov x0, #2\n");
    out.push_str("    mov x1, #1\n");
    out.push_str("    mov x2, #0\n");
    out.push_str(&format!("    bl {}socket\n", p));
    out.push_str("    cmp x0, #0\n");
    out.push_str("    blt .L_arm64_conn_fail\n");
    out.push_str("    mov x21, x0\n"); // socket

    // connect(sock, &sin, 16)
    out.push_str("    mov x0, x21\n");
    out.push_str("    add x1, sp, #48\n");
    out.push_str("    mov x2, #16\n");
    out.push_str(&format!("    bl {}connect\n", p));
    out.push_str("    cmp w0, #0\n");
    out.push_str("    blt .L_arm64_conn_close\n");
    out.push_str("    mov x0, x21\n");
    out.push_str("    b .L_arm64_conn_ret\n");
    out.push_str(".L_arm64_conn_close:\n");
    out.push_str("    mov x0, x21\n");
    out.push_str(&format!("    bl {}close\n", p));
    out.push_str(".L_arm64_conn_fail:\n");
    out.push_str("    mvn x0, xzr\n");
    out.push_str(".L_arm64_conn_ret:\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x21, x22, [sp, #32]\n");
    out.push_str("    ldp x29, x30, [sp], #80\n");
    out.push_str("    ret\n\n");

    // fn_net_listen: net_listen(port, backlog) -> socket or -1
    out.push_str(".align 2\n");
    out.push_str(".global fn_net_listen\n");
    out.push_str("fn_net_listen:\n");
    out.push_str("    stp x29, x30, [sp, #-80]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    stp x21, x22, [sp, #32]\n");
    out.push_str("    mov x19, x0\n"); // port
    out.push_str("    mov x20, x1\n"); // backlog
    out.push_str("    cmp x20, #0\n");
    out.push_str("    bgt .L_arm64_listen_bl_ok\n");
    out.push_str("    mov x20, #10\n");
    out.push_str(".L_arm64_listen_bl_ok:\n");

    // socket(2, 1, 0)
    out.push_str("    mov x0, #2\n");
    out.push_str("    mov x1, #1\n");
    out.push_str("    mov x2, #0\n");
    out.push_str(&format!("    bl {}socket\n", p));
    out.push_str("    cmp x0, #0\n");
    out.push_str("    blt .L_arm64_listen_fail\n");
    out.push_str("    mov x21, x0\n"); // socket

    // sockaddr_in at [sp, #48]
    out.push_str("    str xzr, [sp, #48]\n");
    out.push_str("    str xzr, [sp, #56]\n");
    if is_mac {
        out.push_str("    mov w2, #0x0210\n");
        out.push_str("    strh w2, [sp, #48]\n");
    } else {
        out.push_str("    mov w2, #2\n");
        out.push_str("    strh w2, [sp, #48]\n");
    }
    out.push_str("    rev16 w2, w19\n");
    out.push_str("    strh w2, [sp, #50]\n");
    out.push_str("    str wzr, [sp, #52]\n"); // INADDR_ANY

    // bind(sock, &sin, 16)
    out.push_str("    mov x0, x21\n");
    out.push_str("    add x1, sp, #48\n");
    out.push_str("    mov x2, #16\n");
    out.push_str(&format!("    bl {}bind\n", p));
    out.push_str("    cmp w0, #0\n");
    out.push_str("    blt .L_arm64_listen_close\n");

    // listen(sock, backlog)
    out.push_str("    mov x0, x21\n");
    out.push_str("    mov x1, x20\n");
    out.push_str(&format!("    bl {}listen\n", p));
    out.push_str("    cmp w0, #0\n");
    out.push_str("    blt .L_arm64_listen_close\n");
    out.push_str("    mov x0, x21\n");
    out.push_str("    b .L_arm64_listen_ret\n");
    out.push_str(".L_arm64_listen_close:\n");
    out.push_str("    mov x0, x21\n");
    out.push_str(&format!("    bl {}close\n", p));
    out.push_str(".L_arm64_listen_fail:\n");
    out.push_str("    mvn x0, xzr\n");
    out.push_str(".L_arm64_listen_ret:\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x21, x22, [sp, #32]\n");
    out.push_str("    ldp x29, x30, [sp], #80\n");
    out.push_str("    ret\n\n");

    // fn_net_accept: net_accept(server_sock) -> client_sock or -1
    out.push_str(".align 2\n");
    out.push_str(".global fn_net_accept\n");
    out.push_str("fn_net_accept:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    mov x1, #0\n");
    out.push_str("    mov x2, #0\n");
    out.push_str(&format!("    bl {}accept\n", p));
    out.push_str("    cmp x0, #0\n");
    out.push_str("    bge .L_arm64_accept_ok\n");
    out.push_str("    mvn x0, xzr\n");
    out.push_str(".L_arm64_accept_ok:\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");

    // fn_net_send: net_send(sock, data_str) -> bytes_sent or -1
    out.push_str(".align 2\n");
    out.push_str(".global fn_net_send\n");
    out.push_str("fn_net_send:\n");
    out.push_str("    stp x29, x30, [sp, #-48]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    stp x21, x22, [sp, #32]\n");
    out.push_str("    mov x19, x0\n"); // sock
    out.push_str("    mov x20, x1\n"); // str
    out.push_str("    cbz x20, .L_arm64_send_zero\n");
    // strlen
    out.push_str("    mov x21, x20\n");
    out.push_str("    mov x22, #0\n");
    out.push_str(".L_arm64_send_len:\n");
    out.push_str("    ldrb w2, [x21]\n");
    out.push_str("    cbz w2, .L_arm64_send_do\n");
    out.push_str("    add x21, x21, #1\n");
    out.push_str("    add x22, x22, #1\n");
    out.push_str("    b .L_arm64_send_len\n");
    out.push_str(".L_arm64_send_do:\n");
    out.push_str("    cbz x22, .L_arm64_send_zero\n");
    out.push_str("    mov x0, x19\n");
    out.push_str("    mov x1, x20\n");
    out.push_str("    mov x2, x22\n");
    out.push_str("    mov x3, #0\n");
    out.push_str(&format!("    bl {}send\n", p));
    out.push_str("    b .L_arm64_send_ret\n");
    out.push_str(".L_arm64_send_zero:\n");
    out.push_str("    mov x0, #0\n");
    out.push_str(".L_arm64_send_ret:\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x21, x22, [sp, #32]\n");
    out.push_str("    ldp x29, x30, [sp], #48\n");
    out.push_str("    ret\n\n");

    // fn_net_recv: net_recv(sock, max_bytes) -> string
    out.push_str(".align 2\n");
    out.push_str(".global fn_net_recv\n");
    out.push_str("fn_net_recv:\n");
    out.push_str("    stp x29, x30, [sp, #-48]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str("    stp x19, x20, [sp, #16]\n");
    out.push_str("    stp x21, x22, [sp, #32]\n");
    out.push_str("    mov x19, x0\n"); // sock
    out.push_str("    mov x20, x1\n"); // max_bytes
    out.push_str("    cmp x20, #0\n");
    out.push_str("    bgt .L_arm64_recv_chk\n");
    out.push_str("    mov x20, #4096\n");
    out.push_str(".L_arm64_recv_chk:\n");
    out.push_str("    mov x2, #524288\n");
    out.push_str("    cmp x20, x2\n");
    out.push_str("    csel x20, x2, x20, gt\n");

    emit_adrp_add(out, "x2", "alya_str_idx", os);
    out.push_str("    ldr x3, [x2]\n");
    out.push_str("    mov x4, #1000000\n");
    out.push_str("    sub x4, x4, x20\n");
    out.push_str("    cmp x3, x4\n");
    out.push_str("    csel x3, xzr, x3, gt\n");
    emit_adrp_add(out, "x5", "alya_str_buf", os);
    out.push_str("    add x21, x5, x3\n"); // x21 = buffer
    out.push_str("    mov x22, x3\n");      // x22 = start idx

    out.push_str("    mov x0, x19\n");
    out.push_str("    mov x1, x21\n");
    out.push_str("    mov x2, x20\n");
    out.push_str("    mov x3, #0\n");
    out.push_str(&format!("    bl {}recv\n", p));
    out.push_str("    cmp x0, #0\n");
    out.push_str("    ble .L_arm64_recv_empty\n");
    out.push_str("    strb wzr, [x21, x0]\n");
    emit_adrp_add(out, "x2", "alya_str_idx", os);
    out.push_str("    add x1, x22, x0\n");
    out.push_str("    add x1, x1, #8\n");
    out.push_str("    and x1, x1, #-8\n");
    out.push_str("    str x1, [x2]\n");
    out.push_str("    mov x0, x21\n");
    out.push_str("    b .L_arm64_recv_done\n");
    out.push_str(".L_arm64_recv_empty:\n");
    emit_adrp_add(out, "x0", "alya_str_empty", os);
    out.push_str(".L_arm64_recv_done:\n");
    out.push_str("    ldp x19, x20, [sp, #16]\n");
    out.push_str("    ldp x21, x22, [sp, #32]\n");
    out.push_str("    ldp x29, x30, [sp], #48\n");
    out.push_str("    ret\n\n");

    // fn_net_close: net_close(sock) -> 0
    out.push_str(".align 2\n");
    out.push_str(".global fn_net_close\n");
    out.push_str("fn_net_close:\n");
    out.push_str("    stp x29, x30, [sp, #-16]!\n");
    out.push_str("    mov x29, sp\n");
    out.push_str(&format!("    bl {}close\n", p));
    out.push_str("    mov x0, #0\n");
    out.push_str("    ldp x29, x30, [sp], #16\n");
    out.push_str("    ret\n\n");
}
