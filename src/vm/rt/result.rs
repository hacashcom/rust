
#[derive(Debug)]
pub enum CallExit {
    Tailend,
    Finish,
    Return,
    Abort,
    Call(Funcptr),
}

