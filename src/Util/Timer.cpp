#include "Timer.h"

void Timer::start() {
    this->timer = clock();
}

double Timer::stop() {
    return (clock() - timer) / (double) CLOCKS_PER_SEC;
}