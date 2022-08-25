//! Error enum for the tetr-ch-rs.

use http::status::{InvalidStatusCode, StatusCode};

/// A enum for the response handling errors.
#[derive(Debug)]
pub enum ResponseError {
    /// When there are some mismatches in the API docs,
    /// or when this library is defective.
    DeserializeErr(String),
    /// When redirect loop was detected or redirect limit was exhausted.
    RequestErr(String),
    /// When the HTTP request fails.
    HttpErr(Status),
}

impl ToString for ResponseError {
    /// Converts the [`ResponseError`] to a String.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetr_ch::error::ResponseError;
    ///
    /// let de_err = ResponseError::DeserializeErr("Deserialize error".to_string());
    /// let req_err = ResponseError::RequestErr("Request error".to_string());
    /// let http_err = ResponseError::HttpErr(
    ///     tetr_ch::error::Status::Valid(http::StatusCode::SERVICE_UNAVAILABLE)
    /// );
    /// let invalid_http_err = ResponseError::HttpErr(
    ///     tetr_ch::error::Status::Invalid(
    ///         if let Err(isc) = http::StatusCode::from_u16(0) {
    ///             isc
    ///         } else {
    ///             unreachable!()
    ///         }
    ///     )
    /// );
    /// 
    /// assert_eq!(de_err.to_string(), "Deserialize error");
    /// assert_eq!(req_err.to_string(), "Request error");
    /// assert_eq!(http_err.to_string(), "HTTP error 503 Service Unavailable");
    /// assert_eq!(invalid_http_err.to_string(), "HTTP error (Invalid HTTP status code)");
    /// ```
    fn to_string(&self) -> String {
        match self {
            ResponseError::DeserializeErr(msg) => msg.to_string(),
            ResponseError::RequestErr(msg) => msg.to_string(),
            ResponseError::HttpErr(status) => {
                if let Status::Valid(sc) = status {
                    format!("HTTP error {}", sc)
                } else {
                    "HTTP error (Invalid HTTP status code)".to_string()
                }
            }
        }
    }
}

impl From<ResponseError> for std::io::Error {
    fn from(err: ResponseError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

/// HTTP status codes.
#[derive(Debug)]
pub enum Status {
    /// If the status code greater or equal to 100 but less than 600.
    Valid(StatusCode),
    /// If the status code less than 100 or greater than 599.
    Invalid(InvalidStatusCode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_errors_to_string() {
        let de_err = ResponseError::DeserializeErr("Deserialize error".to_string());
        let req_err = ResponseError::RequestErr("Request error".to_string());
        let http_err = ResponseError::HttpErr(Status::Valid(http::StatusCode::SERVICE_UNAVAILABLE));
        #[rustfmt::skip]
        let invalid_http_err = ResponseError::HttpErr(Status::Invalid(
            if let Err(isc) = StatusCode::from_u16(0) {
                isc
            } else {
                unreachable!()
            }
        ));
        assert_eq!(de_err.to_string(), "Deserialize error");
        assert_eq!(req_err.to_string(), "Request error");
        assert_eq!(http_err.to_string(), "HTTP error 503 Service Unavailable");
        #[rustfmt::skip]
        assert_eq!(invalid_http_err.to_string(), "HTTP error (Invalid HTTP status code)");
    }
}
