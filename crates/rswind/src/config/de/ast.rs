use serde::{de::Visitor, Deserialize, Deserializer};
use smol_str::SmolStr;

use crate::css::{rule::RuleList, Decl, DeclList, Rule};

impl<'de> Deserialize<'de> for DeclList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeclListVisitor;

        impl<'de> Visitor<'de> for DeclListVisitor {
            type Value = DeclList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a decl list")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut decls = DeclList::new();

                while let Some((key, value)) = map.next_entry::<SmolStr, SmolStr>()? {
                    decls.0.push(Decl::new(key, value));
                }

                Ok(decls)
            }
        }

        deserializer.deserialize_map(DeclListVisitor)
    }
}

impl<'de> Deserialize<'de> for RuleList {
    fn deserialize<D>(deserializer: D) -> Result<RuleList, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleListVisitor;

        impl<'de> Visitor<'de> for RuleListVisitor {
            type Value = RuleList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a rule list")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                #[derive(Debug, Deserialize)]
                #[serde(untagged)]
                enum DeclsOrRules {
                    Decls(DeclList),
                    Rules(RuleList),
                }
                let mut rule_list = RuleList::default();

                while let Some(selector) = map.next_key::<SmolStr>()? {
                    match map.next_value::<DeclsOrRules>()? {
                        DeclsOrRules::Decls(decl_list) => {
                            rule_list.push(Rule::new_with_decls(selector, decl_list.0))
                        }
                        DeclsOrRules::Rules(rules) => {
                            rule_list.push(Rule::new_with_rules(selector, rules))
                        }
                    }
                }

                Ok(rule_list)
            }
        }

        deserializer.deserialize_map(RuleListVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::css::{rule::RuleList, Decl, DeclList};

    #[test]
    fn test_deserialize_decl_list() {
        let input = json!({
            "display": "flex",
            "color": "red"
        });

        let res: DeclList = serde_json::from_value(input).unwrap();

        assert_eq!(res.0.len(), 2);
        assert!(res.contains(&Decl::new("display", "flex")));
        assert!(res.contains(&Decl::new("color", "red")));
    }

    #[test]
    fn test_deserialize_rule_list() {
        let input = r#"{
            ".flex": {
                "display": "flex",
                "color": "red"
            },
            "@media (min-width: 768px)": {
                ".flex": {
                    "display": "block"
                }
            }
        }"#;

        let res: RuleList = serde_json::from_str(input).unwrap();
        println!("{:#?}", res);
    }
}
