///handle_command returns a response and whether the connection should be terminated
fn parse_command(cmd: Command) -> COMMAND
{
    use COMMAND::*;
    if let Some(err) = cmd.error
    {
        return Error(err);
    }
    match cmd.cmd.as_str()
    {
        "new_game" =>
        {
            if cmd.args.is_none()
            {
                NewGame(None)
            }
            else
            {
                NewGame(Some(cmd.args.unwrap()[0]))
            }
        }
        "new_ball" =>
        {
            let args = if let Some(args) = cmd.args
            {
                args
            }
            else
            {
                return Error("invalid ball number".to_string());
            };
            if args.len() < 1 || args[0] > 90 || args[0] < 1
            {
                return Error("invalid ball number".to_string());
            }
            NewBall(args[0])
        }
        _ => Error("unknown command".to_string()),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum COMMAND
{
    NewGame(Option<u32>),
    NewBall(u32),
    Winning(Vec<u8>),
    Error(String),
}
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Command
{
    pub cmd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl std::convert::From<Command> for COMMAND
{
    fn from(c: Command) -> COMMAND
    {
        if c.error.is_some()
        {
            return COMMAND::Error(c.error.unwrap());
        }
        parse_command(c)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_command_parser()
    {
        use COMMAND::*;
        let cmds = vec![
            (
                Command {
                    cmd: "new_game".to_string(),
                    args: None,
                    error: None,
                },
                NewGame,
            ),
            (
                Command {
                    cmd: "new_ball".to_string(),
                    args: Some(vec![12]),
                    error: None,
                },
                NewBall(12),
            ),
        ];
        for (input, expected) in cmds
        {
            let actual = parse_command(input);
            assert_eq!(actual, expected)
        }
    }
}
