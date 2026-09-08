use crate::codegen::target::OperatingSystem;

#[rustfmt::skip]
pub fn emit(out: &mut String, os: OperatingSystem) {
    let is_win = matches!(os, OperatingSystem::Windows);
    let close_fn = if is_win { "closesocket" } else { "close" };
    let _ = (is_win, close_fn);

    // fn_net_socket: TCP socket (2, 1, 0)
    out.push_str(".global fn_net_socket\n");
    out.push_str("fn_net_socket:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push $0\n");
    out.push_str("    push $1\n");
    out.push_str("    push $2\n");
    out.push_str("    call socket\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jge .L_x86_socket_ok\n");
    out.push_str("    mov $-1, %eax\n");
    out.push_str(".L_x86_socket_ok:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_net_connect: connect(host, port) -> socket or -1
    out.push_str(".global fn_net_connect\n");
    out.push_str("fn_net_connect:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ebx\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %edi\n");
    out.push_str("    sub $32, %esp\n");
    out.push_str("    mov 8(%ebp), %esi\n");  // host
    out.push_str("    mov 12(%ebp), %ebx\n"); // port
    out.push_str("    test %esi, %esi\n");
    out.push_str("    jz .L_x86_conn_fail\n");

    // sockaddr_in at -44(%ebp)
    out.push_str("    movl $0, -44(%ebp)\n");
    out.push_str("    movl $0, -40(%ebp)\n");
    out.push_str("    movl $0, -36(%ebp)\n");
    out.push_str("    movl $0, -32(%ebp)\n");
    out.push_str("    movw $2, -44(%ebp)\n"); // AF_INET
    out.push_str("    mov %bx, %ax\n");
    out.push_str("    xchg %al, %ah\n");
    out.push_str("    mov %ax, -42(%ebp)\n"); // sin_port

    // inet_addr(host)
    out.push_str("    push %esi\n");
    out.push_str("    call inet_addr\n");
    out.push_str("    add $4, %esp\n");
    out.push_str("    cmp $0xffffffff, %eax\n");
    out.push_str("    jne .L_x86_conn_have_ip\n");

    // gethostbyname(host)
    out.push_str("    push %esi\n");
    out.push_str("    call gethostbyname\n");
    out.push_str("    add $4, %esp\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jz .L_x86_conn_fail\n");
    out.push_str("    mov 16(%eax), %eax\n"); // h_addr_list (offset 16 on 32-bit)
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jz .L_x86_conn_fail\n");
    out.push_str("    mov (%eax), %eax\n");
    out.push_str("    test %eax, %eax\n");
    out.push_str("    jz .L_x86_conn_fail\n");
    out.push_str("    movl (%eax), %eax\n");
    out.push_str(".L_x86_conn_have_ip:\n");
    out.push_str("    movl %eax, -40(%ebp)\n");

    // socket(2, 1, 0)
    out.push_str("    push $0\n");
    out.push_str("    push $1\n");
    out.push_str("    push $2\n");
    out.push_str("    call socket\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jl .L_x86_conn_fail\n");
    out.push_str("    mov %eax, %ebx\n");

    // connect(sock, &sin, 16)
    out.push_str("    push $16\n");
    out.push_str("    lea -44(%ebp), %eax\n");
    out.push_str("    push %eax\n");
    out.push_str("    push %ebx\n");
    out.push_str("    call connect\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jl .L_x86_conn_close\n");
    out.push_str("    mov %ebx, %eax\n");
    out.push_str("    jmp .L_x86_conn_ret\n");
    out.push_str(".L_x86_conn_close:\n");
    out.push_str("    push %ebx\n");
    out.push_str(&format!("    call {}\n", close_fn));
    out.push_str("    add $4, %esp\n");
    out.push_str(".L_x86_conn_fail:\n");
    out.push_str("    mov $-1, %eax\n");
    out.push_str(".L_x86_conn_ret:\n");
    out.push_str("    add $32, %esp\n");
    out.push_str("    pop %edi\n");
    out.push_str("    pop %esi\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_net_listen: net_listen(port, backlog) -> socket or -1
    out.push_str(".global fn_net_listen\n");
    out.push_str("fn_net_listen:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ebx\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %edi\n");
    out.push_str("    sub $32, %esp\n");
    out.push_str("    mov 8(%ebp), %esi\n");  // port
    out.push_str("    mov 12(%ebp), %edi\n"); // backlog
    out.push_str("    cmp $0, %edi\n");
    out.push_str("    jg .L_x86_listen_bl_ok\n");
    out.push_str("    mov $10, %edi\n");
    out.push_str(".L_x86_listen_bl_ok:\n");

    // socket(2, 1, 0)
    out.push_str("    push $0\n");
    out.push_str("    push $1\n");
    out.push_str("    push $2\n");
    out.push_str("    call socket\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jl .L_x86_listen_fail\n");
    out.push_str("    mov %eax, %ebx\n");

    // sockaddr_in
    out.push_str("    movl $0, -44(%ebp)\n");
    out.push_str("    movl $0, -40(%ebp)\n");
    out.push_str("    movl $0, -36(%ebp)\n");
    out.push_str("    movl $0, -32(%ebp)\n");
    out.push_str("    movw $2, -44(%ebp)\n");
    out.push_str("    mov %si, %ax\n");
    out.push_str("    xchg %al, %ah\n");
    out.push_str("    mov %ax, -42(%ebp)\n");
    out.push_str("    movl $0, -40(%ebp)\n");

    // bind(sock, &sin, 16)
    out.push_str("    push $16\n");
    out.push_str("    lea -44(%ebp), %eax\n");
    out.push_str("    push %eax\n");
    out.push_str("    push %ebx\n");
    out.push_str("    call bind\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jl .L_x86_listen_close\n");

    // listen(sock, backlog)
    out.push_str("    push %edi\n");
    out.push_str("    push %ebx\n");
    out.push_str("    call listen\n");
    out.push_str("    add $8, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jl .L_x86_listen_close\n");
    out.push_str("    mov %ebx, %eax\n");
    out.push_str("    jmp .L_x86_listen_ret\n");
    out.push_str(".L_x86_listen_close:\n");
    out.push_str("    push %ebx\n");
    out.push_str(&format!("    call {}\n", close_fn));
    out.push_str("    add $4, %esp\n");
    out.push_str(".L_x86_listen_fail:\n");
    out.push_str("    mov $-1, %eax\n");
    out.push_str(".L_x86_listen_ret:\n");
    out.push_str("    add $32, %esp\n");
    out.push_str("    pop %edi\n");
    out.push_str("    pop %esi\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_net_accept: net_accept(server_sock) -> client_sock or -1
    out.push_str(".global fn_net_accept\n");
    out.push_str("fn_net_accept:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push $0\n");
    out.push_str("    push $0\n");
    out.push_str("    push 8(%ebp)\n");
    out.push_str("    call accept\n");
    out.push_str("    add $12, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jge .L_x86_accept_ok\n");
    out.push_str("    mov $-1, %eax\n");
    out.push_str(".L_x86_accept_ok:\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_net_send: net_send(sock, data_str) -> bytes_sent or -1
    out.push_str(".global fn_net_send\n");
    out.push_str("fn_net_send:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ebx\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %edi\n");
    out.push_str("    mov 8(%ebp), %ebx\n");  // sock
    out.push_str("    mov 12(%ebp), %esi\n"); // str
    out.push_str("    test %esi, %esi\n");
    out.push_str("    jz .L_x86_send_zero\n");
    // strlen
    out.push_str("    mov %esi, %edi\n");
    out.push_str("    xor %ecx, %ecx\n");
    out.push_str(".L_x86_send_len:\n");
    out.push_str("    cmpb $0, (%edi)\n");
    out.push_str("    je .L_x86_send_do\n");
    out.push_str("    inc %edi\n");
    out.push_str("    inc %ecx\n");
    out.push_str("    jmp .L_x86_send_len\n");
    out.push_str(".L_x86_send_do:\n");
    out.push_str("    test %ecx, %ecx\n");
    out.push_str("    jz .L_x86_send_zero\n");
    out.push_str("    push $0\n");
    out.push_str("    push %ecx\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %ebx\n");
    out.push_str("    call send\n");
    out.push_str("    add $16, %esp\n");
    out.push_str("    jmp .L_x86_send_ret\n");
    out.push_str(".L_x86_send_zero:\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str(".L_x86_send_ret:\n");
    out.push_str("    pop %edi\n");
    out.push_str("    pop %esi\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_net_recv: net_recv(sock, max_bytes) -> string
    out.push_str(".global fn_net_recv\n");
    out.push_str("fn_net_recv:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push %ebx\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %edi\n");
    out.push_str("    mov 8(%ebp), %ebx\n");  // sock
    out.push_str("    mov 12(%ebp), %esi\n"); // max_bytes
    out.push_str("    cmp $0, %esi\n");
    out.push_str("    jg .L_x86_recv_chk\n");
    out.push_str("    mov $4096, %esi\n");
    out.push_str(".L_x86_recv_chk:\n");
    out.push_str("    cmp $524288, %esi\n");
    out.push_str("    jle .L_x86_recv_alloc\n");
    out.push_str("    mov $524288, %esi\n");
    out.push_str(".L_x86_recv_alloc:\n");
    out.push_str("    mov alya_str_idx, %edi\n");
    out.push_str("    mov $1000000, %ecx\n");
    out.push_str("    sub %esi, %ecx\n");
    out.push_str("    cmp %ecx, %edi\n");
    out.push_str("    jl .L_x86_recv_buf_ok\n");
    out.push_str("    xor %edi, %edi\n");
    out.push_str(".L_x86_recv_buf_ok:\n");
    out.push_str("    lea alya_str_buf, %edx\n");
    out.push_str("    add %edi, %edx\n");
    out.push_str("    push $0\n");
    out.push_str("    push %esi\n");
    out.push_str("    push %edx\n");
    out.push_str("    push %ebx\n");
    out.push_str("    call recv\n");
    out.push_str("    add $16, %esp\n");
    out.push_str("    cmp $0, %eax\n");
    out.push_str("    jle .L_x86_recv_empty\n");
    out.push_str("    lea alya_str_buf, %edx\n");
    out.push_str("    add %edi, %edx\n");
    out.push_str("    movb $0, (%edx, %eax)\n");
    out.push_str("    lea 1(%eax, %edi), %ecx\n");
    out.push_str("    add $7, %ecx\n");
    out.push_str("    and $-8, %ecx\n");
    out.push_str("    mov %ecx, alya_str_idx\n");
    out.push_str("    mov %edx, %eax\n");
    out.push_str("    jmp .L_x86_recv_done\n");
    out.push_str(".L_x86_recv_empty:\n");
    out.push_str("    mov $alya_str_empty, %eax\n");
    out.push_str(".L_x86_recv_done:\n");
    out.push_str("    pop %edi\n");
    out.push_str("    pop %esi\n");
    out.push_str("    pop %ebx\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");

    // fn_net_close: net_close(sock) -> 0
    out.push_str(".global fn_net_close\n");
    out.push_str("fn_net_close:\n");
    out.push_str("    push %ebp\n");
    out.push_str("    mov %esp, %ebp\n");
    out.push_str("    push 8(%ebp)\n");
    out.push_str(&format!("    call {}\n", close_fn));
    out.push_str("    add $4, %esp\n");
    out.push_str("    xor %eax, %eax\n");
    out.push_str("    mov %ebp, %esp\n");
    out.push_str("    pop %ebp\n");
    out.push_str("    ret\n\n");
}
