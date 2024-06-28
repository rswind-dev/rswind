use std::ops::Deref;

use instance_code::{quote, InstanceCode, TokenStream};
use rustc_hash::FxHashMap;
use serde::Deserialize;

use crate::{ThemeMap, ThemeOptions};

#[derive(Debug, Clone)]
pub struct PhfCodegenMap<K, V>(pub FxHashMap<K, V>);

impl<K, V> Deref for PhfCodegenMap<K, V> {
    type Target = FxHashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> InstanceCode for PhfCodegenMap<K, V>
where
    K: InstanceCode + 'static + AsRef<str>,
    V: InstanceCode + 'static + AsRef<str>,
{
    fn instance_code(&self) -> TokenStream {
        let data = self
            .0
            .iter()
            .map(|(k, v)| {
                let k = k.as_ref().instance_code();
                let v = v.as_ref().instance_code();
                quote!(#k => #v)
            })
            .collect::<Vec<_>>();
        quote! {
            phf::phf_map! { #(#data),* }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeCodegen(ThemeOptions);

impl InstanceCode for ThemeCodegen {
    fn instance_code(&self) -> TokenStream {
        let map = &self
            .0
            .clone()
            .into_iter()
            .map(|(k, v)| match v {
                // turn hashmap into phf_map
                ThemeMap::Dynamic(map) => {
                    let k = k.instance_code();
                    let map = PhfCodegenMap(map).instance_code();
                    quote! { (#k, Arc::new(ThemeMap::Static(&#map))) }
                }
                _ => {
                    let k = k.instance_code();
                    let v = v.instance_code();
                    quote! { (#k, Arc::new(#v)) }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            FxHashMap::from_iter([#(#map),*])
        }
    }
}
