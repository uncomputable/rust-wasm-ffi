#include <stdbool.h>

typedef struct complex {
    int real;
    int im;
} complex;

bool checked_add(int a, int b, int* result) {
    int tmp = a + b;
    if (tmp < a) {
        return false; // Overflow
    } else {
        *result = tmp;
        return true;
    }
}

bool checked_sub(int a, int b, int* result) {
    int tmp = a - b;
    if (tmp > a) {
        return false; // Underflow
    } else {
        *result = tmp;
        return true;
    }
}

bool add(complex* a, complex* b, complex* result) {
    return checked_add(a->real, b->real, &result->real) && checked_add(a->im, b->im, &result->im);
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
