//@ToDo Licence
#include <iostream>
#include <fstream>
#include <string>
#include <openssl/sha.h>

class GitSHAHasher {
public:
    std::string generateSHAHash(const std::string& filename) {
        // Öffne die Datei
        std::ifstream file(filename, std::ios::binary | std::ios::ate);
        if (!file.is_open()) {
            std::cerr << "Fehler beim Öffnen der Datei." << std::endl;
            return "";
        }

        // Ermittle die Größe der Datei
        std::size_t fileSize = file.tellg();
        file.seekg(0, std::ios::beg);

        // Erstelle einen SHA-1-Hasher
        unsigned char hash[SHA_DIGEST_LENGTH];
        SHA_CTX sha;
        SHA1_Init(&sha);

        // Füge die Git-spezifischen Informationen hinzu
        std::string header = "blob " + std::to_string(fileSize) + "\0";
        SHA1_Update(&sha, header.c_str(), header.size());

        // Lies die Datei in Blöcken und aktualisiere den Hash
        char buffer[4096];
        while (file.read(buffer, sizeof(buffer))) {
            SHA1_Update(&sha, buffer, sizeof(buffer));
        }
        // Verarbeite den Rest der Datei
        SHA1_Update(&sha, buffer, file.gcount());

        // Finalisiere den Hash
        SHA1_Final(hash, &sha);

        // Konvertiere den Hash in einen Hex-String
        std::string shaHash;
        for (int i = 0; i < SHA_DIGEST_LENGTH; ++i) {
            char hex[3];
            sprintf(hex, "%02x", hash[i]);
            shaHash += hex;
        }

        return shaHash;
    }
};

int main2() {
    GitSHAHasher hasher;
    std::string filename = "example.txt"; // Ersetze durch deine Datei
    std::string shaHash = hasher.generateSHAHash(filename);
    std::cout << "SHA-Hash: " << shaHash << std::endl;
    return 0;
}