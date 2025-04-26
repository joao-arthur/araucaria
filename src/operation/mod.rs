pub use compare::compare;
pub use operand_value::OperandValue;
pub use operation::{Operand, Operation};

use operand_value::value_to_operand_value;
use operation::resolve_operand_value;

mod compare;
mod operand_value;
mod operation;
