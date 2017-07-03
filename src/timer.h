#ifndef ODICT_TIMER_H_H
#define ODICT_TIMER_H_H

class Timer {
private:
    clock_t timer;
public:
    void start() {
        this->timer = clock();
    }

    double stop() {
        return (clock() - timer) / (double)CLOCKS_PER_SEC;
    }
};

#endif //ODICT_TIMER_H_H
