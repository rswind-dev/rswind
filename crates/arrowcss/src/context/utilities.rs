use arrowcss_css_macro::css;
use fxhash::FxHashMap as HashMap;
use lightningcss::values::string::CowArcStr;
use phf::Map;

use crate::css::{AstNode, Decl, NodeList};
use crate::rule::{MetaData, Utility, UtilityHandler};

pub trait UtilityStorage<'c>: Sync + Send {
    fn insert(&mut self, key: String, value: Utility<'c>);
    fn get(&self, key: &str) -> Option<&Vec<Utility<'c>>>;
    fn try_apply<'a>(&self, key: &str, input: &'a str) -> Option<NodeList<'c>>;
}


#[derive(Default)]
pub struct HashMapUtilityStorage<'c> {
    pub utilities: HashMap<String, Vec<Utility<'c>>>,
    // pub theme: Arc<RefCell<Theme<'static>>>,
}

impl<'c> UtilityStorage<'c> for HashMapUtilityStorage<'c> {

    fn insert(&mut self, key: String, value: Utility<'c>) {
        self.utilities.entry(key).or_default().push(value.into());
    }

    fn get(&self, key: &str) -> Option<&Vec<Utility<'c>>> {
        self.utilities.get(key)
    }

    fn try_apply<'a>(
        &self,
        key: &str,
        input: &'a str,
    ) -> Option<NodeList<'c>> {
        self.get(key)?
            .into_iter()
            .find_map(|rule| rule.apply_to(input))
    }
}

#[cfg(test)]
mod tests {

    use crate::css;

    use super::*;

    #[test]
    fn test_utility_storage() {
        // let mut storage = UtilityStorage::new();
        // storage.insert(
        //     "text".into(),
        //     Rule::new(|_, input| decl!("color".into(), input).into()),
        // );

        // assert_eq!(
        //     storage.try_apply("text", "red"),
        //     Some(decl!("color": "red").into())
        // );
    }
}
