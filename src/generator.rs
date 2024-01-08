use crate::parser::{Node, NodeType};
use std::process;

pub fn generate(node: Box<Node>, name: String) {
    // eprintln!("{:?}", node);
    match node.value {
        NodeType::NUM(n) => {
            generate_li("t0".to_string(), n);
            generate_push("t0".to_string());
            return;
        }
        NodeType::LVAR(_, _) => {
            generate_lvar(node);
            generate_pop("t0".to_string());
            println!("  lw t1, 0(t0)");
            generate_push("t1".to_string());
            return;
        }
        NodeType::ASSIGN => {
            generate_lvar(node.children[0].clone());
            generate(node.children[1].clone(), format!("{}_1", name));

            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  sw t0, 0(t1)");
            generate_push("t0".to_string());
            return;
        }
        _ => {}
    }

    for i in 0..node.children.len() {
        generate(node.children[i].clone(), format!("{}_{}", name, i));
    }

    match node.value {
        NodeType::ADD => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  add t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::SUB => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  sub t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::MUL => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  mul t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::DIV => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  div t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::LT => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  slt t2, t1, t0");
            generate_push("t2".to_string());
        }
        NodeType::LTE => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  slt t2, t0, t1");
            println!("  beq t2, zero, L_SET_{}", name);
            println!("  addi t2, zero, 0");
            println!("  j L_FIN_{}", name);
            println!("L_SET_{}:", name);
            println!("  addi t2, zero, 1");
            println!("L_FIN_{}:", name);
            generate_push("t2".to_string());
        }
        NodeType::EQ => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  beq t0, t1, L_SET_{}", name);
            println!("  addi t2, zero, 0");
            println!("  j L_FIN_{}", name);
            println!("L_SET_{}:", name);
            println!("  addi t2, zero, 1");
            println!("L_FIN_{}:", name);
            generate_push("t2".to_string());
        }
        NodeType::NEQ => {
            generate_pop("t0".to_string());
            generate_pop("t1".to_string());
            println!("  bne t0, t1, L_SET_{}", name);
            println!("  addi t2, zero, 0");
            println!("  j L_FIN_{}", name);
            println!("L_SET_{}:", name);
            println!("  addi t2, zero, 1");
            println!("L_FIN_{}:", name);
            generate_push("t2".to_string());
        }
        NodeType::RETURN => {
            generate_pop("a0".to_string());
            println!("  j L_END");
        }
        _ => {}
    }
}

pub fn generate_push(register: String) {
    println!("  addi sp, sp, -4");
    println!("  sw {}, 0(sp)", register);
}

pub fn generate_pop(register: String) {
    println!("  lw {}, 0(sp)", register);
    println!("  addi sp, sp, 4");
}

fn generate_li(register: String, n: i64) {
    println!("  addi {}, zero, {}", register, n);
}

fn generate_lvar(node: Box<Node>) {
    match node.value {
        NodeType::LVAR(_, offset) => {
            println!("  addi t0, fp, {}", offset);
            generate_push("t0".to_string());
        }
        _ => {
            eprintln!("代入の左辺値が変数ではありません");
            process::exit(1);
        }
    }
}
