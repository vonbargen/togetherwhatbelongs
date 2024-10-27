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
