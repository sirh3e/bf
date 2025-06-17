#include <stdio.h>

typedef unsigned char byte;
typedef unsigned long long usize;

#define MEMORY memory
#define MEMORY_LENGTH 30000
#define MEMORY_DEFINE \
    byte MEMORY[MEMORY_LENGTH] = { 0 }

#define POINTER pointer
#define POINTER_DEFINE \
    usize POINTER = 0

#define DEC_VAL_BY(amount) \
    MEMORY[POINTER] -= amount

#define INC_VAL_BY(amount) \
    MEMORY[POINTER] += amount

#define DEC_PTR_BY(amount) \
    POINTER -= amount

#define INC_PTR_BY(amount) \
    POINTER += amount

#define CLEAR \
    MEMORY[POINTER] = 0

#define MUL_VAL_BY(offset, amount) \
    MEMORY[POINTER + offset] += MEMORY[POINTER] * amount

#define LOOP(expressions)         \
    while(MEMORY[POINTER] != 0) { \
        expressions               \
    }

#define OUTPUT \
    printf("%c", MEMORY[POINTER])

int main() {
    POINTER_DEFINE;
    MEMORY_DEFINE;

    <CODE>

    return 0;
}