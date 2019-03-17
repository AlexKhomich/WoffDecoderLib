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
} Error;

typedef struct {
    uint8_t *decoded_data;
    size_t decoded_data_len;
    Error error;
} Result;

Result *decode_from_file_wrapped(const char *path);

Result *decode_from_data_wrapped(const uint8_t *source_buf, size_t woff_data_size);

void destroy_buffer(Result *buff_data);

#endif //WOFF_DECODER_WOFFDECODER_H
