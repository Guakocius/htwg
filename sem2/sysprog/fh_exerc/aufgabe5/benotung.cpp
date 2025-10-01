#include "benotung.h"

#include <string>
#include <stdexcept>


/**
 * @author Alexander Engelhardt
 * @version 19.06.2025
 */

namespace {
    const int valid_grades[] = {10, 13, 17, 20, 23, 27, 30, 33, 37, 40, 50};

}

benotung::benotung(int note) 
    : note(note) {
    switch(note) {
        case 10:
        case 13:
        case 17:
        case 20:
        case 23:
        case 27:
        case 30:
        case 33:
        case 37:
        case 40:
        case 50:
        break;
        default:
        throw std::invalid_argument("unzulaessige Note " + std::to_string(note));
        break;
    }
}
const benotung benotung::beste = benotung(10);

const benotung benotung::schlechteste = benotung(50);

int benotung::int_value() const {
    return this->note;
}

bool benotung::ist_bestanden() const {
    return this->note <= 40;
}

bool operator==(benotung n1, benotung n2) {
    return n1.note - n2.note == 0;
}