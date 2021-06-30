//https://leetcode.com/problems/goal-parser-interpretation/submissions/
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut parsed_command = String::from("");

        let mut al = false;
        for c in command.chars() {
            match c {
                'G' => parsed_command.push(c),
                ')' => match al {
                    true => {
                        parsed_command.push_str("al");
                        al = false;
                    },
                    false => parsed_command.push('o')
                },
                'a' => al = true,
                _ => ()
            }
        }
        
        parsed_command
    }
}