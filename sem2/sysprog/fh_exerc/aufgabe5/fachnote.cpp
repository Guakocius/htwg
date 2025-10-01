#include "fachnote.h"

#include <stdexcept>

/**
 * @author Alexander Engelhardt
 * @version 19.06.2025
 */

fachnote::fachnote(const std::string& fach, const benotung& note)
    : fach(fach), note(note) {
        if (fach.length() == 0) 
            throw std::invalid_argument("Der Fachname darf nicht die Länge 0 haben!");
    }