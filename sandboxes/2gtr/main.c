#include <stdio.h>
#include <unicode/ucnv.h>

int main() {
    // Initialize an ICU converter
    UErrorCode error = U_ZERO_ERROR;
    UConverter *conv = ucnv_open("utf-8", &error);
    if (U_FAILURE(error)) {
        printf("Failed to open converter: %s\n", u_errorName(error));
        return 1;
    }

    // Use the converter (example with no actual conversion)
    printf("ICU converter for UTF-8 opened successfully.\n");

    // Close the converter
    ucnv_close(conv);
    return 0;
}
