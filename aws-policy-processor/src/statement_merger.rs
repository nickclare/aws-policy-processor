use std::collections::HashSet;

use crate::model::*;

pub fn merge_statements<C: IntoIterator<Item = Statement>>(statements: C) -> Vec<Statement> {
    let statements: Vec<_> = statements.into_iter().collect();
    let mut merged_statements = Vec::new();
    let mut merged_indices = HashSet::<usize>::new();

    for i in 0..statements.len() {
        let mut statement_to_compare = statements.get(i).expect("should be in range").clone();
        if merged_indices.contains(&i) {
            // this item was already merged, so we can skip it
            continue;
        }
        for j in (i + 1)..statements.len() {
            let curr_statement = statements.get(j).expect("should be in range");
            if is_mergable(&statement_to_compare, curr_statement) {
                let new_statement = merge_statement(&statement_to_compare, curr_statement);
                statement_to_compare = new_statement;
                merged_indices.insert(j);
            }
        }
        merged_statements.push(statement_to_compare);
    }

    merged_statements
}

fn is_mergable(a: &Statement, b: &Statement) -> bool {
    let conditions_equal = a.condition == b.condition;
    let resources_equal = a.resources == b.resources;
    let effects_equal = a.effect == b.effect;

    let same_action_type = match (&a.actions, &b.actions) {
        (Actions::Actions(_), Actions::Actions(_))
        | (Actions::NotActions(_), Actions::NotActions(_)) => true,
        _ => false,
    };

    conditions_equal && resources_equal && effects_equal && same_action_type
}

// assumes they're already mergable as per above
fn merge_statement(a: &Statement, b: &Statement) -> Statement {
    let mut merged_actions: Vec<String> = a.actions.get_actions().cloned().collect();
    merged_actions.extend(b.actions.get_actions().cloned());
    let merged_actions = match &a.actions {
        Actions::Actions(_) => Actions::Actions(merged_actions),
        Actions::NotActions(_) => Actions::NotActions(merged_actions),
    };

    Statement {
        actions: merged_actions,
        ..a.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nothing_to_merge() {
        let statement_1: Statement = serde_json::from_str(indoc::indoc! {r#"
                {
                    "Effect": "Allow",
                    "Resource": "*",
                    "Action": [
                        "ec2:StartInstance"
                    ]    
                }
            "#})
        .unwrap();

        let statement_2: Statement = serde_json::from_str(indoc::indoc! {r#"
                {
                    "Effect": "Deny",
                    "Resource": "*",
                    "Action": [
                        "ec2:StartInstance"
                    ]    
                }
            "#})
        .unwrap();

        let output = merge_statements(vec![statement_1, statement_2]);
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_simple_merge() {
        let statement_1: Statement = serde_json::from_str(indoc::indoc! {r#"
                {
                    "Effect": "Allow",
                    "Resource": "*",
                    "Action": [
                        "ec2:StartInstance"
                    ]    
                }
            "#})
        .unwrap();

        let statement_2: Statement = serde_json::from_str(indoc::indoc! {r#"
                {
                    "Effect": "Allow",
                    "Resource": "*",
                    "Action": [
                        "ec2:StopInstance"
                    ]    
                }
            "#})
        .unwrap();

        let output = merge_statements(vec![statement_1, statement_2]);
        assert_eq!(output.len(), 1);
        let stmt = output.get(0).unwrap();
        if let Actions::Actions(ref actions) = stmt.actions {
            assert!(actions.contains(&"ec2:StartInstance".into()));
            assert!(actions.contains(&"ec2:StopInstance".into()));
            assert_eq!(actions.len(), 2);
        } else {
            panic!("Statement should have had an Actions() list");
        }
    }
}
