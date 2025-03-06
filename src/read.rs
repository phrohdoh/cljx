
use crate::value::Value;

pub type Result<'e, T> = core::result::Result<T, self::Error<'e>>;
pub type Error<'e> = Box<dyn core::error::Error + 'e>;

pub type ReadInput<'i> = &'i str;
pub type ReadOneOutput = Option<Value>;
pub type ReadOneResult<'e> = Result<'e, ReadOneOutput>;
pub type ReadAllOutput = Vec<Value>;
pub type ReadAllResult<'e> = Result<'e, ReadAllOutput>;


#[path = "without_eval.rs"]
mod without_eval;

#[path = "with_eval.rs"]
mod with_eval;


/// will not eval macros
pub use without_eval::read_one as read_one;
pub use without_eval::read_many as read_many;

/// will eval macros
pub use with_eval::read_one as read_one_expanding;
// pub use with_eval::read_many as read_many_expanding;

