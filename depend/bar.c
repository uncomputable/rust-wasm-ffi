#include <stdbool.h>

typedef struct complex {
    int real;
    int im;
} complex;

static inline bool checked_add(int a, int b, int* result) {
    return !__builtin_add_overflow(a, b, result);
}

static inline bool checked_sub(int a, int b, int* result) {
    return !__builtin_sub_overflow(a, b, result);
}

// Functions that don't occur in Rust FFI may take structs by value
complex add_helper(complex a, complex b, bool* success) {
    complex ret;
    *success = checked_add(a.real, b.real, &ret.real) && checked_add(a.im, b.im, &ret.im);
    return ret;
}

// Functions that occur in Rust FFI must take structs by reference
bool add(complex* a, complex* b, complex* result) {
    bool success;
    *result = add_helper(*a, *b, &success);
    return success;
}

bool sub(complex* a, complex* b, complex* result) {
    return checked_sub(a->real, b->real, &result->real) && checked_sub(a->im, b->im, &result->im);
}

bool mul(complex* a, complex* b, complex* result) {
    int tmp_real, tmp_im;
    bool ok = checked_sub(a->real * b->real, a->im * b->im, &tmp_real) &&
              checked_add(a->real * b->im, a->im * b->real, &tmp_im);
    if (ok) {
        result->real = tmp_real;
        result->im = tmp_im;
    }
    return ok;
}

bool div(complex* a, complex* b, complex* result) {
    int denominator = b->real * b->real + b->im * b->im;
    if (denominator == 0) {
        return false; // Division by zero
    } else {
        int tmp_real, tmp_im;
        bool ok = checked_add(a->real * b->real, a->im * b->im, &tmp_real) &&
                  checked_sub(a->im * b->real, a->real * b->im, &tmp_im);
        if (ok) {
            result->real = tmp_real / denominator;
            result->im = tmp_im / denominator;
        }
        return ok;
    }
}
