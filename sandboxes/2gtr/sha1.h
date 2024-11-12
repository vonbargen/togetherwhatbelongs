// sha1.h
#ifndef SHA1_H
#define SHA1_H

#include <string>
#include <openssl/evp.h>

class SHA1 {
public:
    static std::string hashFile(const std::string& filename);

private:
    static constexpr size_t BUFFER_SIZE = 8192;
};

#endif // SHA1_H
