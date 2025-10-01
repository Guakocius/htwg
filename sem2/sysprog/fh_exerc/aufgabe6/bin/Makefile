CXX = g++
HEADERSRC = ../../aufgabe5/
CXXFLAGS = -Wall -fpic
CPPFLAGS = -I$(HEADERSRC)
LDFLAGS = -L -l

RM = rm -f

TARGET = notenspiegel

SRC = ../lib
LIB = lib
LIBNAME = aufgabe6
LIBTYPE = so
OBJLIB = $(SRC)/$(LIB)$(LIBNAME).$(LIBTYPE)

.PHONY: all clean install uninstall

all: $(TARGET)


$(TARGET): $(OBJLIB)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(LDFLAGS) $^ -o $@

clean:
	$(RM) $(TARGET)

