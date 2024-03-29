CC = clang
CXX = clang++
CPPFLAGS = -Iinclude -g
CFLAGS = -Wall -Wextra -std=c11 -pedantic
CXXNOWARN := -Wno-unused-private-field -Wno-unused-parameter
# -Wno-mismatched-tags
CXXNOWARN := $(CXXNOWARN) -Wno-c++98-compat
CXXWARN := -Wall -Wextra -Wdeprecated-copy-dtor
CXXFLAGS = -std=c++11 -pedantic $(CXXWARN) -O3 -fPIE $(CXXNOWARN)

cxxobjs = disasm.o idents.o loader.o norris.o object.o objref.o values.o
cxxobjs := $(cxxobjs) main.o
cobjs = alzhmr.o common.o reader.o
objs = $(cxxobjs) $(cobjs)

#gch := dynarr; idents; values;
#gch := include/common.hpp.gch $(gch:%;=include/%.h.gch)
#$(info $(gch))

all: flatvm
	# strip $(cxxobjs)
	wc -c *.o

flatvm: $(objs)
	$(CXX) -o flatvm $(objs)

main.o: src/main.cpp
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $<

#include/common.hpp.gch: include/common.hpp
#	g++ -Iinclude -Wall $<

#%.h.gch: %.h
#	g++ -Iinclude -Wall $<

%.o: src/%.cpp include/%.h
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $<

%.o: src/%.c
	$(CC) -c $(CPPFLAGS) $(CFLAGS) $<

clean: $(wildcard *.o)
	rm -f *.o *.s include/*.gch
