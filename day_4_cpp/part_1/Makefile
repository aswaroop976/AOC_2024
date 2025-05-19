# Compiler and flags
CXX      := g++
CXXFLAGS := -std=c++17 -O2 -Wall -Wextra

# Target executable name
TARGET   := day_4

# Source files
SRCS     := day_4.cpp

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) -o $@ $^

clean:
	rm -f $(TARGET)

