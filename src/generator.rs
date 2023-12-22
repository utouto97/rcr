use crate::parser::{Node, NodeType};

pub fn generate(node: Box<Node>) {
    match node.value {
        NodeType::NUM(n) => {
            generate_li("t0".to_string(), n);
            generate_push("t0".to_string());
            return;
        }
        _ => {}
    }

    for child in node.children {
        generate(child);
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
        _ => {}
    }
}

fn generate_push(register: String) {
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
