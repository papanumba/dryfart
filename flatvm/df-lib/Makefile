CC = clang
CXX = clang++
CPPFLAGS = -I../include -I. -DSAFE
CFLAGS = -Wall -Wextra -std=c11 -pedantic
LDFLAGS = -shared
CXXNOWARN := -Wno-unused-private-field -Wno-unused-parameter
# -Wno-mismatched-tags
CXXNOWARN := $(CXXNOWARN) -Wno-c++98-compat
CXXWARN := -Wall -Wextra -Wdeprecated-copy-dtor
CXXFLAGS = -std=c++17 -pedantic $(CXXWARN) -fPIE -fpic #$(CXXNOWARN)


all: libdf-lib.so

debug: CPPFLAGS += -DDEBUG -g
debug: libdf-lib.so

release: LDFLAGS += -O3 -flto
release: CPPFLAGS += -O3 -flto
release: libdf-lib.so

clean:
	rm libdf-lib.so *.o

libdf-lib.so: df-lib.o
	$(CXX)  $(LDFLAGS) -o libdf-lib.so *.o
	cp libdf-lib.so ../

df-lib.o: df-lib.cpp df-lib.h
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $<
