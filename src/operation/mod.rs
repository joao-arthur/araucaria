pub use compare::compare;
pub use operand_value::OperandValue;
pub use operation::{Operand, Operation};

use operand_value::operand_value_from_value;
use operation::resolve_operand_value;

mod compare;
mod operand_value;
mod operation;
