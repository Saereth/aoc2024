#include <iostream>
#include <fstream>
#include <string>
#include <regex>

int main() {
    std::string filePath = "d3.csv";
    std::ifstream inputFile(filePath);

    if (!inputFile.is_open()) {
        std::cerr << "Err: Could not open the file " << filePath << std::endl;
        return 1;
    }

    // Read the entire content of the file into a string
    std::string data((std::istreambuf_iterator<char>(inputFile)),
                     (std::istreambuf_iterator<char>()));
    inputFile.close();

    // Regex the instructions
    std::regex mulPattern(R"(mul\((\d+),(\d+)\))");
    std::regex doPattern(R"(do\(\))");
    std::regex dontPattern(R"(don't\(\))");

    bool isMultiplyEnabled = true;
    int totalSum = 0;

    // Iterate through the data and process instructions
    std::regex instructionPattern(R"(mul\(\d+,\d+\)|do\(\)|don't\(\))");
    std::sregex_iterator iter(data.begin(), data.end(), instructionPattern);
    std::sregex_iterator end;

    while (iter != end) {
        std::smatch match = *iter;
        std::string instruction = match.str();

        if (std::regex_match(instruction, doPattern)) {
            isMultiplyEnabled = true;
        } else if (std::regex_match(instruction, dontPattern)) {
            isMultiplyEnabled = false;
        } else if (std::regex_match(instruction, mulPattern) && isMultiplyEnabled) {
            std::smatch mulMatch;
            if (std::regex_match(instruction, mulMatch, mulPattern)) {
                int x = std::stoi(mulMatch[1].str());
                int y = std::stoi(mulMatch[2].str());
                totalSum += x * y;
            }
        }

        ++iter;
    }

    std::cout << "Total Sum with Conditions: " << totalSum << std::endl;

    return 0;
}
