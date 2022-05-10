ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: ../../target/debug/libmann_kendall.$(EXT)
	g++ -v main.cpp -L ../../target/debug/ -lmann_kendall -o run
	LD_LIBRARY_PATH=../../target/debug/ ./run


clean:
	rm -rf target
	rm -rf run
	rm -f *.o 
	rm -f *.x 
	rm -f *.flat
	rm -f *.map
	rm -f temp
	rm -f *.img
	rm -f $(TARGET)	
	rm -f *.gdb
	rm -f *.bak