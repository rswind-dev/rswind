use rswind_extractor::html::HtmlExtractor;
use rustc_hash::FxHashSet;

fn main() {
    let input = r#"
        <script lang="ts">
            console.log('hello');
        </script>
        <div ref="container" :class="containerClass">
            <a href="https://google.com" class="flex" />
        </div>
        <style>
            .a {
                background-image: url('https://google.com');
            }
        </style>
        "#;
    println!("{:#?}", HtmlExtractor::new(input).collect::<FxHashSet<_>>().len());
}
