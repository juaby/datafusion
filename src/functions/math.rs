use std::rc::Rc;

use super::super::api::*;
use super::super::rel::*;
use super::super::exec::*;

pub struct SqrtFunction {
}

impl ScalarFunction for SqrtFunction {

    fn name(&self) -> String {
        "sqrt".to_string()
    }

    fn execute(&self, args: Vec<Rc<ColumnData>>) -> Result<Rc<ColumnData>,Box<String>> {
        match args[0].as_ref() {
            &ColumnData::Float(ref v) => Ok(Rc::new(ColumnData::Float(v.iter().map(|v| v.sqrt()).collect()))),
            &ColumnData::Double(ref v) => Ok(Rc::new(ColumnData::Double(v.iter().map(|v| v.sqrt()).collect()))),
//            &ColumnData::Int(ref v) => Ok(ColumnData::Int(v.iter().map(|v| v.sqrt()).collect())),
//            &ColumnData::UnsignedInt(ref v) => Ok(ColumnData::UnsignedInt(v.iter().map(|v| v.sqrt()).collect())),
//            &ColumnData::Long(ref v) => Ok(ColumnData::Long(v.iter().map(|v| v.sqrt()).collect())),
//            &ColumnData::UnsignedLong(ref v) => Ok(ColumnData::UnsignedLong(v.iter().map(|v| v.sqrt()).collect())),
            _ => Err(Box::new("Unsupported arg type for sqrt".to_string()))
        }
    }

    fn args(&self) -> Vec<Field> {
        vec![
            Field::new("x", DataType::Double, false),
            Field::new("y", DataType::Double, false)
        ]
    }

    fn return_type(&self) -> DataType {
        DataType::Double
    }
}



