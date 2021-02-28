use super::ast::{
  expression::Expression, program::Program, statement::Statement, BinaryOperator, UnaryOperator,
};
use super::object::{Object, RuntimeType, TypeOf};
use super::runtime_error::RuntimeError;
use log::debug;
use std::collections::BTreeMap;

pub struct Executor {
  variables: BTreeMap<String, Object>,
}

impl Executor {
  pub fn new() -> Executor {
    return Executor {
      variables: BTreeMap::new(),
    };
  }
  pub fn execute(&mut self, program: &Program) -> Result<Object, RuntimeError> {
    let mut r = Object::Undefined;
    for s in program.statements.iter() {
      r = self.execute_statement(s)?;
      debug!("Statement: {}", r);
    }
    Ok(r)
  }

  pub fn set_variable(&mut self, name: String, value: &Object) {
    debug!("set_variable: {}={}", name, value);
    self.variables.insert(name, value.clone());
  }

  pub fn get_variable(&mut self, name: &str) -> Option<Object> {
    match self.variables.get(name) {
      Some(value) => {
        debug!("get_variable: {}: {}", name, value);
        Some(value.clone())
      }
      None => {
        debug!("get_variable: {}: None", name);
        None
      }
    }
  }

  fn execute_statement(&mut self, statement: &Statement) -> Result<Object, RuntimeError> {
    match statement {
      Statement::Declaration {
        identifier,
        expression,
      } => self.execute_const_assignment(identifier.to_string(), &expression),
      Statement::Assignment {
        identifier,
        expression,
      } => self.execute_const_assignment(identifier.to_string(), &expression),
      Statement::MethodInvocation {
        identifier,
        arguments,
      } => self.execute_method(identifier, &arguments),
      Statement::Empty => Ok(Object::Undefined),
    }
  }

  fn execute_method(
    &mut self,
    identifier: &str,
    arguments: &Vec<Expression>,
  ) -> Result<Object, RuntimeError> {
    match identifier {
      "Print" => {
        for a in arguments {
          let evaluated = self.execute_expression(a)?;
          println!("{}", evaluated);
        }
        Ok(Object::Undefined)
      }
      _ => Err(RuntimeError::UnknownMethod(identifier.to_string())),
    }
  }

  fn execute_const_assignment(
    &mut self,
    identifier: String,
    expression: &Expression,
  ) -> Result<Object, RuntimeError> {
    let evaluated = self.execute_expression(expression)?;
    self.set_variable(identifier.to_owned(), &evaluated);
    Ok(evaluated)
  }

  fn execute_expression(&mut self, expression: &Expression) -> Result<Object, RuntimeError> {
    match expression {
      Expression::Identifier(name) => match self.get_variable(name) {
        Some(value) => Ok(value),
        _ => Ok(Object::Undefined),
      },
      Expression::Integer(value) => Ok(Object::Integer(*value)),
      Expression::Binary {
        left,
        operator,
        right,
      } => {
        let l = self.execute_expression(&left)?;
        let r = self.execute_expression(&right)?;
        match (l, r) {
          (Object::Integer(l), Object::Integer(r)) => match operator {
            BinaryOperator::ADD => Ok(Object::Integer(l + r)),
            BinaryOperator::SUB => Ok(Object::Integer(l - r)),
            BinaryOperator::MUL => Ok(Object::Integer(l * r)),
            BinaryOperator::DIV => Ok(Object::Integer(l / r)),
            BinaryOperator::MOD => Ok(Object::Integer(l % r)),
            BinaryOperator::EQ => Ok(Object::Boolean(l == r)),
            BinaryOperator::NE => Ok(Object::Boolean(l != r)),
            BinaryOperator::GT => Ok(Object::Boolean(l > r)),
            BinaryOperator::LT => Ok(Object::Boolean(l < r)),
            BinaryOperator::LE => Ok(Object::Boolean(l >= r)),
            BinaryOperator::GE => Ok(Object::Boolean(l <= r)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Boolean,
            }),
          },
          (Object::Boolean(l), Object::Boolean(r)) => match operator {
            BinaryOperator::AND => Ok(Object::Boolean(l && r)),
            BinaryOperator::XOR => Ok(Object::Boolean(l || r)),
            BinaryOperator::OR => Ok(Object::Boolean(l || r)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Boolean,
              actual: RuntimeType::Integer,
            }),
          },
          (l, r) => Err(RuntimeError::TypeMismatch {
            expected: l.type_of(),
            actual: r.type_of(),
          }),
        }
      }
      Expression::Unary {
        operator,
        expression,
      } => {
        let evaluated = self.execute_expression(&expression)?;
        match operator {
          UnaryOperator::NEGATIVE => match evaluated {
            Object::Integer(n) => Ok(Object::Integer(-n)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Integer,
            }),
          },
          UnaryOperator::POSITIVE => match evaluated {
            Object::Integer(n) => Ok(Object::Integer(n)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Integer,
            }),
          },
          UnaryOperator::NOT => match evaluated {
            Object::Boolean(n) => Ok(Object::Boolean(!n)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Integer,
            }),
          },
        }
      }
    }
  }
}
