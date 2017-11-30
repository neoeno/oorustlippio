#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Line { name: String, message: String },
    Dialogue(Vec<ASTNode>)
}

fn main() {
    println!("Hello, world!");
    let result = convert(String::from("Kay: Hello"));

        println!("result: {:?}", result);
}

fn convert(input: String) -> ASTNode {
    let mut iter = input.chars();

    let name: String = iter.by_ref()
                           .take_while(|ch| *ch != ':')
                           .collect();

    let message: String = iter.collect();
    let trimmed_message: String = String::from(message.trim());

    return ASTNode::Dialogue(vec![
        ASTNode::Line { name: name, message: trimmed_message },
    ]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_one_line_into_our_magic_json() {
        let result = convert(String::from("Kay: Hello"));

        println!("result: {:?}", result);

        let line = ASTNode::Line {
            name: String::from("Kay"),
            message: String::from("Hello"),
        };
        let lines = vec![line];
        let dialogue = ASTNode::Dialogue(lines);

        assert!(result == dialogue);
    }
}
