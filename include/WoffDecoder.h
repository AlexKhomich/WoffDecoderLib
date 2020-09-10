//
// Created by Alex Khomich on 2019-02-06.
//

#include <stdint.h>

#ifndef WOFF_DECODER_WOFFDECODER_H
#define WOFF_DECODER_WOFFDECODER_H

/// Enum with types of error
/// If `Error` with type `None` that means no errors occurred
typedef enum Error {
    None,
    DecodeError,
    DataSourceIsEmpty,
    OutBufferFull,
    BuffError,
    DecompressError,
    InvalidWoffSize,
    InputBufferIsEmpty,
    InvalidWoffSignature,
    InvalidWoffStructure,
    CreateFileError,
    OpenFileError,
    WriteToFileError,
    ReadFromFileError,
    InputPathError,
    OutputPathError
} Error;

/// Result structure with decoded SFNT data
///
/// #Fields
///
/// `decoded_data` - decoded SFNT data
/// `decoded_data_len` - length of decoded SFNT data
/// `error` - type of error. None - returned result has no errors.
typedef struct DecodedResult {
    uint8_t* decoded_data;
    size_t decoded_data_len;
    Error error;
} DecodedResult;

/// `FileRWResult` structure with length of decoded data and error
///
/// #Fields
///
/// `data_len` - length of decoded SFNT data that was written to file.
/// `error` - type of error. None - returned result has no errors and file was written successfully.
typedef struct FileRWResult {
    size_t data_len;
    Error error;
} FileRWResult;

#ifdef __cplusplus
extern "C" {
#endif

/// Decode .woff file data to SFNT bytes wrapped for using with C wrapper
/// And returns Result structure with decoded data
DecodedResult* decode_from_file_wrapped(const char* path);

/// Decode WOFF data to SFNT data wrapped for using with C wrapper
DecodedResult* decode_from_data_wrapped(const uint8_t* source_buf, size_t woff_data_size);

/// Decode .woff file data to SFNT file wrapped for using with C wrapper
/// And returns FileRWResult structure with decoded data
FileRWResult* decode_file_to_file_wrapped(const char* in_path, const char* out_path);

/// Decode WOFF data to SFNT file wrapped for using with C wrapper
FileRWResult* decode_data_to_file_wrapped(const uint8_t* source_buf, size_t woff_data_size, const char* out_path);

/// Destroys buffer with decoded data. Using with C wrapper
void destroy_decoded_result(DecodedResult* buff_data);

/// Destroys buffer with decoded data. Using with C wrapper
void destroy_file_rw_result(FileRWResult* buff_data);

#ifdef __cplusplus
}
#endif

#endif //WOFF_DECODER_WOFFDECODER_H
