//
// Created by Alex Khomich on 2019-02-06.
//

#include <stdint.h>

#ifndef WOFF_DECODER_WOFFDECODER_H
#define WOFF_DECODER_WOFFDECODER_H

size_t decofe_from_file(const char* path, uint8_t* dest_buf);
size_t decode_from_data(const uint8_t* source_buf, size_t woff_data_size, uint8_t* dest_buf);

#endif //WOFF_DECODER_WOFFDECODER_H
