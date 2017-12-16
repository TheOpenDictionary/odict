/**
 * Code taken from
 * https://www.gamedev.net/articles/programming/general-and-gameplay-programming/writing-endian-independent-code-in-c-r2091
 */
#ifndef ODICT_ENDIAN_H
#define ODICT_ENDIAN_H

extern short (*big_short)(short s);

extern short (*little_short)(short s);

extern int (*big_long)(int i);

extern int (*little_long)(int i);

extern uint8_t *(*big_unit8_t)(uint8_t *u, int size);

extern uint8_t *(*little_uint8_t)(uint8_t *u, int size);

class EndianTypes {
public:
    EndianTypes();

    void init();
};


#endif //ODICT_ENDIAN_H
