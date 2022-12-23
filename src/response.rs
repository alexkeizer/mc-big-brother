#![allow(dead_code)]

use std::io;
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[repr(u8)]
pub enum OpCode {
    Noop = 0,
    Eval = 1,
}

pub enum Response {
    Noop,
    Eval(EvalResponse),
}

impl Response {
    fn op_code(&self) -> OpCode {
        match self {
            Self::Noop => OpCode::Noop,
            Self::Eval(_) => OpCode::Eval,
        }
    }

    pub async fn send_over(&self, w: &mut (impl AsyncWrite + Unpin)) -> io::Result<()> {
        w.write_u8(self.op_code() as u8).await?;

        match self {
            Self::Noop => { /* no payload */ }
            Self::Eval(p) => { p.send_over(w).await?; }
        }

        Ok(())
    }
}

/** An `eval` response instructs the client to execute arbitrary lua code.
   If the code returns `true`, the computer will then reboot.

   The payload is formatted as a (big-endian) 4 byte, unsigned integer `n` followed by `n` bytes
   of code to execute
 */
pub struct EvalResponse {
    data: Vec<u8>,
}

impl From<String> for EvalResponse {
    fn from(value: String) -> Self {
        Self {
            data: value.into_bytes()
        }
    }
}

impl From<&str> for EvalResponse {
    fn from(value: &str) -> Self {
        String::from(value).into()
    }
}

impl EvalResponse {
    async fn send_over(&self, w: &mut (impl AsyncWrite + Unpin)) -> io::Result<()> {
        let n: u32 = self.data.len().try_into().unwrap();

        w.write_u32(n).await?;
        w.write_all(&self.data).await?;

        Ok(())
    }
}

