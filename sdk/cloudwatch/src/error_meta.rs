// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    DashboardInvalidInputError(crate::error::DashboardInvalidInputError),
    DashboardNotFoundError(crate::error::DashboardNotFoundError),
    InternalServiceFault(crate::error::InternalServiceFault),
    InvalidFormatFault(crate::error::InvalidFormatFault),
    InvalidNextToken(crate::error::InvalidNextToken),
    InvalidParameterCombinationException(crate::error::InvalidParameterCombinationException),
    InvalidParameterValueException(crate::error::InvalidParameterValueException),
    LimitExceededException(crate::error::LimitExceededException),
    LimitExceededFault(crate::error::LimitExceededFault),
    MissingRequiredParameterException(crate::error::MissingRequiredParameterException),
    ResourceNotFound(crate::error::ResourceNotFound),
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::DashboardInvalidInputError(inner) => inner.fmt(f),
            Error::DashboardNotFoundError(inner) => inner.fmt(f),
            Error::InternalServiceFault(inner) => inner.fmt(f),
            Error::InvalidFormatFault(inner) => inner.fmt(f),
            Error::InvalidNextToken(inner) => inner.fmt(f),
            Error::InvalidParameterCombinationException(inner) => inner.fmt(f),
            Error::InvalidParameterValueException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::LimitExceededFault(inner) => inner.fmt(f),
            Error::MissingRequiredParameterException(inner) => inner.fmt(f),
            Error::ResourceNotFound(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteAlarmsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteAlarmsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteAlarmsErrorKind::ResourceNotFound(inner) => {
                    Error::ResourceNotFound(inner)
                }
                crate::error::DeleteAlarmsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteAnomalyDetectorError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteAnomalyDetectorError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteAnomalyDetectorErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::DeleteAnomalyDetectorErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DeleteAnomalyDetectorErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::DeleteAnomalyDetectorErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteAnomalyDetectorErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteDashboardsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteDashboardsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDashboardsErrorKind::DashboardNotFoundError(inner) => {
                    Error::DashboardNotFoundError(inner)
                }
                crate::error::DeleteDashboardsErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::DeleteDashboardsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::DeleteDashboardsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteInsightRulesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteInsightRulesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteInsightRulesErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DeleteInsightRulesErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::DeleteInsightRulesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteMetricStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteMetricStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteMetricStreamErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::DeleteMetricStreamErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DeleteMetricStreamErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::DeleteMetricStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeAlarmHistoryError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeAlarmHistoryError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAlarmHistoryErrorKind::InvalidNextToken(inner) => {
                    Error::InvalidNextToken(inner)
                }
                crate::error::DescribeAlarmHistoryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeAlarmsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeAlarmsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAlarmsErrorKind::InvalidNextToken(inner) => {
                    Error::InvalidNextToken(inner)
                }
                crate::error::DescribeAlarmsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeAlarmsForMetricError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeAlarmsForMetricError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAlarmsForMetricErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeAnomalyDetectorsError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeAnomalyDetectorsError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAnomalyDetectorsErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::DescribeAnomalyDetectorsErrorKind::InvalidNextToken(inner) => {
                    Error::InvalidNextToken(inner)
                }
                crate::error::DescribeAnomalyDetectorsErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DescribeAnomalyDetectorsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeInsightRulesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeInsightRulesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeInsightRulesErrorKind::InvalidNextToken(inner) => {
                    Error::InvalidNextToken(inner)
                }
                crate::error::DescribeInsightRulesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DisableAlarmActionsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DisableAlarmActionsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisableAlarmActionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DisableInsightRulesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DisableInsightRulesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisableInsightRulesErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::DisableInsightRulesErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::DisableInsightRulesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::EnableAlarmActionsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::EnableAlarmActionsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::EnableAlarmActionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::EnableInsightRulesError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::EnableInsightRulesError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::EnableInsightRulesErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::EnableInsightRulesErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::EnableInsightRulesErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::EnableInsightRulesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetDashboardError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetDashboardError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDashboardErrorKind::DashboardNotFoundError(inner) => {
                    Error::DashboardNotFoundError(inner)
                }
                crate::error::GetDashboardErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::GetDashboardErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::GetDashboardErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetInsightRuleReportError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetInsightRuleReportError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetInsightRuleReportErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::GetInsightRuleReportErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::GetInsightRuleReportErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetInsightRuleReportErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetMetricDataError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetMetricDataError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetMetricDataErrorKind::InvalidNextToken(inner) => {
                    Error::InvalidNextToken(inner)
                }
                crate::error::GetMetricDataErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetMetricStatisticsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetMetricStatisticsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetMetricStatisticsErrorKind::InternalServiceFault(inner) => Error::InternalServiceFault(inner),
                crate::error::GetMetricStatisticsErrorKind::InvalidParameterCombinationException(inner) => Error::InvalidParameterCombinationException(inner),
                crate::error::GetMetricStatisticsErrorKind::InvalidParameterValueException(inner) => Error::InvalidParameterValueException(inner),
                crate::error::GetMetricStatisticsErrorKind::MissingRequiredParameterException(inner) => Error::MissingRequiredParameterException(inner),
                crate::error::GetMetricStatisticsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetMetricStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetMetricStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetMetricStreamErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::GetMetricStreamErrorKind::InvalidParameterCombinationException(
                    inner,
                ) => Error::InvalidParameterCombinationException(inner),
                crate::error::GetMetricStreamErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::GetMetricStreamErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::GetMetricStreamErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetMetricStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetMetricWidgetImageError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetMetricWidgetImageError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetMetricWidgetImageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListDashboardsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListDashboardsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDashboardsErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::ListDashboardsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::ListDashboardsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListMetricsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListMetricsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListMetricsErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::ListMetricsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::ListMetricsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListMetricStreamsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListMetricStreamsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListMetricStreamsErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::ListMetricStreamsErrorKind::InvalidNextToken(inner) => {
                    Error::InvalidNextToken(inner)
                }
                crate::error::ListMetricStreamsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::ListMetricStreamsErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::ListMetricStreamsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListTagsForResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::ListTagsForResourceErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutAnomalyDetectorError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutAnomalyDetectorError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutAnomalyDetectorErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::PutAnomalyDetectorErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::PutAnomalyDetectorErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::PutAnomalyDetectorErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::PutAnomalyDetectorErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutCompositeAlarmError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutCompositeAlarmError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutCompositeAlarmErrorKind::LimitExceededFault(inner) => {
                    Error::LimitExceededFault(inner)
                }
                crate::error::PutCompositeAlarmErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutDashboardError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutDashboardError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutDashboardErrorKind::DashboardInvalidInputError(inner) => {
                    Error::DashboardInvalidInputError(inner)
                }
                crate::error::PutDashboardErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::PutDashboardErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutInsightRuleError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutInsightRuleError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutInsightRuleErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::PutInsightRuleErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::PutInsightRuleErrorKind::MissingRequiredParameterException(inner) => {
                    Error::MissingRequiredParameterException(inner)
                }
                crate::error::PutInsightRuleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutMetricAlarmError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutMetricAlarmError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutMetricAlarmErrorKind::LimitExceededFault(inner) => {
                    Error::LimitExceededFault(inner)
                }
                crate::error::PutMetricAlarmErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutMetricDataError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutMetricDataError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutMetricDataErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::PutMetricDataErrorKind::InvalidParameterCombinationException(
                    inner,
                ) => Error::InvalidParameterCombinationException(inner),
                crate::error::PutMetricDataErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::PutMetricDataErrorKind::MissingRequiredParameterException(inner) => {
                    Error::MissingRequiredParameterException(inner)
                }
                crate::error::PutMetricDataErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutMetricStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutMetricStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutMetricStreamErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::PutMetricStreamErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::PutMetricStreamErrorKind::InvalidParameterCombinationException(
                    inner,
                ) => Error::InvalidParameterCombinationException(inner),
                crate::error::PutMetricStreamErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::PutMetricStreamErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::PutMetricStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SetAlarmStateError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SetAlarmStateError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SetAlarmStateErrorKind::InvalidFormatFault(inner) => {
                    Error::InvalidFormatFault(inner)
                }
                crate::error::SetAlarmStateErrorKind::ResourceNotFound(inner) => {
                    Error::ResourceNotFound(inner)
                }
                crate::error::SetAlarmStateErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartMetricStreamsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartMetricStreamsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartMetricStreamsErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::StartMetricStreamsErrorKind::InvalidParameterValueException(
                    inner,
                ) => Error::InvalidParameterValueException(inner),
                crate::error::StartMetricStreamsErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::StartMetricStreamsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopMetricStreamsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StopMetricStreamsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopMetricStreamsErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::StopMetricStreamsErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::StopMetricStreamsErrorKind::MissingRequiredParameterException(
                    inner,
                ) => Error::MissingRequiredParameterException(inner),
                crate::error::StopMetricStreamsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::TagResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::TagResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagResourceErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::TagResourceErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::TagResourceErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UntagResourceError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UntagResourceError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagResourceErrorKind::ConcurrentModificationException(inner) => {
                    Error::ConcurrentModificationException(inner)
                }
                crate::error::UntagResourceErrorKind::InternalServiceFault(inner) => {
                    Error::InternalServiceFault(inner)
                }
                crate::error::UntagResourceErrorKind::InvalidParameterValueException(inner) => {
                    Error::InvalidParameterValueException(inner)
                }
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
