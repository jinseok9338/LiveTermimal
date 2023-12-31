use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;
use std::path::PathBuf;
/**
 * Standard libc error codes. Add more to this enum and ErrorStrings as they are
 * needed.
 * @url http://www.gnu.org/software/libc/manual/html_node/Error-Codes.html
 */
#[derive(Deserialize, Debug, PartialEq, Serialize, Clone)]
pub enum ErrorCode {
    EPERM = 1,
    ENOENT = 2,
    EIO = 5,
    EBADF = 9,
    EACCES = 13,
    EBUSY = 16,
    EEXIST = 17,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    EFBIG = 27,
    ENOSPC = 28,
    EROFS = 30,
    ENOTEMPTY = 39,
    ENOTSUP = 95,
}

pub fn get_error_string(code: &ErrorCode) -> String {
    let error_string = match code {
        ErrorCode::EPERM => "Operation not permitted.",
        ErrorCode::ENOENT => "No such file or directory.",
        ErrorCode::EIO => "Input/output error.",
        ErrorCode::EBADF => "Bad file descriptor.",
        ErrorCode::EACCES => "Permission denied.",
        ErrorCode::EBUSY => "Resource busy or locked.",
        ErrorCode::EEXIST => "File exists.",
        ErrorCode::ENOTDIR => "File is not a directory.",
        ErrorCode::EISDIR => "File is a directory.",
        ErrorCode::EINVAL => "Invalid argument.",
        ErrorCode::EFBIG => "File is too big.",
        ErrorCode::ENOSPC => "No space left on disk.",
        ErrorCode::EROFS => "Cannot modify a read-only file system.",
        ErrorCode::ENOTEMPTY => "Directory is not empty.",
        ErrorCode::ENOTSUP => "Operation is not supported.",
    };
    error_string.to_string()
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ApiErrorJSON {
    pub errno: ErrorCode,
    pub message: String,
    pub path: String,
    pub code: String,
    pub stack: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ApiError {
    pub errno: ErrorCode,
    pub code: String,
    pub path: Option<String>,
    pub syscall: String,
    pub stack: Option<String>,
    pub message: String,
}

impl ApiError {
    pub fn new(
        errno: &ErrorCode,
        code: String,
        path: Option<String>,
        stack: Option<String>,
    ) -> ApiError {
        let syscall = "".to_owned();
        let errno = errno.clone();
        let path_for_error: Option<String>;
        let message: String;
        if let Some(p) = path {
            message = format!("Error: {} {}", code, p);
            path_for_error = Some(p);
        } else {
            message = format!("Error: {}", code);
            path_for_error = None;
        }

        ApiError {
            errno,
            code,
            path: path_for_error,
            syscall,
            stack,
            message,
        }
    }

    pub fn from_json(json: ApiErrorJSON) -> ApiError {
        ApiError::new(&json.errno, json.code, Some(json.path), Some(json.stack))
    }

    pub fn from_buffer(buffer: &[u8], i: usize) -> Result<ApiError, JsonError> {
        let length = u32::from_le_bytes(buffer[i..i + 4].try_into().unwrap()) as usize;
        let json_str = String::from_utf8_lossy(&buffer[i + 4..i + 4 + length]);
        let json: ApiErrorJSON = serde_json::from_str(&json_str)?;
        Ok(ApiError::from_json(json))
    }

    pub fn file_error(code: ErrorCode, path: PathBuf) -> ApiError {
        ApiError::new(
            &code,
            get_error_string(&code),
            Some(path.to_string_lossy().to_string()),
            None,
        )
    }

    pub fn eacces(path: PathBuf) -> ApiError {
        ApiError::file_error(ErrorCode::EACCES, path)
    }

    pub fn enoent(path: PathBuf) -> ApiError {
        ApiError::file_error(ErrorCode::ENOENT, path)
    }

    pub fn eexist(path: PathBuf) -> ApiError {
        ApiError::file_error(ErrorCode::EEXIST, path)
    }

    pub fn eisdir(path: PathBuf) -> ApiError {
        ApiError::file_error(ErrorCode::EISDIR, path)
    }

    pub fn enotdir(path: PathBuf) -> ApiError {
        ApiError::file_error(ErrorCode::ENOTDIR, path)
    }

    pub fn eperm(path: PathBuf) -> ApiError {
        ApiError::file_error(ErrorCode::EPERM, path)
    }

    pub fn enotempty(path: PathBuf) -> ApiError {
        ApiError::file_error(ErrorCode::ENOTEMPTY, path)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn api_buffer_size(&self) -> usize {
        4 + serde_json::to_string(&self.to_json()).unwrap().len()
    }

    pub fn write_to_buffer(&self, buffer: &mut Vec<u8>, i: usize) {
        let json_str = serde_json::to_string(&self.to_json()).unwrap();
        buffer.extend_from_slice(&(json_str.len() as u32).to_le_bytes());
        buffer.extend_from_slice(json_str.as_bytes());
    }
}

impl ToString for ApiError {
    fn to_string(&self) -> String {
        self.message.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let json = ApiErrorJSON {
            errno: ErrorCode::EPERM,
            message: "Test message".to_owned(),
            path: "/test/path".to_owned(),
            code: "TEST".to_owned(),
            stack: "Test stack trace".to_owned(),
        };

        let api_error = ApiError::from_json(json);

        assert_eq!(api_error.errno, ErrorCode::EPERM);
        assert_eq!(api_error.code, "TEST");
        assert_eq!(api_error.path, Some("/test/path".to_owned()));
        assert_eq!(api_error.syscall, "");
        assert_eq!(api_error.stack, Some("Test stack trace".to_owned()));
    }

    #[test]
    fn test_from_buffer() {
        let json = ApiErrorJSON {
            errno: ErrorCode::EPERM,
            message: "Test message".to_owned(),
            path: "/test/path".to_owned(),
            code: "TEST".to_owned(),
            stack: "Test stack trace".to_owned(),
        };

        let json_str = serde_json::to_string(&json).unwrap();
        let length = json_str.len() as u32;
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&length.to_le_bytes());
        buffer.extend_from_slice(json_str.as_bytes());

        let result = ApiError::from_buffer(&buffer, 0);

        assert!(result.is_ok());
        let api_error = result.unwrap();
        assert_eq!(api_error.errno, ErrorCode::EPERM);
        assert_eq!(api_error.code, "TEST");
        assert_eq!(api_error.path, Some("/test/path".to_owned()));
        assert_eq!(api_error.syscall, "");
        assert_eq!(api_error.stack, Some("Test stack trace".to_owned()));
    }
    #[test]
    fn test_buffer_size() {
        let api_error = ApiError {
            errno: ErrorCode::EPERM,
            message: "Test message".to_owned(),
            syscall: "".to_owned(),
            path: Some("/test/path".to_owned()),
            code: "TEST".to_owned(),
            stack: Some("Test stack trace".to_owned()),
        };

        let expected_size = 4 + serde_json::to_string(&api_error.to_json()).unwrap().len();
        let actual_size = api_error.api_buffer_size();

        assert_eq!(expected_size, actual_size);
    }
    #[test]
    fn test_write_to_buffer() {
        let api_error = ApiError {
            errno: ErrorCode::EPERM,
            message: "Test message".to_owned(),
            syscall: "".to_owned(),
            path: Some("/test/path".to_owned()),
            code: "TEST".to_owned(),
            stack: Some("Test stack trace".to_owned()),
        };

        let mut buffer = Vec::new();
        let i = 0;
        api_error.write_to_buffer(&mut buffer, i);
        let expected_json_str = serde_json::to_string(&api_error.to_json()).unwrap();
        let expected_buffer = [
            &(expected_json_str.len() as u32).to_le_bytes(),
            expected_json_str.as_bytes(),
        ]
        .concat();

        assert_eq!(buffer, expected_buffer);
    }
}
