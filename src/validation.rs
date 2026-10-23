use anyhow::{Context, bail};

pub fn validate_parameter_colors(param: &str) -> anyhow::Result<u8> {
    let msg = "The 'colors' parameter must be an integer between 1 and 32";
    let val: u8 = param.parse().context(msg)?;

    if !(1..=32).contains(&val) {
        bail!(msg);
    }

    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_parameter_colors() {
        assert_eq!(validate_parameter_colors("8").unwrap(), 8);
        assert_eq!(validate_parameter_colors("1").unwrap(), 1);
        assert_eq!(validate_parameter_colors("32").unwrap(), 32);

        assert!(validate_parameter_colors("0").is_err());
        assert!(validate_parameter_colors("33").is_err());
        assert!(validate_parameter_colors("abc").is_err());
    }
}
