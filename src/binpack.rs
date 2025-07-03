use std::hash::Hash;

use crate::model::*;

// TODO: think of a better name for this
/// Memoizes the json representation of the given statement, so that we can
/// avoid redoing it when we need to hash the value. We might want to find a way to
/// memozise the actual hash as well, but that can come later.
#[derive(Debug, PartialEq)]
pub struct StatementRepr<'a> {
    statement: &'a Statement,
    repr: String,
}

impl<'a> Hash for StatementRepr<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.hash(state);
    }
}

impl<'a> StatementRepr<'a> {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.repr.len()
    }
}

impl<'a> StatementRepr<'a> {
    pub(crate) fn new(stmt: &'a Statement) -> Self {
        let repr = serde_json::to_string(stmt).expect("statement should be serializable");
        Self {
            statement: stmt,
            repr,
        }
    }
}

pub struct BinPacker {
    /// maximum size of policies that can be put into a single policy
    target_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::*;

    #[test]
    fn test_repr_str() {
        let sample_one = indoc! {r#"
            {
                "Sid": "test_statement_one",
                "Effect": "Allow",
                "Action": "ec2:*",
                "NotPrincipal": ["some-arn", "some-other-arn"],
                "Condition": {
                    "StringEqualsIgnoreCase": {
                        "aws:PrincipalTag/something": "sometagvalue"
                    }
                }
            } 
        "#};

        let statement: Statement =
            serde_json::from_str(sample_one).expect("statement should be valid");
        let repr = dbg!(StatementRepr::new(&statement));
        dbg!(repr.len());
        assert!(repr.len() > 0);
        assert!(repr.len() <= sample_one.len());
    }
}
