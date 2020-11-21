#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

// Change to be the type you want to use in memcpy

void 
copy(void * dst, const void * src, size_t size) {
    // Implement copy here
    typedef uint64_t TYPE;
    typedef uint8_t BYTE;

    size_t type_size = sizeof(TYPE);
    size_t new_size = size / type_size;
    size_t bytes_copied = 0;
    TYPE * dst_ptr = (TYPE *) dst;
    int num_times = 0;
    for (const TYPE * it = (TYPE *) src; num_times < new_size; ++it, ++dst_ptr, ++num_times){
        *dst_ptr = *it;
        bytes_copied += type_size;
    }
    BYTE * dst_byte_ptr = (BYTE *) dst + bytes_copied;
    BYTE * src_byte_ptr = (BYTE *) src + bytes_copied;
    while (bytes_copied < size){
        *dst_byte_ptr = *src_byte_ptr;
        ++dst_byte_ptr;
        ++src_byte_ptr;
        ++bytes_copied;
    }
}

void
test_one(void){
    typedef int TYPE;

    const int num_elts = 20, val = 3;
    size_t num_bytes = sizeof(TYPE) * num_elts;
    TYPE * src = (int *) malloc(num_bytes);
    memset(src, val, num_bytes);
    TYPE * dst = (int *) malloc(num_bytes);
    copy(dst, src, num_bytes);
    for (int i = 0; i < num_elts; ++i){
        assert(dst[i] == src[i]);
    }
}

void
test_two(void){
    typedef char TYPE;

    const int num_elts = 21, val = 70;
    size_t num_bytes = sizeof(TYPE) * num_elts;
    TYPE * src = (TYPE *) malloc(num_bytes);
    memset(src, val, num_bytes);
    TYPE * dst = (TYPE *) malloc(num_bytes);
    copy(dst, src, num_bytes);
    for (int i = 0; i < num_elts; ++i){
        assert(dst[i] == src[i]);
    }
}

int 
main(void) {
    test_one();
    test_two();
    return 0;
}
