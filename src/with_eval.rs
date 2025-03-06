
use crate::read::{ReadInput, ReadOneResult, ReadAllResult};
use crate::rt::env::Env;

pub fn read_one<'env, 'err, 'input: 'err>(
    _env: &'env mut Env,
    _input: ReadInput<'input>,
) -> ReadOneResult<'err> {
    todo!()
}

pub fn _read_many<'env, 'err, 'input: 'err>(
    _env: &'env mut Env,
    _input: ReadInput<'input>,
) -> ReadAllResult<'err> {
    todo!()
}
