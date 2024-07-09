CC = clang
CXX = clang++
CPPFLAGS = -Iinclude -DSAFE
CFLAGS = -Wall -Wextra -std=c11 -pedantic
LDFLAGS = 
CXXNOWARN := -Wno-unused-private-field -Wno-unused-parameter
# -Wno-mismatched-tags
CXXNOWARN := $(CXXNOWARN) -Wno-c++98-compat
CXXWARN := -Wall -Wextra -Wdeprecated-copy-dtor
CXXFLAGS = -std=c++17 -pedantic $(CXXWARN) -fPIE #$(CXXNOWARN)

cxxobjs = bitarr.o disasm.o garcol.o htable.o idents.o loader.o maitre.o native.o norris.o object.o objref.o values.o virmac.o
cxxobjs := $(cxxobjs) main.o
cobjs = common.o reader.o
objs = $(cxxobjs) $(cobjs)

#gch := dynarr; idents; values;
#gch := include/common.hpp.gch $(gch:%;=include/%.h.gch)
#$(info $(gch))

all: flatvm

debug: CPPFLAGS += -DDEBUG -g
debug: flatvm

release: LDFLAGS += -O3 -flto
release: CPPFLAGS += -O3 -flto
release: flatvm
#	strip flatvm

install: LDFLAGS += -O3 -flto
install: CPPFLAGS += -O3 -flto
install: flatvm
	cp flatvm ~/.local/bin/
	clear

flatvm: $(objs)
	$(CXX) -o flatvm $(LDFLAGS) $(objs)

main.o: src/main.cpp
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $<

#include/common.hpp.gch: include/common.hpp
#	g++ -Iinclude -Wall $<

#%.h.gch: %.h
#	g++ -Iinclude -Wall $<

%.o: src/%.cpp include/%.h
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $<

virmac.o: src/virmac.cpp src/vm-ops.cpp include/virmac.h
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $<

native.o: src/native.cpp src/df-std/df_std.cpp include/native.h
	$(CXX) -c $(CPPFLAGS) $(CXXFLAGS) $<

%.o: src/%.c
	$(CC) -c $(CPPFLAGS) $(CFLAGS) $<

clean: $(wildcard *.o)
	rm -f *.o *.s include/*.gch
