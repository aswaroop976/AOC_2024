#include <iostream>
#include <fstream>
#include <vector>
#include <string>

int main() {
    // Hard-coded input filename hello
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
    static constexpr std::pair<int,int> dir1[2] = {
        {-1, -1}, // up and to the left
        {+1, +1}, // down and to the right
    };

    static constexpr std::pair<int,int> dir2[2] = {
        {+1, -1}, // up and to the right
        {-1, +1}, // down and to the left
    };
    //std::string xmas = "XMAS";
    int num_xmas = 0;
    char S = 'S';
    char M = 'M';
    char A = 'A';

    for (int r = 0; r < int(grid.size()); ++r) {
        for (int c = 0; c < int(grid[r].size()); ++c) {
            if(grid[r][c] == A){
                num_xmas++;
                int num_s = 0; int num_m = 0;
                for(auto [dx, dy] : dir1) {
                    int nr = r + dy;
                    int nc = c + dx;

                    if(nr < 0 || nr >= int(grid.size()) || nc < 0
                    || nc >= int(grid[nr].size())){
                        break;
                    }
                    if (grid[nr][nc] == S){
                        num_s++;
                    } else if(grid[nr][nc] == M){
                        num_m++;
                    }
                }
                if(!(num_s == 1 && num_m == 1)){
                    num_xmas--;
                    continue;
                }
            
                num_s = 0; num_m = 0;
                for(auto [dx, dy] : dir2) {
                    int nr = r + dy;
                    int nc = c + dx;

                    if(nr < 0 || nr >= int(grid.size()) || nc < 0
                    || nc >= int(grid[nr].size())){
                        break;
                    }
                    if (grid[nr][nc] == S){
                        num_s++;
                    } else if(grid[nr][nc] == M){
                        num_m++;
                    }
                }
                if(!(num_s == 1 && num_m == 1)){
                    num_xmas--;
                    continue;
                }
            }           
        }
    }

    printf("number of XMAS occurences: %d\n", num_xmas);



    return 0;
}

