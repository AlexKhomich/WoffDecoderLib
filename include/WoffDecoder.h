//
// Created by Alex Khomich on 2019-02-06.
//

#include <stdint.h>

#ifndef WOFF_DECODER_WOFFDECODER_H
#define WOFF_DECODER_WOFFDECODER_H

uint8_t* decofe_from_file_wrapped(const char* path, uint8_t* decoded_data_len);
uint8_t* decode_from_data_wrapped(const uint8_t* source_buf, size_t woff_data_size, uint8_t* decoded_data_len);

#endif //WOFF_DECODER_WOFFDECODER_H
