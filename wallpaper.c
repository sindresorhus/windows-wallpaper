#include <stdio.h>
#include <windows.h>

int wmain(int argc, wchar_t **argv) {
	if (argc > 1) {
		if (wcscmp(argv[1], L"--version") == 0) {
			puts("1.0.0");
			return 0;
		}

		if (wcscmp(argv[1], L"--help") == 0) {
			puts("\n  Get or set the desktop wallpaper\n\n  Usage: wallpaper [file]\n\n  Created by Sindre Sorhus");
			return 0;
		}

		wchar_t fullPath[MAX_PATH];

		if (!_wfullpath(fullPath, argv[1], MAX_PATH)) {
			fputs("Invalid path", stderr);
			return 1;
		}

		if (!SystemParametersInfoW(SPI_SETDESKWALLPAPER, 0, fullPath, SPIF_SENDCHANGE)) {
			fputs("Failed to set the desktop wallpaper", stderr);
			return 1;
		}
	} else {
		wchar_t imagePath[MAX_PATH];

		if (SystemParametersInfoW(SPI_GETDESKWALLPAPER, MAX_PATH, imagePath, 0)) {
			DWORD written;
			WriteConsoleW(GetStdHandle(STD_OUTPUT_HANDLE), &imagePath[0], wcslen(&imagePath[0]), &written, 0);
		} else {
			fputs("Failed to get the desktop wallpaper", stderr);
			return 1;
		}
	}

	return 0;
}
