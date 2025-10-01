CXX = g++
CXXFLAGS = -Wall -fpic
RM = rm -f

LIBDYNFLAGS = -shared
LIBSTATFLAGS = ar rcs

SRC = ../../aufgabe5
LIBTYPE = a # Default, kann durch Userinput geändert werden
CXXFILES = $(wildcard $(SRC)/*.cpp)
HEADERS = $(wildcard $(SRC)/*.h)
OBJ = $(notdir $(CXXFILES:.cpp=.o))
LIBNAME = aufgabe6
LIB = lib

.PHONY: all clean install uninstall

all: $(LIB)$(LIBNAME).$(LIBTYPE) 

%.o:$(SRC)/%.cpp
	$(CXX) $(CXXFLAGS) -c -o $@ $<

$(LIB)$(LIBNAME).so: $(OBJ)
	$(CXX) $(LIBDYNFLAGS) $^ -o $@

$(LIB)$(LIBNAME).a: $(OBJ)
	$(LIBSTATFLAGS) $@ $^

clean:
	$(RM) $(OBJ) lib* depend

depend: $(CXXFILES) $(HEADERS)
	$(CXX) $(CXXFLAGS) -MM $(CXXFILES) > $@

include depend