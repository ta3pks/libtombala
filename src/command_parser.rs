///handle_command returns a response and whether the connection should be terminated
pub fn parse_command(cmd: Command) -> COMMAND
{
    use COMMAND::*;
    if let Some(err) = cmd.error
    {
        return Error(err);
    }
    match cmd.cmd
    {
        "new_game" => NewGame,
        "new_ball" =>
        //{{{
        {
            if cmd.args.len() < 1 || cmd.args[0] > 90 || cmd.args[0] < 1
            {
                return Error("invalid ball number");
            }
            NewBall(cmd.args[0])
        } //}}}
        _ => Error("unknown command"),
    }
} //}}}

#[derive(Debug, Clone, PartialEq)]
pub enum COMMAND
{
    NewGame,
    NewBall(u32),
    Winning(Vec<u8>),
    Error(&'static str),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command
{
    cmd: &'static str,
    args: Vec<u32>,
    error: Option<&'static str>,
}
#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_command_parser()
    //{{{
    {
        use COMMAND::*;
        //{{{cases
        let cmds = vec![
            (
                Command {
                    cmd: "new_game",
                    args: vec![],
                    error: None,
                },
                NewGame,
            ),
            (
                Command {
                    cmd: "new_ball",
                    args: vec![12],
                    error: None,
                },
                NewBall(12),
            ),
        ];
        //}}}
        for (input, expected) in cmds
        {
            let actual = parse_command(input);
            assert_eq!(actual, expected)
        }
    } //}}}
}
