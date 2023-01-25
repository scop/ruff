//! Rules from [flake8-type-checking](https://pypi.org/project/flake8-type-checking/).
pub(crate) mod helpers;
pub(crate) mod rules;

#[cfg(test)]
mod tests {
    use std::convert::AsRef;
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::linter::test_path;
    use crate::registry::Rule;
    use crate::settings;

    #[test_case(Rule::TypingOnlyFirstPartyImport, Path::new("TYP001.py"); "TYP001")]
    #[test_case(Rule::TypingOnlyThirdPartyImport, Path::new("TYP002.py"); "TYP002")]
    #[test_case(Rule::TypingOnlyStandardLibraryImport, Path::new("TYP003.py"); "TYP003")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_1.py"); "TYP004_1")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_2.py"); "TYP004_2")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_3.py"); "TYP004_3")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_4.py"); "TYP004_4")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_5.py"); "TYP004_5")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_6.py"); "TYP004_6")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_7.py"); "TYP004_7")]
    #[test_case(Rule::RuntimeImportInTypeCheckingBlock, Path::new("TYP004_8.py"); "TYP004_8")]
    #[test_case(Rule::EmptyTypeCheckingBlock, Path::new("TYP005.py"); "TYP005")]
    fn rules(rule_code: Rule, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", rule_code.as_ref(), path.to_string_lossy());
        let diagnostics = test_path(
            Path::new("./resources/test/fixtures/flake8_type_checking")
                .join(path)
                .as_path(),
            &settings::Settings::for_rule(rule_code),
        )?;
        insta::assert_yaml_snapshot!(snapshot, diagnostics);
        Ok(())
    }
}