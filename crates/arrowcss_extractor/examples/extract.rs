use std::collections::HashSet;

use arrowcss_extractor::html::HtmlExtractor;

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
    println!(
        "{:#?}",
        HtmlExtractor::new(input).collect::<HashSet<_>>().len()
    );
}
