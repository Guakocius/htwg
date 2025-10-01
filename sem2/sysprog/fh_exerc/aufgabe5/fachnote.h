#ifndef FACHNOTE_H
#define FACHNOTE_H

#include "benotung.h"

#include <string>

class fachnote {
    public:
    const std::string fach;
    const benotung note;

    fachnote(const std::string& fach, const benotung& note);

    fachnote(const fachnote&) = delete;
    fachnote& operator=(const fachnote&) = delete;
};
#endif // !FACHNOTE_H