#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>

#include <sys/types.h>



static size_t modulo(const ssize_t n, const size_t m) {
    const ssize_t signed_m = (ssize_t)m;
    assert(signed_m > 0);
    const ssize_t result = ((n % signed_m) + signed_m) % signed_m;
    assert(result >= 0);
    return (size_t)result;
}

int main() {
    ssize_t n = -5;
    size_t m = 1;

    for (; n < 5; n++) {
        for (m=1; m < 5; m++) {
            size_t result = modulo(n, m);
            printf("modulo(%d, %d) == %d\n", n, m, result);
        }
    }
}
