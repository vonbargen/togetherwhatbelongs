#include <fmt/core.h>
#include <filesystem>
#include <fstream>
#include <string>

int main() {
    const std::filesystem::path filename = "./example/hello.2gth";
    const std::string targetContent = "hello.2gth";

    // Überprüfe, ob die Datei existiert
    if (!std::filesystem::exists(filename)) {
        fmt::print("Datei {} existiert nicht.\n", filename.string());
        return 1;
    }

    // Lese den Inhalt der Datei
    std::ifstream file(filename);
    if (!file.is_open()) {
        fmt::print("Fehler beim Öffnen der Datei {}\n", filename.string());
        return 1;
    }

    std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());

    // Überprüfe, ob der Inhalt die Zielinhalte enthält
    if (content.find(targetContent) != std::string::npos) {
        fmt::print("Datei {} enthält '{}'.\n", filename.string(), targetContent);
    } else {
        fmt::print("Datei {} enthält nicht '{}'.\n", filename.string(), targetContent);
    }

    return 0;
}