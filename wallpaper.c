#include <stdio.h>
#include <windows.h>

int main(int argc, char **argv) {
	if (argc > 1) {
		if (strcmp(argv[1], "--version") == 0) {
			puts("1.0.0");
			exit(0);
		}

		if (strcmp(argv[1], "--help") == 0) {
			puts("\n  Get or set the desktop wallpaper\n\n  Usage: wallpaper [file]\n\n  Created by Sindre Sorhus");
			exit(0);
		}

		char fullPath[MAX_PATH];

		if (!_fullpath(fullPath, argv[1], MAX_PATH)) {
			fputs("Invalid path", stderr);
			exit(1);
		}

		if (!SystemParametersInfo(SPI_SETDESKWALLPAPER, 0, fullPath, SPIF_SENDCHANGE)) {
			fputs("Failed to set the desktop wallpaper", stderr);
			exit(1);
		}
	} else {
		char imagePath[MAX_PATH];

		if (SystemParametersInfo(SPI_GETDESKWALLPAPER, sizeof(imagePath) - 1, imagePath, 0)) {
			puts(imagePath);
		} else {
			fputs("Failed to get the desktop wallpaper", stderr);
			exit(1);
		}
	}

	return 0;
}
