#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Line { name: String, message: String },
    Dialogue(Vec<ASTNode>),
    Noop
}

pub enum Dialogue {
    Output(String)
}

fn main() {
        let line_1 = ASTNode::Line {
            name: String::from("Kay"),
            message: String::from("Hello"),
        };
        let line_2 = ASTNode::Line {
            name: String::from("Alex"),
            message: String::from("Hi"),
        };
        let lines = vec![line_1, line_2];
        let dialogue = ASTNode::Dialogue(vec![ASTNode::Dialogue(lines)]);

        play_live(dialogue);
}

fn play_live(node: ASTNode) {
    println!("NEW ITERATION: {:?}", node);
    if (node == ASTNode::Noop) { return }
    let (response, dialogue) = play(node);
    match response {
        Dialogue::Output(message) => {
            println!("{}", message);
            play_live(dialogue);
        }
    }
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

fn play(node: ASTNode) -> (Dialogue, ASTNode) {
    println!("{:?}", node);
    return match node {
        ASTNode::Line { name: n, message: m } => (Dialogue::Output(format!("{} says {}", n, m)), ASTNode::Noop),
        ASTNode::Noop => panic!("No don't do that :(!!!"),
        ASTNode::Dialogue(items) => match items.first() {
            Option::None => (Dialogue::Output(String::from("")), ASTNode::Noop),
            Option::Some(&ASTNode::Noop) => play(ASTNode::Dialogue(items[1..].to_vec())),
            Option::Some(item) => {
                let (dialogue, new_node) = play((*item).clone());
                return (dialogue, ASTNode::Dialogue([vec![new_node], items[1..].to_vec()].concat()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_one_line_into_our_magic_json() {
        let result = convert(String::from("Kay: Hello"));

        let line = ASTNode::Line {
            name: String::from("Kay"),
            message: String::from("Hello"),
        };
        let lines = vec![line];
        let dialogue = ASTNode::Dialogue(lines);

        assert!(result == dialogue);
    }

    #[test]
    fn plays_simple_game() {
        let line_1 = ASTNode::Line {
            name: String::from("Kay"),
            message: String::from("Hello"),
        };
        let line_2 = ASTNode::Line {
            name: String::from("Alex"),
            message: String::from("Hi"),
        };
        let lines = vec![line_1, line_2];
        let dialogue = ASTNode::Dialogue(lines);


        let next_step_1 = play(dialogue);
        match next_step_1 {
            (Dialogue::Output(string), dialogue_2) => {
                assert_eq!("Kay says Hello", string);
                let next_step_2 = play(dialogue_2);
                match next_step_2 {
                    (Dialogue::Output(string), _) => assert_eq!("Alex says Hi", string),
                    (_, _) => ()
                }
            },
            (_, _) => ()
        }
    }

    #[test]
    fn plays_nested_game() {
        let line_1 = ASTNode::Line {
            name: String::from("Kay"),
            message: String::from("Hello"),
        };
        let line_2 = ASTNode::Line {
            name: String::from("Alex"),
            message: String::from("Hi"),
        };
        let lines = vec![line_1, line_2];
        let dialogue = ASTNode::Dialogue(vec![ASTNode::Dialogue(lines)]);


        let next_step_1 = play(dialogue);
        match next_step_1 {
            (Dialogue::Output(string), dialogue_2) => {
                assert_eq!("Kay says Hello", string);
                let next_step_2 = play(dialogue_2);
                match next_step_2 {
                    (Dialogue::Output(string), _) => assert_eq!("Alex says Hi", string),
                    (_, _) => panic!("Can't do this!")
                }
            },
            (_, _) => panic!("Can't do this!")
        }
    }
}
