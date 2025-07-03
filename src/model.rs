use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeMap};

#[derive(Clone, PartialEq, Serialize, Debug)]
pub struct Policy {
    #[serde(rename = "Statement")]
    pub statements: Vec<Statement>,
}

/// The different type of supported IAM-like policies
#[derive(Clone, Copy, Debug)]
pub enum PolicyKind {
    Iam,
    Resource,
    ServiceControl,
    ResourceControl,
}

/// Effect value, either `Allow` or `Deny`.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Effect {
    Allow,
    Deny,
}

/// Wraps either the `Action` or `NotAction` value(s) in a policy
#[derive(Clone, PartialEq, Debug)]
pub enum Actions {
    Actions(Vec<String>),
    NotActions(Vec<String>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Principals {
    Principals(Vec<String>),
    NotPrincipals(Vec<String>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Resources {
    Resources(Vec<String>),
    NotResources(Vec<String>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Statement {
    /// Statement ID.
    pub sid: Option<String>,
    /// Whether to allow or deny the actions that match this policy.
    pub effect: Effect,
    /// List of api actions controlled by the policy.
    pub actions: Actions,
    /// Which IAM prinicipals are targeted by the policy. `None` for IAM policies
    pub principals: Option<Principals>,
    /// Which resources are targeted by the policy. `None` for resource policies.
    pub resources: Option<Resources>,
    /// Condition(s) applied to the policy.
    pub condition: Option<serde_json::Value>,
}

impl Serialize for Statement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if let Some(ref sid) = self.sid {
            map.serialize_key("Sid")?;
            map.serialize_value(sid)?;
        }

        let (key, items) = match &self.actions {
            Actions::Actions(items) => ("Action", items),
            Actions::NotActions(items) => ("NotAction", items),
        };
        map.serialize_entry(key, items)?;

        if let Some(p) = &self.principals {
            let (key, items) = match p {
                Principals::Principals(items) => ("Principal", items),
                Principals::NotPrincipals(items) => ("NotPrincipal", items),
            };
            map.serialize_key(key)?;
            if items.len() == 1 {
                map.serialize_value(items.first().expect("already checked there is one element"))?;
            } else {
                map.serialize_value(items)?;
            }
        }

        if let Some(p) = &self.resources {
            let (key, items) = match p {
                Resources::Resources(items) => ("Resource", items),
                Resources::NotResources(items) => ("NotResource", items),
            };
            map.serialize_key(key)?;
            if items.len() == 1 {
                map.serialize_value(items.first().expect("already checked there is one element"))?;
            } else {
                map.serialize_value(items)?;
            }
        }

        if let Some(c) = &self.condition {
            map.serialize_entry("Condition", c)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for Statement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const KNOWN_FIELDS: &[&str] = &[
            "Sid",
            "Effect",
            "Action",
            "NotAction",
            "Principal",
            "NotPrincipal",
            "Resource",
            "NotResource",
            "Condition",
        ];
        struct StatementVisitor;
        impl<'de> Visitor<'de> for StatementVisitor {
            type Value = Statement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "struct Statement")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut sid: Option<String> = None;
                let mut effect: Option<Effect> = None;
                let mut actions: Option<Actions> = None;
                let mut principals: Option<Principals> = None;
                let mut resources: Option<Resources> = None;
                let mut condition: Option<serde_json::Value> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "Sid" => {
                            if sid.is_some() {
                                return Err(serde::de::Error::duplicate_field("Sid"));
                            }
                            sid = Some(map.next_value()?);
                        }
                        "Effect" => {
                            if effect.is_some() {
                                return Err(serde::de::Error::duplicate_field("Effect"));
                            }
                            effect = Some(map.next_value()?);
                        }
                        "Action" => {
                            if actions.is_some() {
                                return Err(serde::de::Error::duplicate_field("Action"));
                            }
                            actions = Some(Actions::Actions(string_or_seq(&mut map)?));
                        }
                        "NotAction" => {
                            if actions.is_some() {
                                return Err(serde::de::Error::duplicate_field("Action"));
                            }
                            actions = Some(Actions::NotActions(string_or_seq(&mut map)?));
                        }
                        "Principal" => {
                            if principals.is_some() {
                                return Err(serde::de::Error::duplicate_field("Principal"));
                            }
                            principals = Some(Principals::Principals(string_or_seq(&mut map)?));
                        }
                        "NotPrincipal" => {
                            if principals.is_some() {
                                return Err(serde::de::Error::duplicate_field("Principal"));
                            }
                            principals = Some(Principals::NotPrincipals(string_or_seq(&mut map)?));
                        }
                        "Resource" => {
                            if resources.is_some() {
                                return Err(serde::de::Error::duplicate_field("Resource"));
                            }
                            resources = Some(Resources::Resources(string_or_seq(&mut map)?));
                        }
                        "NotResource" => {
                            if resources.is_some() {
                                return Err(serde::de::Error::duplicate_field("Resource"));
                            }
                            resources = Some(Resources::NotResources(string_or_seq(&mut map)?));
                        }
                        "Condition" => {
                            if condition.is_some() {
                                return Err(serde::de::Error::duplicate_field("Condition"));
                            }
                            condition = Some(map.next_value()?);
                        }

                        key => return Err(serde::de::Error::unknown_field(key, KNOWN_FIELDS)),
                    }
                }
                Ok(Statement {
                    sid,
                    effect: effect.ok_or_else(|| serde::de::Error::missing_field("Effect"))?,
                    actions: actions.ok_or_else(|| serde::de::Error::missing_field("Action"))?,
                    principals,
                    resources,
                    condition,
                })
            }
        }

        deserializer.deserialize_map(StatementVisitor)
    }
}

fn string_or_seq<'de, A>(mut map: A) -> Result<Vec<String>, A::Error>
where
    A: serde::de::MapAccess<'de>,
{
    enum StringOrSeq {
        String(String),
        Seq(Vec<String>),
    }

    struct StringOrSeqVisitor;
    impl<'de> Visitor<'de> for StringOrSeqVisitor {
        type Value = StringOrSeq;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "string or sequence of strings")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(StringOrSeq::String(v.into()))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut buf = Vec::new();
            while let Some(s) = seq.next_element()? {
                buf.push(s);
            }
            Ok(StringOrSeq::Seq(buf))
        }
    }

    impl<'de> Deserialize<'de> for StringOrSeq {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any(StringOrSeqVisitor)
        }
    }

    map.next_value::<StringOrSeq>().map(|s| match s {
        StringOrSeq::String(s) => vec![s],
        StringOrSeq::Seq(s) => s,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre;
    use indoc::indoc;

    #[test]
    fn test_deserialize_basic() -> eyre::Result<()> {
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
        let statement: Statement = serde_json::from_str(sample_one)?;
        assert_eq!(statement.sid, Some("test_statement_one".into()));
        assert_eq!(statement.effect, Effect::Allow);
        match statement.actions {
            Actions::Actions(a) => assert_eq!(a.len(), 1),
            _ => panic!("actions should have been positive and had one element"),
        }
        match statement.principals {
            Some(Principals::NotPrincipals(p)) => assert_eq!(p.len(), 2),
            _ => panic!("principals should have been negative with 2 elements"),
        }

        Ok(())
    }
}
