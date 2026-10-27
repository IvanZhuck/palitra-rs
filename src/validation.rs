use anyhow::{Context, bail};

pub fn validate_parameter_colors(param: &str) -> anyhow::Result<u8> {
    let msg = "The 'colors' parameter must be an integer between 1 and 64";
    let val: u8 = param.parse().context(msg)?;

    if !(1..=64).contains(&val) {
        bail!(msg);
    }

    Ok(val)
}

pub fn validate_swatch_size(param: &str) -> anyhow::Result<u32> {
    let msg = "The 'output-image-square-size' parameter must be an integer between 10 and 512";
    let val: u32 = param.parse().context(msg)?;

    if !(10..=512).contains(&val) {
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
        assert!(validate_parameter_colors("65").is_err());
        assert!(validate_parameter_colors("abc").is_err());
    }

    #[test]
    fn test_validate_swatch_size() {
        assert_eq!(validate_swatch_size("50").unwrap(), 50);
        assert_eq!(validate_swatch_size("10").unwrap(), 10);
        assert_eq!(validate_swatch_size("512").unwrap(), 512);

        assert!(validate_swatch_size("0").is_err());
        assert!(validate_swatch_size("513").is_err());
        assert!(validate_swatch_size("abc").is_err());
    }
}
