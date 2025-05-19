#include <iostream>
#include <fstream>
#include <vector>
#include <string>

int main() {
    // Hard-coded input filename
    const std::string filename = "input";

    std::ifstream infile{filename};
    if (!infile) {
        std::cerr << "Error opening file: " << filename << "\n";
        return 1;
    }

    // Read into a 2D vector of chars
    std::vector<std::vector<char>> grid;
    for (std::string line; std::getline(infile, line); ) {
        grid.emplace_back(line.begin(), line.end());
    }

    // 1) Define the 8 directions (dx,dy)
    static constexpr std::pair<int,int> dirs[8] = {
        {+1,  0}, // right
        {+1, +1}, // down-right
        { 0, +1}, // down
        {-1, +1}, // down-left
        {-1,  0}, // left
        {-1, -1}, // up-left
        { 0, -1}, // up
        {+1, -1}  // up-right
    };

    std::string xmas = "XMAS";
    int num_xmas = 0;

    for (int r = 0; r < int(grid.size()); ++r) {
        for (int c = 0; c < int(grid[r].size()); ++c) {
            
            for(auto [dx, dy] : dirs) {
                for(int step = 0; step <= 3; ++step) {
                    int nr = r + dy * (step);
                    int nc = c + dx * (step);

                    if(nr < 0 || nr >= int(grid.size()) || nc < 0 || nc >= int(grid[nr].size())){
                        break;
                    }

                    char to_process = grid[nr][nc];
                    if(to_process != xmas[step]){
                        break;
                    }

                    if(step == 3){
                        num_xmas ++;
                    }
                }
            }
        }
    }

    printf("number of XMAS occurences: %d\n", num_xmas);


    return 0;
}

