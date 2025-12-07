use crate::ResourceNameError;

/// the number of segments in a valid resource name.
const SEGMENT_COUNT: usize = 6;

/// the segment separator in a resource name.
const SEGMENT_SEPARATOR: &str = ":";

/// the resource path segment separator in a resource name.
const RESOURCE_PATH_SEGMENT_SEPARATOR: &str = "/";

/// a structured resource name, which embeds the following information:
/// - {prefix} (e.g. 'mrn', 'arn', 'crn', etc.)
/// - {partition} (e.g. 'tycho', 'prod', 'dev', etc.)
/// - {service} (e.g. 'opa', 's3', 'sqs', etc.)
// - {service} (e.g. 'opa', 's3', 'sqs', etc.)
/// - {region} (e.g. 'sol-belt-1', 'us-east-1', 'eu-central-1', etc.)
/// - {account_id} (e.g. '36UeVtK7fIxhHyD9Dd5gc1XSd77', '123456789012', etc.)
/// - {resource_type} (e.g. 'member', 'bucket', 'queue', etc.)
/// - {resource_path} (e.g. 'anderson-dawes', 'my-bucket', 'my-queue', etc.)
///
// format:
/// `{prefix}:{partition}:{service}:{region}:{account_id}:{resource_type}/{resource_path}`
///
/// example:
/// `mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes`
///
/// note: the {region} and {account_id} segments are optional, and may be
/// omitted for global resources.
#[derive(Debug)]
pub struct ResourceName<'a> {
    prefix: &'a str,
    partition: &'a str,
    service: &'a str,
    region: &'a str,
    account_id: &'a str,
    resource_type: &'a str,
    resource_path: &'a str,
}

impl<'a> ResourceName<'a> {
    /// parse a `ResourceName` from a string.
    pub fn parse(input: &'a str) -> Result<Self, ResourceNameError> {
        // did you even bother?
        if input.is_empty() {
            return Err(ResourceNameError::Empty);
        }

        // verify we have the correct structure
        let mut segments = input.split(SEGMENT_SEPARATOR);

        let prefix = segments
            .next()
            .ok_or(ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: 0,
            })?;

        let partition = segments
            .next()
            .ok_or(ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: 1,
            })?;

        let service = segments
            .next()
            .ok_or(ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: 2,
            })?;

        let region = segments
            .next()
            .ok_or(ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: 3,
            })?;

        let account_id = segments
            .next()
            .ok_or(ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: 4,
            })?;

        let qualified_resource_path =
            segments
                .next()
                .ok_or(ResourceNameError::InvalidSegmentCount {
                    expected: SEGMENT_COUNT,
                    found: 5,
                })?;

        // verify we don't have any more segments, this is the same kind of
        // error as above - it's an invalid number of segments.
        // NOTE: the magic 1 is because segments.next() advances the iterator,
        // so we need to account for the current segment.
        if segments.next().is_some() {
            let remaining_segments = segments.count();
            return Err(ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: SEGMENT_COUNT + remaining_segments + 1, // 1 for the current segment
            });
        }

        // verify we have our required segments, where a value must always be
        // present. (e.g. a global resource name may omit the region).
        // optional segments:
        // - region
        // - account_id
        if prefix.is_empty() {
            return Err(ResourceNameError::EmptyPrefix);
        }
        if partition.is_empty() {
            return Err(ResourceNameError::EmptyPartition);
        }
        if service.is_empty() {
            return Err(ResourceNameError::EmptyService);
        }
        if qualified_resource_path.is_empty() {
            return Err(ResourceNameError::EmptyQualifiedResourcePath);
        }

        // verify we have a valid qualified resource path
        let mut resource_path_segments =
            qualified_resource_path.split(RESOURCE_PATH_SEGMENT_SEPARATOR);

        // look my dude, we could totally "just" unwrap here. but like, have
        // you seen what that kind of behavior did to cloudflare? half the
        // bloody internet exploded. did your LLM save you then?
        let resource_type = resource_path_segments
            .next()
            .ok_or(ResourceNameError::EmptyResourceType)?;

        let resource_path = resource_path_segments
            .next()
            .ok_or(ResourceNameError::EmptyResourcePath)?;

        // verify we have a valid resource type and resource path
        if resource_type.is_empty() {
            return Err(ResourceNameError::EmptyResourceType);
        }
        if resource_path.is_empty() {
            return Err(ResourceNameError::EmptyResourcePath);
        }

        Ok(ResourceName {
            prefix,
            partition,
            service,
            region,
            account_id,
            resource_type,
            resource_path,
        })
    }

    /// convert a `ResourceName` to a string.
    pub fn as_str(&self) -> String {
        self.to_string()
    }
}

