#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Line { name: String, message: String },
    Dialogue(Vec<ASTNode>),
    Noop
}

pub enum Action {
    Print(String),
    Noop
}

fn main() {
    cli_play(convert(String::from("Kay: Hello\nAlex: Hi")));
}

fn convert(input: String) -> ASTNode {
    let mut lines = vec![];
    let mut chars = input.chars().peekable();

    while chars.peek() != None {
        let name: String = chars.by_ref()
                                .take_while(|ch| *ch != ':')
                                .collect();

        let message: String = chars.by_ref()
                                   .take_while(|ch| *ch != '\n')
                                   .collect();

        let trimmed_message: String = String::from(message.trim());

        lines.push(ASTNode::Line { name: name, message: trimmed_message })
    }

    return ASTNode::Dialogue(lines);
}

fn cli_play(game: ASTNode) {
    if game == ASTNode::Noop { return }
    let (response, advanced_game) = advance(game);
    match response {
        Action::Print(message) => println!("{}", message),
        Action::Noop => { }
    }
    cli_play(advanced_game);
}

fn advance(node: ASTNode) -> (Action, ASTNode) {
    return match node {
        ASTNode::Dialogue(items) => advance_dialogue(items),
        ASTNode::Line { name, message } => advance_line(name, message),
        ASTNode::Noop => panic!("Can't advance a noop :("),
    }
}

fn advance_dialogue(items: Vec<ASTNode>) -> (Action, ASTNode) {
    return match items.first() {
        Option::Some(&ASTNode::Noop) => advance(ASTNode::Dialogue(items[1..].to_vec())),
        Option::Some(item) => {
            let (action, advanced_node) = advance((*item).clone());
            return (action, ASTNode::Dialogue([vec![advanced_node], items[1..].to_vec()].concat()))
        },
        Option::None => (Action::Noop, ASTNode::Noop),
    }
}

fn advance_line(name: String, message: String) -> (Action, ASTNode) {
    return (Action::Print(format!("{} says {}", name, message)), ASTNode::Noop);
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
        let node = ASTNode::Dialogue(lines);

        assert_eq!(result, node);
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
        let node = ASTNode::Dialogue(lines);


        let next_step_1 = advance(node);
        match next_step_1 {
            (Action::Print(string), dialogue_2) => {
                assert_eq!("Kay says Hello", string);
                let next_step_2 = advance(dialogue_2);
                match next_step_2 {
                    (Action::Print(string), _) => assert_eq!("Alex says Hi", string),
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
        let node = ASTNode::Dialogue(vec![ASTNode::Dialogue(lines)]);


        let next_step_1 = advance(node);
        match next_step_1 {
            (Action::Print(string), dialogue_2) => {
                assert_eq!("Kay says Hello", string);
                let next_step_2 = advance(dialogue_2);
                match next_step_2 {
                    (Action::Print(string), _) => assert_eq!("Alex says Hi", string),
                    (_, _) => panic!("Can't do this!")
                }
            },
            (_, _) => panic!("Can't do this!")
        }
    }
}
