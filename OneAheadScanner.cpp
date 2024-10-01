#include <fstream>
#include <iostream>
#include <string>

class OneAheadScanner {
private:
    std::ifstream file;
    char lookahead;

public:
    OneAheadScanner(const std::string& filename) : file(filename) {
        lookahead = '\0';
    }

    char nextChar() {
        if (lookahead != '\0') {
            char temp = lookahead;
            lookahead = '\0';
            return temp;
        }
        return file.get();
    }

    void pushBack(char c) {
        lookahead = c;
    }

    std::string nextToken() {
        std::string token;
        char c = nextChar();
        while (isalnum(c)) {
            token += c;
            c = nextChar();
        }
        pushBack(c); // Das letzte Zeichen zurücksetzen
        return token;
    }

    bool isEof() {
        return file.eof();
    }
};

int main() {
    OneAheadScanner scanner("example.txt");
    while (!scanner.isEof()) {
        std::cout << scanner.nextToken() << std::endl;
    }
    return 0;
}