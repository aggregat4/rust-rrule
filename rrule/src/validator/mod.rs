//! This module includes everything needed to validate an [RRuleProperties].
//! And in turn create a RRule.

use crate::{RRuleError, RRuleProperties};

mod check_limits;
mod validate_properties;

#[allow(unused_imports)]
pub(crate) use check_limits::{
    FREQ_DAILY_INTERVAL_MAX, FREQ_HOURLY_INTERVAL_MAX, FREQ_MINUTELY_INTERVAL_MAX,
    FREQ_MONTHLY_INTERVAL_MAX, FREQ_SECONDLY_INTERVAL_MAX, FREQ_WEEKLY_INTERVAL_MAX,
    FREQ_YEARLY_INTERVAL_MAX,
};
pub(crate) use validate_properties::{DAY_RANGE, MONTH_RANGE, YEAR_RANGE};

/// Validate [`RRuleProperties`] and make sure it meets all requirements.
/// This function always need to be called before creating an [`RRule`] struct.
pub(crate) fn validate_properties(
    properties: RRuleProperties,
) -> Result<RRuleProperties, RRuleError> {
    // Validate required checks (defined by RFC 5545)
    validate_properties::validate_properties_forced(&properties)?;
    // Validate (optional) sanity checks. (arbitrary limits)
    // Can be disabled by `no-validation-limits` feature flag, see README.md for more info.
    check_limits::check_limits(&properties)?;

    Ok(properties)
}
