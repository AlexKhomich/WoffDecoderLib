//
// Created by Alex Khomich on 2019-02-06.
//

#include <stdint.h>

#ifndef WOFF_DECODER_WOFFDECODER_H
#define WOFF_DECODER_WOFFDECODER_H

typedef enum {
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

typedef struct {
    uint8_t *decoded_data;
    size_t decoded_data_len;
    Error error;
} DecodedResult;

typedef struct {
    size_t data_len;
    Error error;
} FileRWResult;

DecodedResult *decode_from_file_wrapped(const char *path);

DecodedResult *decode_from_data_wrapped(const uint8_t *source_buf, size_t woff_data_size);

FileRWResult *decode_file_to_file_wrapped(const char *in_path, const char *out_path);

FileRWResult *decode_data_to_file_wrapped(const uint8_t *source_buf, size_t woff_data_size, const char *out_path);

void destroy_decoded_result(DecodedResult *buff_data);

void destroy_file_rw_result(FileRWResult *buff_data);

#endif //WOFF_DECODER_WOFFDECODER_H