impl<'a> std::fmt::Display for ResourceName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{prefix}:{partition}:{service}:{region}:{account_id}:{resource_type}/{resource_path}",
            prefix = self.prefix,
            partition = self.partition,
            service = self.service,
            region = self.region,
            account_id = self.account_id,
            resource_type = self.resource_type,
            resource_path = self.resource_path
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::resource_name::SEGMENT_COUNT;

    const VALID_RESOURCE_NAME: &str =
        "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";
    const VALID_RESOURCE_NAME_AWS: &str =
        "arn:aws:ec2:us-east-1:123456789012:instance/i-01234567890123456";
    const VALID_RESOURCE_NAME_KONNECT: &str = "krn:konnect:identity:us:df40c456-7dbb-4fbf-8b2c-a1c89997b7c4:team/157807aa-3a85-4504-8340-ad9c0baae569";

    #[test]
    fn test_parse_valid_resource_name() {
        let rn = ResourceName::parse(VALID_RESOURCE_NAME).unwrap();
        assert_eq!(rn.to_string(), VALID_RESOURCE_NAME);
    }

    #[test]
    fn test_parse_valid_resource_name_aws() {
        let rn = ResourceName::parse(VALID_RESOURCE_NAME_AWS).unwrap();
        assert_eq!(rn.to_string(), VALID_RESOURCE_NAME_AWS);
    }

    #[test]
    fn test_parse_valid_resource_name_konnect() {
        let rn = ResourceName::parse(VALID_RESOURCE_NAME_KONNECT).unwrap();
        assert_eq!(rn.to_string(), VALID_RESOURCE_NAME_KONNECT);
    }

    #[test]
    fn test_parse_resource_name_with_invalid_number_of_segments() {
        let s = "mrn:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: 4,
            }
        );
    }

    #[test]
    fn test_parse_resource_name_with_empty_prefix() {
        let s = ":tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ResourceNameError::EmptyPrefix);
    }

    #[test]
    fn test_parse_resource_name_with_empty_partition() {
        let s = "mrn::opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ResourceNameError::EmptyPartition);
    }

    #[test]
    fn test_parse_resource_name_with_empty_service() {
        let s = "mrn:tycho::sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ResourceNameError::EmptyService);
    }

    #[test]
    fn test_parse_resource_name_with_invalid_qualified_resource_path() {
        let s = "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ResourceNameError::EmptyQualifiedResourcePath
        );
    }

    #[test]
    fn test_parse_resource_name_with_empty_resource_type() {
        let s = "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:/anderson-dawes";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ResourceNameError::EmptyResourceType);
    }

    #[test]
    fn test_parse_resource_name_with_empty_resource_path() {
        let s = "mrn:tycho:opa:sol-belt-1:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ResourceNameError::EmptyResourcePath);
    }

    #[test]
    fn test_parse_resource_name_with_extra_segments() {
        let s = "mrn:foo:tycho:bar:opa:baz:sol-belt-1:buzz:36UeVtK7fIxhHyD9Dd5gc1XSd77:member/anderson-dawes:extra";
        let result = ResourceName::parse(s);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ResourceNameError::InvalidSegmentCount {
                expected: SEGMENT_COUNT,
                found: 11
            }
        );
    }

    #[test]
    fn test_parse_and_as_str_are_inverses() {
        let rn = ResourceName::parse(VALID_RESOURCE_NAME).unwrap();
        assert_eq!(rn.as_str(), VALID_RESOURCE_NAME);
    }
}
