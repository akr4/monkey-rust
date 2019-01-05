use crate::lexer;
use std::io;

pub fn start<R, W>(read: &mut R, write: &mut W)
where
    R: io::BufRead,
    W: io::Write,
{
    let mut buf = String::new();

    loop {
        match read.read_line(&mut buf) {
            Ok(_) => {
                if buf == "\n" {
                    return;
                }
                for token in lexer::Lexer::new(&buf) {
                    // TODO: format (not use debug)
                    writeln!(write, "{:?}", token);
                    buf.clear();
                }
            }
            Err(_) => return,
        }
    }
}
