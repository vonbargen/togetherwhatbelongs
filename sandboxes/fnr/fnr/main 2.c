//
//  main.c
//  fnr
//
//  Created by Thorsten von Bargen on 24.10.24.
//

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <locale.h>
#include <wchar.h>

void replace_in_file(const char *filename, const char *search, const char *replace) {
    FILE *file, *temp;
    wchar_t *line = NULL;
    size_t len = 0;
    ssize_t read;
    char temp_filename[256];

    // UTF-8-Unterstützung aktivieren
    setlocale(LC_ALL, "en_US.UTF-8");

    // Originaldatei öffnen
    file = fopen(filename, "r, ccs=UTF-8");
    if (file == NULL) {
        perror("Error opening file");
        return;
    }

    // Temporäre Datei erstellen
    snprintf(temp_filename, sizeof(temp_filename), "%s.tmp", filename);
    temp = fopen(temp_filename, "w, ccs=UTF-8");
    if (temp == NULL) {
        perror("Error creating temporary file");
        fclose(file);
        return;
    }

    // Datei zeilenweise lesen
    while ((read = getline((char **)&line, &len, file)) != -1) {
        wchar_t *pos, *temp_line;
        size_t wide_len = mbstowcs(NULL, (char *)line, 0);
        wchar_t *wide_line = malloc((wide_len + 1) * sizeof(wchar_t));
        mbstowcs(wide_line, (char *)line, wide_len + 1);

        // Alle Vorkommen in der aktuellen Zeile ersetzen
        temp_line = wide_line;
        wchar_t wide_search[256], wide_replace[256];
        mbstowcs(wide_search, search, strlen(search) + 1);
        mbstowcs(wide_replace, replace, strlen(replace) + 1);

        while ((pos = wcsstr(temp_line, wide_search)) != NULL) {
            fwrite(temp_line, sizeof(wchar_t), pos - temp_line, temp);
            fputws(wide_replace, temp);
            temp_line = pos + wcslen(wide_search);
        }
        fputws(temp_line, temp);

        free(wide_line);
    }

    free(line);
    fclose(file);
    fclose(temp);

    remove(filename);
    rename(temp_filename, filename);
}

void process_directory(const char *dir_name, const char *search, const char *replace) {
    struct dirent *entry;
    DIR *dp = opendir(dir_name);

    if (dp == NULL) {
        perror("opendir");
        return;
    }

    while ((entry = readdir(dp))) {
        if (entry->d_type == DT_DIR) {
            if (strcmp(entry->d_name, ".") != 0 && strcmp(entry->d_name, "..") != 0) {
                char path[1024];
                snprintf(path, sizeof(path), "%s/%s", dir_name, entry->d_name);
                process_directory(path, search, replace);
            }
        } else if (strstr(entry->d_name, ".md")) {
            char filepath[1024];
            snprintf(filepath, sizeof(filepath), "%s/%s", dir_name, entry->d_name);
            replace_in_file(filepath, search, replace);
        }
    }

    closedir(dp);
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <search_string> <replace_string>\n", argv[0]);
        return EXIT_FAILURE;
    }

    const char *search = argv[1];
    const char *replace = argv[2];

    process_directory(".", search, replace);

    return EXIT_SUCCESS;
}

// UTF8

#include <stdint.h>

// Funktion zum Lesen eines UTF-8-Zeichens
uint32_t read_utf8_char(FILE *f) {
    uint32_t codepoint = 0;
    uint8_t byte;
    int len = 0;

    byte = fgetc(f);
    if (byte == EOF) return EOF;

    if ((byte & 0x80) == 0) {
        return byte;
    } else if ((byte & 0xE0) == 0xC0) {
        codepoint = byte & 0x1F;
        len = 1;
    } else if ((byte & 0xF0) == 0xE0) {
        codepoint = byte & 0x0F;
        len = 2;
    } else if ((byte & 0xF8) == 0xF0) {
        codepoint = byte & 0x07;
        len = 3;
    } else {
        return 0xFFFD; // Ungültiges UTF-8
    }

    for (int i = 0; i < len; i++) {
        byte = fgetc(f);
        if (byte == EOF || (byte & 0xC0) != 0x80) {
            return 0xFFFD; // Ungültiges UTF-8
        }
        codepoint = (codepoint << 6) | (byte & 0x3F);
    }

    return codepoint;
}

int main2() {
    FILE *file = fopen("utf8_file.txt", "rb");
    if (!file) {
        perror("Fehler beim Öffnen der Datei");
        return 1;
    }

    uint32_t ch;
    while ((ch = read_utf8_char(file)) != EOF) {
        printf("U+%04X ", ch);
    }

    fclose(file);
    return 0;
}

#include <unicode/ustring.h>
#include <unicode/uchar.h>
#include <unicode/utf8.h>

#define BUFFER_SIZE 1024

int main3() {
    FILE *file = fopen("utf8_file.txt", "rb");
    if (!file) {
        perror("Fehler beim Öffnen der Datei");
        return 1;
    }

    char buffer[BUFFER_SIZE];
    UChar32 c;
    const char *src;
    int32_t srcLen;

    while ((srcLen = fread(buffer, 1, BUFFER_SIZE, file)) > 0) {
        src = buffer;
        while (srcLen > 0) {
            U8_NEXT(src, srcLen, BUFFER_SIZE, c);
            if (c < 0) {
                printf("Ungültige UTF-8-Sequenz gefunden\n");
                continue;
            }
            printf("U+%04X ", c);
        }
    }

    fclose(file);
    return 0;
}