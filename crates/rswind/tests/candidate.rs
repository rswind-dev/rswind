macro_rules! maybe_arb {
    (named $named:literal) => {
        Some(rswind::common::MaybeArbitrary::Named($named))
    };
    (arb $arb:literal) => {
        Some(rswind::common::MaybeArbitrary::Arbitrary($arb))
    };
    (None) => {
        None
    };
    () => {
        None
    };
}

macro_rules! get_neg {
    ($bool:literal) => {
        true
    };
    () => {
        false
    };
}

macro_rules! candidate {
        ( $key:literal: $value_ty:ident $value:literal $( / $mod_ty:ident $mod:literal )? $(, neg: $neg:literal)? $(, imp: $imp:literal)? ) => {

            rswind::parse::UtilityCandidate {
                key: $key,
                value: maybe_arb!($value_ty $value),
                modifier: maybe_arb!($($mod_ty $mod)?),
                arbitrary: false,
                important: get_neg!($($imp)?),
                negative: get_neg!($($neg)?),
            }
        };
        ( [ $key:literal: $value:literal ] $(, neg: $neg:literal)? $(, imp: $imp:literal)? ) => {
            rswind::parse::UtilityCandidate {
                key: $key,
                value: maybe_arb!(arb $value),
                modifier: None,
                arbitrary: true,
                important: get_neg!($($imp)?),
                negative: get_neg!($($neg)?),
            }
        };
    }

macro_rules! test_group {
        (utility => $($fn_name:ident $input:expr => $expected:expr),* $(,)?) => {
            $(
                paste::item! {
                    #[test]
                    fn [< test_utility_ $fn_name >] () {
                        assert_eq!(run($input), Some($expected));
                    }
                }
            )*
        };
        (variant => $($fn_name:ident $input:expr => $expected:expr),* $(,)?) => {
            $(
                paste::item! {
                    #[test]
                    fn [< test_variant_ $fn_name >] () {
                        assert_eq!(run_variant($input), Some($expected.into()));
                    }
                }
            )*
        };
    }

mod utility {
    use rswind::{
        parse::{candidate::CandidateParser, UtilityCandidate},
        preset::preset_tailwind,
        DesignSystem,
    };

    fn run(input: &str) -> Option<UtilityCandidate> {
        let mut design = DesignSystem::default();
        preset_tailwind(&mut design);
        let mut parser = CandidateParser::new(input);

        parser.parse_utility(&design.utilities)
    }

    test_group! { utility =>
        basic           "text-blue-500"     => candidate!("text": named "blue-500"),
        basic_important "text-blue-500!"    => candidate!("text": named "blue-500", imp: true),
        basic_w         "w-10"              => candidate!("w": named "10"),
        neg_w           "-w-10"             => candidate!("w": named "10", neg: true),
        imp_w           "!w-10"             => candidate!("w": named "10", imp: true),
        neg_imp_w       "-!w-10"            => candidate!("w": named "10", neg: true, imp: true),
        neg_imp_w2      "!-w-10"            => candidate!("w": named "10", neg: true, imp: true),
        neg_imp_w3      "-w-10!"            => candidate!("w": named "10", neg: true, imp: true),
        arb_w           "w-[10px]"          => candidate!("w": arb "10px"),
        arb_mod_w       "text-[10px]/100"   => candidate!("text": arb "10px" / named "100"),
        arb_arbmod_w    "text-[10px]/[100]" => candidate!("text": arb "10px" / arb "100"),
    }
}

mod variant {
    use rswind::css::rule_list;
    use smol_str::SmolStr;

    use rswind::{
        parse::candidate::CandidateParser,
        preset::{preset_tailwind, theme::load_theme},
        DesignSystem,
    };

    fn run_variant(input: &str) -> Option<SmolStr> {
        let mut design = DesignSystem::default();
        load_theme(&mut design);
        preset_tailwind(&mut design);

        let mut parser = CandidateParser::new(input);

        parser
            .parse_variant(&design.variants)?
            .handle(rule_list!("&" {}))
            .as_single()?
            .selector
            .into()
    }

    test_group! { variant =>
        basic "hover" => "&:hover",
        functional "aria-hidden" => r#"&[aria-hidden="true"]"#,
        composable "group-hover" => "&:is(:where(.group):hover *)",
        arb "[:hover]" => "&:is(:hover)",
        arb2 "[&:hover]" => "&:hover",
        arb_at "[@supports(display:grid)]" => "@supports(display:grid)",
        // TODO: [@media(any-hover:hover){&:hover}]
        modifier "group-hover/name" => r#"&:is(:where(.group\/name):hover *)"#,
        modifier2 "group-hover/the-name" => r#"&:is(:where(.group\/the-name):hover *)"#,
        modifier3 "group-hover/[the-name]" => r#"&:is(:where(.group\/the-name):hover *)"#,
        basic_composable "has-hover" => r#"&:has(*:hover)"#,
        composable_funtional "has-aria-hidden" => "&:has(*[aria-hidden=\"true\"])",
        composable_funtional_arb "has-aria-[sort=ascending]" => "&:has(*[aria-sort=ascending])",
        composable_funtional_arb_modifier "has-group-hover/the-name" => "&:has(*:is(:where(.group\\/the-name):hover *))",
        composable_funtional_arb_modifier_arb "has-group-hover/[the-name]" => "&:has(*:is(:where(.group\\/the-name):hover *))",
        multi_composable "has-not-group-hover" => "&:has(*:not(*:is(:where(.group):hover *)))",
        multi_composable2 "has-not-group-hover/the-name" => r#"&:has(*:not(*:is(:where(.group\/the-name):hover *)))"#,
    }
}
