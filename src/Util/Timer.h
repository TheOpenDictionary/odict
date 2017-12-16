#ifndef ODICT_TIMER_H_H
#define ODICT_TIMER_H_H

#include <ctime>

class Timer {
private:
    clock_t timer;
public:
    void start();

    double stop();
};

#endif //ODICT_TIMER_H_H
